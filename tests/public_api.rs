use cudrix::LANGUAGE_NAME;
use cudrix::compiler::{Lexer, Parser, Token, TokenKind};
use cudrix::source::{SourceFile, SourceLocation, SourceSpan, SourceText};
use cudrix::syntax::{
    ArrayLiteralExpressionSyntax, AssignmentExpressionSyntax, BinaryExpressionSyntax, BlockSyntax,
    CallExpressionSyntax, CompilationUnitSyntax, ConditionalExpressionSyntax,
    ConditionalStatementSyntax, ElseClauseSyntax, ElseIfClauseSyntax, ExpressionStatementSyntax,
    ExpressionSyntax, ForStatementSyntax, FunctionDeclarationSyntax, FunctionParameterSyntax,
    ImportAliasSyntax, ImportDeclarationSyntax, IndexExpressionSyntax, LiteralExpressionSyntax,
    MemberAccessExpressionSyntax, ModuleDeclarationSyntax, NameExpressionSyntax,
    ObjectLiteralExpressionSyntax, ObjectMemberSyntax, ParenthesizedExpressionSyntax,
    QualifiedImportNameSegmentSyntax, ReturnStatementSyntax, StatementSyntax,
    UnaryExpressionSyntax, VariableDeclarationStatementSyntax, WhileStatementSyntax,
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
fn cudrix_exposes_elseif_clause_syntax() {
    let elseif_keyword = Token::new(TokenKind::ElseIfKeyword, SourceSpan::new(0, 6));
    let condition_token = Token::new(TokenKind::Identifier, SourceSpan::new(7, 9));
    let left_brace_token = Token::new(TokenKind::LeftBrace, SourceSpan::new(17, 1));
    let right_brace_token = Token::new(TokenKind::RightBrace, SourceSpan::new(18, 1));

    let condition = ExpressionSyntax::Name(NameExpressionSyntax::new(condition_token));
    let block = BlockSyntax::new(left_brace_token, Vec::new(), right_brace_token);

    let elseif_clause = ElseIfClauseSyntax::new(elseif_keyword, condition, block);

    assert_eq!(elseif_clause.elseif_keyword(), elseif_keyword);
    assert!(matches!(
        elseif_clause.condition(),
        ExpressionSyntax::Name(_)
    ));
    assert_eq!(elseif_clause.block().left_brace_token(), left_brace_token);
    assert!(elseif_clause.block().statements().is_empty());
    assert_eq!(elseif_clause.block().right_brace_token(), right_brace_token);
}

#[test]
fn cudrix_exposes_conditional_statement_syntax() {
    let if_keyword = Token::new(TokenKind::IfKeyword, SourceSpan::new(0, 2));
    let condition_token = Token::new(TokenKind::Identifier, SourceSpan::new(3, 9));
    let then_left_brace = Token::new(TokenKind::LeftBrace, SourceSpan::new(13, 1));
    let then_right_brace = Token::new(TokenKind::RightBrace, SourceSpan::new(14, 1));
    let elseif_keyword = Token::new(TokenKind::ElseIfKeyword, SourceSpan::new(16, 6));
    let elseif_condition_token = Token::new(TokenKind::Identifier, SourceSpan::new(23, 9));
    let elseif_left_brace = Token::new(TokenKind::LeftBrace, SourceSpan::new(33, 1));
    let elseif_right_brace = Token::new(TokenKind::RightBrace, SourceSpan::new(34, 1));
    let else_keyword = Token::new(TokenKind::ElseKeyword, SourceSpan::new(36, 4));
    let else_left_brace = Token::new(TokenKind::LeftBrace, SourceSpan::new(41, 1));
    let else_right_brace = Token::new(TokenKind::RightBrace, SourceSpan::new(42, 1));
    let condition = ExpressionSyntax::Name(NameExpressionSyntax::new(condition_token));
    let then_block = BlockSyntax::new(then_left_brace, Vec::new(), then_right_brace);
    let elseif_condition =
        ExpressionSyntax::Name(NameExpressionSyntax::new(elseif_condition_token));
    let elseif_block = BlockSyntax::new(elseif_left_brace, Vec::new(), elseif_right_brace);
    let elseif_clause = ElseIfClauseSyntax::new(elseif_keyword, elseif_condition, elseif_block);
    let else_block = BlockSyntax::new(else_left_brace, Vec::new(), else_right_brace);
    let else_clause = ElseClauseSyntax::new(else_keyword, else_block);

    let conditional_statement = ConditionalStatementSyntax::new(
        if_keyword,
        condition,
        then_block,
        vec![elseif_clause],
        Some(else_clause),
    );

    assert_eq!(conditional_statement.if_keyword(), if_keyword);
    assert!(matches!(
        conditional_statement.condition(),
        ExpressionSyntax::Name(_)
    ));
    assert!(conditional_statement.then_block().statements().is_empty());
    assert_eq!(conditional_statement.elseif_clauses().len(), 1);

    let elseif_clause = &conditional_statement.elseif_clauses()[0];

    assert_eq!(elseif_clause.elseif_keyword(), elseif_keyword);
    assert!(matches!(
        elseif_clause.condition(),
        ExpressionSyntax::Name(_)
    ));
    assert!(elseif_clause.block().statements().is_empty());

    let else_clause = conditional_statement
        .else_clause()
        .expect("expected else clause");

    assert_eq!(else_clause.else_keyword(), else_keyword);
    assert!(else_clause.block().statements().is_empty());
}

#[test]
fn cudrix_exposes_while_statement_syntax() {
    let while_keyword = Token::new(TokenKind::WhileKeyword, SourceSpan::new(0, 5));
    let condition_token = Token::new(TokenKind::Identifier, SourceSpan::new(6, 7));
    let left_brace_token = Token::new(TokenKind::LeftBrace, SourceSpan::new(14, 1));
    let right_brace_token = Token::new(TokenKind::RightBrace, SourceSpan::new(15, 1));

    let condition = ExpressionSyntax::Name(NameExpressionSyntax::new(condition_token));
    let block = BlockSyntax::new(left_brace_token, Vec::new(), right_brace_token);

    let while_statement = WhileStatementSyntax::new(while_keyword, condition, block);

    assert_eq!(while_statement.while_keyword(), while_keyword);
    assert!(matches!(
        while_statement.condition(),
        ExpressionSyntax::Name(_)
    ));
    assert_eq!(while_statement.block().left_brace_token(), left_brace_token);
    assert!(while_statement.block().statements().is_empty());
    assert_eq!(
        while_statement.block().right_brace_token(),
        right_brace_token
    );

    let statement = StatementSyntax::While(while_statement.clone());

    assert_eq!(statement, StatementSyntax::While(while_statement));
}

#[test]
fn cudrix_exposes_for_statement_syntax() {
    let for_keyword = Token::new(TokenKind::ForKeyword, SourceSpan::new(0, 3));
    let initializer = ExpressionSyntax::Name(NameExpressionSyntax::new(Token::new(
        TokenKind::Identifier,
        SourceSpan::new(4, 1),
    )));
    let first_semicolon_token = Token::new(TokenKind::Semicolon, SourceSpan::new(5, 1));
    let condition = ExpressionSyntax::Literal(LiteralExpressionSyntax::new(Token::new(
        TokenKind::TrueKeyword,
        SourceSpan::new(7, 4),
    )));
    let second_semicolon_token = Token::new(TokenKind::Semicolon, SourceSpan::new(11, 1));
    let update = ExpressionSyntax::Name(NameExpressionSyntax::new(Token::new(
        TokenKind::Identifier,
        SourceSpan::new(13, 1),
    )));
    let block = BlockSyntax::new(
        Token::new(TokenKind::LeftBrace, SourceSpan::new(15, 1)),
        Vec::new(),
        Token::new(TokenKind::RightBrace, SourceSpan::new(16, 1)),
    );

    let statement = ForStatementSyntax::new(
        for_keyword,
        initializer,
        first_semicolon_token,
        condition,
        second_semicolon_token,
        update,
        block,
    );

    assert_eq!(statement.for_keyword(), for_keyword);
    assert!(matches!(statement.initializer(), ExpressionSyntax::Name(_)));
    assert_eq!(statement.first_semicolon_token(), first_semicolon_token);
    assert!(matches!(
        statement.condition(),
        ExpressionSyntax::Literal(_)
    ));
    assert_eq!(statement.second_semicolon_token(), second_semicolon_token);
    assert!(matches!(statement.update(), ExpressionSyntax::Name(_)));
    assert!(statement.block().statements().is_empty());

    let statement = StatementSyntax::For(statement);
    assert!(matches!(statement, StatementSyntax::For(_)));
}

#[test]
fn cudrix_exposes_return_statement_syntax() {
    let return_keyword = Token::new(TokenKind::ReturnKeyword, SourceSpan::new(0, 6));
    let expression_token = Token::new(TokenKind::Identifier, SourceSpan::new(7, 5));
    let semicolon_token = Token::new(TokenKind::Semicolon, SourceSpan::new(12, 1));

    let expression = ExpressionSyntax::Name(NameExpressionSyntax::new(expression_token));

    let return_statement =
        ReturnStatementSyntax::new(return_keyword, expression.clone(), semicolon_token);

    assert_eq!(return_statement.return_keyword(), return_keyword);
    assert_eq!(return_statement.expression(), &expression);
    assert_eq!(return_statement.semicolon_token(), semicolon_token);
}

#[test]
fn cudrix_exposes_variable_declaration_statement_syntax() {
    let declaration_keyword = Token::new(TokenKind::LetKeyword, SourceSpan::new(0, 3));
    let identifier_token = Token::new(TokenKind::Identifier, SourceSpan::new(4, 5));
    let assignment_token = Token::new(TokenKind::Assignment, SourceSpan::new(10, 1));
    let initializer_token = Token::new(TokenKind::Number, SourceSpan::new(12, 1));
    let semicolon_token = Token::new(TokenKind::Semicolon, SourceSpan::new(13, 1));

    let initializer = ExpressionSyntax::Literal(LiteralExpressionSyntax::new(initializer_token));

    let declaration = VariableDeclarationStatementSyntax::new(
        declaration_keyword,
        identifier_token,
        assignment_token,
        initializer.clone(),
        semicolon_token,
    );

    assert_eq!(declaration.declaration_keyword(), declaration_keyword);
    assert_eq!(declaration.identifier_token(), identifier_token);
    assert_eq!(declaration.assignment_token(), assignment_token);
    assert_eq!(declaration.initializer(), &initializer);
    assert_eq!(declaration.semicolon_token(), semicolon_token);
}

#[test]
fn cudrix_exposes_module_declaration_syntax() {
    let module_keyword = Token::new(TokenKind::ModuleKeyword, SourceSpan::new(0, 6));
    let identifier_token = Token::new(TokenKind::Identifier, SourceSpan::new(7, 11));
    let semicolon_token = Token::new(TokenKind::Semicolon, SourceSpan::new(18, 1));

    let declaration =
        ModuleDeclarationSyntax::new(module_keyword, identifier_token, semicolon_token);

    assert_eq!(declaration.module_keyword(), module_keyword);
    assert_eq!(declaration.identifier_token(), identifier_token);
    assert_eq!(declaration.semicolon_token(), semicolon_token);
}

#[test]
fn cudrix_exposes_import_alias_syntax() {
    let as_keyword = Token::new(TokenKind::AsKeyword, SourceSpan::new(0, 2));
    let identifier_token = Token::new(TokenKind::Identifier, SourceSpan::new(3, 2));

    let alias = ImportAliasSyntax::new(as_keyword, identifier_token);

    assert_eq!(alias.as_keyword(), as_keyword);
    assert_eq!(alias.identifier_token(), identifier_token);
}

#[test]
fn cudrix_exposes_selective_import_member_syntax() {
    let identifier_token = Token::new(TokenKind::Identifier, SourceSpan::new(0, 4));
    let comma_token = Token::new(TokenKind::Comma, SourceSpan::new(4, 1));

    let member =
        cudrix::syntax::SelectiveImportMemberSyntax::new(identifier_token, Some(comma_token));

    assert_eq!(member.identifier_token(), identifier_token);
    assert_eq!(member.comma_token(), Some(comma_token));

    let final_identifier_token = Token::new(TokenKind::Identifier, SourceSpan::new(6, 5));

    let final_member =
        cudrix::syntax::SelectiveImportMemberSyntax::new(final_identifier_token, None);

    assert_eq!(final_member.identifier_token(), final_identifier_token);
    assert!(final_member.comma_token().is_none());
}

#[test]
fn cudrix_exposes_selective_import_clause_syntax() {
    let dot_token = Token::new(TokenKind::Dot, SourceSpan::new(0, 1));
    let left_brace_token = Token::new(TokenKind::LeftBrace, SourceSpan::new(1, 1));
    let first_identifier_token = Token::new(TokenKind::Identifier, SourceSpan::new(2, 4));
    let comma_token = Token::new(TokenKind::Comma, SourceSpan::new(6, 1));
    let second_identifier_token = Token::new(TokenKind::Identifier, SourceSpan::new(8, 5));
    let right_brace_token = Token::new(TokenKind::RightBrace, SourceSpan::new(13, 1));

    let members = vec![
        cudrix::syntax::SelectiveImportMemberSyntax::new(first_identifier_token, Some(comma_token)),
        cudrix::syntax::SelectiveImportMemberSyntax::new(second_identifier_token, None),
    ];

    let clause = cudrix::syntax::SelectiveImportClauseSyntax::new(
        dot_token,
        left_brace_token,
        members,
        right_brace_token,
    );

    assert_eq!(clause.dot_token(), dot_token);
    assert_eq!(clause.left_brace_token(), left_brace_token);
    assert_eq!(clause.members().len(), 2);
    assert_eq!(
        clause.members()[0].identifier_token(),
        first_identifier_token
    );
    assert_eq!(clause.members()[0].comma_token(), Some(comma_token));
    assert_eq!(
        clause.members()[1].identifier_token(),
        second_identifier_token
    );
    assert!(clause.members()[1].comma_token().is_none());
    assert_eq!(clause.right_brace_token(), right_brace_token);
}

#[test]
fn cudrix_exposes_import_declaration_syntax() {
    let import_keyword = Token::new(TokenKind::ImportKeyword, SourceSpan::new(0, 6));
    let identifier_token = Token::new(TokenKind::Identifier, SourceSpan::new(7, 8));
    let dot_token = Token::new(TokenKind::Dot, SourceSpan::new(15, 1));
    let qualified_identifier_token = Token::new(TokenKind::Identifier, SourceSpan::new(16, 2));
    let semicolon_token = Token::new(TokenKind::Semicolon, SourceSpan::new(18, 1));

    let qualified_name_segments = vec![QualifiedImportNameSegmentSyntax::new(
        dot_token,
        qualified_identifier_token,
    )];

    let declaration = ImportDeclarationSyntax::new(
        import_keyword,
        identifier_token,
        qualified_name_segments,
        None,
        None,
        semicolon_token,
    );

    assert_eq!(declaration.import_keyword(), import_keyword);
    assert_eq!(declaration.identifier_token(), identifier_token);
    assert_eq!(declaration.qualified_name_segments().len(), 1);
    assert_eq!(
        declaration.qualified_name_segments()[0].dot_token(),
        dot_token
    );
    assert_eq!(
        declaration.qualified_name_segments()[0].identifier_token(),
        qualified_identifier_token
    );
    assert!(declaration.alias().is_none());
    assert!(declaration.selective_import_clause().is_none());
    assert_eq!(declaration.semicolon_token(), semicolon_token);
}

#[test]
fn cudrix_exposes_import_declaration_with_selective_clause() {
    let import_keyword = Token::new(TokenKind::ImportKeyword, SourceSpan::new(0, 6));
    let identifier_token = Token::new(TokenKind::Identifier, SourceSpan::new(7, 8));
    let qualified_dot_token = Token::new(TokenKind::Dot, SourceSpan::new(15, 1));
    let qualified_identifier_token = Token::new(TokenKind::Identifier, SourceSpan::new(16, 2));
    let selective_dot_token = Token::new(TokenKind::Dot, SourceSpan::new(18, 1));
    let left_brace_token = Token::new(TokenKind::LeftBrace, SourceSpan::new(19, 1));
    let first_member_token = Token::new(TokenKind::Identifier, SourceSpan::new(20, 4));
    let comma_token = Token::new(TokenKind::Comma, SourceSpan::new(24, 1));
    let second_member_token = Token::new(TokenKind::Identifier, SourceSpan::new(26, 5));
    let right_brace_token = Token::new(TokenKind::RightBrace, SourceSpan::new(31, 1));
    let semicolon_token = Token::new(TokenKind::Semicolon, SourceSpan::new(32, 1));

    let qualified_name_segments = vec![QualifiedImportNameSegmentSyntax::new(
        qualified_dot_token,
        qualified_identifier_token,
    )];

    let members = vec![
        cudrix::syntax::SelectiveImportMemberSyntax::new(first_member_token, Some(comma_token)),
        cudrix::syntax::SelectiveImportMemberSyntax::new(second_member_token, None),
    ];

    let selective_clause = cudrix::syntax::SelectiveImportClauseSyntax::new(
        selective_dot_token,
        left_brace_token,
        members,
        right_brace_token,
    );

    let declaration = ImportDeclarationSyntax::new(
        import_keyword,
        identifier_token,
        qualified_name_segments,
        None,
        Some(selective_clause),
        semicolon_token,
    );

    assert_eq!(declaration.import_keyword(), import_keyword);
    assert_eq!(declaration.identifier_token(), identifier_token);
    assert_eq!(declaration.qualified_name_segments().len(), 1);
    assert!(declaration.alias().is_none());

    let clause = declaration
        .selective_import_clause()
        .expect("expected selective import clause");

    assert_eq!(clause.dot_token(), selective_dot_token);
    assert_eq!(clause.left_brace_token(), left_brace_token);
    assert_eq!(clause.members().len(), 2);
    assert_eq!(clause.members()[0].identifier_token(), first_member_token);
    assert_eq!(clause.members()[0].comma_token(), Some(comma_token));
    assert_eq!(clause.members()[1].identifier_token(), second_member_token);
    assert!(clause.members()[1].comma_token().is_none());
    assert_eq!(clause.right_brace_token(), right_brace_token);
    assert_eq!(declaration.semicolon_token(), semicolon_token);
}

#[test]
fn cudrix_exposes_function_declaration_syntax() {
    let function_keyword = Token::new(TokenKind::FunctionKeyword, SourceSpan::new(0, 8));
    let identifier_token = Token::new(TokenKind::Identifier, SourceSpan::new(9, 3));
    let left_parenthesis_token = Token::new(TokenKind::LeftParenthesis, SourceSpan::new(12, 1));
    let parameter_identifier_token = Token::new(TokenKind::Identifier, SourceSpan::new(13, 5));
    let comma_token = Token::new(TokenKind::Comma, SourceSpan::new(18, 1));
    let second_parameter_identifier_token =
        Token::new(TokenKind::Identifier, SourceSpan::new(20, 6));
    let right_parenthesis_token = Token::new(TokenKind::RightParenthesis, SourceSpan::new(26, 1));
    let left_brace_token = Token::new(TokenKind::LeftBrace, SourceSpan::new(28, 1));
    let right_brace_token = Token::new(TokenKind::RightBrace, SourceSpan::new(29, 1));

    let parameters = vec![
        FunctionParameterSyntax::new(parameter_identifier_token, Some(comma_token)),
        FunctionParameterSyntax::new(second_parameter_identifier_token, None),
    ];

    let body = BlockSyntax::new(left_brace_token, Vec::new(), right_brace_token);

    let declaration = FunctionDeclarationSyntax::new(
        function_keyword,
        identifier_token,
        left_parenthesis_token,
        parameters,
        right_parenthesis_token,
        body,
    );

    assert_eq!(declaration.function_keyword(), function_keyword);
    assert_eq!(declaration.identifier_token(), identifier_token);
    assert_eq!(declaration.left_parenthesis_token(), left_parenthesis_token);
    assert_eq!(declaration.parameters().len(), 2);

    assert_eq!(
        declaration.parameters()[0].identifier_token(),
        parameter_identifier_token
    );
    assert_eq!(declaration.parameters()[0].comma_token(), Some(comma_token));
    assert_eq!(
        declaration.parameters()[1].identifier_token(),
        second_parameter_identifier_token
    );
    assert_eq!(declaration.parameters()[1].comma_token(), None);

    assert_eq!(
        declaration.right_parenthesis_token(),
        right_parenthesis_token
    );
    assert_eq!(declaration.body().left_brace_token(), left_brace_token);
    assert!(declaration.body().statements().is_empty());
    assert_eq!(declaration.body().right_brace_token(), right_brace_token);
}

#[test]
fn cudrix_exposes_compilation_unit_syntax() {
    let end_of_file_token = Token::new(TokenKind::EndOfFile, SourceSpan::new(0, 0));

    let compilation_unit =
        CompilationUnitSyntax::new(None, Vec::new(), Vec::new(), end_of_file_token);

    assert!(compilation_unit.import_declarations().is_empty());
    assert!(compilation_unit.function_declarations().is_empty());
    assert_eq!(compilation_unit.end_of_file_token(), end_of_file_token);
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
    let source = SourceText::new(
        "if condition { answer; } elseif alternative { backup; } else { fallback; }",
    );
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

    assert_eq!(conditional_statement.elseif_clauses().len(), 1);

    let elseif_clause = &conditional_statement.elseif_clauses()[0];

    assert_eq!(
        elseif_clause.elseif_keyword().kind(),
        TokenKind::ElseIfKeyword
    );
    assert!(matches!(
        elseif_clause.condition(),
        ExpressionSyntax::Name(_)
    ));
    assert_eq!(elseif_clause.block().statements().len(), 1);

    let else_clause = conditional_statement
        .else_clause()
        .expect("expected else clause");

    assert_eq!(else_clause.else_keyword().kind(), TokenKind::ElseKeyword);
    assert_eq!(else_clause.block().statements().len(), 1);
    assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
}

#[test]
fn cudrix_parser_exposes_while_statement_parsing() {
    let source = SourceText::new("while running { process(); }");
    let mut parser = Parser::new(&source);

    let statement = parser
        .parse_statement()
        .expect("while statement should parse through the public API");

    let StatementSyntax::While(while_statement) = statement else {
        panic!("expected while statement");
    };

    assert_eq!(
        while_statement.while_keyword().kind(),
        TokenKind::WhileKeyword
    );
    assert!(matches!(
        while_statement.condition(),
        ExpressionSyntax::Name(_)
    ));
    assert_eq!(while_statement.block().statements().len(), 1);
    assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
}

#[test]
fn cudrix_parser_exposes_for_statement_parsing() {
    let source = SourceText::new("for i = 0; i < 10; i = i + 1 {}");
    let mut parser = Parser::new(&source);

    let statement = parser
        .parse_statement()
        .expect("for statement should parse through public API");

    let StatementSyntax::For(for_statement) = statement else {
        panic!("expected for statement");
    };

    assert_eq!(for_statement.for_keyword().kind(), TokenKind::ForKeyword);
    assert!(matches!(
        for_statement.initializer(),
        ExpressionSyntax::Assignment(_)
    ));
    assert_eq!(
        for_statement.first_semicolon_token().kind(),
        TokenKind::Semicolon
    );
    assert!(matches!(
        for_statement.condition(),
        ExpressionSyntax::Binary(_)
    ));
    assert_eq!(
        for_statement.second_semicolon_token().kind(),
        TokenKind::Semicolon
    );
    assert!(matches!(
        for_statement.update(),
        ExpressionSyntax::Assignment(_)
    ));
    assert!(for_statement.block().statements().is_empty());
    assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
}

#[test]
fn cudrix_parser_exposes_variable_declaration_statement_parsing() {
    let source = SourceText::new("let count = 0;");
    let mut parser = Parser::new(&source);

    let statement = parser.parse_statement().unwrap();

    let StatementSyntax::VariableDeclaration(declaration) = statement else {
        panic!("expected variable declaration statement");
    };

    assert_eq!(
        declaration.declaration_keyword().kind(),
        TokenKind::LetKeyword
    );
    assert_eq!(declaration.identifier_token().kind(), TokenKind::Identifier);
    assert_eq!(declaration.assignment_token().kind(), TokenKind::Assignment);
    assert!(matches!(
        declaration.initializer(),
        ExpressionSyntax::Literal(_)
    ));
    assert_eq!(declaration.semicolon_token().kind(), TokenKind::Semicolon);
    assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
}

#[test]
fn cudrix_parser_exposes_return_statement_parsing() {
    let source = SourceText::new("return result;");
    let mut parser = Parser::new(&source);

    let statement = parser.parse_statement().unwrap();

    let StatementSyntax::Return(return_statement) = statement else {
        panic!("expected return statement");
    };

    assert_eq!(
        return_statement.return_keyword().kind(),
        TokenKind::ReturnKeyword
    );
    assert!(matches!(
        return_statement.expression(),
        ExpressionSyntax::Name(_)
    ));
    assert_eq!(
        return_statement.semicolon_token().kind(),
        TokenKind::Semicolon
    );
    assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
}

#[test]
fn cudrix_exposes_variable_declaration_statement_variant() {
    let declaration_keyword = Token::new(TokenKind::VarKeyword, SourceSpan::new(0, 3));
    let identifier_token = Token::new(TokenKind::Identifier, SourceSpan::new(4, 5));
    let assignment_token = Token::new(TokenKind::Assignment, SourceSpan::new(10, 1));
    let initializer_token = Token::new(TokenKind::Number, SourceSpan::new(12, 2));
    let semicolon_token = Token::new(TokenKind::Semicolon, SourceSpan::new(14, 1));

    let initializer = ExpressionSyntax::Literal(LiteralExpressionSyntax::new(initializer_token));

    let declaration = VariableDeclarationStatementSyntax::new(
        declaration_keyword,
        identifier_token,
        assignment_token,
        initializer,
        semicolon_token,
    );

    let statement = StatementSyntax::VariableDeclaration(declaration.clone());

    assert_eq!(statement, StatementSyntax::VariableDeclaration(declaration));
}

#[test]
fn cudrix_exposes_return_statement_variant() {
    let return_keyword = Token::new(TokenKind::ReturnKeyword, SourceSpan::new(0, 6));
    let expression_token = Token::new(TokenKind::Identifier, SourceSpan::new(7, 5));
    let semicolon_token = Token::new(TokenKind::Semicolon, SourceSpan::new(12, 1));

    let expression = ExpressionSyntax::Name(NameExpressionSyntax::new(expression_token));

    let return_statement = ReturnStatementSyntax::new(return_keyword, expression, semicolon_token);

    let statement = StatementSyntax::Return(return_statement.clone());

    assert_eq!(statement, StatementSyntax::Return(return_statement));
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

#[test]
fn cudrix_parser_exposes_module_declaration_parsing() {
    let source = SourceText::new("module application;");
    let mut parser = Parser::new(&source);

    let declaration = parser
        .parse_module_declaration()
        .expect("expected module declaration");

    assert_eq!(
        declaration.module_keyword().kind(),
        TokenKind::ModuleKeyword
    );
    assert_eq!(declaration.identifier_token().kind(), TokenKind::Identifier);
    assert_eq!(declaration.semicolon_token().kind(), TokenKind::Semicolon);
    assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
}

#[test]
fn cudrix_parser_exposes_import_declaration_parsing() {
    let source = SourceText::new("import utilities;");
    let mut parser = Parser::new(&source);

    let declaration = parser
        .parse_import_declaration()
        .expect("expected import declaration");

    assert_eq!(
        declaration.import_keyword().kind(),
        TokenKind::ImportKeyword
    );
    assert_eq!(declaration.identifier_token().kind(), TokenKind::Identifier);
    assert_eq!(declaration.semicolon_token().kind(), TokenKind::Semicolon);
    assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
}

#[test]
fn cudrix_parser_exposes_qualified_import_declaration_parsing() {
    let source = SourceText::new("import standard.io;");
    let mut parser = Parser::new(&source);

    let declaration = parser
        .parse_import_declaration()
        .expect("expected qualified import declaration");

    assert_eq!(
        declaration.import_keyword().kind(),
        TokenKind::ImportKeyword
    );
    assert_eq!(declaration.identifier_token().kind(), TokenKind::Identifier);
    assert_eq!(declaration.qualified_name_segments().len(), 1);
    assert_eq!(
        declaration.qualified_name_segments()[0].dot_token().kind(),
        TokenKind::Dot
    );
    assert_eq!(
        declaration.qualified_name_segments()[0]
            .identifier_token()
            .kind(),
        TokenKind::Identifier
    );
    assert_eq!(declaration.semicolon_token().kind(), TokenKind::Semicolon);
    assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
}

#[test]
fn cudrix_parser_exposes_import_alias_declaration_parsing() {
    let source = SourceText::new("import standard.io as io;");
    let mut parser = Parser::new(&source);

    let declaration = parser
        .parse_import_declaration()
        .expect("expected aliased import declaration");

    assert_eq!(
        declaration.import_keyword().kind(),
        TokenKind::ImportKeyword
    );
    assert_eq!(declaration.identifier_token().kind(), TokenKind::Identifier);
    assert_eq!(declaration.qualified_name_segments().len(), 1);

    let alias = declaration.alias().expect("expected import alias");

    assert_eq!(alias.as_keyword().kind(), TokenKind::AsKeyword);
    assert_eq!(alias.identifier_token().kind(), TokenKind::Identifier);
    assert_eq!(declaration.semicolon_token().kind(), TokenKind::Semicolon);
    assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
}

#[test]
fn cudrix_parser_exposes_selective_import_declaration_parsing() {
    let source = SourceText::new("import standard.io.{read, write};");
    let mut parser = Parser::new(&source);

    let declaration = parser
        .parse_import_declaration()
        .expect("expected selective import declaration");

    assert_eq!(
        declaration.import_keyword().kind(),
        TokenKind::ImportKeyword
    );
    assert_eq!(declaration.identifier_token().kind(), TokenKind::Identifier);
    assert_eq!(declaration.qualified_name_segments().len(), 1);
    assert!(declaration.alias().is_none());

    let clause = declaration
        .selective_import_clause()
        .expect("expected selective import clause");

    assert_eq!(clause.dot_token().kind(), TokenKind::Dot);
    assert_eq!(clause.left_brace_token().kind(), TokenKind::LeftBrace);
    assert_eq!(clause.members().len(), 2);

    assert_eq!(
        clause.members()[0].identifier_token().span(),
        SourceSpan::new(20, 4)
    );
    assert_eq!(
        clause.members()[0]
            .comma_token()
            .expect("expected comma")
            .kind(),
        TokenKind::Comma
    );

    assert_eq!(
        clause.members()[1].identifier_token().span(),
        SourceSpan::new(26, 5)
    );
    assert!(clause.members()[1].comma_token().is_none());

    assert_eq!(clause.right_brace_token().kind(), TokenKind::RightBrace);
    assert_eq!(declaration.semicolon_token().kind(), TokenKind::Semicolon);
    assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
}

#[test]
fn cudrix_parser_exposes_function_declaration_parsing() {
    let source = SourceText::new("function add(first, second) { return first + second; }");
    let mut parser = Parser::new(&source);

    let declaration = parser
        .parse_function_declaration()
        .expect("expected function declaration");

    assert_eq!(
        declaration.function_keyword().kind(),
        TokenKind::FunctionKeyword
    );
    assert_eq!(declaration.identifier_token().kind(), TokenKind::Identifier);
    assert_eq!(
        declaration.left_parenthesis_token().kind(),
        TokenKind::LeftParenthesis
    );

    assert_eq!(declaration.parameters().len(), 2);
    assert_eq!(
        declaration.parameters()[0].identifier_token().kind(),
        TokenKind::Identifier
    );
    assert_eq!(
        declaration.parameters()[0]
            .comma_token()
            .expect("expected comma")
            .kind(),
        TokenKind::Comma
    );
    assert_eq!(
        declaration.parameters()[1].identifier_token().kind(),
        TokenKind::Identifier
    );
    assert_eq!(declaration.parameters()[1].comma_token(), None);

    assert_eq!(
        declaration.right_parenthesis_token().kind(),
        TokenKind::RightParenthesis
    );
    assert_eq!(
        declaration.body().left_brace_token().kind(),
        TokenKind::LeftBrace
    );
    assert_eq!(declaration.body().statements().len(), 1);
    assert!(matches!(
        declaration.body().statements()[0],
        StatementSyntax::Return(_)
    ));
    assert_eq!(
        declaration.body().right_brace_token().kind(),
        TokenKind::RightBrace
    );

    assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
}

#[test]
fn cudrix_parser_exposes_compilation_unit_parsing() {
    let source = SourceText::new(
        r#"
        module application;

        import utilities;
        import graphics;

        function first() {
        }

        function second(value) {
            return value;
        }
        "#,
    );
    let mut parser = Parser::new(&source);

    let compilation_unit = parser
        .parse_compilation_unit()
        .expect("expected compilation unit");

    let module_declaration = compilation_unit
        .module_declaration()
        .expect("expected module declaration");

    assert_eq!(
        module_declaration.module_keyword().kind(),
        TokenKind::ModuleKeyword
    );
    assert_eq!(
        module_declaration.identifier_token().kind(),
        TokenKind::Identifier
    );
    assert_eq!(
        module_declaration.semicolon_token().kind(),
        TokenKind::Semicolon
    );

    assert_eq!(compilation_unit.import_declarations().len(), 2);
    assert_eq!(
        compilation_unit.import_declarations()[0]
            .import_keyword()
            .kind(),
        TokenKind::ImportKeyword
    );
    assert_eq!(
        compilation_unit.import_declarations()[0]
            .identifier_token()
            .kind(),
        TokenKind::Identifier
    );
    assert_eq!(
        compilation_unit.import_declarations()[1]
            .identifier_token()
            .kind(),
        TokenKind::Identifier
    );

    assert_eq!(compilation_unit.function_declarations().len(), 2);
    assert_eq!(
        compilation_unit.function_declarations()[0]
            .parameters()
            .len(),
        0
    );
    assert_eq!(
        compilation_unit.function_declarations()[1]
            .parameters()
            .len(),
        1
    );
    assert_eq!(
        compilation_unit.end_of_file_token().kind(),
        TokenKind::EndOfFile
    );
    assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
}

#[test]
fn cudrix_parser_exposes_compilation_unit_with_selective_import() {
    let source =
        SourceText::new("module application; import standard.io.{read, write}; function main() {}");
    let mut parser = Parser::new(&source);

    let compilation_unit = parser
        .parse_compilation_unit()
        .expect("expected compilation unit");

    assert!(compilation_unit.module_declaration().is_some());
    assert_eq!(compilation_unit.import_declarations().len(), 1);

    let declaration = &compilation_unit.import_declarations()[0];

    assert_eq!(declaration.qualified_name_segments().len(), 1);
    assert!(declaration.alias().is_none());

    let clause = declaration
        .selective_import_clause()
        .expect("expected selective import clause");

    assert_eq!(clause.members().len(), 2);
    assert_eq!(
        clause.members()[0].identifier_token().kind(),
        TokenKind::Identifier
    );
    assert_eq!(
        clause.members()[1].identifier_token().kind(),
        TokenKind::Identifier
    );

    assert_eq!(compilation_unit.function_declarations().len(), 1);
    assert_eq!(
        compilation_unit.end_of_file_token().kind(),
        TokenKind::EndOfFile
    );
    assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
}

#[test]
fn cudrix_public_api_resolves_top_level_function_names() {
    let source =
        cudrix::source::SourceText::new("function main() { greet(); }\nfunction greet() {}");

    let compilation_unit = cudrix::compiler::Parser::new(&source)
        .parse_compilation_unit()
        .expect("Compilation unit should parse");

    let result = cudrix::lookup_top_level_function(&compilation_unit, &source, "greet");

    let Ok(cudrix::TopLevelFunctionLookup::Unique(declaration)) = result else {
        panic!("Expected exactly one matching function");
    };

    assert_eq!(
        source.slice(&declaration.identifier_token().span()),
        Some("greet")
    );

    let unknown = cudrix::lookup_top_level_function(&compilation_unit, &source, "grete");

    assert!(matches!(
        unknown,
        Ok(cudrix::TopLevelFunctionLookup::NotFound)
    ));
}

#[test]
fn cudrix_public_api_preserves_multiple_function_matches() {
    let source = cudrix::source::SourceText::new("function greet() {}\nfunction greet() {}");

    let compilation_unit = cudrix::compiler::Parser::new(&source)
        .parse_compilation_unit()
        .expect("Compilation unit should parse");

    let result = cudrix::lookup_top_level_function(&compilation_unit, &source, "greet");

    let Ok(cudrix::TopLevelFunctionLookup::Multiple(matches)) = result else {
        panic!("Expected multiple matching function declarations");
    };

    assert_eq!(matches.len(), 2);

    for (index, declaration) in matches.iter().enumerate() {
        assert!(std::ptr::eq(
            *declaration,
            &compilation_unit.function_declarations()[index]
        ));
    }
}

#[test]
fn cudrix_public_api_looks_up_direct_call_top_level_function() {
    let source =
        cudrix::source::SourceText::new("function main() { greet(); }\nfunction greet() {}");

    let compilation_unit = cudrix::compiler::Parser::new(&source)
        .parse_compilation_unit()
        .expect("Compilation unit should parse");

    let main_function = &compilation_unit.function_declarations()[0];

    let cudrix::syntax::StatementSyntax::Expression(statement) =
        &main_function.body().statements()[0]
    else {
        panic!("Expected an expression statement");
    };

    let cudrix::syntax::ExpressionSyntax::Call(call) = statement.expression() else {
        panic!("Expected a call expression");
    };

    let result = cudrix::lookup_direct_call_top_level_functions(&compilation_unit, &source, call);

    let Ok(Some(cudrix::TopLevelFunctionLookup::Unique(declaration))) = result else {
        panic!("Expected one matching top-level function declaration");
    };

    assert!(std::ptr::eq(
        declaration,
        &compilation_unit.function_declarations()[1]
    ));
}

#[test]
fn cudrix_public_api_distinguishes_unsupported_call_callee() {
    let source =
        cudrix::source::SourceText::new("function main() { object.greet(); }\nfunction greet() {}");

    let compilation_unit = cudrix::compiler::Parser::new(&source)
        .parse_compilation_unit()
        .expect("Compilation unit should parse");

    let main_function = &compilation_unit.function_declarations()[0];

    let cudrix::syntax::StatementSyntax::Expression(statement) =
        &main_function.body().statements()[0]
    else {
        panic!("Expected an expression statement");
    };

    let cudrix::syntax::ExpressionSyntax::Call(call) = statement.expression() else {
        panic!("Expected a call expression");
    };

    let result = cudrix::lookup_direct_call_top_level_functions(&compilation_unit, &source, call);

    assert!(matches!(result, Ok(None)));
}

#[test]
fn cudrix_public_api_discovers_calls_and_looks_up_candidates() {
    let source = cudrix::source::SourceText::new(
        "function main() {\n\
             greet();\n\
             missing();\n\
         }\n\
         function greet() {}",
    );

    let compilation_unit = cudrix::compiler::Parser::new(&source)
        .parse_compilation_unit()
        .expect("Compilation unit should parse");

    let discovered = cudrix::discover_direct_function_calls(&compilation_unit);

    assert_eq!(discovered.len(), 2);

    let names: Vec<&str> = discovered
        .iter()
        .map(|item| {
            let cudrix::syntax::ExpressionSyntax::Name(name) = item.call().callee() else {
                panic!("Expected a direct named call");
            };

            source
                .slice(&name.identifier_token().span())
                .expect("Identifier span should be valid")
        })
        .collect();

    assert_eq!(names, ["greet", "missing"]);

    let functions = compilation_unit.function_declarations();

    for item in &discovered {
        assert!(std::ptr::eq(item.function(), &functions[0]));
    }

    let cudrix::syntax::StatementSyntax::Expression(statement) =
        &functions[0].body().statements()[0]
    else {
        panic!("Expected an expression statement");
    };

    let cudrix::syntax::ExpressionSyntax::Call(original_call) = statement.expression() else {
        panic!("Expected a call expression");
    };

    assert!(std::ptr::eq(discovered[0].call(), original_call));

    let matching = cudrix::lookup_direct_call_top_level_functions(
        &compilation_unit,
        &source,
        discovered[0].call(),
    );

    let Ok(Some(cudrix::TopLevelFunctionLookup::Unique(declaration))) = matching else {
        panic!("Expected one matching top-level function");
    };

    assert!(std::ptr::eq(declaration, &functions[1]));

    let missing = cudrix::lookup_direct_call_top_level_functions(
        &compilation_unit,
        &source,
        discovered[1].call(),
    );

    assert!(matches!(
        missing,
        Ok(Some(cudrix::TopLevelFunctionLookup::NotFound))
    ));
}

#[test]
fn cudrix_public_api_discovers_nested_calls_in_source_order() {
    let source = cudrix::source::SourceText::new(
        r#"
function main() {
    object.run(helper());
    (named)();
    get_handler()();
    entries[index()]();
    outer(inner());
}
"#,
    );

    let compilation_unit = cudrix::compiler::Parser::new(&source)
        .parse_compilation_unit()
        .expect("Compilation unit should parse");

    let discovered = cudrix::discover_direct_function_calls(&compilation_unit);

    let names: Vec<&str> = discovered
        .iter()
        .map(|item| {
            let cudrix::syntax::ExpressionSyntax::Name(name) = item.call().callee() else {
                panic!("Expected a direct named call");
            };

            source
                .slice(&name.identifier_token().span())
                .expect("Identifier span should be valid")
        })
        .collect();

    assert_eq!(names, ["helper", "get_handler", "index", "outer", "inner"]);

    let positions: Vec<usize> = discovered
        .iter()
        .map(|item| {
            let cudrix::syntax::ExpressionSyntax::Name(name) = item.call().callee() else {
                panic!("Expected a direct named call");
            };

            name.identifier_token().span().start()
        })
        .collect();

    assert!(positions.windows(2).all(|pair| pair[0] < pair[1]));
}

#[test]
fn public_api_recognizes_enclosing_parameters_without_changing_function_candidates() {
    let source = cudrix::source::SourceText::new(
        "function greet() {}\n\
         function main(greet, other) { greet(); missing(); }",
    );

    let compilation_unit = cudrix::compiler::Parser::new(&source)
        .parse_compilation_unit()
        .expect("Compilation unit should parse");

    let discovered = cudrix::discover_direct_function_calls(&compilation_unit);
    let functions = compilation_unit.function_declarations();

    assert_eq!(discovered.len(), 2);

    assert!(std::ptr::eq(discovered[0].function(), &functions[1]));
    assert!(std::ptr::eq(discovered[1].function(), &functions[1]));

    let parameter_result =
        cudrix::lookup_direct_call_enclosing_function_parameters(discovered[0], &source);

    let Ok(cudrix::FunctionParameterLookup::Unique(parameter)) = parameter_result else {
        panic!("Expected one matching enclosing function parameter");
    };

    assert!(std::ptr::eq(parameter, &functions[1].parameters()[0]));

    let missing_parameter_result =
        cudrix::lookup_direct_call_enclosing_function_parameters(discovered[1], &source);

    assert!(matches!(
        missing_parameter_result,
        Ok(cudrix::FunctionParameterLookup::NotFound)
    ));

    let function_result = cudrix::lookup_direct_call_top_level_functions(
        &compilation_unit,
        &source,
        discovered[0].call(),
    );

    let Ok(Some(cudrix::TopLevelFunctionLookup::Unique(declaration))) = function_result else {
        panic!("Expected one matching top-level function candidate");
    };

    assert!(std::ptr::eq(declaration, &functions[0]));
}

#[test]
fn public_api_preserves_multiple_enclosing_parameter_matches() {
    let source = cudrix::source::SourceText::new("function main(value, other, value) { value(); }");

    let compilation_unit = cudrix::compiler::Parser::new(&source)
        .parse_compilation_unit()
        .expect("Compilation unit should parse");

    let discovered = cudrix::discover_direct_function_calls(&compilation_unit);

    assert_eq!(discovered.len(), 1);

    let result = cudrix::lookup_direct_call_enclosing_function_parameters(discovered[0], &source);

    let Ok(cudrix::FunctionParameterLookup::Multiple(matches)) = result else {
        panic!("Expected multiple matching parameter declarations");
    };

    assert_eq!(matches.len(), 2);

    let parameters = compilation_unit.function_declarations()[0].parameters();

    assert!(std::ptr::eq(matches[0], &parameters[0]));
    assert!(std::ptr::eq(matches[1], &parameters[2]));
}

#[test]
fn public_api_reports_invalid_parameter_lookup_identifier_span() {
    let source = cudrix::source::SourceText::new("function main(value) { value(); }");

    let compilation_unit = cudrix::compiler::Parser::new(&source)
        .parse_compilation_unit()
        .expect("Compilation unit should parse");

    let discovered = cudrix::discover_direct_function_calls(&compilation_unit);

    assert_eq!(discovered.len(), 1);

    let cudrix::syntax::ExpressionSyntax::Name(name) = discovered[0].call().callee() else {
        panic!("Expected a direct named call");
    };

    let expected_span = name.identifier_token().span();

    let unrelated_source = cudrix::source::SourceText::new("");

    let result =
        cudrix::lookup_direct_call_enclosing_function_parameters(discovered[0], &unrelated_source);

    assert!(matches!(
        result,
        Err(cudrix::ParameterLookupError::InvalidIdentifierSpan(span))
            if span == expected_span
    ));
}

#[test]
fn public_api_looks_up_general_identifier_reference_enclosing_blocks() {
    let source = cudrix::source::SourceText::new(
        r#"
function first() {
    let source = initial;

    if ready {
        target = source + delta;
        consume(target);
    }
}

function second() {
    return target;
}
"#,
    );

    let compilation_unit = cudrix::compiler::Parser::new(&source)
        .parse_compilation_unit()
        .expect("Compilation unit should parse");

    // Verify the public discovery result type.
    let discovered: Vec<cudrix::DiscoveredIdentifierReference<'_>> =
        cudrix::discover_identifier_references(&compilation_unit);

    assert_eq!(discovered.len(), 8);

    let functions = compilation_unit.function_declarations();

    assert_eq!(functions.len(), 2);

    let first_function = &functions[0];
    let second_function = &functions[1];

    let first_body = first_function.body();
    let second_body = second_function.body();

    let cudrix::syntax::StatementSyntax::Conditional(conditional) = &first_body.statements()[1]
    else {
        panic!("Expected conditional statement");
    };

    let if_block = conditional.then_block();

    // Verify identifier discovery order across both functions.
    let names: Vec<&str> = discovered
        .iter()
        .map(|item| {
            source
                .slice(&item.reference().identifier_span())
                .expect("Identifier span should be valid")
        })
        .collect();

    assert_eq!(
        names,
        [
            "initial", "ready", "target", "source", "delta", "consume", "target", "target",
        ]
    );

    // Define the expected lexical ancestry using original AST blocks.
    let expected_paths = [
        vec![first_body],
        vec![first_body],
        vec![first_body, if_block],
        vec![first_body, if_block],
        vec![first_body, if_block],
        vec![first_body, if_block],
        vec![first_body, if_block],
        vec![second_body],
    ];

    for (index, (item, expected_path)) in discovered.iter().zip(expected_paths.iter()).enumerate() {
        let expected_function = if index < 7 {
            first_function
        } else {
            second_function
        };

        assert!(std::ptr::eq(item.function(), expected_function));

        // Explicitly verify the public return type.
        let actual_path: Vec<&cudrix::syntax::BlockSyntax> =
            cudrix::lookup_identifier_reference_enclosing_blocks(*item);

        assert_eq!(
            actual_path.len(),
            expected_path.len(),
            "Unexpected ancestry depth for reference {index}"
        );

        for (actual_block, expected_block) in actual_path.iter().zip(expected_path.iter()) {
            assert!(
                std::ptr::eq(*actual_block, *expected_block),
                "Incorrect block identity for reference {index}"
            );
        }
    }

    // Verify original assignment target identity.
    let cudrix::syntax::StatementSyntax::Expression(assignment_statement) =
        &if_block.statements()[0]
    else {
        panic!("Expected assignment statement");
    };

    let cudrix::syntax::ExpressionSyntax::Assignment(assignment) =
        assignment_statement.expression()
    else {
        panic!("Expected assignment expression");
    };

    let cudrix::IdentifierReference::AssignmentTarget(assignment_target) =
        discovered[2].reference()
    else {
        panic!("Expected assignment target reference");
    };

    assert!(std::ptr::eq(assignment, assignment_target));

    // Verify original ordinary name identity in the second function.
    let cudrix::syntax::StatementSyntax::Return(return_statement) = &second_body.statements()[0]
    else {
        panic!("Expected return statement");
    };

    let cudrix::syntax::ExpressionSyntax::Name(return_name) = return_statement.expression() else {
        panic!("Expected return name expression");
    };

    let cudrix::IdentifierReference::Name(discovered_return_name) = discovered[7].reference()
    else {
        panic!("Expected discovered return name");
    };

    assert!(std::ptr::eq(return_name, discovered_return_name));

    // Verify compatibility with existing direct-call ancestry.
    let direct_calls = cudrix::discover_direct_function_calls(&compilation_unit);

    assert_eq!(direct_calls.len(), 1);

    let direct_call = direct_calls[0];

    assert!(std::ptr::eq(direct_call.function(), first_function));

    let cudrix::syntax::ExpressionSyntax::Name(call_callee) = direct_call.call().callee() else {
        panic!("Expected named direct-call callee");
    };

    let cudrix::IdentifierReference::Name(discovered_callee) = discovered[5].reference() else {
        panic!("Expected discovered call callee reference");
    };

    assert!(std::ptr::eq(call_callee, discovered_callee));

    let direct_call_blocks = cudrix::lookup_direct_call_enclosing_blocks(direct_call);

    let identifier_blocks = cudrix::lookup_identifier_reference_enclosing_blocks(discovered[5]);

    assert_eq!(direct_call_blocks.len(), identifier_blocks.len());

    for (direct_block, identifier_block) in direct_call_blocks.iter().zip(identifier_blocks.iter())
    {
        assert!(std::ptr::eq(*direct_block, *identifier_block));
    }

    // Repeated public lookups must preserve exact block identities.
    for item in &discovered {
        let first = cudrix::lookup_identifier_reference_enclosing_blocks(*item);
        let second = cudrix::lookup_identifier_reference_enclosing_blocks(*item);

        assert_eq!(first.len(), second.len());

        for (first_block, second_block) in first.iter().zip(second.iter()) {
            assert!(std::ptr::eq(*first_block, *second_block));
        }
    }
}

#[test]
fn public_api_discovers_general_identifier_references() {
    let source = cudrix::source::SourceText::new(
        r#"
function first() {
    let raw = input;
    consume(raw);
}

function second() {
    var total = 0;
    total = total + delta;
}
"#,
    );

    let compilation_unit = cudrix::compiler::Parser::new(&source)
        .parse_compilation_unit()
        .expect("Compilation unit should parse");

    // Explicitly verify that the discovery result is publicly accessible.
    let discovered: Vec<cudrix::DiscoveredIdentifierReference<'_>> =
        cudrix::discover_identifier_references(&compilation_unit);

    assert_eq!(discovered.len(), 6);

    // Verify that public identifier spans expose the correct source names.
    let names: Vec<&str> = discovered
        .iter()
        .map(|item| {
            source
                .slice(&item.reference().identifier_span())
                .expect("Identifier span should be valid")
        })
        .collect();

    assert_eq!(
        names,
        ["input", "consume", "raw", "total", "total", "delta"]
    );

    // Verify that both public reference variants are available.
    let assignment_targets: Vec<bool> = discovered
        .iter()
        .map(|item| {
            matches!(
                item.reference(),
                cudrix::IdentifierReference::AssignmentTarget(_)
            )
        })
        .collect();

    assert_eq!(
        assignment_targets,
        [false, false, false, true, false, false]
    );

    // Verify that each reference retains its enclosing function.
    let functions = compilation_unit.function_declarations();

    assert_eq!(functions.len(), 2);

    for item in &discovered[..3] {
        assert!(std::ptr::eq(item.function(), &functions[0]));
    }

    for item in &discovered[3..] {
        assert!(std::ptr::eq(item.function(), &functions[1]));
    }

    // Verify original AST identity for an ordinary name expression.
    let cudrix::syntax::StatementSyntax::VariableDeclaration(first_declaration) =
        &functions[0].body().statements()[0]
    else {
        panic!("Expected variable declaration");
    };

    let cudrix::syntax::ExpressionSyntax::Name(initializer_name) = first_declaration.initializer()
    else {
        panic!("Expected name expression initializer");
    };

    let cudrix::IdentifierReference::Name(discovered_name) = discovered[0].reference() else {
        panic!("Expected ordinary name reference");
    };

    assert!(std::ptr::eq(discovered_name, initializer_name));

    // Verify original AST identity for an assignment target.
    let cudrix::syntax::StatementSyntax::Expression(second_statement) =
        &functions[1].body().statements()[1]
    else {
        panic!("Expected expression statement");
    };

    let cudrix::syntax::ExpressionSyntax::Assignment(assignment) = second_statement.expression()
    else {
        panic!("Expected assignment expression");
    };

    let cudrix::IdentifierReference::AssignmentTarget(discovered_target) =
        discovered[3].reference()
    else {
        panic!("Expected assignment target reference");
    };

    assert!(std::ptr::eq(discovered_target, assignment));

    // Verify deterministic UTF-8 source ordering.
    let offsets: Vec<usize> = discovered
        .iter()
        .map(|item| item.reference().identifier_span().start())
        .collect();

    assert!(offsets.windows(2).all(|pair| pair[0] < pair[1]));

    // Existing public direct-call discovery remains available.
    let direct_calls = cudrix::discover_direct_function_calls(&compilation_unit);

    assert_eq!(direct_calls.len(), 1);
}

#[test]
fn public_api_exposes_scope_aware_local_variable_candidates() {
    let source = cudrix::source::SourceText::new(
        "function main() {
            let outer = 1;

            if true {
                var branch = 2;

                outer();
                branch();
            } else {
                branch();
            }
        }",
    );

    let compilation_unit = cudrix::compiler::Parser::new(&source)
        .parse_compilation_unit()
        .expect("Compilation unit should parse");

    let discovered = cudrix::discover_direct_function_calls(&compilation_unit);

    assert_eq!(discovered.len(), 3);

    let function_body = compilation_unit.function_declarations()[0].body();

    let statements = function_body.statements();

    let cudrix::syntax::StatementSyntax::VariableDeclaration(outer_declaration) = &statements[0]
    else {
        panic!("Expected outer declaration");
    };

    let cudrix::syntax::StatementSyntax::Conditional(conditional) = &statements[1] else {
        panic!("Expected conditional statement");
    };

    let if_block = conditional.then_block();

    let else_block = conditional
        .else_clause()
        .expect("Expected else clause")
        .block();

    let cudrix::syntax::StatementSyntax::VariableDeclaration(branch_declaration) =
        &if_block.statements()[0]
    else {
        panic!("Expected branch declaration");
    };

    // A nested call recognizes a declaration in its enclosing function body.
    // The explicit type annotation also verifies the public candidate export.
    let outer_candidates: Vec<cudrix::EnclosingBlockLocalVariableCandidate<'_>> =
        cudrix::lookup_direct_call_enclosing_block_local_variable_candidates(
            discovered[0],
            &source,
        )
        .expect("Outer candidate lookup should succeed");

    assert_eq!(outer_candidates.len(), 1);

    assert!(std::ptr::eq(
        outer_candidates[0].declaration(),
        outer_declaration
    ));

    assert!(std::ptr::eq(outer_candidates[0].block(), function_body));

    // A call inside the if block recognizes its directly contained declaration.
    let branch_candidates = cudrix::lookup_direct_call_enclosing_block_local_variable_candidates(
        discovered[1],
        &source,
    )
    .expect("Branch candidate lookup should succeed");

    assert_eq!(branch_candidates.len(), 1);

    assert!(std::ptr::eq(
        branch_candidates[0].declaration(),
        branch_declaration
    ));

    assert!(std::ptr::eq(branch_candidates[0].block(), if_block));

    // The sibling else block does not contain or enclose the declaration.
    let else_blocks = cudrix::lookup_direct_call_enclosing_blocks(discovered[2]);

    assert_eq!(else_blocks.len(), 2);
    assert!(std::ptr::eq(else_blocks[0], function_body));
    assert!(std::ptr::eq(else_blocks[1], else_block));

    let else_candidates = cudrix::lookup_direct_call_enclosing_block_local_variable_candidates(
        discovered[2],
        &source,
    )
    .expect("Else candidate lookup should succeed");

    assert!(else_candidates.is_empty());

    // The existing function-wide operation must retain its original behavior.
    let function_wide_result =
        cudrix::lookup_direct_call_enclosing_function_local_variables(discovered[2], &source)
            .expect("Function-wide lookup should succeed");

    let cudrix::LocalVariableLookup::Unique(matched_declaration) = function_wide_result else {
        panic!("Expected function-wide matching declaration");
    };

    assert!(std::ptr::eq(matched_declaration, branch_declaration));
}

#[test]
fn public_api_exposes_lexical_blocks_without_changing_declaration_recognition() {
    let source = cudrix::source::SourceText::new(
        "function main(value) {
            value();

            let value = 10;

            if true {
                value();
            } else {
                value();
            }
        }",
    );

    let compilation_unit = cudrix::compiler::Parser::new(&source)
        .parse_compilation_unit()
        .expect("Compilation unit should parse");

    let discovered = cudrix::discover_direct_function_calls(&compilation_unit);

    assert_eq!(discovered.len(), 3);

    let function = &compilation_unit.function_declarations()[0];
    let function_body = function.body();

    let statements = function_body.statements();

    let cudrix::syntax::StatementSyntax::VariableDeclaration(local_declaration) = &statements[1]
    else {
        panic!("Expected local variable declaration");
    };

    let cudrix::syntax::StatementSyntax::Conditional(conditional) = &statements[2] else {
        panic!("Expected conditional statement");
    };

    let if_block = conditional.then_block();

    let else_block = conditional
        .else_clause()
        .expect("Expected else clause")
        .block();

    // The first call belongs directly to the function body.
    let outer_blocks = cudrix::lookup_direct_call_enclosing_blocks(discovered[0]);

    assert_eq!(outer_blocks.len(), 1);
    assert!(std::ptr::eq(outer_blocks[0], function_body));

    // The second call belongs to the if block.
    let if_blocks = cudrix::lookup_direct_call_enclosing_blocks(discovered[1]);

    assert_eq!(if_blocks.len(), 2);
    assert!(std::ptr::eq(if_blocks[0], function_body));
    assert!(std::ptr::eq(if_blocks[1], if_block));

    // The third call belongs to the sibling else block.
    let else_blocks = cudrix::lookup_direct_call_enclosing_blocks(discovered[2]);

    assert_eq!(else_blocks.len(), 2);
    assert!(std::ptr::eq(else_blocks[0], function_body));
    assert!(std::ptr::eq(else_blocks[1], else_block));

    assert!(!std::ptr::eq(if_block, else_block));

    // Existing parameter recognition remains independent
    // of the new structural block lookup.
    for discovered_call in &discovered {
        let parameter_result =
            cudrix::lookup_direct_call_enclosing_function_parameters(*discovered_call, &source);

        let Ok(cudrix::FunctionParameterLookup::Unique(parameter)) = parameter_result else {
            panic!("Expected matching function parameter");
        };

        assert!(std::ptr::eq(parameter, &function.parameters()[0]));
    }

    // Existing function-wide local declaration recognition must
    // continue finding the declaration, even for the earlier call.
    for discovered_call in &discovered {
        let local_result = cudrix::lookup_direct_call_enclosing_function_local_variables(
            *discovered_call,
            &source,
        );

        let Ok(cudrix::LocalVariableLookup::Unique(declaration)) = local_result else {
            panic!("Expected matching function-wide local declaration");
        };

        assert!(std::ptr::eq(declaration, local_declaration));
    }
}

