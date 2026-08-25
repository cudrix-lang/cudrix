//! Parser infrastructure for Forge source code.

use crate::compiler::lexer::{Lexer, Token, TokenKind};
use crate::source::SourceText;

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
    use crate::compiler::lexer::TokenKind;

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
}
