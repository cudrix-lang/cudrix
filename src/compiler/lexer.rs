use crate::source::{SourceSpan, SourceText};

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

    pub fn current(&self) -> Option<char> {
        self.source.as_str()[self.position..].chars().next()
    }

    pub fn peek(&self) -> Option<char> {
        self.source.as_str()[self.position..].chars().nth(1)
    }

    pub fn advance(&mut self) -> Option<char> {
        let current = self.current()?;

        self.position += current.len_utf8();

        Some(current)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
