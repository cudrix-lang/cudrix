use cudrix::LANGUAGE_NAME;
use cudrix::compiler::{Lexer, Parser, Token, TokenKind};
use cudrix::source::{SourceFile, SourceLocation, SourceSpan, SourceText};
use cudrix::syntax::{
    ArrayLiteralExpressionSyntax, AssignmentExpressionSyntax, BinaryExpressionSyntax, BlockSyntax,
    CallExpressionSyntax, ConditionalExpressionSyntax, ConditionalStatementSyntax,
    ElseClauseSyntax, ExpressionStatementSyntax, ExpressionSyntax, IndexExpressionSyntax,
    LiteralExpressionSyntax, MemberAccessExpressionSyntax, NameExpressionSyntax,
    ObjectLiteralExpressionSyntax, ObjectMemberSyntax, ParenthesizedExpressionSyntax,
    StatementSyntax, UnaryExpressionSyntax,
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
fn cudrix_exposes_unary_expression_syntax() {
    let operator_token = Token::new(TokenKind::Minus, SourceSpan::new(0, 1));
    let literal_token = Token::new(TokenKind::Number, SourceSpan::new(1, 2));

    let operand = ExpressionSyntax::Literal(LiteralExpressionSyntax::new(literal_token));

    let unary_expression = UnaryExpressionSyntax::new(operator_token, operand.clone());

    assert_eq!(unary_expression.operator_token(), operator_token);
    assert_eq!(unary_expression.operand(), &operand);
}

#[test]
fn cudrix_exposes_binary_expression_syntax() {
    let left_token = Token::new(TokenKind::Number, SourceSpan::new(0, 1));
    let operator_token = Token::new(TokenKind::Plus, SourceSpan::new(2, 1));
    let right_token = Token::new(TokenKind::Number, SourceSpan::new(4, 1));

    let left = ExpressionSyntax::Literal(LiteralExpressionSyntax::new(left_token));
    let right = ExpressionSyntax::Literal(LiteralExpressionSyntax::new(right_token));

    let expression = BinaryExpressionSyntax::new(left, operator_token, right);

    assert_eq!(expression.operator_token(), operator_token);
    assert!(matches!(expression.left(), ExpressionSyntax::Literal(_)));
    assert!(matches!(expression.right(), ExpressionSyntax::Literal(_)));
}

#[test]
fn cudrix_exposes_conditional_expression_syntax() {
    let condition_token = Token::new(TokenKind::Identifier, SourceSpan::new(0, 5));
    let question_token = Token::new(TokenKind::QuestionMark, SourceSpan::new(6, 1));
    let when_true_token = Token::new(TokenKind::Number, SourceSpan::new(8, 1));
    let colon_token = Token::new(TokenKind::Colon, SourceSpan::new(10, 1));
    let when_false_token = Token::new(TokenKind::Number, SourceSpan::new(12, 1));

    let condition = ExpressionSyntax::Name(NameExpressionSyntax::new(condition_token));
    let when_true = ExpressionSyntax::Literal(LiteralExpressionSyntax::new(when_true_token));
    let when_false = ExpressionSyntax::Literal(LiteralExpressionSyntax::new(when_false_token));

    let conditional_expression = ConditionalExpressionSyntax::new(
        condition.clone(),
        question_token,
        when_true.clone(),
        colon_token,
        when_false.clone(),
    );

    assert_eq!(conditional_expression.condition(), &condition);
    assert_eq!(conditional_expression.question_token(), question_token);
    assert_eq!(conditional_expression.when_true(), &when_true);
    assert_eq!(conditional_expression.colon_token(), colon_token);
    assert_eq!(conditional_expression.when_false(), &when_false);

    let expression = ExpressionSyntax::Conditional(conditional_expression.clone());

    assert_eq!(
        expression,
        ExpressionSyntax::Conditional(conditional_expression)
    );
}

#[test]
fn cudrix_exposes_assignment_expression_syntax() {
    let identifier_token = Token::new(TokenKind::Identifier, SourceSpan::new(0, 6));
    let assignment_token = Token::new(TokenKind::Assignment, SourceSpan::new(7, 1));
    let literal_token = Token::new(TokenKind::Number, SourceSpan::new(9, 2));

    let expression = ExpressionSyntax::Literal(LiteralExpressionSyntax::new(literal_token));

    let assignment_expression =
        AssignmentExpressionSyntax::new(identifier_token, assignment_token, expression);

    assert_eq!(
        assignment_expression.identifier_token().kind(),
        TokenKind::Identifier
    );
    assert_eq!(
        assignment_expression.assignment_token().kind(),
        TokenKind::Assignment
    );
    assert!(matches!(
        assignment_expression.expression(),
        ExpressionSyntax::Literal(_)
    ));
}

#[test]
fn cudrix_exposes_call_expression_syntax() {
    let identifier_token = Token::new(TokenKind::Identifier, SourceSpan::new(0, 3));
    let left_parenthesis_token = Token::new(TokenKind::LeftParenthesis, SourceSpan::new(3, 1));
    let argument_token = Token::new(TokenKind::Number, SourceSpan::new(4, 2));
    let right_parenthesis_token = Token::new(TokenKind::RightParenthesis, SourceSpan::new(6, 1));

    let callee = ExpressionSyntax::Name(NameExpressionSyntax::new(identifier_token));
    let argument = ExpressionSyntax::Literal(LiteralExpressionSyntax::new(argument_token));

    let call_expression = CallExpressionSyntax::new(
        callee.clone(),
        left_parenthesis_token,
        vec![argument.clone()],
        right_parenthesis_token,
    );

    assert_eq!(call_expression.callee(), &callee);
    assert_eq!(
        call_expression.left_parenthesis_token(),
        left_parenthesis_token
    );
    assert_eq!(call_expression.arguments(), &[argument]);
    assert_eq!(
        call_expression.right_parenthesis_token(),
        right_parenthesis_token
    );

    let expression = ExpressionSyntax::Call(call_expression.clone());

    assert_eq!(expression, ExpressionSyntax::Call(call_expression));
}

#[test]
fn cudrix_exposes_member_access_expression_syntax() {
    let identifier_token = Token::new(TokenKind::Identifier, SourceSpan::new(0, 6));
    let dot_token = Token::new(TokenKind::Dot, SourceSpan::new(6, 1));
    let member_name_token = Token::new(TokenKind::Identifier, SourceSpan::new(7, 6));

    let expression = ExpressionSyntax::Name(NameExpressionSyntax::new(identifier_token));

    let member_access_expression =
        MemberAccessExpressionSyntax::new(expression, dot_token, member_name_token);

    assert!(matches!(
        member_access_expression.expression(),
        ExpressionSyntax::Name(_)
    ));
    assert_eq!(member_access_expression.dot_token().kind(), TokenKind::Dot);
    assert_eq!(
        member_access_expression.member_name_token().kind(),
        TokenKind::Identifier
    );
}

#[test]
fn cudrix_exposes_index_expression_syntax() {
    let identifier_token = Token::new(TokenKind::Identifier, SourceSpan::new(0, 5));
    let left_bracket_token = Token::new(TokenKind::LeftBracket, SourceSpan::new(5, 1));
    let index_token = Token::new(TokenKind::Number, SourceSpan::new(6, 1));
    let right_bracket_token = Token::new(TokenKind::RightBracket, SourceSpan::new(7, 1));

    let index_expression = IndexExpressionSyntax::new(
        ExpressionSyntax::Name(NameExpressionSyntax::new(identifier_token)),
        left_bracket_token,
        ExpressionSyntax::Literal(LiteralExpressionSyntax::new(index_token)),
        right_bracket_token,
    );

    assert!(matches!(
        index_expression.expression(),
        ExpressionSyntax::Name(_)
    ));
    assert_eq!(
        index_expression.left_bracket_token().kind(),
        TokenKind::LeftBracket
    );
    assert!(matches!(
        index_expression.index(),
        ExpressionSyntax::Literal(_)
    ));
    assert_eq!(
        index_expression.right_bracket_token().kind(),
        TokenKind::RightBracket
    );

    let expression = ExpressionSyntax::Index(index_expression.clone());

    assert_eq!(expression, ExpressionSyntax::Index(index_expression));
}

#[test]
fn cudrix_parser_exposes_index_expression_parsing() {
    let source = SourceText::new("items[0]");
    let mut parser = Parser::new(&source);

    let expression = parser
        .parse_expression()
        .expect("index expression should parse through the public parser API");

    let ExpressionSyntax::Index(index_expression) = expression else {
        panic!("expected index expression");
    };

    assert!(matches!(
        index_expression.expression(),
        ExpressionSyntax::Name(_)
    ));
    assert!(matches!(
        index_expression.index(),
        ExpressionSyntax::Literal(_)
    ));
    assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
}

#[test]
fn cudrix_exposes_array_literal_expression_syntax() {
    let left_bracket_token = Token::new(TokenKind::LeftBracket, SourceSpan::new(0, 1));
    let element_token = Token::new(TokenKind::Number, SourceSpan::new(1, 1));
    let right_bracket_token = Token::new(TokenKind::RightBracket, SourceSpan::new(2, 1));

    let element = ExpressionSyntax::Literal(LiteralExpressionSyntax::new(element_token));

    let array_literal_expression = ArrayLiteralExpressionSyntax::new(
        left_bracket_token,
        vec![element.clone()],
        right_bracket_token,
    );

    assert_eq!(
        array_literal_expression.left_bracket_token(),
        left_bracket_token
    );
    assert_eq!(array_literal_expression.elements(), &[element]);
    assert_eq!(
        array_literal_expression.right_bracket_token(),
        right_bracket_token
    );
}

#[test]
fn cudrix_exposes_object_literal_expression_syntax() {
    let left_brace_token = Token::new(TokenKind::LeftBrace, SourceSpan::new(0, 1));
    let name_token = Token::new(TokenKind::Identifier, SourceSpan::new(1, 4));
    let colon_token = Token::new(TokenKind::Colon, SourceSpan::new(5, 1));
    let value_token = Token::new(TokenKind::Number, SourceSpan::new(7, 2));
    let right_brace_token = Token::new(TokenKind::RightBrace, SourceSpan::new(9, 1));

    let member = ObjectMemberSyntax::new(
        name_token,
        colon_token,
        ExpressionSyntax::Literal(LiteralExpressionSyntax::new(value_token)),
    );

    let object_literal_expression = ObjectLiteralExpressionSyntax::new(
        left_brace_token,
        vec![member.clone()],
        right_brace_token,
    );

    assert_eq!(
        object_literal_expression.left_brace_token(),
        left_brace_token
    );
    assert_eq!(object_literal_expression.members(), &[member]);
    assert_eq!(
        object_literal_expression.right_brace_token(),
        right_brace_token
    );

    let expression = ExpressionSyntax::ObjectLiteral(object_literal_expression.clone());

    assert_eq!(
        expression,
        ExpressionSyntax::ObjectLiteral(object_literal_expression)
    );
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
fn cudrix_exposes_statement_syntax() {
    let identifier_token = Token::new(TokenKind::Identifier, SourceSpan::new(0, 6));
    let semicolon_token = Token::new(TokenKind::Semicolon, SourceSpan::new(6, 1));

    let expression = ExpressionSyntax::Name(NameExpressionSyntax::new(identifier_token));
    let expression_statement = ExpressionStatementSyntax::new(expression.clone(), semicolon_token);

    assert_eq!(expression_statement.expression(), &expression);
    assert_eq!(expression_statement.semicolon_token(), semicolon_token);

    let statement = StatementSyntax::Expression(expression_statement.clone());

    assert_eq!(statement, StatementSyntax::Expression(expression_statement));
}

#[test]
fn cudrix_exposes_block_syntax() {
    let left_brace_token = Token::new(TokenKind::LeftBrace, SourceSpan::new(0, 1));
    let identifier_token = Token::new(TokenKind::Identifier, SourceSpan::new(2, 6));
    let semicolon_token = Token::new(TokenKind::Semicolon, SourceSpan::new(8, 1));
    let right_brace_token = Token::new(TokenKind::RightBrace, SourceSpan::new(10, 1));

    let expression = ExpressionSyntax::Name(NameExpressionSyntax::new(identifier_token));
    let statement =
        StatementSyntax::Expression(ExpressionStatementSyntax::new(expression, semicolon_token));

    let block = BlockSyntax::new(left_brace_token, vec![statement.clone()], right_brace_token);

    assert_eq!(block.left_brace_token(), left_brace_token);
    assert_eq!(block.statements(), &[statement]);
    assert_eq!(block.right_brace_token(), right_brace_token);
}

#[test]
fn cudrix_exposes_else_clause_syntax() {
    let else_keyword = Token::new(TokenKind::ElseKeyword, SourceSpan::new(0, 4));
    let left_brace_token = Token::new(TokenKind::LeftBrace, SourceSpan::new(5, 1));
    let right_brace_token = Token::new(TokenKind::RightBrace, SourceSpan::new(6, 1));

    let block = BlockSyntax::new(left_brace_token, Vec::new(), right_brace_token);
    let else_clause = ElseClauseSyntax::new(else_keyword, block);

    assert_eq!(else_clause.else_keyword(), else_keyword);
    assert_eq!(else_clause.block().left_brace_token(), left_brace_token);
    assert!(else_clause.block().statements().is_empty());
    assert_eq!(else_clause.block().right_brace_token(), right_brace_token);
}

#[test]
fn cudrix_exposes_conditional_statement_syntax() {
    let if_keyword = Token::new(TokenKind::IfKeyword, SourceSpan::new(0, 2));
    let condition_token = Token::new(TokenKind::Identifier, SourceSpan::new(3, 9));
    let then_left_brace = Token::new(TokenKind::LeftBrace, SourceSpan::new(13, 1));
    let then_right_brace = Token::new(TokenKind::RightBrace, SourceSpan::new(14, 1));
    let else_keyword = Token::new(TokenKind::ElseKeyword, SourceSpan::new(16, 4));
    let else_left_brace = Token::new(TokenKind::LeftBrace, SourceSpan::new(21, 1));
    let else_right_brace = Token::new(TokenKind::RightBrace, SourceSpan::new(22, 1));

    let condition = ExpressionSyntax::Name(NameExpressionSyntax::new(condition_token));
    let then_block = BlockSyntax::new(then_left_brace, Vec::new(), then_right_brace);
    let else_block = BlockSyntax::new(else_left_brace, Vec::new(), else_right_brace);
    let else_clause = ElseClauseSyntax::new(else_keyword, else_block);

    let conditional_statement = ConditionalStatementSyntax::new(
        if_keyword,
        condition,
        then_block,
        Vec::new(),
        Some(else_clause),
    );

    assert_eq!(conditional_statement.if_keyword(), if_keyword);
    assert!(matches!(
        conditional_statement.condition(),
        ExpressionSyntax::Name(_)
    ));
    assert!(conditional_statement.then_block().statements().is_empty());

    let else_clause = conditional_statement
        .else_clause()
        .expect("expected else clause");

    assert_eq!(else_clause.else_keyword(), else_keyword);
    assert!(else_clause.block().statements().is_empty());
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
fn cudrix_parser_exposes_unary_expression_parsing() {
    let source = SourceText::new("-42");
    let mut parser = Parser::new(&source);

    let unary_expression = parser
        .parse_unary_expression()
        .expect("unary literal should parse");

    assert_eq!(unary_expression.operator_token().kind(), TokenKind::Minus);
    assert_eq!(
        unary_expression.operator_token().span(),
        SourceSpan::new(0, 1)
    );

    let ExpressionSyntax::Literal(literal_expression) = unary_expression.operand() else {
        panic!("expected literal operand");
    };

    assert_eq!(literal_expression.literal_token().kind(), TokenKind::Number);
    assert_eq!(
        literal_expression.literal_token().span(),
        SourceSpan::new(1, 2)
    );
    assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
}

#[test]
fn cudrix_parser_exposes_binary_expression_parsing() {
    let source = SourceText::new("1 + 2");
    let mut parser = Parser::new(&source);

    let expression = parser
        .parse_expression()
        .expect("binary expression should parse through the public API");

    let ExpressionSyntax::Binary(binary_expression) = expression else {
        panic!("expected binary expression");
    };

    assert_eq!(binary_expression.operator_token().kind(), TokenKind::Plus);
    assert!(matches!(
        binary_expression.left(),
        ExpressionSyntax::Literal(_)
    ));
    assert!(matches!(
        binary_expression.right(),
        ExpressionSyntax::Literal(_)
    ));
    assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
}

#[test]
fn cudrix_parser_exposes_conditional_expression_parsing() {
    let source = SourceText::new("condition ? when_true : when_false");
    let mut parser = Parser::new(&source);

    let expression = parser
        .parse_expression()
        .expect("conditional expression should parse through the public API");

    let ExpressionSyntax::Conditional(conditional_expression) = expression else {
        panic!("expected conditional expression");
    };

    assert!(matches!(
        conditional_expression.condition(),
        ExpressionSyntax::Name(_)
    ));
    assert_eq!(
        conditional_expression.question_token().kind(),
        TokenKind::QuestionMark
    );
    assert!(matches!(
        conditional_expression.when_true(),
        ExpressionSyntax::Name(_)
    ));
    assert_eq!(
        conditional_expression.colon_token().kind(),
        TokenKind::Colon
    );
    assert!(matches!(
        conditional_expression.when_false(),
        ExpressionSyntax::Name(_)
    ));
    assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
}

#[test]
fn cudrix_parser_exposes_assignment_expression_parsing() {
    let source = SourceText::new("answer = 42");
    let mut parser = Parser::new(&source);

    let expression = parser
        .parse_assignment_expression()
        .expect("assignment expression should parse through public API");

    let ExpressionSyntax::Assignment(assignment) = expression else {
        panic!("expected assignment expression");
    };

    assert_eq!(assignment.identifier_token().kind(), TokenKind::Identifier);
    assert_eq!(assignment.assignment_token().kind(), TokenKind::Assignment);
    assert!(matches!(
        assignment.expression(),
        ExpressionSyntax::Literal(_)
    ));
    assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
}

#[test]
fn cudrix_parser_exposes_member_access_expression_parsing() {
    let source = SourceText::new("object.member");
    let mut parser = Parser::new(&source);

    let expression = parser.parse_expression().unwrap();

    let ExpressionSyntax::MemberAccess(member_access) = expression else {
        panic!("expected member access expression");
    };

    assert!(matches!(
        member_access.expression(),
        ExpressionSyntax::Name(_)
    ));
    assert_eq!(
        member_access.member_name_token().kind(),
        TokenKind::Identifier
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

#[test]
fn cudrix_parser_exposes_statement_parsing() {
    let source = SourceText::new("answer;");
    let mut parser = Parser::new(&source);

    let statement = parser
        .parse_statement()
        .expect("expression statement should parse through the public API");

    let StatementSyntax::Expression(expression_statement) = statement else {
        panic!("expected expression statement");
    };

    assert!(matches!(
        expression_statement.expression(),
        ExpressionSyntax::Name(_)
    ));
    assert_eq!(
        expression_statement.semicolon_token().kind(),
        TokenKind::Semicolon
    );
    assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
}

#[test]
fn cudrix_parser_exposes_conditional_statement_parsing() {
    let source = SourceText::new("if condition { answer; } else { fallback; }");
    let mut parser = Parser::new(&source);

    let statement = parser
        .parse_statement()
        .expect("conditional statement should parse through the public API");

    let StatementSyntax::Conditional(conditional_statement) = statement else {
        panic!("expected conditional statement");
    };

    assert_eq!(
        conditional_statement.if_keyword().kind(),
        TokenKind::IfKeyword
    );
    assert!(matches!(
        conditional_statement.condition(),
        ExpressionSyntax::Name(_)
    ));
    assert_eq!(conditional_statement.then_block().statements().len(), 1);

    let else_clause = conditional_statement
        .else_clause()
        .expect("expected else clause");

    assert_eq!(else_clause.else_keyword().kind(), TokenKind::ElseKeyword);
    assert_eq!(else_clause.block().statements().len(), 1);
    assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
}

#[test]
fn cudrix_parser_exposes_block_parsing() {
    let source = SourceText::new("{ answer; 1 + 2; }");
    let mut parser = Parser::new(&source);

    let block = parser.parse_block().expect("expected block");

    assert_eq!(block.left_brace_token().kind(), TokenKind::LeftBrace);
    assert_eq!(block.statements().len(), 2);
    assert_eq!(block.right_brace_token().kind(), TokenKind::RightBrace);
    assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
}

#[test]
fn cudrix_parser_exposes_call_expression_parsing() {
    let source = SourceText::new("foo(1, 2)");
    let mut parser = Parser::new(&source);

    let expression = parser
        .parse_expression()
        .expect("call expression should parse through the public parser API");

    let ExpressionSyntax::Call(call_expression) = expression else {
        panic!("expected call expression");
    };

    assert_eq!(call_expression.arguments().len(), 2);
    assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
}

#[test]
fn cudrix_parser_exposes_array_literal_expression_parsing() {
    let source = SourceText::new("[1, 2]");
    let mut parser = Parser::new(&source);

    let expression = parser
        .parse_expression()
        .expect("array literal expression should parse");

    let ExpressionSyntax::ArrayLiteral(array_literal_expression) = expression else {
        panic!("expected array literal expression");
    };

    assert_eq!(array_literal_expression.elements().len(), 2);
    assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
}

#[test]
fn cudrix_parser_exposes_object_literal_expression_parsing() {
    let source = SourceText::new("{name: value, age: 42}");
    let mut parser = Parser::new(&source);

    let expression = parser
        .parse_expression()
        .expect("object literal expression should parse");

    let ExpressionSyntax::ObjectLiteral(object_literal_expression) = expression else {
        panic!("expected object literal expression");
    };

    assert_eq!(object_literal_expression.members().len(), 2);
    assert_eq!(
        object_literal_expression.members()[0].name_token().kind(),
        TokenKind::Identifier
    );
    assert_eq!(
        object_literal_expression.members()[0].colon_token().kind(),
        TokenKind::Colon
    );
    assert!(matches!(
        object_literal_expression.members()[0].value(),
        ExpressionSyntax::Name(_)
    ));
    assert!(matches!(
        object_literal_expression.members()[1].value(),
        ExpressionSyntax::Literal(_)
    ));
    assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
}
