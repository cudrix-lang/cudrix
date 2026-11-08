//! Semantic analysis and name resolution for Cudrix.

use crate::source::{SourceSpan, SourceText};
use crate::syntax::{
    BlockSyntax, CallExpressionSyntax, CompilationUnitSyntax, ExpressionSyntax,
    FunctionDeclarationSyntax, FunctionParameterSyntax, StatementSyntax,
    VariableDeclarationStatementSyntax,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FunctionLookupError {
    InvalidIdentifierSpan(SourceSpan),
}

#[derive(Debug)]
pub enum TopLevelFunctionLookup<'syntax> {
    NotFound,
    Unique(&'syntax FunctionDeclarationSyntax),
    Multiple(Vec<&'syntax FunctionDeclarationSyntax>),
}

/// An error encountered while reading identifiers for parameter lookup.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParameterLookupError {
    InvalidIdentifierSpan(SourceSpan),
}

/// Parameter declarations matching the name of a discovered direct call.
///
/// Results describe matching declarations, not definitive lexical bindings.
/// Multiple matches are preserved in parameter declaration order.
#[derive(Debug)]
pub enum FunctionParameterLookup<'syntax> {
    NotFound,
    Unique(&'syntax FunctionParameterSyntax),
    Multiple(Vec<&'syntax FunctionParameterSyntax>),
}

/// An error encountered while reading identifiers for local variable lookup.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalVariableLookupError {
    InvalidIdentifierSpan(SourceSpan),
}

/// Local variable declarations matching the name of a discovered direct call.
///
/// Results describe matching declarations throughout the enclosing function,
/// not definitive lexical bindings or declarations visible at the call site.
///
/// Both `let` and `var` declarations are recognized. Multiple matches are
/// preserved in source order, including declarations in nested blocks.
#[derive(Debug)]
pub enum LocalVariableLookup<'syntax> {
    NotFound,
    Unique(&'syntax VariableDeclarationStatementSyntax),
    Multiple(Vec<&'syntax VariableDeclarationStatementSyntax>),
}

/// A local variable declaration structurally associated with a direct call.
///
/// The declaration belongs directly to one of the call's enclosing lexical
/// blocks. Both references point into the original syntax tree.
///
/// This candidate does not establish declaration visibility, shadowing
/// precedence, name binding, or callability.
#[derive(Debug, Clone, Copy)]
pub struct EnclosingBlockLocalVariableCandidate<'syntax> {
    declaration: &'syntax VariableDeclarationStatementSyntax,
    block: &'syntax BlockSyntax,
}

impl<'syntax> EnclosingBlockLocalVariableCandidate<'syntax> {
    pub fn declaration(&self) -> &'syntax VariableDeclarationStatementSyntax {
        self.declaration
    }

    pub fn block(&self) -> &'syntax BlockSyntax {
        self.block
    }
}

/// A direct named function call discovered within a function body.
///
/// Both references point into the original compilation unit syntax tree.
///
/// Discovery identifies syntactic calls without resolving their targets,
/// validating argument counts, or performing lexical scope analysis.
#[derive(Debug, Clone, Copy)]
pub struct DiscoveredDirectFunctionCall<'syntax> {
    function: &'syntax FunctionDeclarationSyntax,
    call: &'syntax CallExpressionSyntax,
}

impl<'syntax> DiscoveredDirectFunctionCall<'syntax> {
    pub fn function(&self) -> &'syntax FunctionDeclarationSyntax {
        self.function
    }

    pub fn call(&self) -> &'syntax CallExpressionSyntax {
        self.call
    }
}

/// Discovers direct named function calls in top-level function bodies.
///
/// Returns borrowed references to each call and its enclosing function.
///
/// Results are ordered by the UTF-8 byte offset of the callee identifier.
/// Calls with non-name callees are excluded, but their nested expressions
/// are still examined.
///
/// This operation does not perform name resolution, lexical scope analysis,
/// or source span validation.
pub fn discover_direct_function_calls<'syntax>(
    compilation_unit: &'syntax CompilationUnitSyntax,
) -> Vec<DiscoveredDirectFunctionCall<'syntax>> {
    let mut discovered_calls = Vec::new();

    for function in compilation_unit.function_declarations() {
        let mut enclosing_blocks = Vec::new();

        visit_direct_calls_in_block(
            function.body(),
            &mut enclosing_blocks,
            &mut |call, _blocks| {
                discovered_calls.push(DiscoveredDirectFunctionCall { function, call });
            },
        );
    }

    discovered_calls.sort_by_key(|discovered| {
        let ExpressionSyntax::Name(name) = discovered.call().callee() else {
            unreachable!("Discovered direct calls must have named callees");
        };

        name.identifier_token().span().start()
    });

    discovered_calls
}

/// Returns the enclosing block chain of a discovered direct function call.
///
/// Blocks are ordered from the enclosing function body to the innermost
/// block containing the call. The function body is always included.
///
/// The returned references point into the original syntax tree.
/// No syntax nodes are cloned or modified.
///
/// This operation recognizes structural block containment only.
/// It does not determine declaration visibility, shadowing, name binding,
/// or whether the call target is callable.
pub fn lookup_direct_call_enclosing_blocks<'syntax>(
    discovered: DiscoveredDirectFunctionCall<'syntax>,
) -> Vec<&'syntax BlockSyntax> {
    let mut enclosing_blocks = Vec::new();
    let mut matching_blocks = None;

    visit_direct_calls_in_block(
        discovered.function().body(),
        &mut enclosing_blocks,
        &mut |call, blocks| {
            if std::ptr::eq(call, discovered.call()) {
                matching_blocks = Some(blocks.to_vec());
            }
        },
    );

    matching_blocks.expect("A discovered direct call must belong to its enclosing function")
}

/// Visits direct named calls within a block and its nested blocks.
///
/// The enclosing block path is maintained from outermost to innermost.
/// Each visited call is reported with its current enclosing block path.
fn visit_direct_calls_in_block<'syntax>(
    block: &'syntax BlockSyntax,
    enclosing_blocks: &mut Vec<&'syntax BlockSyntax>,
    on_direct_call: &mut impl FnMut(&'syntax CallExpressionSyntax, &[&'syntax BlockSyntax]),
) {
    enclosing_blocks.push(block);

    for statement in block.statements() {
        match statement {
            StatementSyntax::Expression(statement) => {
                visit_direct_calls_in_expression(
                    statement.expression(),
                    enclosing_blocks,
                    on_direct_call,
                );
            }

            StatementSyntax::VariableDeclaration(statement) => {
                visit_direct_calls_in_expression(
                    statement.initializer(),
                    enclosing_blocks,
                    on_direct_call,
                );
            }

            StatementSyntax::Return(statement) => {
                visit_direct_calls_in_expression(
                    statement.expression(),
                    enclosing_blocks,
                    on_direct_call,
                );
            }

            StatementSyntax::Conditional(statement) => {
                visit_direct_calls_in_expression(
                    statement.condition(),
                    enclosing_blocks,
                    on_direct_call,
                );

                visit_direct_calls_in_block(
                    statement.then_block(),
                    enclosing_blocks,
                    on_direct_call,
                );

                for clause in statement.elseif_clauses() {
                    visit_direct_calls_in_expression(
                        clause.condition(),
                        enclosing_blocks,
                        on_direct_call,
                    );

                    visit_direct_calls_in_block(clause.block(), enclosing_blocks, on_direct_call);
                }

                if let Some(clause) = statement.else_clause() {
                    visit_direct_calls_in_block(clause.block(), enclosing_blocks, on_direct_call);
                }
            }

            StatementSyntax::While(statement) => {
                visit_direct_calls_in_expression(
                    statement.condition(),
                    enclosing_blocks,
                    on_direct_call,
                );

                visit_direct_calls_in_block(statement.block(), enclosing_blocks, on_direct_call);
            }

            StatementSyntax::For(statement) => {
                visit_direct_calls_in_expression(
                    statement.initializer(),
                    enclosing_blocks,
                    on_direct_call,
                );

                visit_direct_calls_in_expression(
                    statement.condition(),
                    enclosing_blocks,
                    on_direct_call,
                );

                visit_direct_calls_in_expression(
                    statement.update(),
                    enclosing_blocks,
                    on_direct_call,
                );

                visit_direct_calls_in_block(statement.block(), enclosing_blocks, on_direct_call);
            }
        }
    }

    enclosing_blocks
        .pop()
        .expect("The current block must exist on the enclosing block path");
}

/// Visits direct named calls within an expression and its nested expressions.
///
/// Expression traversal preserves the surrounding block path.
/// Nested expressions do not introduce new block contexts.
fn visit_direct_calls_in_expression<'syntax>(
    expression: &'syntax ExpressionSyntax,
    enclosing_blocks: &mut Vec<&'syntax BlockSyntax>,
    on_direct_call: &mut impl FnMut(&'syntax CallExpressionSyntax, &[&'syntax BlockSyntax]),
) {
    match expression {
        ExpressionSyntax::Literal(_) | ExpressionSyntax::Name(_) => {}

        ExpressionSyntax::Unary(unary) => {
            visit_direct_calls_in_expression(unary.operand(), enclosing_blocks, on_direct_call);
        }

        ExpressionSyntax::Binary(binary) => {
            visit_direct_calls_in_expression(binary.left(), enclosing_blocks, on_direct_call);

            visit_direct_calls_in_expression(binary.right(), enclosing_blocks, on_direct_call);
        }

        ExpressionSyntax::Conditional(conditional) => {
            visit_direct_calls_in_expression(
                conditional.condition(),
                enclosing_blocks,
                on_direct_call,
            );

            visit_direct_calls_in_expression(
                conditional.when_true(),
                enclosing_blocks,
                on_direct_call,
            );

            visit_direct_calls_in_expression(
                conditional.when_false(),
                enclosing_blocks,
                on_direct_call,
            );
        }

        ExpressionSyntax::Assignment(assignment) => {
            visit_direct_calls_in_expression(
                assignment.expression(),
                enclosing_blocks,
                on_direct_call,
            );
        }

        ExpressionSyntax::MemberAccess(member) => {
            visit_direct_calls_in_expression(member.expression(), enclosing_blocks, on_direct_call);
        }

        ExpressionSyntax::Index(index) => {
            visit_direct_calls_in_expression(index.expression(), enclosing_blocks, on_direct_call);

            visit_direct_calls_in_expression(index.index(), enclosing_blocks, on_direct_call);
        }

        ExpressionSyntax::ArrayLiteral(array) => {
            for element in array.elements() {
                visit_direct_calls_in_expression(element, enclosing_blocks, on_direct_call);
            }
        }

        ExpressionSyntax::ObjectLiteral(object) => {
            for member in object.members() {
                visit_direct_calls_in_expression(member.value(), enclosing_blocks, on_direct_call);
            }
        }

        ExpressionSyntax::Call(call) => {
            if matches!(call.callee(), ExpressionSyntax::Name(_)) {
                on_direct_call(call, enclosing_blocks);
            }

            visit_direct_calls_in_expression(call.callee(), enclosing_blocks, on_direct_call);

            for argument in call.arguments() {
                visit_direct_calls_in_expression(argument, enclosing_blocks, on_direct_call);
            }
        }

        ExpressionSyntax::Parenthesized(parenthesized) => {
            visit_direct_calls_in_expression(
                parenthesized.expression(),
                enclosing_blocks,
                on_direct_call,
            );
        }
    }
}

