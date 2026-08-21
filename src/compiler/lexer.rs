use crate::source::{SourceSpan, SourceText};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    EndOfFile,
    Identifier,
    Number,
    Whitespace,
    LineComment,
    BlockComment,
    UnterminatedBlockComment,
    String,
    UnterminatedString,

    LetKeyword,
    VarKeyword,
    FunctionKeyword,
    ImplementationKeyword,

    ReturnKeyword,
    IfKeyword,
    ElseIfKeyword,
    ElseKeyword,
    WhileKeyword,
    ForKeyword,
    BreakKeyword,
    ContinueKeyword,

    TrueKeyword,
    FalseKeyword,

    LeftParenthesis,
    RightParenthesis,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Colon,
    Semicolon,
    Dot,

    Assignment,
    Plus,
    Minus,
    Multiply,
    Divide,
    Remainder,

    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,

    LogicalNot,
    LogicalAnd,
    LogicalOr,

    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token {
    kind: TokenKind,
    span: SourceSpan,
}

impl Token {
    pub fn new(kind: TokenKind, span: SourceSpan) -> Self {
        Self { kind, span }
    }

    pub fn kind(&self) -> TokenKind {
        self.kind
    }

    pub fn span(&self) -> SourceSpan {
        self.span
    }
}

pub struct Lexer<'source> {
    source: &'source SourceText,
    start: usize,
    position: usize,
}

impl<'source> Lexer<'source> {
    pub fn new(source: &'source SourceText) -> Self {
        Self {
            source,
            start: 0,
            position: 0,
        }
    }

