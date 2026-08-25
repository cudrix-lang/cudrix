//! Parser infrastructure for Forge source code.

use crate::compiler::lexer::{Lexer, Token, TokenKind};
use crate::source::SourceText;
use crate::syntax::{LiteralExpressionSyntax, NameExpressionSyntax};

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
}