/// Looks up top-level function declarations by name.
///
/// The supplied `source` must be the same source text used to parse
/// `compilation_unit`.
///
/// Returns all matching declarations without determining whether duplicate
/// function names are permitted.
///
/// An invalid identifier span produces an error. Providing unrelated source
/// text with otherwise valid spans cannot be detected reliably.
///
/// This operation does not resolve local names, imported functions,
/// or function-call expressions.
pub fn lookup_top_level_function<'syntax>(
    compilation_unit: &'syntax CompilationUnitSyntax,
    source: &SourceText,
    name: &str,
) -> Result<TopLevelFunctionLookup<'syntax>, FunctionLookupError> {
    let mut first_match = None;
    let mut multiple_matches = Vec::new();

    for declaration in compilation_unit.function_declarations() {
        let identifier_span = declaration.identifier_token().span();

        let identifier_text = source
            .slice(&identifier_span)
            .ok_or(FunctionLookupError::InvalidIdentifierSpan(identifier_span))?;

        if identifier_text != name {
            continue;
        }

        match first_match {
            None => first_match = Some(declaration),
            Some(first) => {
                if multiple_matches.is_empty() {
                    multiple_matches.push(first);
                }

                multiple_matches.push(declaration);
            }
        }
    }

    if !multiple_matches.is_empty() {
        return Ok(TopLevelFunctionLookup::Multiple(multiple_matches));
    }

    match first_match {
        Some(declaration) => Ok(TopLevelFunctionLookup::Unique(declaration)),
        None => Ok(TopLevelFunctionLookup::NotFound),
    }
}

/// Looks up top-level function declaration candidates for a direct named call.
///
/// The supplied `source` must be the same source text used to parse
/// `compilation_unit` and `call`.
///
/// Returns `Ok(None)` when the call's callee is not directly a name expression.
///
/// For direct named calls, returns the existing top-level function lookup
/// result without resolving parameters, local variables, or shadowing.
///
/// An invalid call identifier span produces an error. Invalid declaration
/// spans are reported by the existing top-level function lookup.
///
/// This operation does not establish that a matching declaration is the
/// actual target of the call.
pub fn lookup_direct_call_top_level_functions<'syntax>(
    compilation_unit: &'syntax CompilationUnitSyntax,
    source: &SourceText,
    call: &CallExpressionSyntax,
) -> Result<Option<TopLevelFunctionLookup<'syntax>>, FunctionLookupError> {
    let ExpressionSyntax::Name(name_expression) = call.callee() else {
        return Ok(None);
    };

    let identifier_span = name_expression.identifier_token().span();

    let identifier_text = source
        .slice(&identifier_span)
        .ok_or(FunctionLookupError::InvalidIdentifierSpan(identifier_span))?;

    lookup_top_level_function(compilation_unit, source, identifier_text).map(Some)
}

/// Finds parameters of the enclosing function matching a discovered call name.
///
/// The supplied `source` must be the same source text used to parse
/// the syntax tree containing the discovered call.
///
/// Returns all matching parameters in declaration order without deciding
/// whether duplicate parameter names are permitted.
///
/// An invalid call or parameter identifier span produces an error.
/// Providing unrelated source text with otherwise valid spans cannot
/// be detected reliably.
///
/// This operation does not resolve local variables, establish lexical
/// bindings, or determine whether a matching parameter is callable.
pub fn lookup_direct_call_enclosing_function_parameters<'syntax>(
    discovered: DiscoveredDirectFunctionCall<'syntax>,
    source: &SourceText,
) -> Result<FunctionParameterLookup<'syntax>, ParameterLookupError> {
    let ExpressionSyntax::Name(name_expression) = discovered.call().callee() else {
        unreachable!("Discovered direct calls must have named callees");
    };

    let call_identifier_span = name_expression.identifier_token().span();

    let call_identifier_text =
        source
            .slice(&call_identifier_span)
            .ok_or(ParameterLookupError::InvalidIdentifierSpan(
                call_identifier_span,
            ))?;

    let mut first_match = None;
    let mut multiple_matches = Vec::new();

    for parameter in discovered.function().parameters() {
        let parameter_identifier_span = parameter.identifier_token().span();

        let parameter_identifier_text = source.slice(&parameter_identifier_span).ok_or(
            ParameterLookupError::InvalidIdentifierSpan(parameter_identifier_span),
        )?;

        if parameter_identifier_text != call_identifier_text {
            continue;
        }

        match first_match {
            None => first_match = Some(parameter),
            Some(first) => {
                if multiple_matches.is_empty() {
                    multiple_matches.push(first);
                }

                multiple_matches.push(parameter);
            }
        }
    }

    if !multiple_matches.is_empty() {
        return Ok(FunctionParameterLookup::Multiple(multiple_matches));
    }

    match first_match {
        Some(parameter) => Ok(FunctionParameterLookup::Unique(parameter)),
        None => Ok(FunctionParameterLookup::NotFound),
    }
}

/// Finds local variable declarations in the enclosing function that match
/// the name of a discovered direct call.
///
/// The supplied `source` must be the same source text used to parse the
/// syntax tree containing the discovered call.
///
/// Both `let` and `var` declarations are recognized throughout the enclosing
/// function, including nested blocks and declarations appearing after the call.
///
/// Returns all matching declarations in source order without determining
/// lexical visibility, shadowing, binding, or callability.
///
/// An invalid call or local declaration identifier span produces an error.
/// Providing unrelated source text with otherwise valid spans cannot
/// be detected reliably.
pub fn lookup_direct_call_enclosing_function_local_variables<'syntax>(
    discovered: DiscoveredDirectFunctionCall<'syntax>,
    source: &SourceText,
) -> Result<LocalVariableLookup<'syntax>, LocalVariableLookupError> {
    let ExpressionSyntax::Name(name_expression) = discovered.call().callee() else {
        unreachable!("Discovered direct calls must have named callees");
    };

    let call_identifier_span = name_expression.identifier_token().span();

    let call_identifier_text = source.slice(&call_identifier_span).ok_or(
        LocalVariableLookupError::InvalidIdentifierSpan(call_identifier_span),
    )?;

    let mut matching_declarations = Vec::new();

    collect_matching_local_variables_in_block(
        discovered.function().body(),
        source,
        call_identifier_text,
        &mut matching_declarations,
    )?;

    match matching_declarations.len() {
        0 => Ok(LocalVariableLookup::NotFound),
        1 => Ok(LocalVariableLookup::Unique(matching_declarations[0])),
        _ => Ok(LocalVariableLookup::Multiple(matching_declarations)),
    }
}

/// Collects matching local declarations from a block and its nested blocks.
fn collect_matching_local_variables_in_block<'syntax>(
    block: &'syntax BlockSyntax,
    source: &SourceText,
    name: &str,
    matching_declarations: &mut Vec<&'syntax VariableDeclarationStatementSyntax>,
) -> Result<(), LocalVariableLookupError> {
    for statement in block.statements() {
        match statement {
            StatementSyntax::VariableDeclaration(declaration) => {
                let identifier_span = declaration.identifier_token().span();

                let identifier_text = source.slice(&identifier_span).ok_or(
                    LocalVariableLookupError::InvalidIdentifierSpan(identifier_span),
                )?;

                if identifier_text == name {
                    matching_declarations.push(declaration);
                }
            }

            StatementSyntax::Conditional(conditional) => {
                collect_matching_local_variables_in_block(
                    conditional.then_block(),
                    source,
                    name,
                    matching_declarations,
                )?;

                for clause in conditional.elseif_clauses() {
                    collect_matching_local_variables_in_block(
                        clause.block(),
                        source,
                        name,
                        matching_declarations,
                    )?;
                }

                if let Some(clause) = conditional.else_clause() {
                    collect_matching_local_variables_in_block(
                        clause.block(),
                        source,
                        name,
                        matching_declarations,
                    )?;
                }
            }

            StatementSyntax::While(while_statement) => {
                collect_matching_local_variables_in_block(
                    while_statement.block(),
                    source,
                    name,
                    matching_declarations,
                )?;
            }

            StatementSyntax::For(for_statement) => {
                collect_matching_local_variables_in_block(
                    for_statement.block(),
                    source,
                    name,
                    matching_declarations,
                )?;
            }

            StatementSyntax::Expression(_) | StatementSyntax::Return(_) => {}
        }
    }

    Ok(())
}

/// Finds local variable declaration candidates in the enclosing lexical
/// blocks of a discovered direct function call.
///
/// The supplied `source` must be the same source text used to parse the
/// syntax tree containing the discovered call.
///
/// Only declarations directly contained in the call's enclosing blocks
/// are considered. Declarations in sibling or descendant blocks are excluded.
///
/// Both `let` and `var` declarations are recognized. All matching
/// declarations are returned in source order, including declarations
/// appearing after the call.
///
/// Each candidate retains references to its original declaration and
/// immediately containing block.
///
/// This operation does not determine declaration visibility, shadowing
/// precedence, name binding, or callability.
///
/// An invalid call or examined declaration identifier span produces an error.
/// Providing unrelated source text with otherwise valid spans cannot
/// be detected reliably.
pub fn lookup_direct_call_enclosing_block_local_variable_candidates<'syntax>(
    discovered: DiscoveredDirectFunctionCall<'syntax>,
    source: &SourceText,
) -> Result<Vec<EnclosingBlockLocalVariableCandidate<'syntax>>, LocalVariableLookupError> {
    let ExpressionSyntax::Name(name_expression) = discovered.call().callee() else {
        unreachable!("Discovered direct calls must have named callees");
    };

    let call_identifier_span = name_expression.identifier_token().span();

    let call_identifier_text = source.slice(&call_identifier_span).ok_or(
        LocalVariableLookupError::InvalidIdentifierSpan(call_identifier_span),
    )?;

    let enclosing_blocks = lookup_direct_call_enclosing_blocks(discovered);
    let mut candidates = Vec::new();

    for block in enclosing_blocks {
        collect_matching_local_variable_candidates_in_block(
            block,
            source,
            call_identifier_text,
            &mut candidates,
        )?;
    }

    candidates.sort_by_key(|candidate| candidate.declaration().identifier_token().span().start());

    Ok(candidates)
}

