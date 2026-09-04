//! Parser infrastructure for Cudrix source code.

use crate::compiler::lexer::{Lexer, Token, TokenKind};
use crate::source::SourceText;
use crate::syntax::{
    ArrayLiteralExpressionSyntax, AssignmentExpressionSyntax, BinaryExpressionSyntax,
    CallExpressionSyntax, ConditionalExpressionSyntax, ExpressionSyntax, IndexExpressionSyntax,
    LiteralExpressionSyntax, MemberAccessExpressionSyntax, NameExpressionSyntax,
    ObjectLiteralExpressionSyntax, ObjectMemberSyntax, ParenthesizedExpressionSyntax,
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
        let left = self.parse_conditional_expression()?;

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

    fn parse_conditional_expression(&mut self) -> Option<ExpressionSyntax> {
        let condition = self.parse_binary_expression(0)?;

        if self.current_token.kind() != TokenKind::QuestionMark {
            return Some(condition);
        }

        let question_token = self.current_token;
        self.advance();

        let when_true = self.parse_expression()?;

        if self.current_token.kind() != TokenKind::Colon {
            return None;
        }

        let colon_token = self.current_token;
        self.advance();

        let when_false = self.parse_conditional_expression()?;

        Some(ExpressionSyntax::Conditional(
            ConditionalExpressionSyntax::new(
                condition,
                question_token,
                when_true,
                colon_token,
                when_false,
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

            _ => self.parse_postfix_expression(),
        }
    }

    fn parse_postfix_expression(&mut self) -> Option<ExpressionSyntax> {
        let mut expression = self.parse_primary_expression()?;

        loop {
            match self.current_token.kind() {
                TokenKind::LeftParenthesis => {
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

                TokenKind::Dot => {
                    let dot_token = self.current_token;
                    self.advance();

                    if self.current_token.kind() != TokenKind::Identifier {
                        return None;
                    }

                    let member_name_token = self.current_token;
                    self.advance();

                    expression = ExpressionSyntax::MemberAccess(MemberAccessExpressionSyntax::new(
                        expression,
                        dot_token,
                        member_name_token,
                    ));
                }

                TokenKind::LeftBracket => {
                    let left_bracket_token = self.current_token;
                    self.advance();

                    let index = self.parse_expression()?;

                    if self.current_token.kind() != TokenKind::RightBracket {
                        return None;
                    }

                    let right_bracket_token = self.current_token;
                    self.advance();

                    expression = ExpressionSyntax::Index(IndexExpressionSyntax::new(
                        expression,
                        left_bracket_token,
                        index,
                        right_bracket_token,
                    ));
                }

                _ => break,
            }
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

            TokenKind::LeftBracket => self
                .parse_array_literal_expression()
                .map(ExpressionSyntax::ArrayLiteral),

            TokenKind::LeftBrace => self
                .parse_object_literal_expression()
                .map(ExpressionSyntax::ObjectLiteral),

            _ => None,
        }
    }

    fn parse_array_literal_expression(&mut self) -> Option<ArrayLiteralExpressionSyntax> {
        if self.current_token.kind() != TokenKind::LeftBracket {
            return None;
        }

        let left_bracket_token = self.current_token;
        self.advance();

        let mut elements = Vec::new();

        if self.current_token.kind() != TokenKind::RightBracket {
            loop {
                elements.push(self.parse_expression()?);

                if self.current_token.kind() != TokenKind::Comma {
                    break;
                }

                self.advance();
            }
        }

        if self.current_token.kind() != TokenKind::RightBracket {
            return None;
        }

        let right_bracket_token = self.current_token;
        self.advance();

        Some(ArrayLiteralExpressionSyntax::new(
            left_bracket_token,
            elements,
            right_bracket_token,
        ))
    }

    fn parse_object_literal_expression(&mut self) -> Option<ObjectLiteralExpressionSyntax> {
        if self.current_token.kind() != TokenKind::LeftBrace {
            return None;
        }

        let left_brace_token = self.current_token;
        self.advance();

        let mut members = Vec::new();

        if self.current_token.kind() != TokenKind::RightBrace {
            loop {
                if self.current_token.kind() != TokenKind::Identifier {
                    return None;
                }

                let name_token = self.current_token;
                self.advance();

                if self.current_token.kind() != TokenKind::Colon {
                    return None;
                }

                let colon_token = self.current_token;
                self.advance();

                let value = self.parse_expression()?;

                members.push(ObjectMemberSyntax::new(name_token, colon_token, value));

                if self.current_token.kind() != TokenKind::Comma {
                    break;
                }

                self.advance();
            }
        }

        if self.current_token.kind() != TokenKind::RightBrace {
            return None;
        }

        let right_brace_token = self.current_token;
        self.advance();

        Some(ObjectLiteralExpressionSyntax::new(
            left_brace_token,
            members,
            right_brace_token,
        ))
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
    fn parser_call_expression_accepts_expression_argument() {
        let source = SourceText::new("foo(x + 1)");
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
            ExpressionSyntax::Binary(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_call_expression_accepts_assignment_argument() {
        let source = SourceText::new("foo(x = 1)");
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
            ExpressionSyntax::Assignment(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_call_expression_supports_parenthesized_callee() {
        let source = SourceText::new("(foo)(42)");
        let mut parser = Parser::new(&source);

        let expression = parser
            .parse_expression()
            .expect("parenthesized call expression should parse");

        let ExpressionSyntax::Call(call_expression) = expression else {
            panic!("expected call expression");
        };

        assert!(matches!(
            call_expression.callee(),
            ExpressionSyntax::Parenthesized(_)
        ));
        assert_eq!(call_expression.arguments().len(), 1);
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_call_expression_supports_chained_calls() {
        let source = SourceText::new("foo()(42)");
        let mut parser = Parser::new(&source);

        let expression = parser
            .parse_expression()
            .expect("chained call expression should parse");

        let ExpressionSyntax::Call(outer_call) = expression else {
            panic!("expected outer call expression");
        };

        assert!(matches!(outer_call.callee(), ExpressionSyntax::Call(_)));
        assert_eq!(outer_call.arguments().len(), 1);
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_call_expression_binds_tighter_than_unary_expression() {
        let source = SourceText::new("-foo()");
        let mut parser = Parser::new(&source);

        let expression = parser
            .parse_expression()
            .expect("unary call expression should parse");

        let ExpressionSyntax::Unary(unary_expression) = expression else {
            panic!("expected unary expression");
        };

        assert_eq!(unary_expression.operator_token().kind(), TokenKind::Minus);
        assert!(matches!(
            unary_expression.operand(),
            ExpressionSyntax::Call(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_call_expression_binds_tighter_than_binary_expression() {
        let source = SourceText::new("foo() + 1");
        let mut parser = Parser::new(&source);

        let expression = parser
            .parse_expression()
            .expect("binary expression containing call should parse");

        let ExpressionSyntax::Binary(binary_expression) = expression else {
            panic!("expected binary expression");
        };

        assert_eq!(binary_expression.operator_token().kind(), TokenKind::Plus);
        assert!(matches!(
            binary_expression.left(),
            ExpressionSyntax::Call(_)
        ));
        assert!(matches!(
            binary_expression.right(),
            ExpressionSyntax::Literal(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_assignment_expression_accepts_call_expression_on_right() {
        let source = SourceText::new("result = foo()");
        let mut parser = Parser::new(&source);

        let expression = parser
            .parse_expression()
            .expect("assignment expression containing call should parse");

        let ExpressionSyntax::Assignment(assignment_expression) = expression else {
            panic!("expected assignment expression");
        };

        assert!(matches!(
            assignment_expression.expression(),
            ExpressionSyntax::Call(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_call_expression_skips_trivia_between_callee_and_parenthesis() {
        let source = SourceText::new("foo /* comment */ (42)");
        let mut parser = Parser::new(&source);

        let expression = parser
            .parse_expression()
            .expect("call expression separated by trivia should parse");

        assert!(matches!(expression, ExpressionSyntax::Call(_)));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_call_expression_skips_trivia_between_arguments() {
        let source = SourceText::new("foo(1, /* comment */ 2)");
        let mut parser = Parser::new(&source);

        let expression = parser
            .parse_expression()
            .expect("call expression containing trivia should parse");

        let ExpressionSyntax::Call(call_expression) = expression else {
            panic!("expected call expression");
        };

        assert_eq!(call_expression.arguments().len(), 2);
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_call_expression_returns_none_without_right_parenthesis() {
        let source = SourceText::new("foo(42");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression();

        assert_eq!(expression, None);
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_call_expression_returns_none_without_argument_after_comma() {
        let source = SourceText::new("foo(1,)");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression();

        assert_eq!(expression, None);
        assert_eq!(parser.current_token().kind(), TokenKind::RightParenthesis);
    }

    #[test]
    fn parser_parses_member_access_expression() {
        let source = SourceText::new("object.member");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression().unwrap();

        let ExpressionSyntax::MemberAccess(member_access) = expression else {
            panic!("expected member access expression");
        };

        assert!(matches!(
            member_access.expression(),
            ExpressionSyntax::Name(_)
        ));
        assert_eq!(member_access.dot_token().kind(), TokenKind::Dot);
        assert_eq!(
            member_access.member_name_token().kind(),
            TokenKind::Identifier
        );
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_member_access_expression_supports_chaining() {
        let source = SourceText::new("object.member.child");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression().unwrap();

        let ExpressionSyntax::MemberAccess(outer_member_access) = expression else {
            panic!("expected outer member access expression");
        };

        assert!(matches!(
            outer_member_access.expression(),
            ExpressionSyntax::MemberAccess(_)
        ));
        assert_eq!(
            outer_member_access.member_name_token().span(),
            SourceSpan::new(14, 5)
        );
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_member_access_expression_supports_member_after_call() {
        let source = SourceText::new("foo().member");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression().unwrap();

        let ExpressionSyntax::MemberAccess(member_access) = expression else {
            panic!("expected member access expression");
        };

        assert!(matches!(
            member_access.expression(),
            ExpressionSyntax::Call(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_member_access_expression_supports_call_after_member() {
        let source = SourceText::new("object.method()");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression().unwrap();

        let ExpressionSyntax::Call(call) = expression else {
            panic!("expected call expression");
        };

        assert!(matches!(call.callee(), ExpressionSyntax::MemberAccess(_)));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_member_access_and_call_expressions_compose_left_to_right() {
        let source = SourceText::new("foo().bar()");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression().unwrap();

        let ExpressionSyntax::Call(call) = expression else {
            panic!("expected outer call expression");
        };

        let ExpressionSyntax::MemberAccess(member_access) = call.callee() else {
            panic!("expected member access callee");
        };

        assert!(matches!(
            member_access.expression(),
            ExpressionSyntax::Call(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_member_access_expression_supports_parenthesized_expression() {
        let source = SourceText::new("(object).member");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression().unwrap();

        let ExpressionSyntax::MemberAccess(member_access) = expression else {
            panic!("expected member access expression");
        };

        assert!(matches!(
            member_access.expression(),
            ExpressionSyntax::Parenthesized(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_member_access_expression_binds_tighter_than_unary_expression() {
        let source = SourceText::new("-object.member");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression().unwrap();

        let ExpressionSyntax::Unary(unary) = expression else {
            panic!("expected unary expression");
        };

        assert!(matches!(unary.operand(), ExpressionSyntax::MemberAccess(_)));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_member_access_expression_binds_tighter_than_binary_expression() {
        let source = SourceText::new("object.member + 1");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression().unwrap();

        let ExpressionSyntax::Binary(binary) = expression else {
            panic!("expected binary expression");
        };

        assert!(matches!(binary.left(), ExpressionSyntax::MemberAccess(_)));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_assignment_expression_accepts_member_access_expression_on_right() {
        let source = SourceText::new("result = object.member");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression().unwrap();

        let ExpressionSyntax::Assignment(assignment) = expression else {
            panic!("expected assignment expression");
        };

        assert!(matches!(
            assignment.expression(),
            ExpressionSyntax::MemberAccess(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_member_access_expression_skips_trivia_around_dot() {
        let source = SourceText::new("object /* comment */ . /* comment */ member");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression().unwrap();

        assert!(matches!(expression, ExpressionSyntax::MemberAccess(_)));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_member_access_expression_returns_none_without_member_name() {
        let source = SourceText::new("object.");
        let mut parser = Parser::new(&source);

        assert!(parser.parse_expression().is_none());
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_member_access_expression_returns_none_for_non_identifier_member_name() {
        let source = SourceText::new("object.42");
        let mut parser = Parser::new(&source);

        assert!(parser.parse_expression().is_none());
        assert_eq!(parser.current_token().kind(), TokenKind::Number);
    }

    #[test]
    fn parser_parses_index_expression() {
        let source = SourceText::new("items[0]");
        let mut parser = Parser::new(&source);

        let expression = parser
            .parse_expression()
            .expect("index expression should parse");

        let ExpressionSyntax::Index(index_expression) = expression else {
            panic!("expected index expression");
        };

        assert!(matches!(
            index_expression.expression(),
            ExpressionSyntax::Name(_)
        ));
        assert_eq!(
            index_expression.left_bracket_token().kind(),
            TokenKind::LeftBracket
        );
        assert!(matches!(
            index_expression.index(),
            ExpressionSyntax::Literal(_)
        ));
        assert_eq!(
            index_expression.right_bracket_token().kind(),
            TokenKind::RightBracket
        );
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_parses_empty_array_literal_expression() {
        let source = SourceText::new("[]");
        let mut parser = Parser::new(&source);

        let expression = parser
            .parse_expression()
            .expect("empty array literal should parse");

        let ExpressionSyntax::ArrayLiteral(array_literal) = expression else {
            panic!("expected array literal expression");
        };

        assert_eq!(
            array_literal.left_bracket_token().kind(),
            TokenKind::LeftBracket
        );
        assert!(array_literal.elements().is_empty());
        assert_eq!(
            array_literal.right_bracket_token().kind(),
            TokenKind::RightBracket
        );
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_parses_empty_object_literal_expression() {
        let source = SourceText::new("{}");
        let mut parser = Parser::new(&source);

        let expression = parser
            .parse_expression()
            .expect("empty object literal should parse");

        let ExpressionSyntax::ObjectLiteral(object_literal) = expression else {
            panic!("expected object literal expression");
        };

        assert_eq!(
            object_literal.left_brace_token().kind(),
            TokenKind::LeftBrace
        );
        assert!(object_literal.members().is_empty());
        assert_eq!(
            object_literal.right_brace_token().kind(),
            TokenKind::RightBrace
        );
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_parses_object_literal_expression_with_multiple_members() {
        let source = SourceText::new("{name: value, age: 42}");
        let mut parser = Parser::new(&source);

        let expression = parser
            .parse_expression()
            .expect("object literal should parse");

        let ExpressionSyntax::ObjectLiteral(object_literal) = expression else {
            panic!("expected object literal expression");
        };

        assert_eq!(object_literal.members().len(), 2);

        assert_eq!(
            object_literal.members()[0].name_token().kind(),
            TokenKind::Identifier
        );
        assert_eq!(
            object_literal.members()[0].colon_token().kind(),
            TokenKind::Colon
        );
        assert!(matches!(
            object_literal.members()[0].value(),
            ExpressionSyntax::Name(_)
        ));

        assert_eq!(
            object_literal.members()[1].name_token().kind(),
            TokenKind::Identifier
        );
        assert_eq!(
            object_literal.members()[1].colon_token().kind(),
            TokenKind::Colon
        );
        assert!(matches!(
            object_literal.members()[1].value(),
            ExpressionSyntax::Literal(_)
        ));

        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_object_literal_expression_accepts_expression_values() {
        let source = SourceText::new("{name: first + last, result: foo()}");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression().unwrap();

        let ExpressionSyntax::ObjectLiteral(object_literal) = expression else {
            panic!("expected object literal expression");
        };

        assert_eq!(object_literal.members().len(), 2);
        assert!(matches!(
            object_literal.members()[0].value(),
            ExpressionSyntax::Binary(_)
        ));
        assert!(matches!(
            object_literal.members()[1].value(),
            ExpressionSyntax::Call(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_object_literal_expression_supports_nesting() {
        let source = SourceText::new("{nested: {value: 1}, items: [1, 2, 3]}");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression().unwrap();

        let ExpressionSyntax::ObjectLiteral(object_literal) = expression else {
            panic!("expected outer object literal expression");
        };

        assert_eq!(object_literal.members().len(), 2);
        assert!(matches!(
            object_literal.members()[0].value(),
            ExpressionSyntax::ObjectLiteral(_)
        ));
        assert!(matches!(
            object_literal.members()[1].value(),
            ExpressionSyntax::ArrayLiteral(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_object_literal_expression_skips_trivia() {
        let source = SourceText::new(
            "{ /* comment */ name /* comment */ : /* comment */ 42 /* comment */ }",
        );
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression().unwrap();

        let ExpressionSyntax::ObjectLiteral(object_literal) = expression else {
            panic!("expected object literal expression");
        };

        assert_eq!(object_literal.members().len(), 1);
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_object_literal_expression_returns_none_without_value() {
        let source = SourceText::new("{name:}");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression();

        assert_eq!(expression, None);
        assert_eq!(parser.current_token().kind(), TokenKind::RightBrace);
    }

    #[test]
    fn parser_object_literal_expression_returns_none_without_colon() {
        let source = SourceText::new("{name 42}");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression();

        assert_eq!(expression, None);
        assert_eq!(parser.current_token().kind(), TokenKind::Number);
    }

    #[test]
    fn parser_object_literal_expression_returns_none_without_member_after_comma() {
        let source = SourceText::new("{name: 42,}");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression();

        assert_eq!(expression, None);
        assert_eq!(parser.current_token().kind(), TokenKind::RightBrace);
    }

    #[test]
    fn parser_object_literal_expression_returns_none_without_right_brace() {
        let source = SourceText::new("{name: 42");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression();

        assert_eq!(expression, None);
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_object_literal_expression_returns_none_for_non_identifier_member_name() {
        let source = SourceText::new("{42: value}");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression();

        assert_eq!(expression, None);
        assert_eq!(parser.current_token().kind(), TokenKind::Number);
    }

    #[test]
    fn parser_parses_array_literal_expression_with_multiple_elements() {
        let source = SourceText::new("[1, 2, 3]");
        let mut parser = Parser::new(&source);

        let expression = parser
            .parse_expression()
            .expect("array literal should parse");

        let ExpressionSyntax::ArrayLiteral(array_literal) = expression else {
            panic!("expected array literal expression");
        };

        assert_eq!(array_literal.elements().len(), 3);
        assert!(
            array_literal
                .elements()
                .iter()
                .all(|element| matches!(element, ExpressionSyntax::Literal(_)))
        );
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_array_literal_expression_accepts_expression_elements() {
        let source = SourceText::new("[a + b, foo()]");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression().unwrap();

        let ExpressionSyntax::ArrayLiteral(array_literal) = expression else {
            panic!("expected array literal expression");
        };

        assert_eq!(array_literal.elements().len(), 2);
        assert!(matches!(
            array_literal.elements()[0],
            ExpressionSyntax::Binary(_)
        ));
        assert!(matches!(
            array_literal.elements()[1],
            ExpressionSyntax::Call(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_array_literal_expression_supports_nesting() {
        let source = SourceText::new("[[1, 2], [3, 4]]");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression().unwrap();

        let ExpressionSyntax::ArrayLiteral(array_literal) = expression else {
            panic!("expected outer array literal expression");
        };

        assert_eq!(array_literal.elements().len(), 2);
        assert!(
            array_literal
                .elements()
                .iter()
                .all(|element| matches!(element, ExpressionSyntax::ArrayLiteral(_)))
        );
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_array_literal_expression_supports_index_postfix() {
        let source = SourceText::new("[1, 2][0]");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression().unwrap();

        let ExpressionSyntax::Index(index_expression) = expression else {
            panic!("expected index expression");
        };

        assert!(matches!(
            index_expression.expression(),
            ExpressionSyntax::ArrayLiteral(_)
        ));
        assert!(matches!(
            index_expression.index(),
            ExpressionSyntax::Literal(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_assignment_expression_accepts_array_literal_on_right() {
        let source = SourceText::new("value = [1, 2]");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression().unwrap();

        let ExpressionSyntax::Assignment(assignment_expression) = expression else {
            panic!("expected assignment expression");
        };

        assert!(matches!(
            assignment_expression.expression(),
            ExpressionSyntax::ArrayLiteral(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_array_literal_expression_participates_in_binary_expression() {
        let source = SourceText::new("[1, 2] + other");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression().unwrap();

        let ExpressionSyntax::Binary(binary_expression) = expression else {
            panic!("expected binary expression");
        };

        assert!(matches!(
            binary_expression.left(),
            ExpressionSyntax::ArrayLiteral(_)
        ));
        assert!(matches!(
            binary_expression.right(),
            ExpressionSyntax::Name(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_array_literal_expression_skips_trivia() {
        let source = SourceText::new("[ /* comment */ 1, /* comment */ 2 /* comment */ ]");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression().unwrap();

        let ExpressionSyntax::ArrayLiteral(array_literal) = expression else {
            panic!("expected array literal expression");
        };

        assert_eq!(array_literal.elements().len(), 2);
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_array_literal_expression_returns_none_without_element_after_comma() {
        let source = SourceText::new("[1,]");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression();

        assert_eq!(expression, None);
        assert_eq!(parser.current_token().kind(), TokenKind::RightBracket);
    }

    #[test]
    fn parser_array_literal_expression_returns_none_without_right_bracket() {
        let source = SourceText::new("[1");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression();

        assert_eq!(expression, None);
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_array_literal_expression_returns_none_for_missing_first_element() {
        let source = SourceText::new("[,]");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression();

        assert_eq!(expression, None);
        assert_eq!(parser.current_token().kind(), TokenKind::Comma);
    }

    #[test]
    fn parser_array_literal_expression_returns_none_for_consecutive_commas() {
        let source = SourceText::new("[1,,2]");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression();

        assert_eq!(expression, None);
        assert_eq!(parser.current_token().kind(), TokenKind::Comma);
    }

    #[test]
    fn parser_index_expression_accepts_name_index() {
        let source = SourceText::new("items[index]");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression().unwrap();

        let ExpressionSyntax::Index(index_expression) = expression else {
            panic!("expected index expression");
        };

        assert!(matches!(
            index_expression.index(),
            ExpressionSyntax::Name(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_index_expression_supports_chaining() {
        let source = SourceText::new("matrix[row][column]");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression().unwrap();

        let ExpressionSyntax::Index(outer_index) = expression else {
            panic!("expected outer index expression");
        };

        assert!(matches!(
            outer_index.expression(),
            ExpressionSyntax::Index(_)
        ));
        assert!(matches!(outer_index.index(), ExpressionSyntax::Name(_)));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_index_expression_supports_index_after_member_access() {
        let source = SourceText::new("object.items[0]");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression().unwrap();

        let ExpressionSyntax::Index(index_expression) = expression else {
            panic!("expected index expression");
        };

        assert!(matches!(
            index_expression.expression(),
            ExpressionSyntax::MemberAccess(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_index_expression_supports_index_after_call() {
        let source = SourceText::new("foo()[index]");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression().unwrap();

        let ExpressionSyntax::Index(index_expression) = expression else {
            panic!("expected index expression");
        };

        assert!(matches!(
            index_expression.expression(),
            ExpressionSyntax::Call(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_index_expression_supports_member_access_after_index() {
        let source = SourceText::new("items[0].member");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression().unwrap();

        let ExpressionSyntax::MemberAccess(member_access) = expression else {
            panic!("expected member access expression");
        };

        assert!(matches!(
            member_access.expression(),
            ExpressionSyntax::Index(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_index_expression_supports_call_after_index() {
        let source = SourceText::new("items[0]()");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression().unwrap();

        let ExpressionSyntax::Call(call_expression) = expression else {
            panic!("expected call expression");
        };

        assert!(matches!(
            call_expression.callee(),
            ExpressionSyntax::Index(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_index_expression_accepts_binary_expression_index() {
        let source = SourceText::new("items[a + b]");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression().unwrap();

        let ExpressionSyntax::Index(index_expression) = expression else {
            panic!("expected index expression");
        };

        assert!(matches!(
            index_expression.index(),
            ExpressionSyntax::Binary(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_index_expression_accepts_assignment_expression_index() {
        let source = SourceText::new("items[x = y]");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression().unwrap();

        let ExpressionSyntax::Index(index_expression) = expression else {
            panic!("expected index expression");
        };

        assert!(matches!(
            index_expression.index(),
            ExpressionSyntax::Assignment(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_index_expression_binds_tighter_than_unary_expression() {
        let source = SourceText::new("-items[0]");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression().unwrap();

        let ExpressionSyntax::Unary(unary_expression) = expression else {
            panic!("expected unary expression");
        };

        assert!(matches!(
            unary_expression.operand(),
            ExpressionSyntax::Index(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_index_expression_binds_tighter_than_binary_expression() {
        let source = SourceText::new("items[0] + 1");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression().unwrap();

        let ExpressionSyntax::Binary(binary_expression) = expression else {
            panic!("expected binary expression");
        };

        assert!(matches!(
            binary_expression.left(),
            ExpressionSyntax::Index(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_assignment_expression_accepts_index_expression_on_right() {
        let source = SourceText::new("value = items[0]");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression().unwrap();

        let ExpressionSyntax::Assignment(assignment_expression) = expression else {
            panic!("expected assignment expression");
        };

        assert!(matches!(
            assignment_expression.expression(),
            ExpressionSyntax::Index(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_index_expression_does_not_become_assignment_target() {
        let source = SourceText::new("items[0] = value");
        let mut parser = Parser::new(&source);

        let expression = parser
            .parse_expression()
            .expect("left index should parse as expression");

        assert!(matches!(expression, ExpressionSyntax::Index(_)));
        assert_eq!(parser.current_token().kind(), TokenKind::Assignment);
    }

    #[test]
    fn parser_index_expression_skips_trivia_inside_brackets() {
        let source = SourceText::new("items /* comment */ [ /* comment */ 0 /* comment */ ]");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression().unwrap();

        assert!(matches!(expression, ExpressionSyntax::Index(_)));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_index_expression_returns_none_without_index() {
        let source = SourceText::new("items[]");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression();

        assert_eq!(expression, None);
        assert_eq!(parser.current_token().kind(), TokenKind::RightBracket);
    }

    #[test]
    fn parser_index_expression_returns_none_without_index_at_end_of_file() {
        let source = SourceText::new("items[");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression();

        assert_eq!(expression, None);
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_index_expression_returns_none_without_right_bracket() {
        let source = SourceText::new("items[0");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression();

        assert_eq!(expression, None);
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

    #[test]
    fn parser_parses_conditional_expression() {
        let source = SourceText::new("condition ? when_true : when_false");
        let mut parser = Parser::new(&source);

        let expression = parser
            .parse_expression()
            .expect("conditional expression should parse");

        let ExpressionSyntax::Conditional(conditional) = expression else {
            panic!("expected conditional expression");
        };

        assert!(matches!(conditional.condition(), ExpressionSyntax::Name(_)));
        assert_eq!(conditional.question_token().kind(), TokenKind::QuestionMark);
        assert!(matches!(conditional.when_true(), ExpressionSyntax::Name(_)));
        assert_eq!(conditional.colon_token().kind(), TokenKind::Colon);
        assert!(matches!(
            conditional.when_false(),
            ExpressionSyntax::Name(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_binary_expression_binds_tighter_than_conditional_expression() {
        let source = SourceText::new("score > 90 ? 1 : 0");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression().unwrap();

        let ExpressionSyntax::Conditional(conditional) = expression else {
            panic!("expected conditional expression");
        };

        let ExpressionSyntax::Binary(condition) = conditional.condition() else {
            panic!("expected binary condition");
        };

        assert_eq!(condition.operator_token().kind(), TokenKind::GreaterThan);
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_conditional_expression_binds_tighter_than_assignment_expression() {
        let source = SourceText::new("result = condition ? when_true : when_false");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression().unwrap();

        let ExpressionSyntax::Assignment(assignment) = expression else {
            panic!("expected assignment expression");
        };

        assert!(matches!(
            assignment.expression(),
            ExpressionSyntax::Conditional(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_conditional_expression_is_right_associative() {
        let source = SourceText::new("a ? b : c ? d : e");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression().unwrap();

        let ExpressionSyntax::Conditional(outer) = expression else {
            panic!("expected outer conditional expression");
        };

        assert!(matches!(outer.condition(), ExpressionSyntax::Name(_)));
        assert!(matches!(outer.when_true(), ExpressionSyntax::Name(_)));
        assert!(matches!(
            outer.when_false(),
            ExpressionSyntax::Conditional(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_conditional_expression_accepts_assignment_in_true_branch() {
        let source = SourceText::new("condition ? value = 1 : fallback");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression().unwrap();

        let ExpressionSyntax::Conditional(conditional) = expression else {
            panic!("expected conditional expression");
        };

        assert!(matches!(
            conditional.when_true(),
            ExpressionSyntax::Assignment(_)
        ));
        assert!(matches!(
            conditional.when_false(),
            ExpressionSyntax::Name(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_conditional_expression_does_not_become_assignment_target() {
        let source = SourceText::new("condition ? first : second = value");
        let mut parser = Parser::new(&source);

        let expression = parser
            .parse_expression()
            .expect("conditional expression should parse");

        assert!(matches!(expression, ExpressionSyntax::Conditional(_)));
        assert_eq!(parser.current_token().kind(), TokenKind::Assignment);
    }

    #[test]
    fn parser_conditional_expression_skips_trivia() {
        let source =
            SourceText::new("condition /* comment */ ? /* comment */ first /* comment */ : second");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression().unwrap();

        assert!(matches!(expression, ExpressionSyntax::Conditional(_)));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_conditional_expression_returns_none_without_true_branch() {
        let source = SourceText::new("condition ? : fallback");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression();

        assert_eq!(expression, None);
        assert_eq!(parser.current_token().kind(), TokenKind::Colon);
    }

    #[test]
    fn parser_conditional_expression_returns_none_without_colon() {
        let source = SourceText::new("condition ? when_true");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression();

        assert_eq!(expression, None);
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_conditional_expression_returns_none_without_false_branch() {
        let source = SourceText::new("condition ? when_true :");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression();

        assert_eq!(expression, None);
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }
}