#[test]
fn public_api_recognizes_local_variables_without_resolving_bindings() {
    let source = cudrix::source::SourceText::new(
        "function target() {}

         function main(target) {
             target();

             let target = 1;

             if true {
                 var target = 2;
             }
         }",
    );

    let compilation_unit = cudrix::compiler::Parser::new(&source)
        .parse_compilation_unit()
        .expect("Compilation unit should parse");

    let discovered = cudrix::discover_direct_function_calls(&compilation_unit);
    let functions = compilation_unit.function_declarations();

    assert_eq!(discovered.len(), 1);
    assert!(std::ptr::eq(discovered[0].function(), &functions[1]));

    let local_result =
        cudrix::lookup_direct_call_enclosing_function_local_variables(discovered[0], &source);

    let Ok(cudrix::LocalVariableLookup::Multiple(matches)) = local_result else {
        panic!("Expected two matching local declarations");
    };

    assert_eq!(matches.len(), 2);

    let statements = functions[1].body().statements();

    let cudrix::syntax::StatementSyntax::VariableDeclaration(first_local) = &statements[1] else {
        panic!("Expected first local declaration");
    };

    let cudrix::syntax::StatementSyntax::Conditional(conditional) = &statements[2] else {
        panic!("Expected conditional statement");
    };

    let cudrix::syntax::StatementSyntax::VariableDeclaration(nested_local) =
        &conditional.then_block().statements()[0]
    else {
        panic!("Expected nested local declaration");
    };

    assert!(std::ptr::eq(matches[0], first_local));
    assert!(std::ptr::eq(matches[1], nested_local));

    assert_eq!(
        source.slice(&matches[0].declaration_keyword().span()),
        Some("let")
    );

    assert_eq!(
        source.slice(&matches[1].declaration_keyword().span()),
        Some("var")
    );

    let parameter_result =
        cudrix::lookup_direct_call_enclosing_function_parameters(discovered[0], &source);

    let Ok(cudrix::FunctionParameterLookup::Unique(parameter)) = parameter_result else {
        panic!("Expected matching function parameter");
    };

    assert!(std::ptr::eq(parameter, &functions[1].parameters()[0]));

    let function_result = cudrix::lookup_direct_call_top_level_functions(
        &compilation_unit,
        &source,
        discovered[0].call(),
    );

    let Ok(Some(cudrix::TopLevelFunctionLookup::Unique(function_candidate))) = function_result
    else {
        panic!("Expected matching top-level function");
    };

    assert!(std::ptr::eq(function_candidate, &functions[0]));
}

