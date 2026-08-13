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
}
