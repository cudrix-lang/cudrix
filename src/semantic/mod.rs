//! Semantic analysis and name resolution for Cudrix.

use crate::source::{SourceSpan, SourceText};
use crate::syntax::{
    BlockSyntax, CallExpressionSyntax, CompilationUnitSyntax, ExpressionSyntax,
    FunctionDeclarationSyntax, StatementSyntax,
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
        collect_direct_calls_in_block(function.body(), function, &mut discovered_calls);
    }

    discovered_calls.sort_by_key(|discovered| {
        let ExpressionSyntax::Name(name) = discovered.call().callee() else {
            unreachable!("Discovered direct calls must have named callees");
        };

        name.identifier_token().span().start()
    });

    discovered_calls
}

fn collect_direct_calls_in_block<'syntax>(
    block: &'syntax BlockSyntax,
    function: &'syntax FunctionDeclarationSyntax,
    discovered_calls: &mut Vec<DiscoveredDirectFunctionCall<'syntax>>,
) {
    for statement in block.statements() {
        match statement {
            StatementSyntax::Expression(statement) => {
                collect_direct_calls_in_expression(
                    statement.expression(),
                    function,
                    discovered_calls,
                );
            }

            StatementSyntax::VariableDeclaration(statement) => {
                collect_direct_calls_in_expression(
                    statement.initializer(),
                    function,
                    discovered_calls,
                );
            }

            StatementSyntax::Return(statement) => {
                collect_direct_calls_in_expression(
                    statement.expression(),
                    function,
                    discovered_calls,
                );
            }

            StatementSyntax::Conditional(statement) => {
                collect_direct_calls_in_expression(
                    statement.condition(),
                    function,
                    discovered_calls,
                );

                collect_direct_calls_in_block(statement.then_block(), function, discovered_calls);

                for clause in statement.elseif_clauses() {
                    collect_direct_calls_in_expression(
                        clause.condition(),
                        function,
                        discovered_calls,
                    );

                    collect_direct_calls_in_block(clause.block(), function, discovered_calls);
                }

                if let Some(clause) = statement.else_clause() {
                    collect_direct_calls_in_block(clause.block(), function, discovered_calls);
                }
            }

            StatementSyntax::While(statement) => {
                collect_direct_calls_in_expression(
                    statement.condition(),
                    function,
                    discovered_calls,
                );

                collect_direct_calls_in_block(statement.block(), function, discovered_calls);
            }

            StatementSyntax::For(statement) => {
                collect_direct_calls_in_expression(
                    statement.initializer(),
                    function,
                    discovered_calls,
                );

                collect_direct_calls_in_expression(
                    statement.condition(),
                    function,
                    discovered_calls,
                );

                collect_direct_calls_in_expression(statement.update(), function, discovered_calls);

                collect_direct_calls_in_block(statement.block(), function, discovered_calls);
            }
        }
    }
}

fn collect_direct_calls_in_expression<'syntax>(
    expression: &'syntax ExpressionSyntax,
    function: &'syntax FunctionDeclarationSyntax,
    discovered_calls: &mut Vec<DiscoveredDirectFunctionCall<'syntax>>,
) {
    match expression {
        ExpressionSyntax::Literal(_) | ExpressionSyntax::Name(_) => {}

        ExpressionSyntax::Unary(unary) => {
            collect_direct_calls_in_expression(unary.operand(), function, discovered_calls);
        }

        ExpressionSyntax::Binary(binary) => {
            collect_direct_calls_in_expression(binary.left(), function, discovered_calls);

            collect_direct_calls_in_expression(binary.right(), function, discovered_calls);
        }

        ExpressionSyntax::Conditional(conditional) => {
            collect_direct_calls_in_expression(conditional.condition(), function, discovered_calls);

            collect_direct_calls_in_expression(conditional.when_true(), function, discovered_calls);

            collect_direct_calls_in_expression(
                conditional.when_false(),
                function,
                discovered_calls,
            );
        }

        ExpressionSyntax::Assignment(assignment) => {
            collect_direct_calls_in_expression(assignment.expression(), function, discovered_calls);
        }

        ExpressionSyntax::MemberAccess(member) => {
            collect_direct_calls_in_expression(member.expression(), function, discovered_calls);
        }

        ExpressionSyntax::Index(index) => {
            collect_direct_calls_in_expression(index.expression(), function, discovered_calls);

            collect_direct_calls_in_expression(index.index(), function, discovered_calls);
        }

        ExpressionSyntax::ArrayLiteral(array) => {
            for element in array.elements() {
                collect_direct_calls_in_expression(element, function, discovered_calls);
            }
        }

        ExpressionSyntax::ObjectLiteral(object) => {
            for member in object.members() {
                collect_direct_calls_in_expression(member.value(), function, discovered_calls);
            }
        }

        ExpressionSyntax::Call(call) => {
            if matches!(call.callee(), ExpressionSyntax::Name(_)) {
                discovered_calls.push(DiscoveredDirectFunctionCall { function, call });
            }

            collect_direct_calls_in_expression(call.callee(), function, discovered_calls);

            for argument in call.arguments() {
                collect_direct_calls_in_expression(argument, function, discovered_calls);
            }
        }

        ExpressionSyntax::Parenthesized(parenthesized) => {
            collect_direct_calls_in_expression(
                parenthesized.expression(),
                function,
                discovered_calls,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::Parser;
    use crate::syntax::StatementSyntax;

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