#[test]
fn public_api_local_variable_lookup_preserves_function_isolation() {
    let source = cudrix::source::SourceText::new(
        "function first() {
            let value = 1;
            value();
        }

        function second() {
            value();
        }",
    );

    let compilation_unit = cudrix::compiler::Parser::new(&source)
        .parse_compilation_unit()
        .expect("Compilation unit should parse");

    let discovered = cudrix::discover_direct_function_calls(&compilation_unit);
    let functions = compilation_unit.function_declarations();

    assert_eq!(discovered.len(), 2);

    assert!(std::ptr::eq(discovered[0].function(), &functions[0]));
    assert!(std::ptr::eq(discovered[1].function(), &functions[1]));

    let first_result =
        cudrix::lookup_direct_call_enclosing_function_local_variables(discovered[0], &source);

    let Ok(cudrix::LocalVariableLookup::Unique(declaration)) = first_result else {
        panic!("Expected one matching local declaration");
    };

    let cudrix::syntax::StatementSyntax::VariableDeclaration(expected) =
        &functions[0].body().statements()[0]
    else {
        panic!("Expected local variable declaration");
    };

    assert!(std::ptr::eq(declaration, expected));

    let second_result =
        cudrix::lookup_direct_call_enclosing_function_local_variables(discovered[1], &source);

    assert!(matches!(
        second_result,
        Ok(cudrix::LocalVariableLookup::NotFound)
    ));
}

