//! Parser infrastructure for Cudrix source code.

use crate::compiler::lexer::{Lexer, Token, TokenKind};
use crate::source::SourceText;
use crate::syntax::{
    AssignmentExpressionSyntax, BinaryExpressionSyntax, CallExpressionSyntax, ExpressionSyntax,
    LiteralExpressionSyntax, NameExpressionSyntax, ParenthesizedExpressionSyntax,
    UnaryExpressionSyntax,
};

pub struct Parser<'source> {
    lexer: Lexer<'source>,
    current_token: Token,
}

impl<'source> Parser<'source> {
    pub fn new(source: &'source SourceText) -> Self {
        let mut lexer = Lexer::new(source);
        let current_token = Self::next_significant_token_from(&mut lexer);

        Self {
            lexer,
            current_token,
        }
    }

    pub fn current_token(&self) -> Token {
        self.current_token
    }

    pub fn parse_expression(&mut self) -> Option<ExpressionSyntax> {
        self.parse_assignment_expression()
    }

    pub fn parse_assignment_expression(&mut self) -> Option<ExpressionSyntax> {
        let left = self.parse_binary_expression(0)?;

        if self.current_token.kind() != TokenKind::Assignment {
            return Some(left);
        }

        let ExpressionSyntax::Name(name_expression) = left else {
            return Some(left);
        };

        let assignment_token = self.current_token;
        self.advance();

        let expression = self.parse_assignment_expression()?;

        Some(ExpressionSyntax::Assignment(
            AssignmentExpressionSyntax::new(
                name_expression.identifier_token(),
                assignment_token,
                expression,
            ),
        ))
    }

    fn parse_binary_expression(&mut self, parent_precedence: u8) -> Option<ExpressionSyntax> {
        let mut left = self.parse_unary_or_primary_expression()?;

        loop {
            let precedence = Self::binary_operator_precedence(self.current_token.kind());

            if precedence == 0 || precedence <= parent_precedence {
                break;
            }

            let operator_token = self.current_token;
            self.advance();

            let right = self.parse_binary_expression(precedence)?;

            left =
                ExpressionSyntax::Binary(BinaryExpressionSyntax::new(left, operator_token, right));
        }

        Some(left)
    }

    fn parse_unary_or_primary_expression(&mut self) -> Option<ExpressionSyntax> {
        match self.current_token.kind() {
            TokenKind::Plus | TokenKind::Minus | TokenKind::LogicalNot => {
                self.parse_unary_expression().map(ExpressionSyntax::Unary)
            }

            _ => self.parse_call_expression(),
        }
    }

    fn parse_call_expression(&mut self) -> Option<ExpressionSyntax> {
        let mut expression = self.parse_primary_expression()?;

        while self.current_token.kind() == TokenKind::LeftParenthesis {
            let left_parenthesis_token = self.current_token;
            self.advance();

            let mut arguments = Vec::new();

            if self.current_token.kind() != TokenKind::RightParenthesis {
                loop {
                    arguments.push(self.parse_expression()?);

                    if self.current_token.kind() != TokenKind::Comma {
                        break;
                    }

                    self.advance();
                }
            }

            if self.current_token.kind() != TokenKind::RightParenthesis {
                return None;
            }

            let right_parenthesis_token = self.current_token;
            self.advance();

            expression = ExpressionSyntax::Call(CallExpressionSyntax::new(
                expression,
                left_parenthesis_token,
                arguments,
                right_parenthesis_token,
            ));
        }

        Some(expression)
    }

    fn parse_primary_expression(&mut self) -> Option<ExpressionSyntax> {
        match self.current_token.kind() {
            TokenKind::Number
            | TokenKind::String
            | TokenKind::TrueKeyword
            | TokenKind::FalseKeyword => self
                .parse_literal_expression()
                .map(ExpressionSyntax::Literal),

            TokenKind::Identifier => self.parse_name_expression().map(ExpressionSyntax::Name),

            TokenKind::LeftParenthesis => self
                .parse_parenthesized_expression()
                .map(ExpressionSyntax::Parenthesized),

            _ => None,
        }
    }

