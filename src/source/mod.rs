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
}