#[test]
fn public_api_reports_invalid_local_variable_lookup_identifier_spans() {
    let source = cudrix::source::SourceText::new("function main() { value(); let other = 1; }");

    let compilation_unit = cudrix::compiler::Parser::new(&source)
        .parse_compilation_unit()
        .expect("Compilation unit should parse");

    let discovered = cudrix::discover_direct_function_calls(&compilation_unit);

    assert_eq!(discovered.len(), 1);

    let cudrix::syntax::ExpressionSyntax::Name(call_name) = discovered[0].call().callee() else {
        panic!("Expected direct named call");
    };

    let call_span = call_name.identifier_token().span();

    let statements = compilation_unit.function_declarations()[0]
        .body()
        .statements();

    let cudrix::syntax::StatementSyntax::VariableDeclaration(declaration) = &statements[1] else {
        panic!("Expected local variable declaration");
    };

    let declaration_span = declaration.identifier_token().span();

    let empty_source = cudrix::source::SourceText::new("");

    let call_result =
        cudrix::lookup_direct_call_enclosing_function_local_variables(discovered[0], &empty_source);

    assert!(matches!(
        call_result,
        Err(cudrix::LocalVariableLookupError::InvalidIdentifierSpan(span))
            if span == call_span
    ));

    let truncated_source = cudrix::source::SourceText::new("function main() { value(); }");

    assert_eq!(truncated_source.slice(&call_span), Some("value"));

    assert!(truncated_source.slice(&declaration_span).is_none());

    let declaration_result = cudrix::lookup_direct_call_enclosing_function_local_variables(
        discovered[0],
        &truncated_source,
    );

    assert!(matches!(
        declaration_result,
        Err(cudrix::LocalVariableLookupError::InvalidIdentifierSpan(span))
            if span == declaration_span
    ));
}