    fn binary_operator_precedence(kind: TokenKind) -> u8 {
        match kind {
            TokenKind::LogicalOr => 1,
            TokenKind::LogicalAnd => 2,

            TokenKind::Equal | TokenKind::NotEqual => 3,

            TokenKind::LessThan
            | TokenKind::LessThanOrEqual
            | TokenKind::GreaterThan
            | TokenKind::GreaterThanOrEqual => 4,

            TokenKind::Plus | TokenKind::Minus => 5,

            TokenKind::Multiply | TokenKind::Divide | TokenKind::Remainder => 6,

            _ => 0,
        }
    }

    pub fn parse_literal_expression(&mut self) -> Option<LiteralExpressionSyntax> {
        if !matches!(
            self.current_token.kind(),
            TokenKind::Number
                | TokenKind::String
                | TokenKind::TrueKeyword
                | TokenKind::FalseKeyword
        ) {
            return None;
        }

        let literal_expression = LiteralExpressionSyntax::new(self.current_token);

        self.advance();

        Some(literal_expression)
    }

    pub fn parse_name_expression(&mut self) -> Option<NameExpressionSyntax> {
        if self.current_token.kind() != TokenKind::Identifier {
            return None;
        }

        let name_expression = NameExpressionSyntax::new(self.current_token);

        self.advance();

        Some(name_expression)
    }

    pub fn parse_unary_expression(&mut self) -> Option<UnaryExpressionSyntax> {
        if !matches!(
            self.current_token.kind(),
            TokenKind::Plus | TokenKind::Minus | TokenKind::LogicalNot
        ) {
            return None;
        }

        let operator_token = self.current_token;
        self.advance();

        let operand = self.parse_unary_or_primary_expression()?;

        Some(UnaryExpressionSyntax::new(operator_token, operand))
    }

    pub fn parse_parenthesized_expression(&mut self) -> Option<ParenthesizedExpressionSyntax> {
        if self.current_token.kind() != TokenKind::LeftParenthesis {
            return None;
        }

        let left_parenthesis_token = self.current_token;
        self.advance();

        let expression = self.parse_expression()?;

        if self.current_token.kind() != TokenKind::RightParenthesis {
            return None;
        }

        let right_parenthesis_token = self.current_token;
        self.advance();

        Some(ParenthesizedExpressionSyntax::new(
            left_parenthesis_token,
            expression,
            right_parenthesis_token,
        ))
    }

    fn next_significant_token(&mut self) -> Token {
        Self::next_significant_token_from(&mut self.lexer)
    }

    fn next_significant_token_from(lexer: &mut Lexer<'source>) -> Token {
        loop {
            let token = lexer.next_token();

            if !matches!(
                token.kind(),
                TokenKind::Whitespace | TokenKind::LineComment | TokenKind::BlockComment
            ) {
                return token;
            }
        }
    }

