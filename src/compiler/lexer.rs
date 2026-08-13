use crate::source::SourceText;

pub struct Lexer<'source> {
    source: &'source SourceText,
    position: usize,
}

impl<'source> Lexer<'source> {
    pub fn new(source: &'source SourceText) -> Self {
        Self {
            source,
            position: 0,
        }
    }

    pub fn position(&self) -> usize {
        self.position
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
}