/// Collects matching declarations directly contained in one lexical block.
///
/// Nested blocks are not traversed. Each matching declaration is associated
/// with the block that directly contains it.
fn collect_matching_local_variable_candidates_in_block<'syntax>(
    block: &'syntax BlockSyntax,
    source: &SourceText,
    name: &str,
    candidates: &mut Vec<EnclosingBlockLocalVariableCandidate<'syntax>>,
) -> Result<(), LocalVariableLookupError> {
    for statement in block.statements() {
        let StatementSyntax::VariableDeclaration(declaration) = statement else {
            continue;
        };

        let identifier_span = declaration.identifier_token().span();

        let identifier_text = source.slice(&identifier_span).ok_or(
            LocalVariableLookupError::InvalidIdentifierSpan(identifier_span),
        )?;

        if identifier_text == name {
            candidates.push(EnclosingBlockLocalVariableCandidate { declaration, block });
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::Parser;
    use crate::syntax::StatementSyntax;

    #[test]
    fn scope_aware_local_variable_candidates_preserve_block_identity_and_source_order() {
        let source = SourceText::new(
            "function main() {
            if true {
                let target = 20;
                target();
            }

            let target = 10;
        }",
        );

        let compilation_unit = Parser::new(&source)
            .parse_compilation_unit()
            .expect("Compilation unit should parse");

        let discovered = discover_direct_function_calls(&compilation_unit);

        assert_eq!(discovered.len(), 1);

        let function_body = compilation_unit.function_declarations()[0].body();

        let StatementSyntax::Conditional(conditional) = &function_body.statements()[0] else {
            panic!("Expected conditional statement");
        };

        let inner_block = conditional.then_block();

        let StatementSyntax::VariableDeclaration(inner_declaration) = &inner_block.statements()[0]
        else {
            panic!("Expected inner local declaration");
        };

        let StatementSyntax::VariableDeclaration(outer_declaration) =
            &function_body.statements()[1]
        else {
            panic!("Expected outer local declaration");
        };

        let candidates =
            lookup_direct_call_enclosing_block_local_variable_candidates(discovered[0], &source)
                .expect("Scope-aware candidate lookup should succeed");

        assert_eq!(candidates.len(), 2);

        // The inner declaration occurs first in source order.
        assert!(std::ptr::eq(candidates[0].declaration(), inner_declaration));
        assert!(std::ptr::eq(candidates[0].block(), inner_block));

        // The outer declaration appears later in the source.
        assert!(std::ptr::eq(candidates[1].declaration(), outer_declaration));
        assert!(std::ptr::eq(candidates[1].block(), function_body));

        // Candidate ordering follows source position, not block depth.
        assert!(
            candidates[0]
                .declaration()
                .identifier_token()
                .span()
                .start()
                < candidates[1]
                    .declaration()
                    .identifier_token()
                    .span()
                    .start()
        );
    }

    #[test]
    fn scope_aware_local_variable_candidates_exclude_sibling_and_descendant_blocks() {
        let source = SourceText::new(
            "function main() {
                if true {
                    let target = 10;
                } else {
                    target();
                }

                target();
            }",
        );

        let compilation_unit = Parser::new(&source)
            .parse_compilation_unit()
            .expect("Compilation unit should parse");

        let discovered = discover_direct_function_calls(&compilation_unit);

        assert_eq!(discovered.len(), 2);

        let function_body = compilation_unit.function_declarations()[0].body();

        let StatementSyntax::Conditional(conditional) = &function_body.statements()[0] else {
            panic!("Expected conditional statement");
        };

        let then_block = conditional.then_block();

        let else_block = conditional
            .else_clause()
            .expect("Expected else clause")
            .block();

        let StatementSyntax::VariableDeclaration(declaration) = &then_block.statements()[0] else {
            panic!("Expected local declaration in if block");
        };

        // The first call belongs to the sibling else block.
        let else_call_blocks = lookup_direct_call_enclosing_blocks(discovered[0]);

        assert_eq!(else_call_blocks.len(), 2);
        assert!(std::ptr::eq(else_call_blocks[0], function_body));
        assert!(std::ptr::eq(else_call_blocks[1], else_block));

        // The second call belongs directly to the function body.
        let outer_call_blocks = lookup_direct_call_enclosing_blocks(discovered[1]);

        assert_eq!(outer_call_blocks.len(), 1);
        assert!(std::ptr::eq(outer_call_blocks[0], function_body));

        for discovered_call in discovered {
            // Neither call has a structurally eligible local declaration.
            let candidates = lookup_direct_call_enclosing_block_local_variable_candidates(
                discovered_call,
                &source,
            )
            .expect("Scope-aware lookup should succeed");

            assert!(candidates.is_empty());

            // The existing function-wide lookup must remain unchanged.
            let function_wide_result =
                lookup_direct_call_enclosing_function_local_variables(discovered_call, &source)
                    .expect("Function-wide lookup should succeed");

            let LocalVariableLookup::Unique(matched_declaration) = function_wide_result else {
                panic!("Expected function-wide matching declaration");
            };

            assert!(std::ptr::eq(matched_declaration, declaration));
        }
    }

    #[test]
    fn scope_aware_local_variable_candidates_distinguish_conditions_and_loop_bodies() {
        let source = SourceText::new(
            "function main() {
                let outer = 1;

                if inner_if() {
                    var inner_if = 2;
                    outer();
                    inner_if();
                }

                while inner_while() {
                    let inner_while = 3;
                    outer();
                    inner_while();
                }
            }",
        );

        let compilation_unit = Parser::new(&source)
            .parse_compilation_unit()
            .expect("Compilation unit should parse");

        let discovered = discover_direct_function_calls(&compilation_unit);

        assert_eq!(discovered.len(), 6);

        let function_body = compilation_unit.function_declarations()[0].body();
        let statements = function_body.statements();

        let StatementSyntax::VariableDeclaration(outer_declaration) = &statements[0] else {
            panic!("Expected outer declaration");
        };

        let StatementSyntax::Conditional(conditional) = &statements[1] else {
            panic!("Expected conditional statement");
        };

        let if_block = conditional.then_block();

        let StatementSyntax::VariableDeclaration(if_declaration) = &if_block.statements()[0] else {
            panic!("Expected if-block declaration");
        };

        let StatementSyntax::While(while_statement) = &statements[2] else {
            panic!("Expected while statement");
        };

        let while_block = while_statement.block();

        let StatementSyntax::VariableDeclaration(while_declaration) = &while_block.statements()[0]
        else {
            panic!("Expected while-block declaration");
        };

        // Calls in conditions belong to the surrounding function body.
        // Declarations inside the associated bodies are not candidates.
        for call_index in [0, 3] {
            let blocks = lookup_direct_call_enclosing_blocks(discovered[call_index]);

            assert_eq!(blocks.len(), 1);
            assert!(std::ptr::eq(blocks[0], function_body));

            let candidates = lookup_direct_call_enclosing_block_local_variable_candidates(
                discovered[call_index],
                &source,
            )
            .expect("Condition candidate lookup should succeed");

            assert!(candidates.is_empty());
        }

        // Calls inside the bodies recognize declarations from their
        // own blocks or their enclosing function body.
        for (call_index, expected_declaration, expected_block) in [
            (1, outer_declaration, function_body),
            (2, if_declaration, if_block),
            (4, outer_declaration, function_body),
            (5, while_declaration, while_block),
        ] {
            let candidates = lookup_direct_call_enclosing_block_local_variable_candidates(
                discovered[call_index],
                &source,
            )
            .expect("Body candidate lookup should succeed");

            assert_eq!(candidates.len(), 1);

            assert!(std::ptr::eq(
                candidates[0].declaration(),
                expected_declaration
            ));

            assert!(std::ptr::eq(candidates[0].block(), expected_block));
        }
    }

    #[test]
    fn scope_aware_local_variable_candidates_distinguish_for_headers_and_body() {
        let source = SourceText::new(
            "function main() {
                let outer = 1;

                for outer(); body_let(); body_var() {
                    let body_let = 2;
                    var body_var = 3;

                    outer();
                    body_let();
                    body_var();
                }
            }",
        );

        let compilation_unit = Parser::new(&source)
            .parse_compilation_unit()
            .expect("Compilation unit should parse");

        let discovered = discover_direct_function_calls(&compilation_unit);

        assert_eq!(discovered.len(), 6);

        let function_body = compilation_unit.function_declarations()[0].body();
        let statements = function_body.statements();

        let StatementSyntax::VariableDeclaration(outer_declaration) = &statements[0] else {
            panic!("Expected outer declaration");
        };

        let StatementSyntax::For(for_statement) = &statements[1] else {
            panic!("Expected for statement");
        };

        let for_body = for_statement.block();

        let StatementSyntax::VariableDeclaration(body_let_declaration) = &for_body.statements()[0]
        else {
            panic!("Expected let declaration in for body");
        };

        let StatementSyntax::VariableDeclaration(body_var_declaration) = &for_body.statements()[1]
        else {
            panic!("Expected var declaration in for body");
        };

        // All three header expressions belong to the function body.
        for discovered_call in discovered.iter().take(3) {
            let blocks = lookup_direct_call_enclosing_blocks(*discovered_call);

            assert_eq!(blocks.len(), 1);
            assert!(std::ptr::eq(blocks[0], function_body));
        }

        // All three body calls belong to the nested for block.
        for discovered_call in discovered.iter().skip(3) {
            let blocks = lookup_direct_call_enclosing_blocks(*discovered_call);

            assert_eq!(blocks.len(), 2);
            assert!(std::ptr::eq(blocks[0], function_body));
            assert!(std::ptr::eq(blocks[1], for_body));
        }

        // Declarations inside the for body are excluded from header calls.
        for call_index in [1, 2] {
            let candidates = lookup_direct_call_enclosing_block_local_variable_candidates(
                discovered[call_index],
                &source,
            )
            .expect("For-header candidate lookup should succeed");

            assert!(candidates.is_empty());
        }

        // The outer declaration is available as a structural candidate
        // in both the for initializer and the for body.
        // Body declarations are candidates only for calls in the body.
        for (call_index, expected_declaration, expected_block) in [
            (0, outer_declaration, function_body),
            (3, outer_declaration, function_body),
            (4, body_let_declaration, for_body),
            (5, body_var_declaration, for_body),
        ] {
            let candidates = lookup_direct_call_enclosing_block_local_variable_candidates(
                discovered[call_index],
                &source,
            )
            .expect("For-loop candidate lookup should succeed");

            assert_eq!(candidates.len(), 1);

            assert!(std::ptr::eq(
                candidates[0].declaration(),
                expected_declaration
            ));

            assert!(std::ptr::eq(candidates[0].block(), expected_block));
        }
    }

    #[test]
    fn scope_aware_local_variable_candidates_are_deterministic_across_repeated_lookups() {
        let source = SourceText::new(
            "function main() {
                let target = 10;

                if ready() {
                    target();
                }
            }",
        );

        let compilation_unit = Parser::new(&source)
            .parse_compilation_unit()
            .expect("Compilation unit should parse");

        let discovered = discover_direct_function_calls(&compilation_unit);

        assert_eq!(discovered.len(), 2);

        let function_body = compilation_unit.function_declarations()[0].body();

        let StatementSyntax::VariableDeclaration(declaration) = &function_body.statements()[0]
        else {
            panic!("Expected outer local declaration");
        };

        let first =
            lookup_direct_call_enclosing_block_local_variable_candidates(discovered[1], &source)
                .expect("First candidate lookup should succeed");

        let second =
            lookup_direct_call_enclosing_block_local_variable_candidates(discovered[1], &source)
                .expect("Second candidate lookup should succeed");

        assert_eq!(first.len(), 1);
        assert_eq!(second.len(), 1);

        // Both operations must return the original declaration.
        assert!(std::ptr::eq(first[0].declaration(), declaration));
        assert!(std::ptr::eq(second[0].declaration(), declaration));

        // Both operations must identify the original containing block.
        assert!(std::ptr::eq(first[0].block(), function_body));
        assert!(std::ptr::eq(second[0].block(), function_body));

        // Repeated operations preserve the same syntax-node identities.
        assert!(std::ptr::eq(
            first[0].declaration(),
            second[0].declaration()
        ));

        assert!(std::ptr::eq(first[0].block(), second[0].block()));
    }

    #[test]
    fn scope_aware_local_variable_candidates_report_invalid_identifier_spans() {
        let source = SourceText::new(
            "function main() {
                target();
                let target = 10;
            }",
        );

        let compilation_unit = Parser::new(&source)
            .parse_compilation_unit()
            .expect("Compilation unit should parse");

        let discovered = discover_direct_function_calls(&compilation_unit);

        assert_eq!(discovered.len(), 1);

        let ExpressionSyntax::Name(name_expression) = discovered[0].call().callee() else {
            panic!("Expected direct named call");
        };

        let call_identifier_span = name_expression.identifier_token().span();

        let function_body = compilation_unit.function_declarations()[0].body();

        let StatementSyntax::VariableDeclaration(declaration) = &function_body.statements()[1]
        else {
            panic!("Expected local variable declaration");
        };

        let declaration_identifier_span = declaration.identifier_token().span();

        // An empty source makes the call identifier span invalid.
        let empty_source = SourceText::new("");

        let call_result = lookup_direct_call_enclosing_block_local_variable_candidates(
            discovered[0],
            &empty_source,
        );

        assert!(matches!(
            call_result,
            Err(LocalVariableLookupError::InvalidIdentifierSpan(span))
                if span == call_identifier_span
        ));

        // Retain the call identifier but truncate the source before
        // the declaration identifier.
        let truncated_source =
            SourceText::new(&source.as_str()[..declaration_identifier_span.start()]);

        let declaration_result = lookup_direct_call_enclosing_block_local_variable_candidates(
            discovered[0],
            &truncated_source,
        );

        assert!(matches!(
            declaration_result,
            Err(LocalVariableLookupError::InvalidIdentifierSpan(span))
                if span == declaration_identifier_span
        ));
    }

    #[test]
    fn direct_call_enclosing_blocks_include_function_and_nested_blocks() {
        let source = crate::source::SourceText::new(
            "function main() {
                outer();

                if true {
                    inner();
                }
            }",
        );

        let compilation_unit = crate::compiler::Parser::new(&source)
            .parse_compilation_unit()
            .expect("Compilation unit should parse");

        let discovered = discover_direct_function_calls(&compilation_unit);

        assert_eq!(discovered.len(), 2);

        let function_body = compilation_unit.function_declarations()[0].body();

        let StatementSyntax::Conditional(conditional) = &function_body.statements()[1] else {
            panic!("Expected conditional statement");
        };

        let outer_blocks = lookup_direct_call_enclosing_blocks(discovered[0]);

        assert_eq!(outer_blocks.len(), 1);
        assert!(std::ptr::eq(outer_blocks[0], function_body));

        let inner_blocks = lookup_direct_call_enclosing_blocks(discovered[1]);

        assert_eq!(inner_blocks.len(), 2);
        assert!(std::ptr::eq(inner_blocks[0], function_body));
        assert!(std::ptr::eq(inner_blocks[1], conditional.then_block()));
    }

    #[test]
    fn direct_call_enclosing_blocks_distinguish_conditional_branches() {
        let source = crate::source::SourceText::new(
            "function main() {
            if first_condition() {
                first_branch();
            } elseif second_condition() {
                second_branch();
            } else {
                fallback();
            }
        }",
        );

        let compilation_unit = crate::compiler::Parser::new(&source)
            .parse_compilation_unit()
            .expect("Compilation unit should parse");

        let discovered = discover_direct_function_calls(&compilation_unit);

        assert_eq!(discovered.len(), 5);

        let function_body = compilation_unit.function_declarations()[0].body();

        let StatementSyntax::Conditional(conditional) = &function_body.statements()[0] else {
            panic!("Expected conditional statement");
        };

        let if_block = conditional.then_block();

        let elseif_block = conditional.elseif_clauses()[0].block();

        let else_block = conditional
            .else_clause()
            .expect("Expected else clause")
            .block();

        // Conditional expressions belong to the enclosing function body.
        for index in [0, 2] {
            let blocks = lookup_direct_call_enclosing_blocks(discovered[index]);

            assert_eq!(blocks.len(), 1);
            assert!(std::ptr::eq(blocks[0], function_body));
        }

        // Each conditional branch has its own enclosing block.
        for (index, expected_block) in [(1, if_block), (3, elseif_block), (4, else_block)] {
            let blocks = lookup_direct_call_enclosing_blocks(discovered[index]);

            assert_eq!(blocks.len(), 2);
            assert!(std::ptr::eq(blocks[0], function_body));
            assert!(std::ptr::eq(blocks[1], expected_block));
        }

        // Sibling branches must not share the same block identity.
        assert!(!std::ptr::eq(if_block, elseif_block));
        assert!(!std::ptr::eq(if_block, else_block));
        assert!(!std::ptr::eq(elseif_block, else_block));
    }

    #[test]
    fn direct_call_enclosing_blocks_distinguish_loop_headers_and_bodies() {
        let source = crate::source::SourceText::new(
            "function main() {
            while while_condition() {
                while_body();
            }

            for initialize(); loop_condition(); advance() {
                for_body();
            }
        }",
        );

        let compilation_unit = crate::compiler::Parser::new(&source)
            .parse_compilation_unit()
            .expect("Compilation unit should parse");

        let discovered = discover_direct_function_calls(&compilation_unit);

        assert_eq!(discovered.len(), 6);

        let function_body = compilation_unit.function_declarations()[0].body();

        let StatementSyntax::While(while_statement) = &function_body.statements()[0] else {
            panic!("Expected while statement");
        };

        let StatementSyntax::For(for_statement) = &function_body.statements()[1] else {
            panic!("Expected for statement");
        };

        let while_body = while_statement.block();
        let for_body = for_statement.block();

        // Loop conditions and for-header expressions belong to
        // the surrounding function-body block.
        for index in [0, 2, 3, 4] {
            let blocks = lookup_direct_call_enclosing_blocks(discovered[index]);

            assert_eq!(blocks.len(), 1);
            assert!(std::ptr::eq(blocks[0], function_body));
        }

        // The while-body call has two enclosing blocks.
        let while_blocks = lookup_direct_call_enclosing_blocks(discovered[1]);

        assert_eq!(while_blocks.len(), 2);
        assert!(std::ptr::eq(while_blocks[0], function_body));
        assert!(std::ptr::eq(while_blocks[1], while_body));

        // The for-body call also has two enclosing blocks.
        let for_blocks = lookup_direct_call_enclosing_blocks(discovered[5]);

        assert_eq!(for_blocks.len(), 2);
        assert!(std::ptr::eq(for_blocks[0], function_body));
        assert!(std::ptr::eq(for_blocks[1], for_body));

        // Separate loops must retain distinct body identities.
        assert!(!std::ptr::eq(while_body, for_body));
    }

    #[test]
    fn direct_call_enclosing_blocks_track_nested_expressions_and_deep_blocks() {
        let source = crate::source::SourceText::new(
            "function main() {
            let result = calculate(seed());

            if check(result) {
                while running() {
                    process(result);
                    report();
                }
            }
        }",
        );

        let compilation_unit = crate::compiler::Parser::new(&source)
            .parse_compilation_unit()
            .expect("Compilation unit should parse");

        let discovered = discover_direct_function_calls(&compilation_unit);

        assert_eq!(discovered.len(), 6);

        let function_body = compilation_unit.function_declarations()[0].body();

        let StatementSyntax::Conditional(conditional) = &function_body.statements()[1] else {
            panic!("Expected conditional statement");
        };

        let if_block = conditional.then_block();

        let StatementSyntax::While(while_statement) = &if_block.statements()[0] else {
            panic!("Expected nested while statement");
        };

        let while_block = while_statement.block();

        // Calls inside the variable initializer and the if condition
        // belong directly to the function body.
        for index in [0, 1, 2] {
            let blocks = lookup_direct_call_enclosing_blocks(discovered[index]);

            assert_eq!(blocks.len(), 1);
            assert!(std::ptr::eq(blocks[0], function_body));
        }

        // The while condition is evaluated in the enclosing if block.
        let condition_blocks = lookup_direct_call_enclosing_blocks(discovered[3]);

        assert_eq!(condition_blocks.len(), 2);
        assert!(std::ptr::eq(condition_blocks[0], function_body));
        assert!(std::ptr::eq(condition_blocks[1], if_block));

        // Both calls in the while body share the same three-block chain.
        for index in [4, 5] {
            let blocks = lookup_direct_call_enclosing_blocks(discovered[index]);

            assert_eq!(blocks.len(), 3);
            assert!(std::ptr::eq(blocks[0], function_body));
            assert!(std::ptr::eq(blocks[1], if_block));
            assert!(std::ptr::eq(blocks[2], while_block));
        }

        // Repeated lookups must return the same syntax-node identities
        // in the same order.
        let first = lookup_direct_call_enclosing_blocks(discovered[4]);
        let second = lookup_direct_call_enclosing_blocks(discovered[4]);

        assert_eq!(first.len(), second.len());

        for (first_block, second_block) in first.iter().zip(second.iter()) {
            assert!(std::ptr::eq(*first_block, *second_block));
        }
    }

    #[test]
    fn direct_call_enclosing_blocks_remain_isolated_between_functions() {
        let source = crate::source::SourceText::new(
            "function first() {
            shared();

            if true {
                nested();
            }
        }

        function second() {
            shared();

            if true {
                nested();
            }
        }",
        );

        let compilation_unit = crate::compiler::Parser::new(&source)
            .parse_compilation_unit()
            .expect("Compilation unit should parse");

        let discovered = discover_direct_function_calls(&compilation_unit);
        let functions = compilation_unit.function_declarations();

        assert_eq!(functions.len(), 2);
        assert_eq!(discovered.len(), 4);

        let first_function = &functions[0];
        let second_function = &functions[1];

        let first_body = first_function.body();
        let second_body = second_function.body();

        let StatementSyntax::Conditional(first_conditional) = &first_body.statements()[1] else {
            panic!("Expected conditional statement in first function");
        };

        let StatementSyntax::Conditional(second_conditional) = &second_body.statements()[1] else {
            panic!("Expected conditional statement in second function");
        };

        let first_if_block = first_conditional.then_block();
        let second_if_block = second_conditional.then_block();

        // Each discovered call must retain its original function identity.
        for discovered_call in &discovered[..2] {
            assert!(std::ptr::eq(discovered_call.function(), first_function));
        }

        for discovered_call in &discovered[2..] {
            assert!(std::ptr::eq(discovered_call.function(), second_function));
        }

        // The first function's calls must reference only its own blocks.
        let first_outer = lookup_direct_call_enclosing_blocks(discovered[0]);

        assert_eq!(first_outer.len(), 1);
        assert!(std::ptr::eq(first_outer[0], first_body));

        let first_inner = lookup_direct_call_enclosing_blocks(discovered[1]);

        assert_eq!(first_inner.len(), 2);
        assert!(std::ptr::eq(first_inner[0], first_body));
        assert!(std::ptr::eq(first_inner[1], first_if_block));

        // The second function's calls must reference only its own blocks.
        let second_outer = lookup_direct_call_enclosing_blocks(discovered[2]);

        assert_eq!(second_outer.len(), 1);
        assert!(std::ptr::eq(second_outer[0], second_body));

        let second_inner = lookup_direct_call_enclosing_blocks(discovered[3]);

        assert_eq!(second_inner.len(), 2);
        assert!(std::ptr::eq(second_inner[0], second_body));
        assert!(std::ptr::eq(second_inner[1], second_if_block));

        // Identical source structure must not imply shared syntax identity.
        assert!(!std::ptr::eq(first_body, second_body));
        assert!(!std::ptr::eq(first_if_block, second_if_block));
    }

    #[test]
    fn local_variable_lookup_returns_not_found_without_matching_declaration() {
        for source_text in [
            "function main() { target(); }",
            "function main() { let other = 1; target(); }",
        ] {
            let source = SourceText::new(source_text);

            let compilation_unit = Parser::new(&source)
                .parse_compilation_unit()
                .expect("Compilation unit should parse");

            let discovered = discover_direct_function_calls(&compilation_unit);

            assert_eq!(discovered.len(), 1);

            let result =
                lookup_direct_call_enclosing_function_local_variables(discovered[0], &source);

            assert!(matches!(result, Ok(LocalVariableLookup::NotFound)));
        }
    }

    #[test]
    fn local_variable_lookup_returns_unique_matching_let_and_var_declarations() {
        for keyword in ["let", "var"] {
            let source = SourceText::new(format!(
                "function main() {{ {keyword} target = 1; target(); }}"
            ));

            let compilation_unit = Parser::new(&source)
                .parse_compilation_unit()
                .expect("Compilation unit should parse");

            let discovered = discover_direct_function_calls(&compilation_unit);

            assert_eq!(discovered.len(), 1);

            let result =
                lookup_direct_call_enclosing_function_local_variables(discovered[0], &source);

            let Ok(LocalVariableLookup::Unique(declaration)) = result else {
                panic!("Expected exactly one matching local declaration");
            };

            let statements = compilation_unit.function_declarations()[0]
                .body()
                .statements();

            let StatementSyntax::VariableDeclaration(expected_declaration) = &statements[0] else {
                panic!("Expected a variable declaration");
            };

            assert!(std::ptr::eq(declaration, expected_declaration));
        }
    }

    #[test]
    fn local_variable_lookup_preserves_multiple_declarations_in_source_order() {
        let source = SourceText::new(
            "function main() {
                target();
                let target = 1;
                if true {
                    var target = 2;
                }
                var target = 3;
            }",
        );

        let compilation_unit = Parser::new(&source)
            .parse_compilation_unit()
            .expect("Compilation unit should parse");

        let discovered = discover_direct_function_calls(&compilation_unit);

        assert_eq!(discovered.len(), 1);

        let result = lookup_direct_call_enclosing_function_local_variables(discovered[0], &source);

        let Ok(LocalVariableLookup::Multiple(matches)) = result else {
            panic!("Expected multiple matching local declarations");
        };

        assert_eq!(matches.len(), 3);

        let statements = compilation_unit.function_declarations()[0]
            .body()
            .statements();

        let StatementSyntax::VariableDeclaration(first_declaration) = &statements[1] else {
            panic!("Expected first local declaration");
        };

        let StatementSyntax::Conditional(conditional) = &statements[2] else {
            panic!("Expected conditional statement");
        };

        let StatementSyntax::VariableDeclaration(nested_declaration) =
            &conditional.then_block().statements()[0]
        else {
            panic!("Expected nested local declaration");
        };

        let StatementSyntax::VariableDeclaration(last_declaration) = &statements[3] else {
            panic!("Expected final local declaration");
        };

        assert!(std::ptr::eq(matches[0], first_declaration));
        assert!(std::ptr::eq(matches[1], nested_declaration));
        assert!(std::ptr::eq(matches[2], last_declaration));
    }

    #[test]
    fn local_variable_lookup_recognizes_declarations_across_conditional_branches() {
        let source = SourceText::new(
            "function main() {
                target();

                if false {
                    let target = 1;
                } elseif true {
                    var target = 2;
                } else {
                    let target = 3;
                }
            }",
        );

        let compilation_unit = Parser::new(&source)
            .parse_compilation_unit()
            .expect("Compilation unit should parse");

        let discovered = discover_direct_function_calls(&compilation_unit);

        assert_eq!(discovered.len(), 1);

        let result = lookup_direct_call_enclosing_function_local_variables(discovered[0], &source);

        let Ok(LocalVariableLookup::Multiple(matches)) = result else {
            panic!("Expected matching declarations from all conditional branches");
        };

        assert_eq!(matches.len(), 3);

        let statements = compilation_unit.function_declarations()[0]
            .body()
            .statements();

        let StatementSyntax::Conditional(conditional) = &statements[1] else {
            panic!("Expected conditional statement");
        };

        let StatementSyntax::VariableDeclaration(then_declaration) =
            &conditional.then_block().statements()[0]
        else {
            panic!("Expected declaration in if branch");
        };

        let StatementSyntax::VariableDeclaration(elseif_declaration) =
            &conditional.elseif_clauses()[0].block().statements()[0]
        else {
            panic!("Expected declaration in elseif branch");
        };

        let else_clause = conditional.else_clause().expect("Expected else clause");

        let StatementSyntax::VariableDeclaration(else_declaration) =
            &else_clause.block().statements()[0]
        else {
            panic!("Expected declaration in else branch");
        };

        assert!(std::ptr::eq(matches[0], then_declaration));
        assert!(std::ptr::eq(matches[1], elseif_declaration));
        assert!(std::ptr::eq(matches[2], else_declaration));
    }

    #[test]
    fn local_variable_lookup_recognizes_declarations_in_nested_loops() {
        let source = SourceText::new(
            "function main() {
                target();

                while true {
                    let target = 1;

                    for index = 0; index < 3; index = index + 1 {
                        var target = 2;
                    }
                }
            }",
        );

        let compilation_unit = Parser::new(&source)
            .parse_compilation_unit()
            .expect("Compilation unit should parse");

        let discovered = discover_direct_function_calls(&compilation_unit);

        assert_eq!(discovered.len(), 1);

        let result = lookup_direct_call_enclosing_function_local_variables(discovered[0], &source);

        let Ok(LocalVariableLookup::Multiple(matches)) = result else {
            panic!("Expected matching declarations in nested loops");
        };

        assert_eq!(matches.len(), 2);

        let statements = compilation_unit.function_declarations()[0]
            .body()
            .statements();

        let StatementSyntax::While(while_statement) = &statements[1] else {
            panic!("Expected while statement");
        };

        let while_statements = while_statement.block().statements();

        let StatementSyntax::VariableDeclaration(while_declaration) = &while_statements[0] else {
            panic!("Expected declaration in while body");
        };

        let StatementSyntax::For(for_statement) = &while_statements[1] else {
            panic!("Expected nested for statement");
        };

        let StatementSyntax::VariableDeclaration(for_declaration) =
            &for_statement.block().statements()[0]
        else {
            panic!("Expected declaration in for body");
        };

        assert!(std::ptr::eq(matches[0], while_declaration));
        assert!(std::ptr::eq(matches[1], for_declaration));
    }

    #[test]
    fn local_variable_lookup_only_examines_the_enclosing_function() {
        let source = SourceText::new(
            "function first() {
                let target = 1;
                target();
            }

            function second() {
                target();
                let other = 2;
            }",
        );

        let compilation_unit = Parser::new(&source)
            .parse_compilation_unit()
            .expect("Compilation unit should parse");

        let discovered = discover_direct_function_calls(&compilation_unit);

        assert_eq!(discovered.len(), 2);

        let functions = compilation_unit.function_declarations();

        assert!(std::ptr::eq(discovered[0].function(), &functions[0]));
        assert!(std::ptr::eq(discovered[1].function(), &functions[1]));

        let first_result =
            lookup_direct_call_enclosing_function_local_variables(discovered[0], &source);

        let Ok(LocalVariableLookup::Unique(declaration)) = first_result else {
            panic!("Expected a matching declaration in the first function");
        };

        let StatementSyntax::VariableDeclaration(expected_declaration) =
            &functions[0].body().statements()[0]
        else {
            panic!("Expected a variable declaration in the first function");
        };

        assert!(std::ptr::eq(declaration, expected_declaration));

        let second_result =
            lookup_direct_call_enclosing_function_local_variables(discovered[1], &source);

        assert!(matches!(second_result, Ok(LocalVariableLookup::NotFound)));
    }

    #[test]
    fn local_variable_lookup_is_deterministic_across_repeated_calls() {
        let source = SourceText::new(
            "function main() {
                let target = 1;

                if true {
                    var target = 2;
                }

                target();
            }",
        );

        let compilation_unit = Parser::new(&source)
            .parse_compilation_unit()
            .expect("Compilation unit should parse");

        let discovered = discover_direct_function_calls(&compilation_unit);

        assert_eq!(discovered.len(), 1);

        let first = lookup_direct_call_enclosing_function_local_variables(discovered[0], &source);

        let second = lookup_direct_call_enclosing_function_local_variables(discovered[0], &source);

        let (
            Ok(LocalVariableLookup::Multiple(first_matches)),
            Ok(LocalVariableLookup::Multiple(second_matches)),
        ) = (first, second)
        else {
            panic!("Expected multiple matching declarations in both lookups");
        };

        assert_eq!(first_matches.len(), 2);
        assert_eq!(second_matches.len(), 2);

        for (first_declaration, second_declaration) in
            first_matches.iter().zip(second_matches.iter())
        {
            assert!(std::ptr::eq(*first_declaration, *second_declaration));
        }
    }

    #[test]
    fn local_variable_lookup_reports_invalid_call_identifier_span() {
        let source = SourceText::new(
            "function main() {
                let target = 1;
                target();
            }",
        );

        let compilation_unit = Parser::new(&source)
            .parse_compilation_unit()
            .expect("Compilation unit should parse");

        let discovered = discover_direct_function_calls(&compilation_unit);

        assert_eq!(discovered.len(), 1);

        let ExpressionSyntax::Name(name) = discovered[0].call().callee() else {
            panic!("Expected a direct named call");
        };

        let expected_span = name.identifier_token().span();

        let unrelated_source = SourceText::new("");

        let result =
            lookup_direct_call_enclosing_function_local_variables(discovered[0], &unrelated_source);

        assert!(matches!(
            result,
            Err(LocalVariableLookupError::InvalidIdentifierSpan(span))
                if span == expected_span
        ));
    }

    #[test]
    fn local_variable_lookup_reports_invalid_declaration_identifier_span() {
        for declaration_name in ["target", "other"] {
            let source = SourceText::new(format!(
                "function main() {{ target(); let {declaration_name} = 1; }}"
            ));

            let compilation_unit = Parser::new(&source)
                .parse_compilation_unit()
                .expect("Compilation unit should parse");

            let discovered = discover_direct_function_calls(&compilation_unit);

            assert_eq!(discovered.len(), 1);

            let statements = compilation_unit.function_declarations()[0]
                .body()
                .statements();

            let StatementSyntax::VariableDeclaration(declaration) = &statements[1] else {
                panic!("Expected a local variable declaration");
            };

            let expected_span = declaration.identifier_token().span();

            let unrelated_source = SourceText::new("function main() { target(); }");

            let ExpressionSyntax::Name(call_name) = discovered[0].call().callee() else {
                panic!("Expected a direct named call");
            };

            let call_span = call_name.identifier_token().span();

            assert_eq!(unrelated_source.slice(&call_span), Some("target"));

            let result = lookup_direct_call_enclosing_function_local_variables(
                discovered[0],
                &unrelated_source,
            );

            assert!(matches!(
                result,
                Err(LocalVariableLookupError::InvalidIdentifierSpan(span))
                    if span == expected_span
            ));
        }
    }

    #[test]
    fn local_variable_lookup_preserves_independent_declaration_candidates() {
        let source = SourceText::new(
            "function target() {}

            function main(target) {
                let target = 1;
                target();
            }",
        );

        let compilation_unit = Parser::new(&source)
            .parse_compilation_unit()
            .expect("Compilation unit should parse");

        let discovered = discover_direct_function_calls(&compilation_unit);

        assert_eq!(discovered.len(), 1);

        let functions = compilation_unit.function_declarations();

        assert!(std::ptr::eq(discovered[0].function(), &functions[1]));

        let local_result =
            lookup_direct_call_enclosing_function_local_variables(discovered[0], &source);

        let Ok(LocalVariableLookup::Unique(local_declaration)) = local_result else {
            panic!("Expected one matching local variable declaration");
        };

        let StatementSyntax::VariableDeclaration(expected_local) =
            &functions[1].body().statements()[0]
        else {
            panic!("Expected local variable declaration");
        };

        assert!(std::ptr::eq(local_declaration, expected_local));

        let parameter_result =
            lookup_direct_call_enclosing_function_parameters(discovered[0], &source);

        let Ok(FunctionParameterLookup::Unique(parameter)) = parameter_result else {
            panic!("Expected one matching function parameter");
        };

        assert!(std::ptr::eq(parameter, &functions[1].parameters()[0]));

        let function_result = lookup_direct_call_top_level_functions(
            &compilation_unit,
            &source,
            discovered[0].call(),
        );

        let Ok(Some(TopLevelFunctionLookup::Unique(function_candidate))) = function_result else {
            panic!("Expected one matching top-level function");
        };

        assert!(std::ptr::eq(function_candidate, &functions[0]));
    }

    #[test]
    fn parameter_lookup_returns_not_found_without_matching_parameter() {
        for source_text in [
            "function main() { greet(); }",
            "function main(other) { greet(); }",
        ] {
            let source = SourceText::new(source_text);

            let compilation_unit = Parser::new(&source)
                .parse_compilation_unit()
                .expect("Compilation unit should parse");

            let discovered = discover_direct_function_calls(&compilation_unit);

            assert_eq!(discovered.len(), 1);

            let result = lookup_direct_call_enclosing_function_parameters(discovered[0], &source);

            assert!(matches!(result, Ok(FunctionParameterLookup::NotFound)));
        }
    }

    #[test]
    fn parameter_lookup_returns_unique_matching_parameter() {
        let source = SourceText::new("function main(first, second, third) { second(); }");

        let compilation_unit = Parser::new(&source)
            .parse_compilation_unit()
            .expect("Compilation unit should parse");

        let discovered = discover_direct_function_calls(&compilation_unit);

        assert_eq!(discovered.len(), 1);

        let result = lookup_direct_call_enclosing_function_parameters(discovered[0], &source);

        let Ok(FunctionParameterLookup::Unique(parameter)) = result else {
            panic!("Expected exactly one matching parameter");
        };

        let expected_parameter = &compilation_unit.function_declarations()[0].parameters()[1];

        assert!(std::ptr::eq(parameter, expected_parameter));
    }

    #[test]
    fn parameter_lookup_preserves_multiple_matching_parameters() {
        let source = SourceText::new("function main(value, other, value) { value(); }");

        let compilation_unit = Parser::new(&source)
            .parse_compilation_unit()
            .expect("Compilation unit should parse");

        let discovered = discover_direct_function_calls(&compilation_unit);

        assert_eq!(discovered.len(), 1);

        let result = lookup_direct_call_enclosing_function_parameters(discovered[0], &source);

        let Ok(FunctionParameterLookup::Multiple(matches)) = result else {
            panic!("Expected multiple matching parameters");
        };

        assert_eq!(matches.len(), 2);

        let parameters = compilation_unit.function_declarations()[0].parameters();

        assert!(std::ptr::eq(matches[0], &parameters[0]));
        assert!(std::ptr::eq(matches[1], &parameters[2]));
    }

    #[test]
    fn parameter_lookup_preserves_independent_top_level_function_candidates() {
        let source = SourceText::new(
            "function greet() {}\n\
             function main(greet) { greet(); }",
        );

        let compilation_unit = Parser::new(&source)
            .parse_compilation_unit()
            .expect("Compilation unit should parse");

        let discovered = discover_direct_function_calls(&compilation_unit);

        assert_eq!(discovered.len(), 1);

        let parameter_result =
            lookup_direct_call_enclosing_function_parameters(discovered[0], &source);

        let Ok(FunctionParameterLookup::Unique(parameter)) = parameter_result else {
            panic!("Expected one matching parameter");
        };

        let functions = compilation_unit.function_declarations();

        assert!(std::ptr::eq(parameter, &functions[1].parameters()[0]));

        let function_result = lookup_direct_call_top_level_functions(
            &compilation_unit,
            &source,
            discovered[0].call(),
        );

        let Ok(Some(TopLevelFunctionLookup::Unique(declaration))) = function_result else {
            panic!("Expected one top-level function candidate");
        };

        assert!(std::ptr::eq(declaration, &functions[0]));
    }

    #[test]
    fn parameter_lookup_only_examines_enclosing_function_parameters() {
        let source = SourceText::new(
            "function first(value) { value(); }\n\
             function second(other) { value(); }",
        );

        let compilation_unit = Parser::new(&source)
            .parse_compilation_unit()
            .expect("Compilation unit should parse");

        let discovered = discover_direct_function_calls(&compilation_unit);
        let functions = compilation_unit.function_declarations();

        assert_eq!(discovered.len(), 2);

        assert!(std::ptr::eq(discovered[0].function(), &functions[0]));
        assert!(std::ptr::eq(discovered[1].function(), &functions[1]));

        let first_result = lookup_direct_call_enclosing_function_parameters(discovered[0], &source);

        let Ok(FunctionParameterLookup::Unique(parameter)) = first_result else {
            panic!("Expected parameter from first function");
        };

        assert!(std::ptr::eq(parameter, &functions[0].parameters()[0]));

        let second_result =
            lookup_direct_call_enclosing_function_parameters(discovered[1], &source);

        assert!(matches!(
            second_result,
            Ok(FunctionParameterLookup::NotFound)
        ));
    }

    #[test]
    fn parameter_lookup_recognizes_parameter_in_nested_control_flow() {
        let source =
            SourceText::new("function main(value) { if true { while true { value(); } } }");

        let compilation_unit = Parser::new(&source)
            .parse_compilation_unit()
            .expect("Compilation unit should parse");

        let discovered = discover_direct_function_calls(&compilation_unit);
        let functions = compilation_unit.function_declarations();

        assert_eq!(discovered.len(), 1);

        assert!(std::ptr::eq(discovered[0].function(), &functions[0]));

        let result = lookup_direct_call_enclosing_function_parameters(discovered[0], &source);

        let Ok(FunctionParameterLookup::Unique(parameter)) = result else {
            panic!("Expected matching parameter inside nested control flow");
        };

        assert!(std::ptr::eq(parameter, &functions[0].parameters()[0]));
    }

    #[test]
    fn parameter_lookup_is_deterministic_across_repeated_calls() {
        let source = SourceText::new("function main(value, other, value) { value(); }");

        let compilation_unit = Parser::new(&source)
            .parse_compilation_unit()
            .expect("Compilation unit should parse");

        let discovered = discover_direct_function_calls(&compilation_unit);

        assert_eq!(discovered.len(), 1);

        let first = lookup_direct_call_enclosing_function_parameters(discovered[0], &source);

        let second = lookup_direct_call_enclosing_function_parameters(discovered[0], &source);

        let (
            Ok(FunctionParameterLookup::Multiple(first_matches)),
            Ok(FunctionParameterLookup::Multiple(second_matches)),
        ) = (first, second)
        else {
            panic!("Both lookups should return multiple matching parameters");
        };

        assert_eq!(first_matches.len(), 2);
        assert_eq!(second_matches.len(), first_matches.len());

        let parameters = compilation_unit.function_declarations()[0].parameters();

        assert!(std::ptr::eq(first_matches[0], &parameters[0]));
        assert!(std::ptr::eq(first_matches[1], &parameters[2]));

        for (first_parameter, second_parameter) in first_matches.iter().zip(second_matches.iter()) {
            assert!(std::ptr::eq(*first_parameter, *second_parameter));
        }
    }

    #[test]
    fn parameter_lookup_reports_invalid_call_identifier_span() {
        let source = SourceText::new("function main(value) { value(); }");

        let compilation_unit = Parser::new(&source)
            .parse_compilation_unit()
            .expect("Compilation unit should parse");

        let discovered = discover_direct_function_calls(&compilation_unit);

        assert_eq!(discovered.len(), 1);

        let ExpressionSyntax::Name(name) = discovered[0].call().callee() else {
            panic!("Expected a direct named call");
        };

        let expected_span = name.identifier_token().span();

        let unrelated_source = SourceText::new("");

        let result =
            lookup_direct_call_enclosing_function_parameters(discovered[0], &unrelated_source);

        assert!(matches!(
            result,
            Err(ParameterLookupError::InvalidIdentifierSpan(span))
                if span == expected_span
        ));
    }

    #[test]
    fn parameter_lookup_reports_invalid_parameter_identifier_span() {
        let source = SourceText::new("function main(value) { value(); }");

        let compilation_unit = Parser::new(&source)
            .parse_compilation_unit()
            .expect("Compilation unit should parse");

        let discovered = discover_direct_function_calls(&compilation_unit);

        assert_eq!(discovered.len(), 1);

        let ExpressionSyntax::Name(name) = discovered[0].call().callee() else {
            panic!("Expected a direct named call");
        };

        let call_span = name.identifier_token().span();

        let parameter_span = compilation_unit.function_declarations()[0].parameters()[0]
            .identifier_token()
            .span();

        // Deliberately mismatched source text:
        // the call span remains valid, but the parameter span
        // begins inside the multibyte UTF-8 character 'é'.
        let unrelated_source = SourceText::new("function mainévalue) {value(); }");

        assert_eq!(unrelated_source.slice(&call_span), Some("value"));
        assert_eq!(unrelated_source.slice(&parameter_span), None);

        let result =
            lookup_direct_call_enclosing_function_parameters(discovered[0], &unrelated_source);

        assert!(matches!(
            result,
            Err(ParameterLookupError::InvalidIdentifierSpan(span))
                if span == parameter_span
        ));
    }

    #[test]
    fn parameter_lookup_does_not_resolve_local_variable_declarations() {
        let source = SourceText::new(
            "function with_parameter(value) { let value = 1; value(); }\n\
             function without_parameter(other) { let value = 1; value(); }",
        );

        let compilation_unit = Parser::new(&source)
            .parse_compilation_unit()
            .expect("Compilation unit should parse");

        let discovered = discover_direct_function_calls(&compilation_unit);
        let functions = compilation_unit.function_declarations();

        assert_eq!(discovered.len(), 2);

        assert!(std::ptr::eq(discovered[0].function(), &functions[0]));
        assert!(std::ptr::eq(discovered[1].function(), &functions[1]));

        let first_result = lookup_direct_call_enclosing_function_parameters(discovered[0], &source);

        let Ok(FunctionParameterLookup::Unique(parameter)) = first_result else {
            panic!("Expected matching parameter despite local declaration");
        };

        assert!(std::ptr::eq(parameter, &functions[0].parameters()[0]));

        let second_result =
            lookup_direct_call_enclosing_function_parameters(discovered[1], &source);

        assert!(matches!(
            second_result,
            Ok(FunctionParameterLookup::NotFound)
        ));
    }

    #[test]
    fn discovery_returns_no_calls_for_empty_or_call_free_functions() {
        for source_text in ["", "function main() { let answer = 42; }"] {
            let source = SourceText::new(source_text);

            let compilation_unit = Parser::new(&source)
                .parse_compilation_unit()
                .expect("Compilation unit should parse");

            let discovered = discover_direct_function_calls(&compilation_unit);

            assert!(discovered.is_empty());
        }
    }

    #[test]
    fn discovery_finds_nested_direct_calls_in_source_order() {
        let source = SourceText::new(
            "function main() {\n\
                 outer(inner());\n\
                 get_handler()();\n\
                 create_object().run();\n\
             }",
        );

        let compilation_unit = Parser::new(&source)
            .parse_compilation_unit()
            .expect("Compilation unit should parse");

        let discovered = discover_direct_function_calls(&compilation_unit);

        let names: Vec<&str> = discovered
            .iter()
            .map(|item| {
                let ExpressionSyntax::Name(name) = item.call().callee() else {
                    panic!("Expected a direct named call");
                };

                source
                    .slice(&name.identifier_token().span())
                    .expect("Identifier span should be valid")
            })
            .collect();

        assert_eq!(names, ["outer", "inner", "get_handler", "create_object"]);

        let main_function = &compilation_unit.function_declarations()[0];

        for item in &discovered {
            assert!(std::ptr::eq(item.function(), main_function));
        }

        let StatementSyntax::Expression(statement) = &main_function.body().statements()[0] else {
            panic!("Expected an expression statement");
        };

        let ExpressionSyntax::Call(outer_call) = statement.expression() else {
            panic!("Expected a call expression");
        };

        assert!(std::ptr::eq(discovered[0].call(), outer_call));
    }

    #[test]
    fn discovery_finds_calls_in_declarations_returns_and_control_flow() {
        let source = SourceText::new(
            r#"
function main() {
    let value = prepare();

    if is_ready() {
        then_call();
    } elseif should_retry() {
        retry_call();
    } else {
        else_call();
    }

    while keep_running() {
        loop_call();
    }

    return finish();
}
"#,
        );

        let compilation_unit = Parser::new(&source)
            .parse_compilation_unit()
            .expect("Compilation unit should parse");

        let discovered = discover_direct_function_calls(&compilation_unit);

        let names: Vec<&str> = discovered
            .iter()
            .map(|item| {
                let ExpressionSyntax::Name(name) = item.call().callee() else {
                    panic!("Expected a direct named call");
                };

                source
                    .slice(&name.identifier_token().span())
                    .expect("Identifier span should be valid")
            })
            .collect();

        assert_eq!(
            names,
            [
                "prepare",
                "is_ready",
                "then_call",
                "should_retry",
                "retry_call",
                "else_call",
                "keep_running",
                "loop_call",
                "finish",
            ]
        );
    }

    #[test]
    fn discovery_preserves_enclosing_function_references() {
        let source = SourceText::new(
            r#"
function first() {
    alpha();
    beta();
}

function second() {
    gamma();
}
"#,
        );

        let compilation_unit = Parser::new(&source)
            .parse_compilation_unit()
            .expect("Compilation unit should parse");

        let discovered = discover_direct_function_calls(&compilation_unit);
        let functions = compilation_unit.function_declarations();

        assert_eq!(functions.len(), 2);
        assert_eq!(discovered.len(), 3);

        assert!(std::ptr::eq(discovered[0].function(), &functions[0]));
        assert!(std::ptr::eq(discovered[1].function(), &functions[0]));
        assert!(std::ptr::eq(discovered[2].function(), &functions[1]));

        let positions: Vec<usize> = discovered
            .iter()
            .map(|item| {
                let ExpressionSyntax::Name(name) = item.call().callee() else {
                    panic!("Expected a direct named call");
                };

                name.identifier_token().span().start()
            })
            .collect();

        assert!(positions.windows(2).all(|pair| pair[0] < pair[1]));
    }

    #[test]
    fn discovery_finds_calls_in_for_loop_headers_and_nested_bodies() {
        let source = SourceText::new(
            r#"
function main() {
    for prepare(seed()); keep_running(); advance() {
        process();

        for nested_init(); nested_check(); nested_update() {
            nested_body();
        }
    }
}
"#,
        );

        let compilation_unit = Parser::new(&source)
            .parse_compilation_unit()
            .expect("Compilation unit should parse");

        let discovered = discover_direct_function_calls(&compilation_unit);

        let names: Vec<&str> = discovered
            .iter()
            .map(|item| {
                let ExpressionSyntax::Name(name) = item.call().callee() else {
                    panic!("Expected a direct named call");
                };

                source
                    .slice(&name.identifier_token().span())
                    .expect("Identifier span should be valid")
            })
            .collect();

        assert_eq!(
            names,
            [
                "prepare",
                "seed",
                "keep_running",
                "advance",
                "process",
                "nested_init",
                "nested_check",
                "nested_update",
                "nested_body",
            ]
        );

        let main_function = &compilation_unit.function_declarations()[0];

        for item in &discovered {
            assert!(std::ptr::eq(item.function(), main_function));
        }
    }

    #[test]
    fn discovery_finds_calls_across_expression_variants() {
        let source = SourceText::new(
            r#"
function main() {
    -negate();

    left() + right();

    choose() ? yes() : no();

    result = assign();

    create().field;

    items()[index()]();

    let values = [first(), 42, second()];

    let record = {
        primary: build(),
        secondary: calculate()
    };

    (wrapped());
}
"#,
        );

        let compilation_unit = Parser::new(&source)
            .parse_compilation_unit()
            .expect("Compilation unit should parse");

        let discovered = discover_direct_function_calls(&compilation_unit);

        let names: Vec<&str> = discovered
            .iter()
            .map(|item| {
                let ExpressionSyntax::Name(name) = item.call().callee() else {
                    panic!("Expected a direct named call");
                };

                source
                    .slice(&name.identifier_token().span())
                    .expect("Identifier span should be valid")
            })
            .collect();

        assert_eq!(
            names,
            [
                "negate",
                "left",
                "right",
                "choose",
                "yes",
                "no",
                "assign",
                "create",
                "items",
                "index",
                "first",
                "second",
                "build",
                "calculate",
                "wrapped",
            ]
        );

        assert_eq!(discovered.len(), 15);
    }

    #[test]
    fn discovery_excludes_unsupported_callees_but_visits_their_children() {
        let source = SourceText::new(
            r#"
function main() {
    object.run(helper());

    (name)();

    (nested())();

    handlers[index()]();
}
"#,
        );

        let compilation_unit = Parser::new(&source)
            .parse_compilation_unit()
            .expect("Compilation unit should parse");

        let discovered = discover_direct_function_calls(&compilation_unit);

        let names: Vec<&str> = discovered
            .iter()
            .map(|item| {
                let ExpressionSyntax::Name(name) = item.call().callee() else {
                    panic!("Expected a direct named call");
                };

                source
                    .slice(&name.identifier_token().span())
                    .expect("Identifier span should be valid")
            })
            .collect();

        assert_eq!(names, ["helper", "nested", "index"]);
        assert_eq!(discovered.len(), 3);
    }

    #[test]
    fn discovery_is_deterministic_across_repeated_calls() {
        let source = SourceText::new(
            r#"
function main() {
    outer(inner());
    helper();
}

function second() {
    finish();
}
"#,
        );

        let compilation_unit = Parser::new(&source)
            .parse_compilation_unit()
            .expect("Compilation unit should parse");

        let first = discover_direct_function_calls(&compilation_unit);
        let second = discover_direct_function_calls(&compilation_unit);

        assert_eq!(first.len(), 4);
        assert_eq!(second.len(), first.len());

        for (first_item, second_item) in first.iter().zip(second.iter()) {
            assert!(std::ptr::eq(first_item.function(), second_item.function()));

            assert!(std::ptr::eq(first_item.call(), second_item.call()));

            let ExpressionSyntax::Name(first_name) = first_item.call().callee() else {
                panic!("Expected a direct named call");
            };

            let ExpressionSyntax::Name(second_name) = second_item.call().callee() else {
                panic!("Expected a direct named call");
            };

            assert_eq!(
                first_name.identifier_token().span().start(),
                second_name.identifier_token().span().start()
            );
        }
    }

    #[test]
    fn direct_call_lookup_finds_later_top_level_function() {
        let source = SourceText::new("function main() { greet(); }\nfunction greet() {}");

        let compilation_unit = Parser::new(&source)
            .parse_compilation_unit()
            .expect("Compilation unit should parse");

        let main_function = &compilation_unit.function_declarations()[0];

        let StatementSyntax::Expression(statement) = &main_function.body().statements()[0] else {
            panic!("Expected an expression statement");
        };

        let ExpressionSyntax::Call(call) = statement.expression() else {
            panic!("Expected a call expression");
        };

        let result = lookup_direct_call_top_level_functions(&compilation_unit, &source, call);

        let Ok(Some(TopLevelFunctionLookup::Unique(declaration))) = result else {
            panic!("Expected exactly one matching function");
        };

        assert!(std::ptr::eq(
            declaration,
            &compilation_unit.function_declarations()[1]
        ));
    }

    #[test]
    fn direct_call_lookup_returns_not_found_for_unknown_name() {
        let source = SourceText::new("function main() { missing(); }");

        let compilation_unit = Parser::new(&source)
            .parse_compilation_unit()
            .expect("Compilation unit should parse");

        let StatementSyntax::Expression(statement) = &compilation_unit.function_declarations()[0]
            .body()
            .statements()[0]
        else {
            panic!("Expected an expression statement");
        };

        let ExpressionSyntax::Call(call) = statement.expression() else {
            panic!("Expected a call expression");
        };

        let result = lookup_direct_call_top_level_functions(&compilation_unit, &source, call);

        assert!(matches!(result, Ok(Some(TopLevelFunctionLookup::NotFound))));
    }

    #[test]
    fn direct_call_lookup_preserves_multiple_matching_declarations() {
        let source = SourceText::new(
            "function main() { greet(); }\n\
             function greet() {}\n\
             function greet() {}",
        );

        let compilation_unit = Parser::new(&source)
            .parse_compilation_unit()
            .expect("Compilation unit should parse");

        let StatementSyntax::Expression(statement) = &compilation_unit.function_declarations()[0]
            .body()
            .statements()[0]
        else {
            panic!("Expected an expression statement");
        };

        let ExpressionSyntax::Call(call) = statement.expression() else {
            panic!("Expected a call expression");
        };

        let result = lookup_direct_call_top_level_functions(&compilation_unit, &source, call);

        let Ok(Some(TopLevelFunctionLookup::Multiple(matches))) = result else {
            panic!("Expected multiple matching declarations");
        };

        assert_eq!(matches.len(), 2);

        for (index, declaration) in matches.iter().enumerate() {
            assert!(std::ptr::eq(
                *declaration,
                &compilation_unit.function_declarations()[index + 1]
            ));
        }
    }

    #[test]
    fn direct_call_lookup_returns_none_for_non_name_callees() {
        let source = SourceText::new(
            "function main() {\n\
                 object.greet();\n\
                 (greet)();\n\
                 get_handler()();\n\
                 handlers[0]();\n\
             }\n\
             function greet() {}\n\
             function get_handler() {}",
        );

        let compilation_unit = Parser::new(&source)
            .parse_compilation_unit()
            .expect("Compilation unit should parse");

        let statements = compilation_unit.function_declarations()[0]
            .body()
            .statements();

        assert_eq!(statements.len(), 4);

        for statement in statements {
            let StatementSyntax::Expression(statement) = statement else {
                panic!("Expected an expression statement");
            };

            let ExpressionSyntax::Call(call) = statement.expression() else {
                panic!("Expected a call expression");
            };

            let result = lookup_direct_call_top_level_functions(&compilation_unit, &source, call);

            assert!(matches!(result, Ok(None)));
        }
    }

    #[test]
    fn direct_call_lookup_reports_invalid_call_identifier_span() {
        let source = SourceText::new("function main() { greet(); }\nfunction greet() {}");

        let compilation_unit = Parser::new(&source)
            .parse_compilation_unit()
            .expect("Compilation unit should parse");

        let StatementSyntax::Expression(statement) = &compilation_unit.function_declarations()[0]
            .body()
            .statements()[0]
        else {
            panic!("Expected an expression statement");
        };

        let ExpressionSyntax::Call(call) = statement.expression() else {
            panic!("Expected a call expression");
        };

        let ExpressionSyntax::Name(name) = call.callee() else {
            panic!("Expected a direct named call");
        };

        let expected_span = name.identifier_token().span();
        let unrelated_source = SourceText::new("");

        let result =
            lookup_direct_call_top_level_functions(&compilation_unit, &unrelated_source, call);

        assert!(matches!(
            result,
            Err(FunctionLookupError::InvalidIdentifierSpan(span))
                if span == expected_span
        ));
    }

    #[test]
    fn direct_call_lookup_propagates_invalid_declaration_span() {
        let source = SourceText::new("function main() { greet(); }\nfunction greet() {}");

        let compilation_unit = Parser::new(&source)
            .parse_compilation_unit()
            .expect("Compilation unit should parse");

        let StatementSyntax::Expression(statement) = &compilation_unit.function_declarations()[0]
            .body()
            .statements()[0]
        else {
            panic!("Expected an expression statement");
        };

        let ExpressionSyntax::Call(call) = statement.expression() else {
            panic!("Expected a call expression");
        };

        let expected_span = compilation_unit.function_declarations()[1]
            .identifier_token()
            .span();

        let truncated_source = SourceText::new("function main() { greet(); }");

        let result =
            lookup_direct_call_top_level_functions(&compilation_unit, &truncated_source, call);

        assert!(matches!(
            result,
            Err(FunctionLookupError::InvalidIdentifierSpan(span))
                if span == expected_span
        ));
    }

    #[test]
    fn direct_call_lookup_is_deterministic_across_repeated_calls() {
        let source = SourceText::new("function main() { greet(); }\nfunction greet() {}");

        let compilation_unit = Parser::new(&source)
            .parse_compilation_unit()
            .expect("Compilation unit should parse");

        let StatementSyntax::Expression(statement) = &compilation_unit.function_declarations()[0]
            .body()
            .statements()[0]
        else {
            panic!("Expected an expression statement");
        };

        let ExpressionSyntax::Call(call) = statement.expression() else {
            panic!("Expected a call expression");
        };

        let first = lookup_direct_call_top_level_functions(&compilation_unit, &source, call);

        let second = lookup_direct_call_top_level_functions(&compilation_unit, &source, call);

        let (
            Ok(Some(TopLevelFunctionLookup::Unique(first_declaration))),
            Ok(Some(TopLevelFunctionLookup::Unique(second_declaration))),
        ) = (first, second)
        else {
            panic!("Both lookups should find the same declaration");
        };

        assert!(std::ptr::eq(first_declaration, second_declaration));
    }

    #[test]
    fn direct_call_lookup_finds_candidate_despite_parameter_shadowing() {
        let source = SourceText::new(
            "function greet() {}\n\
             function main(greet) { greet(); }",
        );

        let compilation_unit = Parser::new(&source)
            .parse_compilation_unit()
            .expect("Compilation unit should parse");

        let StatementSyntax::Expression(statement) = &compilation_unit.function_declarations()[1]
            .body()
            .statements()[0]
        else {
            panic!("Expected an expression statement");
        };

        let ExpressionSyntax::Call(call) = statement.expression() else {
            panic!("Expected a call expression");
        };

        let result = lookup_direct_call_top_level_functions(&compilation_unit, &source, call);

        let Ok(Some(TopLevelFunctionLookup::Unique(declaration))) = result else {
            panic!("Expected a top-level function candidate");
        };

        assert!(std::ptr::eq(
            declaration,
            &compilation_unit.function_declarations()[0]
        ));
    }

    #[test]
    fn lookup_returns_not_found_for_empty_compilation_unit() {
        let source = SourceText::new("");
        let compilation_unit = Parser::new(&source)
            .parse_compilation_unit()
            .expect("Empty compilation unit should parse");

        let result = lookup_top_level_function(&compilation_unit, &source, "main");

        assert!(matches!(result, Ok(TopLevelFunctionLookup::NotFound)));
    }

    #[test]
    fn lookup_finds_function_declared_later() {
        let source = SourceText::new("function main() { greet(); }\nfunction greet() {}");
        let compilation_unit = Parser::new(&source)
            .parse_compilation_unit()
            .expect("Compilation unit should parse");

        let result = lookup_top_level_function(&compilation_unit, &source, "greet");

        let Ok(TopLevelFunctionLookup::Unique(declaration)) = result else {
            panic!("Expected exactly one matching function");
        };

        assert!(std::ptr::eq(
            declaration,
            &compilation_unit.function_declarations()[1]
        ));

        assert_eq!(
            source.slice(&declaration.identifier_token().span()),
            Some("greet")
        );
    }

    #[test]
    fn lookup_returns_not_found_for_unknown_function() {
        let source = SourceText::new("function greet() {}");
        let compilation_unit = Parser::new(&source)
            .parse_compilation_unit()
            .expect("Compilation unit should parse");

        let result = lookup_top_level_function(&compilation_unit, &source, "grete");

        assert!(matches!(result, Ok(TopLevelFunctionLookup::NotFound)));
    }

    #[test]
    fn lookup_preserves_multiple_matching_declarations() {
        let source = SourceText::new("function greet() {}\nfunction greet() {}");
        let compilation_unit = Parser::new(&source)
            .parse_compilation_unit()
            .expect("Compilation unit should parse");

        let result = lookup_top_level_function(&compilation_unit, &source, "greet");

        let Ok(TopLevelFunctionLookup::Multiple(matches)) = result else {
            panic!("Expected multiple matching functions");
        };

        assert_eq!(matches.len(), 2);

        for (index, declaration) in matches.iter().enumerate() {
            assert!(std::ptr::eq(
                *declaration,
                &compilation_unit.function_declarations()[index]
            ));
        }
    }

    #[test]
    fn lookup_reports_invalid_source_span() {
        let source = SourceText::new("function main() {}");
        let compilation_unit = Parser::new(&source)
            .parse_compilation_unit()
            .expect("Compilation unit should parse");

        let unrelated_source = SourceText::new("");

        let result = lookup_top_level_function(&compilation_unit, &unrelated_source, "main");

        assert!(matches!(
            result,
            Err(FunctionLookupError::InvalidIdentifierSpan(_))
        ));
    }

    #[test]
    fn lookup_is_deterministic_across_repeated_calls() {
        let source = SourceText::new("function main() {}\nfunction greet() {}");

        let compilation_unit = Parser::new(&source)
            .parse_compilation_unit()
            .expect("Compilation unit should parse");

        let first = lookup_top_level_function(&compilation_unit, &source, "greet");

        let second = lookup_top_level_function(&compilation_unit, &source, "greet");

        let (
            Ok(TopLevelFunctionLookup::Unique(first_declaration)),
            Ok(TopLevelFunctionLookup::Unique(second_declaration)),
        ) = (first, second)
        else {
            panic!("Both lookups should find the same function");
        };

        assert!(std::ptr::eq(first_declaration, second_declaration));

        assert_eq!(
            first_declaration.identifier_token().span(),
            second_declaration.identifier_token().span()
        );
    }

    #[test]
    fn lookup_ignores_module_and_import_declarations() {
        let source = SourceText::new(
            "module application;\n\
             import standard.io.{read, write};\n\
             import standard.math as math;\n\
             function main() {}\n\
             function greet() {}",
        );

        let compilation_unit = Parser::new(&source)
            .parse_compilation_unit()
            .expect("Compilation unit should parse");

        let result = lookup_top_level_function(&compilation_unit, &source, "greet");

        assert!(matches!(result, Ok(TopLevelFunctionLookup::Unique(_))));

        let imported_name = lookup_top_level_function(&compilation_unit, &source, "read");

        assert!(matches!(
            imported_name,
            Ok(TopLevelFunctionLookup::NotFound)
        ));
    }
}