    pub fn advance(&mut self) {
        self.current_token = self.next_significant_token();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::source::SourceSpan;

    #[test]
    fn parser_starts_with_first_token() {
        let source = SourceText::new("let answer = 42;");
        let parser = Parser::new(&source);

        assert_eq!(parser.current_token().kind(), TokenKind::LetKeyword);
    }

    #[test]
    fn parser_advance_skips_whitespace() {
        let source = SourceText::new("let answer = 42;");
        let mut parser = Parser::new(&source);

        parser.advance();

        assert_eq!(parser.current_token().kind(), TokenKind::Identifier);
    }

    #[test]
    fn parser_starts_with_first_significant_token() {
        let source = SourceText::new("   let answer = 42;");
        let parser = Parser::new(&source);

        assert_eq!(parser.current_token().kind(), TokenKind::LetKeyword);
    }

    #[test]
    fn parser_advance_skips_line_comments() {
        let source = SourceText::new("let // comment\nanswer");
        let mut parser = Parser::new(&source);

        parser.advance();

        assert_eq!(parser.current_token().kind(), TokenKind::Identifier);
    }

    #[test]
    fn parser_advance_skips_block_comments() {
        let source = SourceText::new("let /* comment */ answer");
        let mut parser = Parser::new(&source);

        parser.advance();

        assert_eq!(parser.current_token().kind(), TokenKind::Identifier);
    }

    #[test]
    fn parser_does_not_skip_unterminated_block_comment() {
        let source = SourceText::new("let /* comment");
        let mut parser = Parser::new(&source);

        parser.advance();

        assert_eq!(
            parser.current_token().kind(),
            TokenKind::UnterminatedBlockComment
        );
    }

    #[test]
    fn parser_starts_at_end_of_file_for_empty_source() {
        let source = SourceText::new("");
        let parser = Parser::new(&source);

        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_starts_at_end_of_file_for_trivia_only_source() {
        let source = SourceText::new("  // comment\n/* block */  ");
        let parser = Parser::new(&source);

        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_advance_reaches_end_of_file() {
        let source = SourceText::new("answer");
        let mut parser = Parser::new(&source);

        parser.advance();

        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_advance_remains_at_end_of_file() {
        let source = SourceText::new("");
        let mut parser = Parser::new(&source);

        parser.advance();

        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_parses_literal_expression_token_kinds() {
        let cases = [
            ("42", TokenKind::Number),
            ("\"Forge\"", TokenKind::String),
            ("true", TokenKind::TrueKeyword),
            ("false", TokenKind::FalseKeyword),
        ];

        for (source_text, expected_kind) in cases {
            let source = SourceText::new(source_text);
            let mut parser = Parser::new(&source);

            let literal_expression = parser
                .parse_literal_expression()
                .expect("literal token should parse as literal expression");

            assert_eq!(literal_expression.literal_token().kind(), expected_kind);
            assert_eq!(
                literal_expression.literal_token().span(),
                SourceSpan::new(0, source.len())
            );
        }
    }

    #[test]
    fn parser_literal_expression_advances_to_next_significant_token() {
        let source = SourceText::new("42 /* comment */ answer");
        let mut parser = Parser::new(&source);

        let literal_expression = parser
            .parse_literal_expression()
            .expect("number should parse as literal expression");

        assert_eq!(literal_expression.literal_token().kind(), TokenKind::Number);
        assert_eq!(parser.current_token().kind(), TokenKind::Identifier);
    }

    #[test]
    fn parser_literal_expression_rejects_non_literal_without_advancing() {
        let source = SourceText::new("answer");
        let mut parser = Parser::new(&source);
        let current_token = parser.current_token();

        let literal_expression = parser.parse_literal_expression();

        assert_eq!(literal_expression, None);
        assert_eq!(parser.current_token(), current_token);
    }

    #[test]
    fn parser_literal_expression_rejects_unterminated_string_without_advancing() {
        let source = SourceText::new("\"Forge");
        let mut parser = Parser::new(&source);
        let current_token = parser.current_token();

        let literal_expression = parser.parse_literal_expression();

        assert_eq!(literal_expression, None);
        assert_eq!(parser.current_token().kind(), TokenKind::UnterminatedString);
        assert_eq!(parser.current_token(), current_token);
    }

    #[test]
    fn parser_literal_expression_returns_none_at_end_of_file() {
        let source = SourceText::new("");
        let mut parser = Parser::new(&source);

        let literal_expression = parser.parse_literal_expression();

        assert_eq!(literal_expression, None);
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_parses_name_expression() {
        let source = SourceText::new("answer");
        let mut parser = Parser::new(&source);

        let name_expression = parser
            .parse_name_expression()
            .expect("identifier should parse as name expression");

        assert_eq!(
            name_expression.identifier_token().kind(),
            TokenKind::Identifier
        );
        assert_eq!(
            name_expression.identifier_token().span(),
            SourceSpan::new(0, 6)
        );
    }

    #[test]
    fn parser_name_expression_advances_to_next_significant_token() {
        let source = SourceText::new("answer /* comment */ 42");
        let mut parser = Parser::new(&source);

        let name_expression = parser
            .parse_name_expression()
            .expect("identifier should parse as name expression");

        assert_eq!(
            name_expression.identifier_token().kind(),
            TokenKind::Identifier
        );
        assert_eq!(parser.current_token().kind(), TokenKind::Number);
    }

    #[test]
    fn parser_name_expression_rejects_non_identifier_without_advancing() {
        let source = SourceText::new("42");
        let mut parser = Parser::new(&source);
        let current_token = parser.current_token();

        let name_expression = parser.parse_name_expression();

        assert_eq!(name_expression, None);
        assert_eq!(parser.current_token(), current_token);
    }

    #[test]
    fn parser_name_expression_returns_none_at_end_of_file() {
        let source = SourceText::new("");
        let mut parser = Parser::new(&source);

        let name_expression = parser.parse_name_expression();

        assert_eq!(name_expression, None);
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_parses_literal_as_expression() {
        let source = SourceText::new("42");
        let mut parser = Parser::new(&source);

        let expression = parser
            .parse_expression()
            .expect("literal should parse as expression");

        let ExpressionSyntax::Literal(literal_expression) = expression else {
            panic!("expected literal expression");
        };

        assert_eq!(literal_expression.literal_token().kind(), TokenKind::Number);
        assert_eq!(
            literal_expression.literal_token().span(),
            SourceSpan::new(0, 2)
        );
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_parses_name_as_expression() {
        let source = SourceText::new("answer");
        let mut parser = Parser::new(&source);

        let expression = parser
            .parse_expression()
            .expect("name should parse as expression");

        let ExpressionSyntax::Name(name_expression) = expression else {
            panic!("expected name expression");
        };

        assert_eq!(
            name_expression.identifier_token().kind(),
            TokenKind::Identifier
        );
        assert_eq!(
            name_expression.identifier_token().span(),
            SourceSpan::new(0, 6)
        );
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_expression_advances_to_next_significant_token() {
        let source = SourceText::new("42 /* comment */ answer");
        let mut parser = Parser::new(&source);

        let expression = parser
            .parse_expression()
            .expect("literal should parse as expression");

        assert!(matches!(expression, ExpressionSyntax::Literal(_)));
        assert_eq!(parser.current_token().kind(), TokenKind::Identifier);
    }

    #[test]
    fn parser_expression_rejects_non_expression_without_advancing() {
        let source = SourceText::new(")");
        let mut parser = Parser::new(&source);
        let current_token = parser.current_token();

        let expression = parser.parse_expression();

        assert_eq!(expression, None);
        assert_eq!(parser.current_token(), current_token);
    }

    #[test]
    fn parser_expression_returns_none_at_end_of_file() {
        let source = SourceText::new("");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression();

        assert_eq!(expression, None);
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_parses_parenthesized_literal_expression() {
        let source = SourceText::new("(42)");
        let mut parser = Parser::new(&source);

        let parenthesized_expression = parser
            .parse_parenthesized_expression()
            .expect("parenthesized literal should parse");

        assert_eq!(
            parenthesized_expression.left_parenthesis_token().kind(),
            TokenKind::LeftParenthesis
        );

        let ExpressionSyntax::Literal(literal_expression) = parenthesized_expression.expression()
        else {
            panic!("expected literal expression");
        };

        assert_eq!(literal_expression.literal_token().kind(), TokenKind::Number);
        assert_eq!(
            parenthesized_expression.right_parenthesis_token().kind(),
            TokenKind::RightParenthesis
        );
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_parses_parenthesized_name_expression() {
        let source = SourceText::new("(answer)");
        let mut parser = Parser::new(&source);

        let parenthesized_expression = parser
            .parse_parenthesized_expression()
            .expect("parenthesized name should parse");

        let ExpressionSyntax::Name(name_expression) = parenthesized_expression.expression() else {
            panic!("expected name expression");
        };

        assert_eq!(
            name_expression.identifier_token().kind(),
            TokenKind::Identifier
        );
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_parses_nested_parenthesized_expression() {
        let source = SourceText::new("((42))");
        let mut parser = Parser::new(&source);

        let expression = parser
            .parse_expression()
            .expect("nested parenthesized expression should parse");

        let ExpressionSyntax::Parenthesized(outer) = expression else {
            panic!("expected outer parenthesized expression");
        };

        assert!(matches!(
            outer.expression(),
            ExpressionSyntax::Parenthesized(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_parses_parenthesized_expression_as_expression() {
        let source = SourceText::new("(42)");
        let mut parser = Parser::new(&source);

        let expression = parser
            .parse_expression()
            .expect("parenthesized expression should parse as expression");

        assert!(matches!(expression, ExpressionSyntax::Parenthesized(_)));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_parenthesized_expression_rejects_non_left_parenthesis_without_advancing() {
        let source = SourceText::new("42");
        let mut parser = Parser::new(&source);
        let current_token = parser.current_token();

        let parenthesized_expression = parser.parse_parenthesized_expression();

        assert_eq!(parenthesized_expression, None);
        assert_eq!(parser.current_token(), current_token);
    }

    #[test]
    fn parser_parses_unary_literal_expression() {
        let source = SourceText::new("-42");
        let mut parser = Parser::new(&source);

        let unary_expression = parser
            .parse_unary_expression()
            .expect("unary literal should parse");

        assert_eq!(unary_expression.operator_token().kind(), TokenKind::Minus);

        let ExpressionSyntax::Literal(literal_expression) = unary_expression.operand() else {
            panic!("expected literal operand");
        };

        assert_eq!(literal_expression.literal_token().kind(), TokenKind::Number);
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_parses_unary_name_expression() {
        let source = SourceText::new("!ready");
        let mut parser = Parser::new(&source);

        let unary_expression = parser
            .parse_unary_expression()
            .expect("unary name should parse");

        assert_eq!(
            unary_expression.operator_token().kind(),
            TokenKind::LogicalNot
        );

        assert!(matches!(
            unary_expression.operand(),
            ExpressionSyntax::Name(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_parses_nested_unary_expression() {
        let source = SourceText::new("--42");
        let mut parser = Parser::new(&source);

        let expression = parser
            .parse_expression()
            .expect("nested unary expression should parse");

        let ExpressionSyntax::Unary(outer) = expression else {
            panic!("expected outer unary expression");
        };

        assert_eq!(outer.operator_token().kind(), TokenKind::Minus);
        assert!(matches!(outer.operand(), ExpressionSyntax::Unary(_)));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_parses_parenthesized_expression_as_unary_operand() {
        let source = SourceText::new("-(42)");
        let mut parser = Parser::new(&source);

        let expression = parser
            .parse_expression()
            .expect("parenthesized unary operand should parse");

        let ExpressionSyntax::Unary(unary_expression) = expression else {
            panic!("expected unary expression");
        };

        assert!(matches!(
            unary_expression.operand(),
            ExpressionSyntax::Parenthesized(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_parses_unary_expression_as_expression() {
        let source = SourceText::new("+42");
        let mut parser = Parser::new(&source);

        let expression = parser
            .parse_expression()
            .expect("unary expression should parse as expression");

        assert!(matches!(expression, ExpressionSyntax::Unary(_)));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_unary_expression_rejects_non_unary_operator_without_advancing() {
        let source = SourceText::new("42");
        let mut parser = Parser::new(&source);
        let current_token = parser.current_token();

        let unary_expression = parser.parse_unary_expression();

        assert_eq!(unary_expression, None);
        assert_eq!(parser.current_token(), current_token);
    }

    #[test]
    fn parser_parses_call_expression_without_arguments() {
        let source = SourceText::new("foo()");
        let mut parser = Parser::new(&source);

        let expression = parser
            .parse_expression()
            .expect("call expression should parse");

        let ExpressionSyntax::Call(call_expression) = expression else {
            panic!("expected call expression");
        };

        assert!(matches!(
            call_expression.callee(),
            ExpressionSyntax::Name(_)
        ));
        assert_eq!(
            call_expression.left_parenthesis_token().kind(),
            TokenKind::LeftParenthesis
        );
        assert!(call_expression.arguments().is_empty());
        assert_eq!(
            call_expression.right_parenthesis_token().kind(),
            TokenKind::RightParenthesis
        );
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_parses_call_expression_with_single_argument() {
        let source = SourceText::new("foo(42)");
        let mut parser = Parser::new(&source);

        let expression = parser
            .parse_expression()
            .expect("call expression should parse");

        let ExpressionSyntax::Call(call_expression) = expression else {
            panic!("expected call expression");
        };

        assert_eq!(call_expression.arguments().len(), 1);
        assert!(matches!(
            call_expression.arguments()[0],
            ExpressionSyntax::Literal(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_parses_call_expression_with_multiple_arguments() {
        let source = SourceText::new("foo(1, 2)");
        let mut parser = Parser::new(&source);

        let expression = parser
            .parse_expression()
            .expect("call expression should parse");

        let ExpressionSyntax::Call(call_expression) = expression else {
            panic!("expected call expression");
        };

        assert_eq!(call_expression.arguments().len(), 2);
        assert!(matches!(
            call_expression.arguments()[0],
            ExpressionSyntax::Literal(_)
        ));
        assert!(matches!(
            call_expression.arguments()[1],
            ExpressionSyntax::Literal(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_parses_binary_literal_expression() {
        let source = SourceText::new("1 + 2");
        let mut parser = Parser::new(&source);

        let expression = parser
            .parse_expression()
            .expect("binary literal expression should parse");

        let ExpressionSyntax::Binary(binary_expression) = expression else {
            panic!("expected binary expression");
        };

        assert_eq!(binary_expression.operator_token().kind(), TokenKind::Plus);
        assert!(matches!(
            binary_expression.left(),
            ExpressionSyntax::Literal(_)
        ));
        assert!(matches!(
            binary_expression.right(),
            ExpressionSyntax::Literal(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_binary_expression_respects_multiplicative_precedence() {
        let source = SourceText::new("1 + 2 * 3");
        let mut parser = Parser::new(&source);

        let expression = parser
            .parse_expression()
            .expect("binary expression should parse");

        let ExpressionSyntax::Binary(addition) = expression else {
            panic!("expected addition expression");
        };

        assert_eq!(addition.operator_token().kind(), TokenKind::Plus);
        assert!(matches!(addition.left(), ExpressionSyntax::Literal(_)));

        let ExpressionSyntax::Binary(multiplication) = addition.right() else {
            panic!("expected multiplication on right side");
        };

        assert_eq!(multiplication.operator_token().kind(), TokenKind::Multiply);
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_binary_expression_is_left_associative() {
        let source = SourceText::new("10 - 3 - 2");
        let mut parser = Parser::new(&source);

        let expression = parser
            .parse_expression()
            .expect("binary expression should parse");

        let ExpressionSyntax::Binary(outer) = expression else {
            panic!("expected outer binary expression");
        };

        assert_eq!(outer.operator_token().kind(), TokenKind::Minus);
        assert!(matches!(outer.right(), ExpressionSyntax::Literal(_)));

        let ExpressionSyntax::Binary(inner) = outer.left() else {
            panic!("expected left-associated binary expression");
        };

        assert_eq!(inner.operator_token().kind(), TokenKind::Minus);
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_parses_assignment_expression() {
        let source = SourceText::new("answer = 42");
        let mut parser = Parser::new(&source);

        let expression = parser
            .parse_expression()
            .expect("assignment expression should parse");

        let ExpressionSyntax::Assignment(assignment) = expression else {
            panic!("expected assignment expression");
        };

        assert_eq!(assignment.identifier_token().kind(), TokenKind::Identifier);
        assert_eq!(assignment.assignment_token().kind(), TokenKind::Assignment);
        assert!(matches!(
            assignment.expression(),
            ExpressionSyntax::Literal(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_assignment_expression_accepts_binary_expression_on_right() {
        let source = SourceText::new("answer = 1 + 2");
        let mut parser = Parser::new(&source);

        let expression = parser
            .parse_expression()
            .expect("assignment with binary expression should parse");

        let ExpressionSyntax::Assignment(assignment) = expression else {
            panic!("expected assignment expression");
        };

        let ExpressionSyntax::Binary(binary) = assignment.expression() else {
            panic!("expected binary expression on assignment right side");
        };

        assert_eq!(binary.operator_token().kind(), TokenKind::Plus);
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_assignment_expression_is_right_associative() {
        let source = SourceText::new("a = b = 1");
        let mut parser = Parser::new(&source);

        let expression = parser
            .parse_expression()
            .expect("chained assignment expression should parse");

        let ExpressionSyntax::Assignment(outer_assignment) = expression else {
            panic!("expected outer assignment expression");
        };

        assert_eq!(
            outer_assignment.identifier_token().span(),
            SourceSpan::new(0, 1)
        );

        let ExpressionSyntax::Assignment(inner_assignment) = outer_assignment.expression() else {
            panic!("expected assignment expression on right side");
        };

        assert_eq!(
            inner_assignment.identifier_token().span(),
            SourceSpan::new(4, 1)
        );
        assert!(matches!(
            inner_assignment.expression(),
            ExpressionSyntax::Literal(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_binary_expression_binds_tighter_than_assignment_expression() {
        let source = SourceText::new("a = 1 + 2 * 3");
        let mut parser = Parser::new(&source);

        let expression = parser
            .parse_expression()
            .expect("assignment with binary expression should parse");

        let ExpressionSyntax::Assignment(assignment) = expression else {
            panic!("expected assignment expression");
        };

        let ExpressionSyntax::Binary(addition) = assignment.expression() else {
            panic!("expected addition on assignment right side");
        };

        assert_eq!(addition.operator_token().kind(), TokenKind::Plus);

        let ExpressionSyntax::Binary(multiplication) = addition.right() else {
            panic!("expected multiplication on addition right side");
        };

        assert_eq!(multiplication.operator_token().kind(), TokenKind::Multiply);
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_assignment_expression_requires_identifier_target() {
        let source = SourceText::new("1 = 2");
        let mut parser = Parser::new(&source);

        let expression = parser
            .parse_expression()
            .expect("left literal should parse as expression");

        assert!(matches!(expression, ExpressionSyntax::Literal(_)));
        assert_eq!(parser.current_token().kind(), TokenKind::Assignment);
    }

    #[test]
    fn parser_assignment_expression_rejects_parenthesized_target() {
        let source = SourceText::new("(a) = 1");
        let mut parser = Parser::new(&source);

        let expression = parser
            .parse_expression()
            .expect("parenthesized name should parse as expression");

        assert!(matches!(expression, ExpressionSyntax::Parenthesized(_)));
        assert_eq!(parser.current_token().kind(), TokenKind::Assignment);
    }

    #[test]
    fn parser_unary_expression_binds_tighter_than_binary_expression() {
        let source = SourceText::new("-1 + 2");
        let mut parser = Parser::new(&source);

        let expression = parser
            .parse_expression()
            .expect("unary and binary expression should parse");

        let ExpressionSyntax::Binary(binary_expression) = expression else {
            panic!("expected binary expression");
        };

        assert_eq!(binary_expression.operator_token().kind(), TokenKind::Plus);
        assert!(matches!(
            binary_expression.left(),
            ExpressionSyntax::Unary(_)
        ));
        assert!(matches!(
            binary_expression.right(),
            ExpressionSyntax::Literal(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_parentheses_override_binary_precedence() {
        let source = SourceText::new("(1 + 2) * 3");
        let mut parser = Parser::new(&source);

        let expression = parser
            .parse_expression()
            .expect("parenthesized binary expression should parse");

        let ExpressionSyntax::Binary(multiplication) = expression else {
            panic!("expected multiplication expression");
        };

        assert_eq!(multiplication.operator_token().kind(), TokenKind::Multiply);

        let ExpressionSyntax::Parenthesized(parenthesized) = multiplication.left() else {
            panic!("expected parenthesized left operand");
        };

        let ExpressionSyntax::Binary(addition) = parenthesized.expression() else {
            panic!("expected addition inside parentheses");
        };

        assert_eq!(addition.operator_token().kind(), TokenKind::Plus);
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_comparison_binds_tighter_than_equality() {
        let source = SourceText::new("1 < 2 == true");
        let mut parser = Parser::new(&source);

        let expression = parser
            .parse_expression()
            .expect("comparison and equality expression should parse");

        let ExpressionSyntax::Binary(equality) = expression else {
            panic!("expected equality expression");
        };

        assert_eq!(equality.operator_token().kind(), TokenKind::Equal);

        let ExpressionSyntax::Binary(comparison) = equality.left() else {
            panic!("expected comparison on left side");
        };

        assert_eq!(comparison.operator_token().kind(), TokenKind::LessThan);
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_logical_and_binds_tighter_than_logical_or() {
        let source = SourceText::new("true || false && true");
        let mut parser = Parser::new(&source);

        let expression = parser
            .parse_expression()
            .expect("logical expression should parse");

        let ExpressionSyntax::Binary(logical_or) = expression else {
            panic!("expected logical-or expression");
        };

        assert_eq!(logical_or.operator_token().kind(), TokenKind::LogicalOr);

        let ExpressionSyntax::Binary(logical_and) = logical_or.right() else {
            panic!("expected logical-and expression on right side");
        };

        assert_eq!(logical_and.operator_token().kind(), TokenKind::LogicalAnd);
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_arithmetic_binds_tighter_than_comparison() {
        let source = SourceText::new("1 + 2 < 4");
        let mut parser = Parser::new(&source);

        let expression = parser
            .parse_expression()
            .expect("arithmetic and comparison expression should parse");

        let ExpressionSyntax::Binary(comparison) = expression else {
            panic!("expected comparison expression");
        };

        assert_eq!(comparison.operator_token().kind(), TokenKind::LessThan);

        let ExpressionSyntax::Binary(addition) = comparison.left() else {
            panic!("expected addition on left side");
        };

        assert_eq!(addition.operator_token().kind(), TokenKind::Plus);
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }
}