    pub fn position(&self) -> usize {
        self.position
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn mark_start(&mut self) {
        self.start = self.position;
    }

    pub fn span(&self) -> SourceSpan {
        SourceSpan::new(self.start, self.position - self.start)
    }

    pub fn is_at_end(&self) -> bool {
        self.position == self.source.len()
    }

    pub fn char_at_offset(&self, offset: usize) -> Option<char> {
        self.source.as_str()[self.position..].chars().nth(offset)
    }

    pub fn current(&self) -> Option<char> {
        self.char_at_offset(0)
    }

    pub fn peek(&self) -> Option<char> {
        self.char_at_offset(1)
    }

    pub fn advance(&mut self) -> Option<char> {
        let current = self.current()?;

        self.position += current.len_utf8();

        Some(current)
    }

    pub fn next_token(&mut self) -> Token {
        self.mark_start();

        if self.is_at_end() {
            return Token::new(TokenKind::EndOfFile, self.span());
        }

        let current = self.current().expect("lexer is not at end of source");

        if current.is_alphabetic() || current == '_' {
            self.scan_identifier();

            return Token::new(self.identifier_kind(), self.span());
        }

        if current.is_ascii_digit() {
            self.scan_number();

            return Token::new(TokenKind::Number, self.span());
        }

        if current.is_whitespace() {
            self.scan_whitespace();

            return Token::new(TokenKind::Whitespace, self.span());
        }

        if current == '"' {
            let kind = if self.scan_string() {
                TokenKind::String
            } else {
                TokenKind::UnterminatedString
            };

            return Token::new(kind, self.span());
        }

        let punctuation_kind = match current {
            '(' => Some(TokenKind::LeftParenthesis),
            ')' => Some(TokenKind::RightParenthesis),
            '{' => Some(TokenKind::LeftBrace),
            '}' => Some(TokenKind::RightBrace),
            '[' => Some(TokenKind::LeftBracket),
            ']' => Some(TokenKind::RightBracket),
            ',' => Some(TokenKind::Comma),
            ':' => Some(TokenKind::Colon),
            ';' => Some(TokenKind::Semicolon),
            '.' => Some(TokenKind::Dot),
            _ => None,
        };

        if current == '/' && self.peek() == Some('/') {
            self.scan_line_comment();

            return Token::new(TokenKind::LineComment, self.span());
        }

        if current == '/' && self.peek() == Some('*') {
            let kind = if self.scan_block_comment() {
                TokenKind::BlockComment
            } else {
                TokenKind::UnterminatedBlockComment
            };

            return Token::new(kind, self.span());
        }

        if let Some(kind) = self.scan_operator() {
            return Token::new(kind, self.span());
        }

        if let Some(kind) = punctuation_kind {
            self.advance();

            return Token::new(kind, self.span());
        }

        self.advance();

        Token::new(TokenKind::Unknown, self.span())
    }

    fn match_next(&mut self, expected: char) -> bool {
        if self.peek() != Some(expected) {
            return false;
        }

        self.advance();
        self.advance();

        true
    }

    fn scan_line_comment(&mut self) {
        self.advance();
        self.advance();

        while !matches!(self.current(), None | Some('\n') | Some('\r')) {
            self.advance();
        }
    }

    fn scan_block_comment(&mut self) -> bool {
        self.advance();
        self.advance();

        while !self.is_at_end() {
            if self.current() == Some('*') && self.peek() == Some('/') {
                self.advance();
                self.advance();
                return true;
            }

            self.advance();
        }

        false
    }

    fn scan_string(&mut self) -> bool {
        self.advance();

        while !self.is_at_end() {
            match self.current() {
                Some('"') => {
                    self.advance();
                    return true;
                }
                Some('\n' | '\r') => return false,
                _ => {
                    self.advance();
                }
            }
        }

        false
    }

    fn scan_operator(&mut self) -> Option<TokenKind> {
        let current = self.current()?;

        let kind = match current {
            '=' => {
                if self.match_next('=') {
                    return Some(TokenKind::Equal);
                }

                self.advance();
                TokenKind::Assignment
            }
            '!' => {
                if self.match_next('=') {
                    return Some(TokenKind::NotEqual);
                }

                self.advance();
                TokenKind::LogicalNot
            }
            '<' => {
                if self.match_next('=') {
                    return Some(TokenKind::LessThanOrEqual);
                }

                self.advance();
                TokenKind::LessThan
            }
            '>' => {
                if self.match_next('=') {
                    return Some(TokenKind::GreaterThanOrEqual);
                }

                self.advance();
                TokenKind::GreaterThan
            }
            '+' => {
                self.advance();
                TokenKind::Plus
            }
            '-' => {
                self.advance();
                TokenKind::Minus
            }
            '*' => {
                self.advance();
                TokenKind::Multiply
            }
            '/' => {
                self.advance();
                TokenKind::Divide
            }
            '%' => {
                self.advance();
                TokenKind::Remainder
            }
            '&' if self.match_next('&') => return Some(TokenKind::LogicalAnd),
            '|' if self.match_next('|') => return Some(TokenKind::LogicalOr),
            _ => return None,
        };

        Some(kind)
    }

    fn identifier_kind(&self) -> TokenKind {
        let span = self.span();
        let text = self
            .source
            .slice(&span)
            .expect("lexer identifier span must be valid source text");

        match text {
            "let" => TokenKind::LetKeyword,
            "var" => TokenKind::VarKeyword,
            "function" => TokenKind::FunctionKeyword,
            "implementation" => TokenKind::ImplementationKeyword,

            "return" => TokenKind::ReturnKeyword,
            "if" => TokenKind::IfKeyword,
            "elseif" => TokenKind::ElseIfKeyword,
            "else" => TokenKind::ElseKeyword,
            "while" => TokenKind::WhileKeyword,
            "for" => TokenKind::ForKeyword,
            "break" => TokenKind::BreakKeyword,
            "continue" => TokenKind::ContinueKeyword,

            "true" => TokenKind::TrueKeyword,
            "false" => TokenKind::FalseKeyword,

            _ => TokenKind::Identifier,
        }
    }

    fn scan_identifier(&mut self) {
        while matches!(
            self.current(),
            Some(character) if character.is_alphanumeric() || character == '_'
        ) {
            self.advance();
        }
    }

    fn scan_number(&mut self) {
        while matches!(self.current(), Some('0'..='9')) {
            self.advance();
        }

        if self.current() == Some('.') && matches!(self.peek(), Some('0'..='9')) {
            self.advance();

            while matches!(self.current(), Some('0'..='9')) {
                self.advance();
            }
        }
    }

    fn scan_whitespace(&mut self) {
        while matches!(
            self.current(),
            Some(character) if character.is_whitespace()
        ) {
            self.advance();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_kind_values_compare_equal_to_themselves() {
        assert_eq!(TokenKind::Identifier, TokenKind::Identifier);
    }

    #[test]
    fn token_kind_values_compare_unequal_to_different_kinds() {
        assert_ne!(TokenKind::Identifier, TokenKind::Number);
    }

    #[test]
    fn line_comment_token_kind_is_distinct_from_divide() {
        assert_ne!(TokenKind::LineComment, TokenKind::Divide);
    }

    #[test]
    fn block_comment_token_kind_is_distinct_from_divide() {
        assert_ne!(TokenKind::BlockComment, TokenKind::Divide);
    }

    #[test]
    fn string_token_kind_is_distinct_from_identifier() {
        assert_ne!(TokenKind::String, TokenKind::Identifier);
    }

    #[test]
    fn unterminated_string_token_kind_is_distinct_from_string() {
        assert_ne!(TokenKind::UnterminatedString, TokenKind::String);
    }

    #[test]
    fn lexer_starts_at_beginning_of_source() {
        let source = SourceText::new("Forge");
        let lexer = Lexer::new(&source);

        assert_eq!(lexer.position(), 0);
    }

    #[test]
    fn lexer_reports_empty_source_as_end() {
        let source = SourceText::new("");
        let lexer = Lexer::new(&source);

        assert!(lexer.is_at_end());
    }

    #[test]
    fn lexer_reports_non_empty_source_as_not_at_end() {
        let source = SourceText::new("Forge");
        let lexer = Lexer::new(&source);

        assert!(!lexer.is_at_end());
    }

    #[test]
    fn lexer_current_returns_current_character_without_advancing() {
        let source = SourceText::new("Forge");
        let lexer = Lexer::new(&source);

        assert_eq!(lexer.current(), Some('F'));
        assert_eq!(lexer.position(), 0);
    }

    #[test]
    fn lexer_peek_returns_next_character_without_advancing() {
        let source = SourceText::new("Forge");
        let lexer = Lexer::new(&source);

        assert_eq!(lexer.peek(), Some('o'));
        assert_eq!(lexer.position(), 0);
    }

    #[test]
    fn lexer_advance_returns_current_character_and_moves_forward() {
        let source = SourceText::new("Forge");
        let mut lexer = Lexer::new(&source);

        assert_eq!(lexer.advance(), Some('F'));
        assert_eq!(lexer.position(), 1);
        assert_eq!(lexer.current(), Some('o'));
    }

    #[test]
    fn lexer_advance_uses_utf8_byte_offsets() {
        let source = SourceText::new("éx");
        let mut lexer = Lexer::new(&source);

        assert_eq!(lexer.advance(), Some('é'));
        assert_eq!(lexer.position(), 2);
        assert_eq!(lexer.current(), Some('x'));
    }

    #[test]
    fn lexer_peek_handles_unicode_characters() {
        let source = SourceText::new("aéb");
        let lexer = Lexer::new(&source);

        assert_eq!(lexer.peek(), Some('é'));
    }

    #[test]
    fn lexer_reaches_end_of_source() {
        let source = SourceText::new("A");
        let mut lexer = Lexer::new(&source);

        assert_eq!(lexer.advance(), Some('A'));
        assert!(lexer.is_at_end());
        assert_eq!(lexer.position(), source.len());
    }

    #[test]
    fn lexer_current_returns_none_at_end_of_source() {
        let source = SourceText::new("");
        let lexer = Lexer::new(&source);

        assert_eq!(lexer.current(), None);
    }

    #[test]
    fn lexer_peek_returns_none_when_no_next_character_exists() {
        let source = SourceText::new("A");
        let lexer = Lexer::new(&source);

        assert_eq!(lexer.peek(), None);
    }

    #[test]
    fn lexer_advance_returns_none_at_end_without_moving() {
        let source = SourceText::new("");
        let mut lexer = Lexer::new(&source);

        assert_eq!(lexer.advance(), None);
        assert_eq!(lexer.position(), 0);
    }

    #[test]
    fn lexer_start_initially_matches_beginning_of_source() {
        let source = SourceText::new("Forge");
        let lexer = Lexer::new(&source);

        assert_eq!(lexer.start(), 0);
    }

    #[test]
    fn lexer_mark_start_records_current_position() {
        let source = SourceText::new("Forge");
        let mut lexer = Lexer::new(&source);

        lexer.advance();
        lexer.advance();
        lexer.mark_start();

        assert_eq!(lexer.start(), 2);
        assert_eq!(lexer.position(), 2);
    }

    #[test]
    fn lexer_span_is_initially_empty() {
        let source = SourceText::new("Forge");
        let lexer = Lexer::new(&source);

        let span = lexer.span();

        assert_eq!(span.start(), 0);
        assert_eq!(span.length(), 0);
    }

    #[test]
    fn lexer_span_tracks_advanced_source() {
        let source = SourceText::new("Forge");
        let mut lexer = Lexer::new(&source);

        lexer.advance();
        lexer.advance();
        lexer.advance();

        let span = lexer.span();

        assert_eq!(span.start(), 0);
        assert_eq!(span.length(), 3);
    }

    #[test]
    fn lexer_span_begins_at_marked_start() {
        let source = SourceText::new("let answer");
        let mut lexer = Lexer::new(&source);

        for _ in 0..4 {
            lexer.advance();
        }

        lexer.mark_start();

        for _ in 0..6 {
            lexer.advance();
        }

        let span = lexer.span();

        assert_eq!(span.start(), 4);
        assert_eq!(span.length(), 6);
        assert_eq!(source.slice(&span), Some("answer"));
    }

    #[test]
    fn lexer_span_uses_utf8_byte_offsets() {
        let source = SourceText::new("aéx");
        let mut lexer = Lexer::new(&source);

        lexer.advance();
        lexer.mark_start();
        lexer.advance();

        let span = lexer.span();

        assert_eq!(span.start(), 1);
        assert_eq!(span.length(), 2);
        assert_eq!(source.slice(&span), Some("é"));
    }

    #[test]
    fn lexer_mark_start_can_reset_span_to_empty() {
        let source = SourceText::new("Forge");
        let mut lexer = Lexer::new(&source);

        lexer.advance();
        lexer.advance();
        lexer.mark_start();

        let span = lexer.span();

        assert_eq!(span.start(), 2);
        assert_eq!(span.length(), 0);
    }

    #[test]
    fn lexer_char_at_offset_zero_returns_current_character() {
        let source = SourceText::new("Forge");
        let lexer = Lexer::new(&source);

        assert_eq!(lexer.char_at_offset(0), Some('F'));
    }

    #[test]
    fn lexer_char_at_offset_returns_later_character() {
        let source = SourceText::new("Forge");
        let lexer = Lexer::new(&source);

        assert_eq!(lexer.char_at_offset(2), Some('r'));
        assert_eq!(lexer.char_at_offset(4), Some('e'));
    }

    #[test]
    fn lexer_char_at_offset_handles_unicode_characters() {
        let source = SourceText::new("Aé中Z");
        let lexer = Lexer::new(&source);

        assert_eq!(lexer.char_at_offset(0), Some('A'));
        assert_eq!(lexer.char_at_offset(1), Some('é'));
        assert_eq!(lexer.char_at_offset(2), Some('中'));
        assert_eq!(lexer.char_at_offset(3), Some('Z'));
    }

    #[test]
    fn lexer_char_at_offset_returns_none_beyond_end_of_source() {
        let source = SourceText::new("Forge");
        let lexer = Lexer::new(&source);

        assert_eq!(lexer.char_at_offset(5), None);
        assert_eq!(lexer.char_at_offset(100), None);
    }

    #[test]
    fn lexer_char_at_offset_returns_none_at_end_of_source() {
        let source = SourceText::new("A");
        let mut lexer = Lexer::new(&source);

        lexer.advance();

        assert_eq!(lexer.char_at_offset(0), None);
        assert_eq!(lexer.char_at_offset(1), None);
    }

    #[test]
    fn lexer_char_at_offset_does_not_advance_position() {
        let source = SourceText::new("Forge");
        let lexer = Lexer::new(&source);

        assert_eq!(lexer.char_at_offset(3), Some('g'));
        assert_eq!(lexer.position(), 0);
    }

    #[test]
    fn lexer_char_at_offset_is_relative_to_current_position() {
        let source = SourceText::new("Forge");
        let mut lexer = Lexer::new(&source);

        lexer.advance();
        lexer.advance();

        assert_eq!(lexer.position(), 2);
        assert_eq!(lexer.char_at_offset(0), Some('r'));
        assert_eq!(lexer.char_at_offset(1), Some('g'));
        assert_eq!(lexer.char_at_offset(2), Some('e'));
    }

    #[test]
    fn token_preserves_kind() {
        let token = Token::new(TokenKind::Identifier, SourceSpan::new(0, 5));

        assert_eq!(token.kind(), TokenKind::Identifier);
    }

    #[test]
    fn token_preserves_span() {
        let token = Token::new(TokenKind::Identifier, SourceSpan::new(4, 6));

        assert_eq!(token.span().start(), 4);
        assert_eq!(token.span().length(), 6);
    }

    #[test]
    fn lexer_next_token_returns_end_of_file_for_empty_source() {
        let source = SourceText::new("");
        let mut lexer = Lexer::new(&source);

        let token = lexer.next_token();

        assert_eq!(token.kind(), TokenKind::EndOfFile);
        assert_eq!(token.span(), SourceSpan::new(0, 0));
    }

    #[test]
    fn lexer_next_token_returns_end_of_file_at_end_of_source() {
        let source = SourceText::new("A");
        let mut lexer = Lexer::new(&source);

        lexer.advance();

        let token = lexer.next_token();

        assert_eq!(token.kind(), TokenKind::EndOfFile);
        assert_eq!(token.span(), SourceSpan::new(1, 0));
    }

    #[test]
    fn lexer_next_token_recognizes_identifier() {
        let source = SourceText::new("answer");
        let mut lexer = Lexer::new(&source);

        let token = lexer.next_token();

        assert_eq!(token.kind(), TokenKind::Identifier);
        assert_eq!(token.span(), SourceSpan::new(0, 6));
        assert_eq!(source.slice(&token.span()), Some("answer"));
    }

    #[test]
    fn lexer_next_token_recognizes_identifier_starting_with_underscore() {
        let source = SourceText::new("_answer");
        let mut lexer = Lexer::new(&source);

        let token = lexer.next_token();

        assert_eq!(token.kind(), TokenKind::Identifier);
        assert_eq!(source.slice(&token.span()), Some("_answer"));
    }

    #[test]
    fn lexer_next_token_recognizes_underscore_as_identifier() {
        let source = SourceText::new("_");
        let mut lexer = Lexer::new(&source);

        let token = lexer.next_token();

        assert_eq!(token.kind(), TokenKind::Identifier);
        assert_eq!(source.slice(&token.span()), Some("_"));
    }

    #[test]
    fn lexer_next_token_allows_digits_after_identifier_start() {
        let source = SourceText::new("answer42");
        let mut lexer = Lexer::new(&source);

        let token = lexer.next_token();

        assert_eq!(token.kind(), TokenKind::Identifier);
        assert_eq!(source.slice(&token.span()), Some("answer42"));
    }

    #[test]
    fn lexer_next_token_recognizes_unicode_identifier() {
        let source = SourceText::new("café42");
        let mut lexer = Lexer::new(&source);

        let token = lexer.next_token();

        assert_eq!(token.kind(), TokenKind::Identifier);
        assert_eq!(source.slice(&token.span()), Some("café42"));
    }

    #[test]
    fn lexer_next_token_recognizes_reserved_keywords() {
        let cases = [
            ("let", TokenKind::LetKeyword),
            ("var", TokenKind::VarKeyword),
            ("function", TokenKind::FunctionKeyword),
            ("implementation", TokenKind::ImplementationKeyword),
            ("return", TokenKind::ReturnKeyword),
            ("if", TokenKind::IfKeyword),
            ("elseif", TokenKind::ElseIfKeyword),
            ("else", TokenKind::ElseKeyword),
            ("while", TokenKind::WhileKeyword),
            ("for", TokenKind::ForKeyword),
            ("break", TokenKind::BreakKeyword),
            ("continue", TokenKind::ContinueKeyword),
            ("true", TokenKind::TrueKeyword),
            ("false", TokenKind::FalseKeyword),
        ];

        for (source_text, expected_kind) in cases {
            let source = SourceText::new(source_text);
            let mut lexer = Lexer::new(&source);

            let token = lexer.next_token();

            assert_eq!(token.kind(), expected_kind);
            assert_eq!(source.slice(&token.span()), Some(source_text));
        }
    }

    #[test]
    fn lexer_keywords_are_lowercase_and_case_sensitive() {
        for source_text in ["Let", "LET", "If", "ELSE", "Implementation"] {
            let source = SourceText::new(source_text);
            let mut lexer = Lexer::new(&source);

            let token = lexer.next_token();

            assert_eq!(token.kind(), TokenKind::Identifier);
            assert_eq!(source.slice(&token.span()), Some(source_text));
        }
    }

    #[test]
    fn lexer_keyword_prefixes_with_identifier_suffixes_remain_identifiers() {
        for source_text in [
            "let2",
            "if_value",
            "elseif2",
            "true_value",
            "implementation2",
        ] {
            let source = SourceText::new(source_text);
            let mut lexer = Lexer::new(&source);

            let token = lexer.next_token();

            assert_eq!(token.kind(), TokenKind::Identifier);
            assert_eq!(source.slice(&token.span()), Some(source_text));
        }
    }

    #[test]
    fn lexer_next_token_recognizes_number() {
        let source = SourceText::new("42");
        let mut lexer = Lexer::new(&source);

        let token = lexer.next_token();

        assert_eq!(token.kind(), TokenKind::Number);
        assert_eq!(token.span(), SourceSpan::new(0, 2));
        assert_eq!(source.slice(&token.span()), Some("42"));
    }

    #[test]
    fn lexer_next_token_recognizes_zero_as_number() {
        let source = SourceText::new("0");
        let mut lexer = Lexer::new(&source);

        let token = lexer.next_token();

        assert_eq!(token.kind(), TokenKind::Number);
        assert_eq!(source.slice(&token.span()), Some("0"));
    }

    #[test]
    fn lexer_number_stops_before_identifier_character() {
        let source = SourceText::new("42answer");
        let mut lexer = Lexer::new(&source);

        let token = lexer.next_token();

        assert_eq!(token.kind(), TokenKind::Number);
        assert_eq!(source.slice(&token.span()), Some("42"));
        assert_eq!(lexer.current(), Some('a'));
    }

    #[test]
    fn lexer_next_token_recognizes_decimal_number() {
        let source = SourceText::new("3.14");
        let mut lexer = Lexer::new(&source);

        let token = lexer.next_token();

        assert_eq!(token.kind(), TokenKind::Number);
        assert_eq!(token.span(), SourceSpan::new(0, 4));
        assert_eq!(source.slice(&token.span()), Some("3.14"));
    }

    #[test]
    fn lexer_next_token_recognizes_decimal_number_starting_with_zero() {
        let source = SourceText::new("0.5");
        let mut lexer = Lexer::new(&source);

        let token = lexer.next_token();

        assert_eq!(token.kind(), TokenKind::Number);
        assert_eq!(source.slice(&token.span()), Some("0.5"));
    }

    #[test]
    fn lexer_number_stops_before_dot_without_following_digit() {
        let source = SourceText::new("42.foo");
        let mut lexer = Lexer::new(&source);

        let number = lexer.next_token();
        let dot = lexer.next_token();
        let identifier = lexer.next_token();

        assert_eq!(number.kind(), TokenKind::Number);
        assert_eq!(source.slice(&number.span()), Some("42"));

        assert_eq!(dot.kind(), TokenKind::Dot);
        assert_eq!(source.slice(&dot.span()), Some("."));

        assert_eq!(identifier.kind(), TokenKind::Identifier);
        assert_eq!(source.slice(&identifier.span()), Some("foo"));
    }

    #[test]
    fn lexer_number_stops_before_trailing_dot() {
        let source = SourceText::new("42.");
        let mut lexer = Lexer::new(&source);

        let number = lexer.next_token();
        let dot = lexer.next_token();
        let end_of_file = lexer.next_token();

        assert_eq!(number.kind(), TokenKind::Number);
        assert_eq!(source.slice(&number.span()), Some("42"));

        assert_eq!(dot.kind(), TokenKind::Dot);
        assert_eq!(source.slice(&dot.span()), Some("."));

        assert_eq!(end_of_file.kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn lexer_leading_dot_remains_separate_from_number() {
        let source = SourceText::new(".5");
        let mut lexer = Lexer::new(&source);

        let dot = lexer.next_token();
        let number = lexer.next_token();

        assert_eq!(dot.kind(), TokenKind::Dot);
        assert_eq!(source.slice(&dot.span()), Some("."));

        assert_eq!(number.kind(), TokenKind::Number);
        assert_eq!(source.slice(&number.span()), Some("5"));
    }

    #[test]
    fn lexer_next_token_recognizes_space_as_whitespace() {
        let source = SourceText::new(" ");
        let mut lexer = Lexer::new(&source);

        let token = lexer.next_token();

        assert_eq!(token.kind(), TokenKind::Whitespace);
        assert_eq!(token.span(), SourceSpan::new(0, 1));
        assert_eq!(source.slice(&token.span()), Some(" "));
    }

    #[test]
    fn lexer_next_token_groups_consecutive_whitespace() {
        let source = SourceText::new(" \t\n  ");
        let mut lexer = Lexer::new(&source);

        let token = lexer.next_token();

        assert_eq!(token.kind(), TokenKind::Whitespace);
        assert_eq!(source.slice(&token.span()), Some(" \t\n  "));
    }

    #[test]
    fn lexer_next_token_recognizes_string() {
        let source = SourceText::new("\"hello\"");
        let mut lexer = Lexer::new(&source);

        let token = lexer.next_token();

        assert_eq!(token.kind(), TokenKind::String);
        assert_eq!(token.span(), SourceSpan::new(0, 7));
        assert_eq!(source.slice(&token.span()), Some("\"hello\""));
    }

    #[test]
    fn lexer_next_token_recognizes_empty_string() {
        let source = SourceText::new("\"\"");
        let mut lexer = Lexer::new(&source);

        let token = lexer.next_token();

        assert_eq!(token.kind(), TokenKind::String);
        assert_eq!(token.span(), SourceSpan::new(0, 2));
        assert_eq!(source.slice(&token.span()), Some("\"\""));
    }

    #[test]
    fn lexer_string_stops_after_closing_quote() {
        let source = SourceText::new("\"hello\"answer");
        let mut lexer = Lexer::new(&source);

        let string = lexer.next_token();
        let identifier = lexer.next_token();

        assert_eq!(string.kind(), TokenKind::String);
        assert_eq!(source.slice(&string.span()), Some("\"hello\""));

        assert_eq!(identifier.kind(), TokenKind::Identifier);
        assert_eq!(source.slice(&identifier.span()), Some("answer"));
    }

    #[test]
    fn lexer_recognizes_unterminated_string_at_end_of_file() {
        let source = SourceText::new("\"hello");
        let mut lexer = Lexer::new(&source);

        let string = lexer.next_token();
        let end_of_file = lexer.next_token();

        assert_eq!(string.kind(), TokenKind::UnterminatedString);
        assert_eq!(string.span(), SourceSpan::new(0, source.len()));
        assert_eq!(source.slice(&string.span()), Some("\"hello"));

        assert_eq!(end_of_file.kind(), TokenKind::EndOfFile);
        assert_eq!(end_of_file.span(), SourceSpan::new(source.len(), 0));
    }

    #[test]
    fn lexer_unterminated_string_stops_before_newline() {
        let source = SourceText::new("\"hello\nanswer");
        let mut lexer = Lexer::new(&source);

        let string = lexer.next_token();
        let whitespace = lexer.next_token();
        let identifier = lexer.next_token();

        assert_eq!(string.kind(), TokenKind::UnterminatedString);
        assert_eq!(source.slice(&string.span()), Some("\"hello"));

        assert_eq!(whitespace.kind(), TokenKind::Whitespace);
        assert_eq!(source.slice(&whitespace.span()), Some("\n"));

        assert_eq!(identifier.kind(), TokenKind::Identifier);
        assert_eq!(source.slice(&identifier.span()), Some("answer"));
    }

    #[test]
    fn lexer_unterminated_string_stops_before_carriage_return_line_feed() {
        let source = SourceText::new("\"hello\r\nanswer");
        let mut lexer = Lexer::new(&source);

        let string = lexer.next_token();
        let whitespace = lexer.next_token();
        let identifier = lexer.next_token();

        assert_eq!(string.kind(), TokenKind::UnterminatedString);
        assert_eq!(source.slice(&string.span()), Some("\"hello"));

        assert_eq!(whitespace.kind(), TokenKind::Whitespace);
        assert_eq!(source.slice(&whitespace.span()), Some("\r\n"));

        assert_eq!(identifier.kind(), TokenKind::Identifier);
        assert_eq!(source.slice(&identifier.span()), Some("answer"));
    }

    #[test]
    fn lexer_string_span_uses_utf8_byte_offsets() {
        let source = SourceText::new("\"café 🙂\"");
        let mut lexer = Lexer::new(&source);

        let string = lexer.next_token();

        assert_eq!(string.kind(), TokenKind::String);
        assert_eq!(string.span(), SourceSpan::new(0, source.len()));
        assert_eq!(source.slice(&string.span()), Some("\"café 🙂\""));
    }

    #[test]
    fn lexer_whitespace_stops_before_identifier() {
        let source = SourceText::new("  answer");
        let mut lexer = Lexer::new(&source);

        let token = lexer.next_token();

        assert_eq!(token.kind(), TokenKind::Whitespace);
        assert_eq!(source.slice(&token.span()), Some("  "));
        assert_eq!(lexer.current(), Some('a'));
    }

    #[test]
    fn lexer_next_token_tokenizes_identifier_whitespace_number_sequence() {
        let source = SourceText::new("answer 42");
        let mut lexer = Lexer::new(&source);

        let identifier = lexer.next_token();
        let whitespace = lexer.next_token();
        let number = lexer.next_token();
        let end_of_file = lexer.next_token();

        assert_eq!(identifier.kind(), TokenKind::Identifier);
        assert_eq!(source.slice(&identifier.span()), Some("answer"));

        assert_eq!(whitespace.kind(), TokenKind::Whitespace);
        assert_eq!(source.slice(&whitespace.span()), Some(" "));

        assert_eq!(number.kind(), TokenKind::Number);
        assert_eq!(source.slice(&number.span()), Some("42"));

        assert_eq!(end_of_file.kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn lexer_tokenizes_decimal_number_in_expression() {
        let source = SourceText::new("let result = 3.14;");
        let mut lexer = Lexer::new(&source);

        let expected = [
            TokenKind::LetKeyword,
            TokenKind::Whitespace,
            TokenKind::Identifier,
            TokenKind::Whitespace,
            TokenKind::Assignment,
            TokenKind::Whitespace,
            TokenKind::Number,
            TokenKind::Semicolon,
            TokenKind::EndOfFile,
        ];

        for expected_kind in expected {
            let token = lexer.next_token();

            assert_eq!(token.kind(), expected_kind);
        }
    }

    #[test]
    fn lexer_next_token_recognizes_unknown_character() {
        let source = SourceText::new("@");
        let mut lexer = Lexer::new(&source);

        let token = lexer.next_token();

        assert_eq!(token.kind(), TokenKind::Unknown);
        assert_eq!(token.span(), SourceSpan::new(0, 1));
        assert_eq!(source.slice(&token.span()), Some("@"));
    }

    #[test]
    fn lexer_unknown_character_advances_lexer() {
        let source = SourceText::new("@answer");
        let mut lexer = Lexer::new(&source);

        let unknown = lexer.next_token();
        let identifier = lexer.next_token();

        assert_eq!(unknown.kind(), TokenKind::Unknown);
        assert_eq!(source.slice(&unknown.span()), Some("@"));

        assert_eq!(identifier.kind(), TokenKind::Identifier);
        assert_eq!(source.slice(&identifier.span()), Some("answer"));
    }

    #[test]
    fn lexer_unknown_unicode_character_uses_utf8_byte_span() {
        let source = SourceText::new("🙂");
        let mut lexer = Lexer::new(&source);

        let token = lexer.next_token();

        assert_eq!(token.kind(), TokenKind::Unknown);
        assert_eq!(token.span(), SourceSpan::new(0, 4));
        assert_eq!(source.slice(&token.span()), Some("🙂"));
    }

    #[test]
    fn lexer_next_token_recognizes_structural_punctuation() {
        let cases = [
            ("(", TokenKind::LeftParenthesis),
            (")", TokenKind::RightParenthesis),
            ("{", TokenKind::LeftBrace),
            ("}", TokenKind::RightBrace),
            ("[", TokenKind::LeftBracket),
            ("]", TokenKind::RightBracket),
            (",", TokenKind::Comma),
            (":", TokenKind::Colon),
            (";", TokenKind::Semicolon),
            (".", TokenKind::Dot),
        ];

        for (source_text, expected_kind) in cases {
            let source = SourceText::new(source_text);
            let mut lexer = Lexer::new(&source);

            let token = lexer.next_token();

            assert_eq!(token.kind(), expected_kind);
            assert_eq!(token.span(), SourceSpan::new(0, 1));
            assert_eq!(source.slice(&token.span()), Some(source_text));
        }
    }

    #[test]
    fn lexer_tokenizes_braced_statement_structure() {
        let source = SourceText::new("{answer;}");
        let mut lexer = Lexer::new(&source);

        assert_eq!(lexer.next_token().kind(), TokenKind::LeftBrace);
        assert_eq!(lexer.next_token().kind(), TokenKind::Identifier);
        assert_eq!(lexer.next_token().kind(), TokenKind::Semicolon);
        assert_eq!(lexer.next_token().kind(), TokenKind::RightBrace);
        assert_eq!(lexer.next_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn lexer_next_token_recognizes_assignment_operator() {
        let source = SourceText::new("=");
        let mut lexer = Lexer::new(&source);

        let token = lexer.next_token();

        assert_eq!(token.kind(), TokenKind::Assignment);
        assert_eq!(token.span(), SourceSpan::new(0, 1));
        assert_eq!(source.slice(&token.span()), Some("="));
    }

    #[test]
    fn lexer_next_token_recognizes_equality_operator() {
        let source = SourceText::new("==");
        let mut lexer = Lexer::new(&source);

        let token = lexer.next_token();

        assert_eq!(token.kind(), TokenKind::Equal);
        assert_eq!(token.span(), SourceSpan::new(0, 2));
        assert_eq!(source.slice(&token.span()), Some("=="));
    }

    #[test]
    fn lexer_next_token_recognizes_comparison_and_logical_not_operators() {
        let cases = [
            ("!", TokenKind::LogicalNot),
            ("!=", TokenKind::NotEqual),
            ("<", TokenKind::LessThan),
            ("<=", TokenKind::LessThanOrEqual),
            (">", TokenKind::GreaterThan),
            (">=", TokenKind::GreaterThanOrEqual),
        ];

        for (source_text, expected_kind) in cases {
            let source = SourceText::new(source_text);
            let mut lexer = Lexer::new(&source);

            let token = lexer.next_token();

            assert_eq!(token.kind(), expected_kind);
            assert_eq!(source.slice(&token.span()), Some(source_text));
        }
    }

    #[test]
    fn lexer_next_token_recognizes_line_comment() {
        let source = SourceText::new("// comment");
        let mut lexer = Lexer::new(&source);

        let token = lexer.next_token();

        assert_eq!(token.kind(), TokenKind::LineComment);
        assert_eq!(token.span(), SourceSpan::new(0, 10));
        assert_eq!(source.slice(&token.span()), Some("// comment"));
    }

    #[test]
    fn lexer_next_token_recognizes_block_comment() {
        let source = SourceText::new("/* comment */");
        let mut lexer = Lexer::new(&source);

        let token = lexer.next_token();

        assert_eq!(token.kind(), TokenKind::BlockComment);
        assert_eq!(token.span(), SourceSpan::new(0, 13));
        assert_eq!(source.slice(&token.span()), Some("/* comment */"));
    }

    #[test]
    fn lexer_recognizes_empty_block_comment() {
        let source = SourceText::new("/**/");
        let mut lexer = Lexer::new(&source);

        let comment = lexer.next_token();

        assert_eq!(comment.kind(), TokenKind::BlockComment);
        assert_eq!(source.slice(&comment.span()), Some("/**/"));
    }

    #[test]
    fn lexer_block_comment_can_span_multiple_lines() {
        let source = SourceText::new("/* first\nsecond */");
        let mut lexer = Lexer::new(&source);

        let comment = lexer.next_token();

        assert_eq!(comment.kind(), TokenKind::BlockComment);
        assert_eq!(source.slice(&comment.span()), Some("/* first\nsecond */"));
    }

    #[test]
    fn lexer_block_comment_stops_at_closing_delimiter() {
        let source = SourceText::new("/* comment */answer");
        let mut lexer = Lexer::new(&source);

        let comment = lexer.next_token();
        let identifier = lexer.next_token();

        assert_eq!(comment.kind(), TokenKind::BlockComment);
        assert_eq!(source.slice(&comment.span()), Some("/* comment */"));

        assert_eq!(identifier.kind(), TokenKind::Identifier);
        assert_eq!(source.slice(&identifier.span()), Some("answer"));
    }

    #[test]
    fn lexer_block_comment_span_uses_utf8_byte_offsets() {
        let source = SourceText::new("/* café 🙂 */");
        let mut lexer = Lexer::new(&source);

        let comment = lexer.next_token();

        assert_eq!(comment.kind(), TokenKind::BlockComment);
        assert_eq!(comment.span(), SourceSpan::new(0, source.len()));
        assert_eq!(source.slice(&comment.span()), Some("/* café 🙂 */"));
    }

    #[test]
    fn lexer_recognizes_unterminated_block_comment() {
        let source = SourceText::new("/* comment");
        let mut lexer = Lexer::new(&source);

        let comment = lexer.next_token();
        let end_of_file = lexer.next_token();

        assert_eq!(comment.kind(), TokenKind::UnterminatedBlockComment);
        assert_eq!(comment.span(), SourceSpan::new(0, source.len()));
        assert_eq!(source.slice(&comment.span()), Some("/* comment"));

        assert_eq!(end_of_file.kind(), TokenKind::EndOfFile);
        assert_eq!(end_of_file.span(), SourceSpan::new(source.len(), 0));
    }

    #[test]
    fn lexer_line_comment_stops_before_newline() {
        let source = SourceText::new("// comment\nanswer");
        let mut lexer = Lexer::new(&source);

        let comment = lexer.next_token();
        let whitespace = lexer.next_token();
        let identifier = lexer.next_token();

        assert_eq!(comment.kind(), TokenKind::LineComment);
        assert_eq!(source.slice(&comment.span()), Some("// comment"));

        assert_eq!(whitespace.kind(), TokenKind::Whitespace);
        assert_eq!(source.slice(&whitespace.span()), Some("\n"));

        assert_eq!(identifier.kind(), TokenKind::Identifier);
        assert_eq!(source.slice(&identifier.span()), Some("answer"));
    }

    #[test]
    fn lexer_divide_operator_remains_distinct_from_line_comment() {
        let source = SourceText::new("/ // comment");
        let mut lexer = Lexer::new(&source);

        let divide = lexer.next_token();
        let whitespace = lexer.next_token();
        let comment = lexer.next_token();

        assert_eq!(divide.kind(), TokenKind::Divide);
        assert_eq!(source.slice(&divide.span()), Some("/"));

        assert_eq!(whitespace.kind(), TokenKind::Whitespace);

        assert_eq!(comment.kind(), TokenKind::LineComment);
        assert_eq!(source.slice(&comment.span()), Some("// comment"));
    }

    #[test]
    fn lexer_divide_operator_remains_distinct_from_block_comment() {
        let source = SourceText::new("/ /* comment */");
        let mut lexer = Lexer::new(&source);

        let divide = lexer.next_token();
        let whitespace = lexer.next_token();
        let comment = lexer.next_token();

        assert_eq!(divide.kind(), TokenKind::Divide);
        assert_eq!(source.slice(&divide.span()), Some("/"));

        assert_eq!(whitespace.kind(), TokenKind::Whitespace);

        assert_eq!(comment.kind(), TokenKind::BlockComment);
        assert_eq!(source.slice(&comment.span()), Some("/* comment */"));
    }

    #[test]
    fn lexer_recognizes_empty_line_comment() {
        let source = SourceText::new("//");
        let mut lexer = Lexer::new(&source);

        let comment = lexer.next_token();

        assert_eq!(comment.kind(), TokenKind::LineComment);
        assert_eq!(source.slice(&comment.span()), Some("//"));
    }

    #[test]
    fn lexer_line_comment_can_reach_end_of_file() {
        let source = SourceText::new("// comment");
        let mut lexer = Lexer::new(&source);

        let comment = lexer.next_token();
        let end_of_file = lexer.next_token();

        assert_eq!(comment.kind(), TokenKind::LineComment);
        assert_eq!(source.slice(&comment.span()), Some("// comment"));

        assert_eq!(end_of_file.kind(), TokenKind::EndOfFile);
        assert_eq!(end_of_file.span(), SourceSpan::new(source.len(), 0));
    }

    #[test]
    fn lexer_line_comment_stops_before_carriage_return_line_feed() {
        let source = SourceText::new("// comment\r\nanswer");
        let mut lexer = Lexer::new(&source);

        let comment = lexer.next_token();
        let whitespace = lexer.next_token();
        let identifier = lexer.next_token();

        assert_eq!(comment.kind(), TokenKind::LineComment);
        assert_eq!(source.slice(&comment.span()), Some("// comment"));

        assert_eq!(whitespace.kind(), TokenKind::Whitespace);
        assert_eq!(source.slice(&whitespace.span()), Some("\r\n"));

        assert_eq!(identifier.kind(), TokenKind::Identifier);
        assert_eq!(source.slice(&identifier.span()), Some("answer"));
    }

    #[test]
    fn lexer_line_comment_span_uses_utf8_byte_offsets() {
        let source = SourceText::new("// café 🙂");
        let mut lexer = Lexer::new(&source);

        let comment = lexer.next_token();

        assert_eq!(comment.kind(), TokenKind::LineComment);
        assert_eq!(comment.span(), SourceSpan::new(0, source.len()));
        assert_eq!(source.slice(&comment.span()), Some("// café 🙂"));
    }

    #[test]
    fn lexer_next_token_recognizes_arithmetic_operators() {
        let cases = [
            ("+", TokenKind::Plus),
            ("-", TokenKind::Minus),
            ("*", TokenKind::Multiply),
            ("/", TokenKind::Divide),
            ("%", TokenKind::Remainder),
        ];

        for (source_text, expected_kind) in cases {
            let source = SourceText::new(source_text);
            let mut lexer = Lexer::new(&source);

            let token = lexer.next_token();

            assert_eq!(token.kind(), expected_kind);
            assert_eq!(token.span(), SourceSpan::new(0, 1));
            assert_eq!(source.slice(&token.span()), Some(source_text));
        }
    }

    #[test]
    fn lexer_next_token_recognizes_logical_and_or_operators() {
        let cases = [("&&", TokenKind::LogicalAnd), ("||", TokenKind::LogicalOr)];

        for (source_text, expected_kind) in cases {
            let source = SourceText::new(source_text);
            let mut lexer = Lexer::new(&source);

            let token = lexer.next_token();

            assert_eq!(token.kind(), expected_kind);
            assert_eq!(token.span(), SourceSpan::new(0, 2));
            assert_eq!(source.slice(&token.span()), Some(source_text));
        }
    }

    #[test]
    fn lexer_single_ampersand_and_pipe_remain_unknown() {
        for source_text in ["&", "|"] {
            let source = SourceText::new(source_text);
            let mut lexer = Lexer::new(&source);

            let token = lexer.next_token();

            assert_eq!(token.kind(), TokenKind::Unknown);
            assert_eq!(token.span(), SourceSpan::new(0, 1));
            assert_eq!(source.slice(&token.span()), Some(source_text));
        }
    }

    #[test]
    fn lexer_tokenizes_mixed_comment_stream() {
        let source = SourceText::new("let value = 1; // line\n/* block */ value");
        let mut lexer = Lexer::new(&source);

        let expected = [
            TokenKind::LetKeyword,
            TokenKind::Whitespace,
            TokenKind::Identifier,
            TokenKind::Whitespace,
            TokenKind::Assignment,
            TokenKind::Whitespace,
            TokenKind::Number,
            TokenKind::Semicolon,
            TokenKind::Whitespace,
            TokenKind::LineComment,
            TokenKind::Whitespace,
            TokenKind::BlockComment,
            TokenKind::Whitespace,
            TokenKind::Identifier,
            TokenKind::EndOfFile,
        ];

        for expected_kind in expected {
            let token = lexer.next_token();

            assert_eq!(token.kind(), expected_kind);
        }
    }

    #[test]
    fn lexer_tokenizes_mixed_forge_source() {
        let source = SourceText::new("let result = value * 2;");
        let mut lexer = Lexer::new(&source);

        let expected = [
            TokenKind::LetKeyword,
            TokenKind::Whitespace,
            TokenKind::Identifier,
            TokenKind::Whitespace,
            TokenKind::Assignment,
            TokenKind::Whitespace,
            TokenKind::Identifier,
            TokenKind::Whitespace,
            TokenKind::Multiply,
            TokenKind::Whitespace,
            TokenKind::Number,
            TokenKind::Semicolon,
            TokenKind::EndOfFile,
        ];

        for expected_kind in expected {
            let token = lexer.next_token();

            assert_eq!(token.kind(), expected_kind);
        }
    }

    #[test]
    fn lexer_tokenizes_control_flow_and_block_structure() {
        let source = SourceText::new("if value >= 10 { return true; }");
        let mut lexer = Lexer::new(&source);

        let expected = [
            TokenKind::IfKeyword,
            TokenKind::Whitespace,
            TokenKind::Identifier,
            TokenKind::Whitespace,
            TokenKind::GreaterThanOrEqual,
            TokenKind::Whitespace,
            TokenKind::Number,
            TokenKind::Whitespace,
            TokenKind::LeftBrace,
            TokenKind::Whitespace,
            TokenKind::ReturnKeyword,
            TokenKind::Whitespace,
            TokenKind::TrueKeyword,
            TokenKind::Semicolon,
            TokenKind::Whitespace,
            TokenKind::RightBrace,
            TokenKind::EndOfFile,
        ];

        for expected_kind in expected {
            let token = lexer.next_token();

            assert_eq!(token.kind(), expected_kind);
        }
    }

    #[test]
    fn lexer_mixed_token_spans_are_contiguous() {
        let source = SourceText::new("let result = 42;");
        let mut lexer = Lexer::new(&source);

        let mut previous_end = 0;

        loop {
            let token = lexer.next_token();
            let span = token.span();

            assert_eq!(span.start(), previous_end);

            previous_end = span.start() + span.length();

            if token.kind() == TokenKind::EndOfFile {
                break;
            }
        }

        assert_eq!(previous_end, source.len());
    }
}
