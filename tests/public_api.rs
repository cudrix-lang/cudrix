use forge::LANGUAGE_NAME;
use forge::compiler::{Lexer, Parser, Token, TokenKind};
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

#[test]
fn forge_exposes_lexer_model() {
    let source = SourceText::new("let answer = 42;");
    let mut lexer = Lexer::new(&source);

    let token = lexer.next_token();

    assert_eq!(token.kind(), TokenKind::LetKeyword);
    assert_eq!(token.span(), SourceSpan::new(0, 3));

    let constructed = Token::new(TokenKind::Identifier, SourceSpan::new(4, 6));

    assert_eq!(constructed.kind(), TokenKind::Identifier);
    assert_eq!(constructed.span(), SourceSpan::new(4, 6));
}

#[test]
fn forge_exposes_parser_model() {
    let source = SourceText::new("let answer = 42;");
    let parser = Parser::new(&source);

    assert_eq!(parser.current_token().kind(), TokenKind::LetKeyword);
    assert_eq!(parser.current_token().span(), SourceSpan::new(0, 3));
}
