//! Semantic analysis and name resolution for Cudrix.

use crate::source::{SourceSpan, SourceText};
use crate::syntax::{CompilationUnitSyntax, FunctionDeclarationSyntax};

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::Parser;

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
