//! Source text, files, locations, and spans.

pub struct SourceText {
    text: String,
}

impl SourceText {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }

    pub fn as_str(&self) -> &str {
        &self.text
    }

    pub fn len(&self) -> usize {
        self.text.len()
    }

    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }

    pub fn slice(&self, span: &SourceSpan) -> Option<&str> {
        let end = span.start().checked_add(span.length())?;

        self.text.get(span.start()..end)
    }

    pub fn location_at(&self, byte_offset: usize) -> Option<SourceLocation> {
        if byte_offset > self.text.len() || !self.text.is_char_boundary(byte_offset) {
            return None;
        }

        let prefix = &self.text[..byte_offset];

        let line = prefix.chars().filter(|&ch| ch == '\n').count() + 1;

        let column = prefix.rsplit('\n').next().unwrap_or("").chars().count() + 1;

        Some(SourceLocation::new(line, column))
    }
}

pub struct SourceSpan {
    start: usize,
    length: usize,
}

impl SourceSpan {
    pub fn new(start: usize, length: usize) -> Self {
        Self { start, length }
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn length(&self) -> usize {
        self.length
    }
}

pub struct SourceLocation {
    line: usize,
    column: usize,
}

impl SourceLocation {
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn column(&self) -> usize {
        self.column
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source_text_preserves_text() {
        let source = SourceText::new("let answer = 42;");

        assert_eq!(source.as_str(), "let answer = 42;");
    }

    #[test]
    fn source_text_accepts_owned_string() {
        let text = String::from("Forge");

        let source = SourceText::new(text);

        assert_eq!(source.as_str(), "Forge");
    }

    #[test]
    fn source_text_reports_length() {
        let source = SourceText::new("Forge");

        assert_eq!(source.len(), 5);
    }

    #[test]
    fn source_text_length_is_utf8_byte_length() {
        let source = SourceText::new("é");

        assert_eq!(source.len(), 2);
    }

    #[test]
    fn source_text_reports_empty_source() {
        let source = SourceText::new("");

        assert!(source.is_empty());
    }

    #[test]
    fn source_text_reports_non_empty_source() {
        let source = SourceText::new("let value = 1;");

        assert!(!source.is_empty());
    }

    #[test]
    fn source_span_preserves_start() {
        let span = SourceSpan::new(4, 6);

        assert_eq!(span.start(), 4);
    }

    #[test]
    fn source_span_preserves_length() {
        let span = SourceSpan::new(4, 6);

        assert_eq!(span.length(), 6);
    }

    #[test]
    fn source_span_can_be_empty() {
        let span = SourceSpan::new(4, 0);

        assert_eq!(span.length(), 0);
    }

    #[test]
    fn source_text_slice_returns_matching_text() {
        let source = SourceText::new("let answer = 42;");
        let span = SourceSpan::new(4, 6);

        assert_eq!(source.slice(&span), Some("answer"));
    }

    #[test]
    fn source_text_slice_supports_empty_span() {
        let source = SourceText::new("Forge");
        let span = SourceSpan::new(2, 0);

        assert_eq!(source.slice(&span), Some(""));
    }

    #[test]
    fn source_text_slice_rejects_out_of_bounds_span() {
        let source = SourceText::new("Forge");
        let span = SourceSpan::new(4, 5);

        assert_eq!(source.slice(&span), None);
    }

    #[test]
    fn source_text_slice_rejects_invalid_utf8_boundary() {
        let source = SourceText::new("é");
        let span = SourceSpan::new(1, 1);

        assert_eq!(source.slice(&span), None);
    }

    #[test]
    fn source_location_preserves_line() {
        let location = SourceLocation::new(3, 8);

        assert_eq!(location.line(), 3);
    }

    #[test]
    fn source_location_preserves_column() {
        let location = SourceLocation::new(3, 8);

        assert_eq!(location.column(), 8);
    }

    #[test]
    fn source_text_location_at_start_is_line_one_column_one() {
        let source = SourceText::new("let answer = 42;");

        let location = source.location_at(0);

        assert_eq!(
            location.map(|value| (value.line(), value.column())),
            Some((1, 1))
        );
    }

    #[test]
    fn source_text_location_tracks_new_lines() {
        let source = SourceText::new("let a = 1;\nlet b = 2;");

        let location = source.location_at(11);

        assert_eq!(
            location.map(|value| (value.line(), value.column())),
            Some((2, 1))
        );
    }

    #[test]
    fn source_text_location_counts_unicode_characters_for_columns() {
        let source = SourceText::new("éx");

        let location = source.location_at(2);

        assert_eq!(
            location.map(|value| (value.line(), value.column())),
            Some((1, 2))
        );
    }

    #[test]
    fn source_text_location_rejects_out_of_bounds_offset() {
        let source = SourceText::new("Forge");

        assert!(source.location_at(6).is_none());
    }

    #[test]
    fn source_text_location_rejects_invalid_utf8_boundary() {
        let source = SourceText::new("é");

        assert!(source.location_at(1).is_none());
    }

    #[test]
    fn source_text_location_accepts_end_of_source() {
        let source = SourceText::new("Forge");

        let location = source.location_at(source.len());

        assert_eq!(
            location.map(|value| (value.line(), value.column())),
            Some((1, 6))
        );
    }
}
