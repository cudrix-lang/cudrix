use forge::LANGUAGE_NAME;
use forge::source::{SourceFile, SourceLocation, SourceSpan, SourceText};

#[test]
fn forge_exposes_language_name() {
    assert_eq!(LANGUAGE_NAME, "Forge");
}

#[test]
fn forge_exposes_source_model() {
    let text = SourceText::new("let answer = 42;");
    let span = SourceSpan::new(4, 6);
    let location = SourceLocation::new(1, 5);
    let file = SourceFile::new("src/main.forge", text);

    assert_eq!(file.name(), "src/main.forge");
    assert_eq!(file.text().slice(&span), Some("answer"));
    assert_eq!(location.line(), 1);
    assert_eq!(location.column(), 5);
}
