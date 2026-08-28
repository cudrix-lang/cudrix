use cudrix::LANGUAGE_NAME;
use cudrix::compiler::{Lexer, Parser, Token, TokenKind};
use cudrix::source::{SourceFile, SourceLocation, SourceSpan, SourceText};
use cudrix::syntax::{
    ExpressionSyntax, LiteralExpressionSyntax, NameExpressionSyntax, ParenthesizedExpressionSyntax,
};

#[test]
fn cudrix_exposes_language_name() {
    assert_eq!(LANGUAGE_NAME, "Cudrix");
}

#[test]
fn cudrix_exposes_source_model() {
    let text = SourceText::new("let answer = 42;");
    let span = SourceSpan::new(4, 6);
    let location = SourceLocation::new(1, 5);
    let file = SourceFile::new("src/main.cdrx", text);

    assert_eq!(file.name(), "src/main.cdrx");
    assert_eq!(file.text().slice(&span), Some("answer"));
    assert_eq!(location.line(), 1);
    assert_eq!(location.column(), 5);
}

#[test]
fn cudrix_exposes_lexer_model() {
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
fn cudrix_exposes_parser_model() {
    let source = SourceText::new("let answer = 42;");
    let parser = Parser::new(&source);

    assert_eq!(parser.current_token().kind(), TokenKind::LetKeyword);
    assert_eq!(parser.current_token().span(), SourceSpan::new(0, 3));
}

#[test]
fn cudrix_exposes_literal_expression_syntax() {
    let token = Token::new(TokenKind::Number, SourceSpan::new(0, 2));
    let literal_expression = LiteralExpressionSyntax::new(token);

    assert_eq!(literal_expression.literal_token(), token);
}

#[test]
fn cudrix_exposes_name_expression_syntax() {
    let token = Token::new(TokenKind::Identifier, SourceSpan::new(0, 6));
    let name_expression = NameExpressionSyntax::new(token);

    assert_eq!(name_expression.identifier_token(), token);
}

#[test]
fn cudrix_exposes_parenthesized_expression_syntax() {
    let left_parenthesis_token = Token::new(TokenKind::LeftParenthesis, SourceSpan::new(0, 1));
    let literal_token = Token::new(TokenKind::Number, SourceSpan::new(1, 2));
    let right_parenthesis_token = Token::new(TokenKind::RightParenthesis, SourceSpan::new(3, 1));

    let expression = ExpressionSyntax::Literal(LiteralExpressionSyntax::new(literal_token));

    let parenthesized_expression = ParenthesizedExpressionSyntax::new(
        left_parenthesis_token,
        expression.clone(),
        right_parenthesis_token,
    );

    assert_eq!(
        parenthesized_expression.left_parenthesis_token(),
        left_parenthesis_token
    );
    assert_eq!(parenthesized_expression.expression(), &expression);
    assert_eq!(
        parenthesized_expression.right_parenthesis_token(),
        right_parenthesis_token
    );
}

#[test]
fn cudrix_exposes_expression_syntax() {
    let token = Token::new(TokenKind::Number, SourceSpan::new(0, 2));
    let literal_expression = LiteralExpressionSyntax::new(token);
    let expression = ExpressionSyntax::Literal(literal_expression);

    assert_eq!(expression, ExpressionSyntax::Literal(literal_expression));
}

#[test]
fn cudrix_parser_exposes_literal_expression_parsing() {
    let source = SourceText::new("42");
    let mut parser = Parser::new(&source);

    let literal_expression = parser
        .parse_literal_expression()
        .expect("number should parse as literal expression");

    assert_eq!(literal_expression.literal_token().kind(), TokenKind::Number);
    assert_eq!(
        literal_expression.literal_token().span(),
        SourceSpan::new(0, 2)
    );
    assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
}

#[test]
fn cudrix_parser_exposes_name_expression_parsing() {
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
    assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
}

#[test]
fn cudrix_parser_exposes_parenthesized_expression_parsing() {
    let source = SourceText::new("(42)");
    let mut parser = Parser::new(&source);

    let parenthesized_expression = parser
        .parse_parenthesized_expression()
        .expect("parenthesized literal should parse");

    assert_eq!(
        parenthesized_expression.left_parenthesis_token().kind(),
        TokenKind::LeftParenthesis
    );

    let ExpressionSyntax::Literal(literal_expression) = parenthesized_expression.expression()
    else {
        panic!("expected literal expression");
    };

    assert_eq!(literal_expression.literal_token().kind(), TokenKind::Number);
    assert_eq!(
        parenthesized_expression.right_parenthesis_token().kind(),
        TokenKind::RightParenthesis
    );
    assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
}

#[test]
fn cudrix_parser_exposes_expression_parsing() {
    let source = SourceText::new("answer");
    let mut parser = Parser::new(&source);

    let expression = parser
        .parse_expression()
        .expect("name should parse as expression");

    let ExpressionSyntax::Name(name_expression) = expression else {
        panic!("expected name expression");
    };

    assert_eq!(
        name_expression.identifier_token().kind(),
        TokenKind::Identifier
    );
    assert_eq!(
        name_expression.identifier_token().span(),
        SourceSpan::new(0, 6)
    );
    assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
}
