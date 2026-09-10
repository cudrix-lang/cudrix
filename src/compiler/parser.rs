//! Parser infrastructure for Cudrix source code.

use crate::compiler::lexer::{Lexer, Token, TokenKind};
use crate::source::SourceText;
use crate::syntax::{
    ArrayLiteralExpressionSyntax, AssignmentExpressionSyntax, BinaryExpressionSyntax, BlockSyntax,
    CallExpressionSyntax, ConditionalExpressionSyntax, ConditionalStatementSyntax,
    ElseClauseSyntax, ElseIfClauseSyntax, ExpressionStatementSyntax, ExpressionSyntax,
    ForStatementSyntax, FunctionDeclarationSyntax, FunctionParameterSyntax, IndexExpressionSyntax,
    LiteralExpressionSyntax, MemberAccessExpressionSyntax, NameExpressionSyntax,
    ObjectLiteralExpressionSyntax, ObjectMemberSyntax, ParenthesizedExpressionSyntax,
    ReturnStatementSyntax, StatementSyntax, UnaryExpressionSyntax,
    VariableDeclarationStatementSyntax, WhileStatementSyntax,
};

pub struct Parser<'source> {
    lexer: Lexer<'source>,
    current_token: Token,
}

impl<'source> Parser<'source> {
    pub fn new(source: &'source SourceText) -> Self {
        let mut lexer = Lexer::new(source);
        let current_token = Self::next_significant_token_from(&mut lexer);

        Self {
            lexer,
            current_token,
        }
    }

    pub fn current_token(&self) -> Token {
        self.current_token
    }

    pub fn parse_function_declaration(&mut self) -> Option<FunctionDeclarationSyntax> {
        if self.current_token.kind() != TokenKind::FunctionKeyword {
            return None;
        }

        let function_keyword = self.current_token;
        self.advance();

        if self.current_token.kind() != TokenKind::Identifier {
            return None;
        }

        let identifier_token = self.current_token;
        self.advance();

        if self.current_token.kind() != TokenKind::LeftParenthesis {
            return None;
        }

        let left_parenthesis_token = self.current_token;
        self.advance();

        let mut parameters = Vec::new();

        if self.current_token.kind() != TokenKind::RightParenthesis {
            loop {
                if self.current_token.kind() != TokenKind::Identifier {
                    return None;
                }

                let identifier_token = self.current_token;
                self.advance();

                let comma_token = if self.current_token.kind() == TokenKind::Comma {
                    let comma_token = self.current_token;
                    self.advance();
                    Some(comma_token)
                } else {
                    None
                };

                parameters.push(FunctionParameterSyntax::new(identifier_token, comma_token));

                if comma_token.is_none() {
                    break;
                }
            }
        }

        if self.current_token.kind() != TokenKind::RightParenthesis {
            return None;
        }

        let right_parenthesis_token = self.current_token;
        self.advance();

        let body = self.parse_block()?;

        Some(FunctionDeclarationSyntax::new(
            function_keyword,
            identifier_token,
            left_parenthesis_token,
            parameters,
            right_parenthesis_token,
            body,
        ))
    }

    pub fn parse_block(&mut self) -> Option<BlockSyntax> {
        if self.current_token.kind() != TokenKind::LeftBrace {
            return None;
        }

        let left_brace_token = self.current_token;
        self.advance();

        let mut statements = Vec::new();

        while self.current_token.kind() != TokenKind::RightBrace {
            let statement = self.parse_statement()?;
            statements.push(statement);
        }

        let right_brace_token = self.current_token;
        self.advance();

        Some(BlockSyntax::new(
            left_brace_token,
            statements,
            right_brace_token,
        ))
    }

    pub fn parse_statement(&mut self) -> Option<StatementSyntax> {
        match self.current_token.kind() {
            TokenKind::LetKeyword | TokenKind::VarKeyword => {
                self.parse_variable_declaration_statement()
            }
            TokenKind::ReturnKeyword => self.parse_return_statement(),
            TokenKind::IfKeyword => self.parse_conditional_statement(),
            TokenKind::WhileKeyword => self.parse_while_statement(),
            TokenKind::ForKeyword => self.parse_for_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_variable_declaration_statement(&mut self) -> Option<StatementSyntax> {
        let declaration_keyword = self.current_token;

        if !matches!(
            declaration_keyword.kind(),
            TokenKind::LetKeyword | TokenKind::VarKeyword
        ) {
            return None;
        }

        self.advance();

        if self.current_token.kind() != TokenKind::Identifier {
            return None;
        }

        let identifier_token = self.current_token;
        self.advance();

        if self.current_token.kind() != TokenKind::Assignment {
            return None;
        }

        let assignment_token = self.current_token;
        self.advance();

        let initializer = self.parse_expression()?;

        if self.current_token.kind() != TokenKind::Semicolon {
            return None;
        }

        let semicolon_token = self.current_token;
        self.advance();

        Some(StatementSyntax::VariableDeclaration(
            VariableDeclarationStatementSyntax::new(
                declaration_keyword,
                identifier_token,
                assignment_token,
                initializer,
                semicolon_token,
            ),
        ))
    }

    fn parse_return_statement(&mut self) -> Option<StatementSyntax> {
        if self.current_token.kind() != TokenKind::ReturnKeyword {
            return None;
        }

        let return_keyword = self.current_token;
        self.advance();

        let expression = self.parse_expression()?;

        if self.current_token.kind() != TokenKind::Semicolon {
            return None;
        }

        let semicolon_token = self.current_token;
        self.advance();

        Some(StatementSyntax::Return(ReturnStatementSyntax::new(
            return_keyword,
            expression,
            semicolon_token,
        )))
    }

    fn parse_conditional_statement(&mut self) -> Option<StatementSyntax> {
        if self.current_token.kind() != TokenKind::IfKeyword {
            return None;
        }

        let if_keyword = self.current_token;
        self.advance();

        let condition = self.parse_expression()?;
        let then_block = self.parse_block()?;

        let mut elseif_clauses = Vec::new();

        while self.current_token.kind() == TokenKind::ElseIfKeyword {
            let elseif_keyword = self.current_token;
            self.advance();

            let condition = self.parse_expression()?;
            let block = self.parse_block()?;

            elseif_clauses.push(ElseIfClauseSyntax::new(elseif_keyword, condition, block));
        }

        let else_clause = if self.current_token.kind() == TokenKind::ElseKeyword {
            let else_keyword = self.current_token;
            self.advance();

            let block = self.parse_block()?;

            Some(ElseClauseSyntax::new(else_keyword, block))
        } else {
            None
        };

        Some(StatementSyntax::Conditional(
            ConditionalStatementSyntax::new(
                if_keyword,
                condition,
                then_block,
                elseif_clauses,
                else_clause,
            ),
        ))
    }

    fn parse_while_statement(&mut self) -> Option<StatementSyntax> {
        if self.current_token.kind() != TokenKind::WhileKeyword {
            return None;
        }

        let while_keyword = self.current_token;
        self.advance();

        let condition = self.parse_expression()?;
        let block = self.parse_block()?;

        Some(StatementSyntax::While(WhileStatementSyntax::new(
            while_keyword,
            condition,
            block,
        )))
    }

    fn parse_for_statement(&mut self) -> Option<StatementSyntax> {
        if self.current_token.kind() != TokenKind::ForKeyword {
            return None;
        }

        let for_keyword = self.current_token;
        self.advance();

        let initializer = self.parse_expression()?;

        if self.current_token.kind() != TokenKind::Semicolon {
            return None;
        }

        let first_semicolon_token = self.current_token;
        self.advance();

        let condition = self.parse_expression()?;

        if self.current_token.kind() != TokenKind::Semicolon {
            return None;
        }

        let second_semicolon_token = self.current_token;
        self.advance();

        let update = self.parse_expression()?;
        let block = self.parse_block()?;

        Some(StatementSyntax::For(ForStatementSyntax::new(
            for_keyword,
            initializer,
            first_semicolon_token,
            condition,
            second_semicolon_token,
            update,
            block,
        )))
    }

    fn parse_expression_statement(&mut self) -> Option<StatementSyntax> {
        let expression = self.parse_expression()?;

        if self.current_token.kind() != TokenKind::Semicolon {
            return None;
        }

        let semicolon_token = self.current_token;
        self.advance();

        Some(StatementSyntax::Expression(ExpressionStatementSyntax::new(
            expression,
            semicolon_token,
        )))
    }

    pub fn parse_expression(&mut self) -> Option<ExpressionSyntax> {
        self.parse_assignment_expression()
    }

    pub fn parse_assignment_expression(&mut self) -> Option<ExpressionSyntax> {
        let left = self.parse_conditional_expression()?;

        if self.current_token.kind() != TokenKind::Assignment {
            return Some(left);
        }

        let ExpressionSyntax::Name(name_expression) = left else {
            return Some(left);
        };

        let assignment_token = self.current_token;
        self.advance();

        let expression = self.parse_assignment_expression()?;

        Some(ExpressionSyntax::Assignment(
            AssignmentExpressionSyntax::new(
                name_expression.identifier_token(),
                assignment_token,
                expression,
            ),
        ))
    }

    fn parse_conditional_expression(&mut self) -> Option<ExpressionSyntax> {
        let condition = self.parse_binary_expression(0)?;

        if self.current_token.kind() != TokenKind::QuestionMark {
            return Some(condition);
        }

        let question_token = self.current_token;
        self.advance();

        let when_true = self.parse_expression()?;

        if self.current_token.kind() != TokenKind::Colon {
            return None;
        }

        let colon_token = self.current_token;
        self.advance();

        let when_false = self.parse_conditional_expression()?;

        Some(ExpressionSyntax::Conditional(
            ConditionalExpressionSyntax::new(
                condition,
                question_token,
                when_true,
                colon_token,
                when_false,
            ),
        ))
    }

    fn parse_binary_expression(&mut self, parent_precedence: u8) -> Option<ExpressionSyntax> {
        let mut left = self.parse_unary_or_primary_expression()?;

        loop {
            let precedence = Self::binary_operator_precedence(self.current_token.kind());

            if precedence == 0 || precedence <= parent_precedence {
                break;
            }

            let operator_token = self.current_token;
            self.advance();

            let right = self.parse_binary_expression(precedence)?;

            left =
                ExpressionSyntax::Binary(BinaryExpressionSyntax::new(left, operator_token, right));
        }

        Some(left)
    }

    fn parse_unary_or_primary_expression(&mut self) -> Option<ExpressionSyntax> {
        match self.current_token.kind() {
            TokenKind::Plus | TokenKind::Minus | TokenKind::LogicalNot => {
                self.parse_unary_expression().map(ExpressionSyntax::Unary)
            }

            _ => self.parse_postfix_expression(),
        }
    }

    fn parse_postfix_expression(&mut self) -> Option<ExpressionSyntax> {
        let mut expression = self.parse_primary_expression()?;

        loop {
            match self.current_token.kind() {
                TokenKind::LeftParenthesis => {
                    let left_parenthesis_token = self.current_token;
                    self.advance();

                    let mut arguments = Vec::new();

                    if self.current_token.kind() != TokenKind::RightParenthesis {
                        loop {
                            arguments.push(self.parse_expression()?);

                            if self.current_token.kind() != TokenKind::Comma {
                                break;
                            }

                            self.advance();
                        }
                    }

                    if self.current_token.kind() != TokenKind::RightParenthesis {
                        return None;
                    }

                    let right_parenthesis_token = self.current_token;
                    self.advance();

                    expression = ExpressionSyntax::Call(CallExpressionSyntax::new(
                        expression,
                        left_parenthesis_token,
                        arguments,
                        right_parenthesis_token,
                    ));
                }

                TokenKind::Dot => {
                    let dot_token = self.current_token;
                    self.advance();

                    if self.current_token.kind() != TokenKind::Identifier {
                        return None;
                    }

                    let member_name_token = self.current_token;
                    self.advance();

                    expression = ExpressionSyntax::MemberAccess(MemberAccessExpressionSyntax::new(
                        expression,
                        dot_token,
                        member_name_token,
                    ));
                }

                TokenKind::LeftBracket => {
                    let left_bracket_token = self.current_token;
                    self.advance();

                    let index = self.parse_expression()?;

                    if self.current_token.kind() != TokenKind::RightBracket {
                        return None;
                    }

                    let right_bracket_token = self.current_token;
                    self.advance();

                    expression = ExpressionSyntax::Index(IndexExpressionSyntax::new(
                        expression,
                        left_bracket_token,
                        index,
                        right_bracket_token,
                    ));
                }

                _ => break,
            }
        }

        Some(expression)
    }

    fn parse_primary_expression(&mut self) -> Option<ExpressionSyntax> {
        match self.current_token.kind() {
            TokenKind::Number
            | TokenKind::String
            | TokenKind::TrueKeyword
            | TokenKind::FalseKeyword => self
                .parse_literal_expression()
                .map(ExpressionSyntax::Literal),

            TokenKind::Identifier => self.parse_name_expression().map(ExpressionSyntax::Name),

            TokenKind::LeftParenthesis => self
                .parse_parenthesized_expression()
                .map(ExpressionSyntax::Parenthesized),

            TokenKind::LeftBracket => self
                .parse_array_literal_expression()
                .map(ExpressionSyntax::ArrayLiteral),

            TokenKind::LeftBrace => self
                .parse_object_literal_expression()
                .map(ExpressionSyntax::ObjectLiteral),

            _ => None,
        }
    }

    fn parse_array_literal_expression(&mut self) -> Option<ArrayLiteralExpressionSyntax> {
        if self.current_token.kind() != TokenKind::LeftBracket {
            return None;
        }

        let left_bracket_token = self.current_token;
        self.advance();

        let mut elements = Vec::new();

        if self.current_token.kind() != TokenKind::RightBracket {
            loop {
                elements.push(self.parse_expression()?);

                if self.current_token.kind() != TokenKind::Comma {
                    break;
                }

                self.advance();
            }
        }

        if self.current_token.kind() != TokenKind::RightBracket {
            return None;
        }

        let right_bracket_token = self.current_token;
        self.advance();

        Some(ArrayLiteralExpressionSyntax::new(
            left_bracket_token,
            elements,
            right_bracket_token,
        ))
    }

    fn parse_object_literal_expression(&mut self) -> Option<ObjectLiteralExpressionSyntax> {
        if self.current_token.kind() != TokenKind::LeftBrace {
            return None;
        }

        let left_brace_token = self.current_token;
        self.advance();

        let mut members = Vec::new();

        if self.current_token.kind() != TokenKind::RightBrace {
            loop {
                if self.current_token.kind() != TokenKind::Identifier {
                    return None;
                }

                let name_token = self.current_token;
                self.advance();

                if self.current_token.kind() != TokenKind::Colon {
                    return None;
                }

                let colon_token = self.current_token;
                self.advance();

                let value = self.parse_expression()?;

                members.push(ObjectMemberSyntax::new(name_token, colon_token, value));

                if self.current_token.kind() != TokenKind::Comma {
                    break;
                }

                self.advance();
            }
        }

        if self.current_token.kind() != TokenKind::RightBrace {
            return None;
        }

        let right_brace_token = self.current_token;
        self.advance();

        Some(ObjectLiteralExpressionSyntax::new(
            left_brace_token,
            members,
            right_brace_token,
        ))
    }

    fn binary_operator_precedence(kind: TokenKind) -> u8 {
        match kind {
            TokenKind::LogicalOr => 1,
            TokenKind::LogicalAnd => 2,

            TokenKind::Equal | TokenKind::NotEqual => 3,

            TokenKind::LessThan
            | TokenKind::LessThanOrEqual
            | TokenKind::GreaterThan
            | TokenKind::GreaterThanOrEqual => 4,

            TokenKind::Plus | TokenKind::Minus => 5,

            TokenKind::Multiply | TokenKind::Divide | TokenKind::Remainder => 6,

            _ => 0,
        }
    }

    pub fn parse_literal_expression(&mut self) -> Option<LiteralExpressionSyntax> {
        if !matches!(
            self.current_token.kind(),
            TokenKind::Number
                | TokenKind::String
                | TokenKind::TrueKeyword
                | TokenKind::FalseKeyword
        ) {
            return None;
        }

        let literal_expression = LiteralExpressionSyntax::new(self.current_token);

        self.advance();

        Some(literal_expression)
    }

    pub fn parse_name_expression(&mut self) -> Option<NameExpressionSyntax> {
        if self.current_token.kind() != TokenKind::Identifier {
            return None;
        }

        let name_expression = NameExpressionSyntax::new(self.current_token);

        self.advance();

        Some(name_expression)
    }

    pub fn parse_unary_expression(&mut self) -> Option<UnaryExpressionSyntax> {
        if !matches!(
            self.current_token.kind(),
            TokenKind::Plus | TokenKind::Minus | TokenKind::LogicalNot
        ) {
            return None;
        }

        let operator_token = self.current_token;
        self.advance();

        let operand = self.parse_unary_or_primary_expression()?;

        Some(UnaryExpressionSyntax::new(operator_token, operand))
    }

    pub fn parse_parenthesized_expression(&mut self) -> Option<ParenthesizedExpressionSyntax> {
        if self.current_token.kind() != TokenKind::LeftParenthesis {
            return None;
        }

        let left_parenthesis_token = self.current_token;
        self.advance();

        let expression = self.parse_expression()?;

        if self.current_token.kind() != TokenKind::RightParenthesis {
            return None;
        }

        let right_parenthesis_token = self.current_token;
        self.advance();

        Some(ParenthesizedExpressionSyntax::new(
            left_parenthesis_token,
            expression,
            right_parenthesis_token,
        ))
    }

    fn next_significant_token(&mut self) -> Token {
        Self::next_significant_token_from(&mut self.lexer)
    }

    fn next_significant_token_from(lexer: &mut Lexer<'source>) -> Token {
        loop {
            let token = lexer.next_token();

            if !matches!(
                token.kind(),
                TokenKind::Whitespace | TokenKind::LineComment | TokenKind::BlockComment
            ) {
                return token;
            }
        }
    }

    pub fn advance(&mut self) {
        self.current_token = self.next_significant_token();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::source::SourceSpan;

    #[test]
    fn parser_starts_with_first_token() {
        let source = SourceText::new("let answer = 42;");
        let parser = Parser::new(&source);

        assert_eq!(parser.current_token().kind(), TokenKind::LetKeyword);
    }

    #[test]
    fn parser_advance_skips_whitespace() {
        let source = SourceText::new("let answer = 42;");
        let mut parser = Parser::new(&source);

        parser.advance();

        assert_eq!(parser.current_token().kind(), TokenKind::Identifier);
    }

    #[test]
    fn parser_starts_with_first_significant_token() {
        let source = SourceText::new("   let answer = 42;");
        let parser = Parser::new(&source);

        assert_eq!(parser.current_token().kind(), TokenKind::LetKeyword);
    }

    #[test]
    fn parser_advance_skips_line_comments() {
        let source = SourceText::new("let // comment\nanswer");
        let mut parser = Parser::new(&source);

        parser.advance();

        assert_eq!(parser.current_token().kind(), TokenKind::Identifier);
    }

    #[test]
    fn parser_advance_skips_block_comments() {
        let source = SourceText::new("let /* comment */ answer");
        let mut parser = Parser::new(&source);

        parser.advance();

        assert_eq!(parser.current_token().kind(), TokenKind::Identifier);
    }

    #[test]
    fn parser_does_not_skip_unterminated_block_comment() {
        let source = SourceText::new("let /* comment");
        let mut parser = Parser::new(&source);

        parser.advance();

        assert_eq!(
            parser.current_token().kind(),
            TokenKind::UnterminatedBlockComment
        );
    }

    #[test]
    fn parser_starts_at_end_of_file_for_empty_source() {
        let source = SourceText::new("");
        let parser = Parser::new(&source);

        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_starts_at_end_of_file_for_trivia_only_source() {
        let source = SourceText::new("  // comment\n/* block */  ");
        let parser = Parser::new(&source);

        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_advance_reaches_end_of_file() {
        let source = SourceText::new("answer");
        let mut parser = Parser::new(&source);

        parser.advance();

        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_advance_remains_at_end_of_file() {
        let source = SourceText::new("");
        let mut parser = Parser::new(&source);

        parser.advance();

        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_parses_literal_expression_token_kinds() {
        let cases = [
            ("42", TokenKind::Number),
            ("\"Forge\"", TokenKind::String),
            ("true", TokenKind::TrueKeyword),
            ("false", TokenKind::FalseKeyword),
        ];

        for (source_text, expected_kind) in cases {
            let source = SourceText::new(source_text);
            let mut parser = Parser::new(&source);

            let literal_expression = parser
                .parse_literal_expression()
                .expect("literal token should parse as literal expression");

            assert_eq!(literal_expression.literal_token().kind(), expected_kind);
            assert_eq!(
                literal_expression.literal_token().span(),
                SourceSpan::new(0, source.len())
            );
        }
    }

    #[test]
    fn parser_literal_expression_advances_to_next_significant_token() {
        let source = SourceText::new("42 /* comment */ answer");
        let mut parser = Parser::new(&source);

        let literal_expression = parser
            .parse_literal_expression()
            .expect("number should parse as literal expression");

        assert_eq!(literal_expression.literal_token().kind(), TokenKind::Number);
        assert_eq!(parser.current_token().kind(), TokenKind::Identifier);
    }

    #[test]
    fn parser_literal_expression_rejects_non_literal_without_advancing() {
        let source = SourceText::new("answer");
        let mut parser = Parser::new(&source);
        let current_token = parser.current_token();

        let literal_expression = parser.parse_literal_expression();

        assert_eq!(literal_expression, None);
        assert_eq!(parser.current_token(), current_token);
    }

    #[test]
    fn parser_literal_expression_rejects_unterminated_string_without_advancing() {
        let source = SourceText::new("\"Forge");
        let mut parser = Parser::new(&source);
        let current_token = parser.current_token();

        let literal_expression = parser.parse_literal_expression();

        assert_eq!(literal_expression, None);
        assert_eq!(parser.current_token().kind(), TokenKind::UnterminatedString);
        assert_eq!(parser.current_token(), current_token);
    }

    #[test]
    fn parser_literal_expression_returns_none_at_end_of_file() {
        let source = SourceText::new("");
        let mut parser = Parser::new(&source);

        let literal_expression = parser.parse_literal_expression();

        assert_eq!(literal_expression, None);
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_parses_name_expression() {
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
    }

    #[test]
    fn parser_name_expression_advances_to_next_significant_token() {
        let source = SourceText::new("answer /* comment */ 42");
        let mut parser = Parser::new(&source);

        let name_expression = parser
            .parse_name_expression()
            .expect("identifier should parse as name expression");

        assert_eq!(
            name_expression.identifier_token().kind(),
            TokenKind::Identifier
        );
        assert_eq!(parser.current_token().kind(), TokenKind::Number);
    }

    #[test]
    fn parser_name_expression_rejects_non_identifier_without_advancing() {
        let source = SourceText::new("42");
        let mut parser = Parser::new(&source);
        let current_token = parser.current_token();

        let name_expression = parser.parse_name_expression();

        assert_eq!(name_expression, None);
        assert_eq!(parser.current_token(), current_token);
    }

    #[test]
    fn parser_name_expression_returns_none_at_end_of_file() {
        let source = SourceText::new("");
        let mut parser = Parser::new(&source);

        let name_expression = parser.parse_name_expression();

        assert_eq!(name_expression, None);
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_parses_literal_as_expression() {
        let source = SourceText::new("42");
        let mut parser = Parser::new(&source);

        let expression = parser
            .parse_expression()
            .expect("literal should parse as expression");

        let ExpressionSyntax::Literal(literal_expression) = expression else {
            panic!("expected literal expression");
        };

        assert_eq!(literal_expression.literal_token().kind(), TokenKind::Number);
        assert_eq!(
            literal_expression.literal_token().span(),
            SourceSpan::new(0, 2)
        );
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_parses_name_as_expression() {
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
    fn parser_expression_advances_to_next_significant_token() {
        let source = SourceText::new("42 /* comment */ answer");
        let mut parser = Parser::new(&source);

        let expression = parser
            .parse_expression()
            .expect("literal should parse as expression");

        assert!(matches!(expression, ExpressionSyntax::Literal(_)));
        assert_eq!(parser.current_token().kind(), TokenKind::Identifier);
    }

    #[test]
    fn parser_expression_rejects_non_expression_without_advancing() {
        let source = SourceText::new(")");
        let mut parser = Parser::new(&source);
        let current_token = parser.current_token();

        let expression = parser.parse_expression();

        assert_eq!(expression, None);
        assert_eq!(parser.current_token(), current_token);
    }

    #[test]
    fn parser_expression_returns_none_at_end_of_file() {
        let source = SourceText::new("");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression();

        assert_eq!(expression, None);
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_parses_parenthesized_literal_expression() {
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
    fn parser_parses_parenthesized_name_expression() {
        let source = SourceText::new("(answer)");
        let mut parser = Parser::new(&source);

        let parenthesized_expression = parser
            .parse_parenthesized_expression()
            .expect("parenthesized name should parse");

        let ExpressionSyntax::Name(name_expression) = parenthesized_expression.expression() else {
            panic!("expected name expression");
        };

        assert_eq!(
            name_expression.identifier_token().kind(),
            TokenKind::Identifier
        );
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_parses_nested_parenthesized_expression() {
        let source = SourceText::new("((42))");
        let mut parser = Parser::new(&source);

        let expression = parser
            .parse_expression()
            .expect("nested parenthesized expression should parse");

        let ExpressionSyntax::Parenthesized(outer) = expression else {
            panic!("expected outer parenthesized expression");
        };

        assert!(matches!(
            outer.expression(),
            ExpressionSyntax::Parenthesized(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_parses_parenthesized_expression_as_expression() {
        let source = SourceText::new("(42)");
        let mut parser = Parser::new(&source);

        let expression = parser
            .parse_expression()
            .expect("parenthesized expression should parse as expression");

        assert!(matches!(expression, ExpressionSyntax::Parenthesized(_)));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_parenthesized_expression_rejects_non_left_parenthesis_without_advancing() {
        let source = SourceText::new("42");
        let mut parser = Parser::new(&source);
        let current_token = parser.current_token();

        let parenthesized_expression = parser.parse_parenthesized_expression();

        assert_eq!(parenthesized_expression, None);
        assert_eq!(parser.current_token(), current_token);
    }

    #[test]
    fn parser_parses_unary_literal_expression() {
        let source = SourceText::new("-42");
        let mut parser = Parser::new(&source);

        let unary_expression = parser
            .parse_unary_expression()
            .expect("unary literal should parse");

        assert_eq!(unary_expression.operator_token().kind(), TokenKind::Minus);

        let ExpressionSyntax::Literal(literal_expression) = unary_expression.operand() else {
            panic!("expected literal operand");
        };

        assert_eq!(literal_expression.literal_token().kind(), TokenKind::Number);
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_parses_unary_name_expression() {
        let source = SourceText::new("!ready");
        let mut parser = Parser::new(&source);

        let unary_expression = parser
            .parse_unary_expression()
            .expect("unary name should parse");

        assert_eq!(
            unary_expression.operator_token().kind(),
            TokenKind::LogicalNot
        );

        assert!(matches!(
            unary_expression.operand(),
            ExpressionSyntax::Name(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_parses_nested_unary_expression() {
        let source = SourceText::new("--42");
        let mut parser = Parser::new(&source);

        let expression = parser
            .parse_expression()
            .expect("nested unary expression should parse");

        let ExpressionSyntax::Unary(outer) = expression else {
            panic!("expected outer unary expression");
        };

        assert_eq!(outer.operator_token().kind(), TokenKind::Minus);
        assert!(matches!(outer.operand(), ExpressionSyntax::Unary(_)));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_parses_parenthesized_expression_as_unary_operand() {
        let source = SourceText::new("-(42)");
        let mut parser = Parser::new(&source);

        let expression = parser
            .parse_expression()
            .expect("parenthesized unary operand should parse");

        let ExpressionSyntax::Unary(unary_expression) = expression else {
            panic!("expected unary expression");
        };

        assert!(matches!(
            unary_expression.operand(),
            ExpressionSyntax::Parenthesized(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_parses_unary_expression_as_expression() {
        let source = SourceText::new("+42");
        let mut parser = Parser::new(&source);

        let expression = parser
            .parse_expression()
            .expect("unary expression should parse as expression");

        assert!(matches!(expression, ExpressionSyntax::Unary(_)));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_unary_expression_rejects_non_unary_operator_without_advancing() {
        let source = SourceText::new("42");
        let mut parser = Parser::new(&source);
        let current_token = parser.current_token();

        let unary_expression = parser.parse_unary_expression();

        assert_eq!(unary_expression, None);
        assert_eq!(parser.current_token(), current_token);
    }

    #[test]
    fn parser_parses_call_expression_without_arguments() {
        let source = SourceText::new("foo()");
        let mut parser = Parser::new(&source);

        let expression = parser
            .parse_expression()
            .expect("call expression should parse");

        let ExpressionSyntax::Call(call_expression) = expression else {
            panic!("expected call expression");
        };

        assert!(matches!(
            call_expression.callee(),
            ExpressionSyntax::Name(_)
        ));
        assert_eq!(
            call_expression.left_parenthesis_token().kind(),
            TokenKind::LeftParenthesis
        );
        assert!(call_expression.arguments().is_empty());
        assert_eq!(
            call_expression.right_parenthesis_token().kind(),
            TokenKind::RightParenthesis
        );
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_parses_call_expression_with_single_argument() {
        let source = SourceText::new("foo(42)");
        let mut parser = Parser::new(&source);

        let expression = parser
            .parse_expression()
            .expect("call expression should parse");

        let ExpressionSyntax::Call(call_expression) = expression else {
            panic!("expected call expression");
        };

        assert_eq!(call_expression.arguments().len(), 1);
        assert!(matches!(
            call_expression.arguments()[0],
            ExpressionSyntax::Literal(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_parses_call_expression_with_multiple_arguments() {
        let source = SourceText::new("foo(1, 2)");
        let mut parser = Parser::new(&source);

        let expression = parser
            .parse_expression()
            .expect("call expression should parse");

        let ExpressionSyntax::Call(call_expression) = expression else {
            panic!("expected call expression");
        };

        assert_eq!(call_expression.arguments().len(), 2);
        assert!(matches!(
            call_expression.arguments()[0],
            ExpressionSyntax::Literal(_)
        ));
        assert!(matches!(
            call_expression.arguments()[1],
            ExpressionSyntax::Literal(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_call_expression_accepts_expression_argument() {
        let source = SourceText::new("foo(x + 1)");
        let mut parser = Parser::new(&source);

        let expression = parser
            .parse_expression()
            .expect("call expression should parse");

        let ExpressionSyntax::Call(call_expression) = expression else {
            panic!("expected call expression");
        };

        assert_eq!(call_expression.arguments().len(), 1);
        assert!(matches!(
            call_expression.arguments()[0],
            ExpressionSyntax::Binary(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_call_expression_accepts_assignment_argument() {
        let source = SourceText::new("foo(x = 1)");
        let mut parser = Parser::new(&source);

        let expression = parser
            .parse_expression()
            .expect("call expression should parse");

        let ExpressionSyntax::Call(call_expression) = expression else {
            panic!("expected call expression");
        };

        assert_eq!(call_expression.arguments().len(), 1);
        assert!(matches!(
            call_expression.arguments()[0],
            ExpressionSyntax::Assignment(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_call_expression_supports_parenthesized_callee() {
        let source = SourceText::new("(foo)(42)");
        let mut parser = Parser::new(&source);

        let expression = parser
            .parse_expression()
            .expect("parenthesized call expression should parse");

        let ExpressionSyntax::Call(call_expression) = expression else {
            panic!("expected call expression");
        };

        assert!(matches!(
            call_expression.callee(),
            ExpressionSyntax::Parenthesized(_)
        ));
        assert_eq!(call_expression.arguments().len(), 1);
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_call_expression_supports_chained_calls() {
        let source = SourceText::new("foo()(42)");
        let mut parser = Parser::new(&source);

        let expression = parser
            .parse_expression()
            .expect("chained call expression should parse");

        let ExpressionSyntax::Call(outer_call) = expression else {
            panic!("expected outer call expression");
        };

        assert!(matches!(outer_call.callee(), ExpressionSyntax::Call(_)));
        assert_eq!(outer_call.arguments().len(), 1);
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_call_expression_binds_tighter_than_unary_expression() {
        let source = SourceText::new("-foo()");
        let mut parser = Parser::new(&source);

        let expression = parser
            .parse_expression()
            .expect("unary call expression should parse");

        let ExpressionSyntax::Unary(unary_expression) = expression else {
            panic!("expected unary expression");
        };

        assert_eq!(unary_expression.operator_token().kind(), TokenKind::Minus);
        assert!(matches!(
            unary_expression.operand(),
            ExpressionSyntax::Call(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_call_expression_binds_tighter_than_binary_expression() {
        let source = SourceText::new("foo() + 1");
        let mut parser = Parser::new(&source);

        let expression = parser
            .parse_expression()
            .expect("binary expression containing call should parse");

        let ExpressionSyntax::Binary(binary_expression) = expression else {
            panic!("expected binary expression");
        };

        assert_eq!(binary_expression.operator_token().kind(), TokenKind::Plus);
        assert!(matches!(
            binary_expression.left(),
            ExpressionSyntax::Call(_)
        ));
        assert!(matches!(
            binary_expression.right(),
            ExpressionSyntax::Literal(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_assignment_expression_accepts_call_expression_on_right() {
        let source = SourceText::new("result = foo()");
        let mut parser = Parser::new(&source);

        let expression = parser
            .parse_expression()
            .expect("assignment expression containing call should parse");

        let ExpressionSyntax::Assignment(assignment_expression) = expression else {
            panic!("expected assignment expression");
        };

        assert!(matches!(
            assignment_expression.expression(),
            ExpressionSyntax::Call(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_call_expression_skips_trivia_between_callee_and_parenthesis() {
        let source = SourceText::new("foo /* comment */ (42)");
        let mut parser = Parser::new(&source);

        let expression = parser
            .parse_expression()
            .expect("call expression separated by trivia should parse");

        assert!(matches!(expression, ExpressionSyntax::Call(_)));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_call_expression_skips_trivia_between_arguments() {
        let source = SourceText::new("foo(1, /* comment */ 2)");
        let mut parser = Parser::new(&source);

        let expression = parser
            .parse_expression()
            .expect("call expression containing trivia should parse");

        let ExpressionSyntax::Call(call_expression) = expression else {
            panic!("expected call expression");
        };

        assert_eq!(call_expression.arguments().len(), 2);
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_call_expression_returns_none_without_right_parenthesis() {
        let source = SourceText::new("foo(42");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression();

        assert_eq!(expression, None);
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_call_expression_returns_none_without_argument_after_comma() {
        let source = SourceText::new("foo(1,)");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression();

        assert_eq!(expression, None);
        assert_eq!(parser.current_token().kind(), TokenKind::RightParenthesis);
    }

    #[test]
    fn parser_parses_member_access_expression() {
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
        assert_eq!(member_access.dot_token().kind(), TokenKind::Dot);
        assert_eq!(
            member_access.member_name_token().kind(),
            TokenKind::Identifier
        );
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_member_access_expression_supports_chaining() {
        let source = SourceText::new("object.member.child");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression().unwrap();

        let ExpressionSyntax::MemberAccess(outer_member_access) = expression else {
            panic!("expected outer member access expression");
        };

        assert!(matches!(
            outer_member_access.expression(),
            ExpressionSyntax::MemberAccess(_)
        ));
        assert_eq!(
            outer_member_access.member_name_token().span(),
            SourceSpan::new(14, 5)
        );
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_member_access_expression_supports_member_after_call() {
        let source = SourceText::new("foo().member");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression().unwrap();

        let ExpressionSyntax::MemberAccess(member_access) = expression else {
            panic!("expected member access expression");
        };

        assert!(matches!(
            member_access.expression(),
            ExpressionSyntax::Call(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_member_access_expression_supports_call_after_member() {
        let source = SourceText::new("object.method()");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression().unwrap();

        let ExpressionSyntax::Call(call) = expression else {
            panic!("expected call expression");
        };

        assert!(matches!(call.callee(), ExpressionSyntax::MemberAccess(_)));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_member_access_and_call_expressions_compose_left_to_right() {
        let source = SourceText::new("foo().bar()");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression().unwrap();

        let ExpressionSyntax::Call(call) = expression else {
            panic!("expected outer call expression");
        };

        let ExpressionSyntax::MemberAccess(member_access) = call.callee() else {
            panic!("expected member access callee");
        };

        assert!(matches!(
            member_access.expression(),
            ExpressionSyntax::Call(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_member_access_expression_supports_parenthesized_expression() {
        let source = SourceText::new("(object).member");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression().unwrap();

        let ExpressionSyntax::MemberAccess(member_access) = expression else {
            panic!("expected member access expression");
        };

        assert!(matches!(
            member_access.expression(),
            ExpressionSyntax::Parenthesized(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_member_access_expression_binds_tighter_than_unary_expression() {
        let source = SourceText::new("-object.member");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression().unwrap();

        let ExpressionSyntax::Unary(unary) = expression else {
            panic!("expected unary expression");
        };

        assert!(matches!(unary.operand(), ExpressionSyntax::MemberAccess(_)));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_member_access_expression_binds_tighter_than_binary_expression() {
        let source = SourceText::new("object.member + 1");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression().unwrap();

        let ExpressionSyntax::Binary(binary) = expression else {
            panic!("expected binary expression");
        };

        assert!(matches!(binary.left(), ExpressionSyntax::MemberAccess(_)));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_assignment_expression_accepts_member_access_expression_on_right() {
        let source = SourceText::new("result = object.member");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression().unwrap();

        let ExpressionSyntax::Assignment(assignment) = expression else {
            panic!("expected assignment expression");
        };

        assert!(matches!(
            assignment.expression(),
            ExpressionSyntax::MemberAccess(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_member_access_expression_skips_trivia_around_dot() {
        let source = SourceText::new("object /* comment */ . /* comment */ member");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression().unwrap();

        assert!(matches!(expression, ExpressionSyntax::MemberAccess(_)));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_member_access_expression_returns_none_without_member_name() {
        let source = SourceText::new("object.");
        let mut parser = Parser::new(&source);

        assert!(parser.parse_expression().is_none());
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_member_access_expression_returns_none_for_non_identifier_member_name() {
        let source = SourceText::new("object.42");
        let mut parser = Parser::new(&source);

        assert!(parser.parse_expression().is_none());
        assert_eq!(parser.current_token().kind(), TokenKind::Number);
    }

    #[test]
    fn parser_parses_index_expression() {
        let source = SourceText::new("items[0]");
        let mut parser = Parser::new(&source);

        let expression = parser
            .parse_expression()
            .expect("index expression should parse");

        let ExpressionSyntax::Index(index_expression) = expression else {
            panic!("expected index expression");
        };

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
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_parses_empty_array_literal_expression() {
        let source = SourceText::new("[]");
        let mut parser = Parser::new(&source);

        let expression = parser
            .parse_expression()
            .expect("empty array literal should parse");

        let ExpressionSyntax::ArrayLiteral(array_literal) = expression else {
            panic!("expected array literal expression");
        };

        assert_eq!(
            array_literal.left_bracket_token().kind(),
            TokenKind::LeftBracket
        );
        assert!(array_literal.elements().is_empty());
        assert_eq!(
            array_literal.right_bracket_token().kind(),
            TokenKind::RightBracket
        );
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_parses_empty_object_literal_expression() {
        let source = SourceText::new("{}");
        let mut parser = Parser::new(&source);

        let expression = parser
            .parse_expression()
            .expect("empty object literal should parse");

        let ExpressionSyntax::ObjectLiteral(object_literal) = expression else {
            panic!("expected object literal expression");
        };

        assert_eq!(
            object_literal.left_brace_token().kind(),
            TokenKind::LeftBrace
        );
        assert!(object_literal.members().is_empty());
        assert_eq!(
            object_literal.right_brace_token().kind(),
            TokenKind::RightBrace
        );
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_parses_object_literal_expression_with_multiple_members() {
        let source = SourceText::new("{name: value, age: 42}");
        let mut parser = Parser::new(&source);

        let expression = parser
            .parse_expression()
            .expect("object literal should parse");

        let ExpressionSyntax::ObjectLiteral(object_literal) = expression else {
            panic!("expected object literal expression");
        };

        assert_eq!(object_literal.members().len(), 2);

        assert_eq!(
            object_literal.members()[0].name_token().kind(),
            TokenKind::Identifier
        );
        assert_eq!(
            object_literal.members()[0].colon_token().kind(),
            TokenKind::Colon
        );
        assert!(matches!(
            object_literal.members()[0].value(),
            ExpressionSyntax::Name(_)
        ));

        assert_eq!(
            object_literal.members()[1].name_token().kind(),
            TokenKind::Identifier
        );
        assert_eq!(
            object_literal.members()[1].colon_token().kind(),
            TokenKind::Colon
        );
        assert!(matches!(
            object_literal.members()[1].value(),
            ExpressionSyntax::Literal(_)
        ));

        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_object_literal_expression_accepts_expression_values() {
        let source = SourceText::new("{name: first + last, result: foo()}");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression().unwrap();

        let ExpressionSyntax::ObjectLiteral(object_literal) = expression else {
            panic!("expected object literal expression");
        };

        assert_eq!(object_literal.members().len(), 2);
        assert!(matches!(
            object_literal.members()[0].value(),
            ExpressionSyntax::Binary(_)
        ));
        assert!(matches!(
            object_literal.members()[1].value(),
            ExpressionSyntax::Call(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_object_literal_expression_supports_nesting() {
        let source = SourceText::new("{nested: {value: 1}, items: [1, 2, 3]}");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression().unwrap();

        let ExpressionSyntax::ObjectLiteral(object_literal) = expression else {
            panic!("expected outer object literal expression");
        };

        assert_eq!(object_literal.members().len(), 2);
        assert!(matches!(
            object_literal.members()[0].value(),
            ExpressionSyntax::ObjectLiteral(_)
        ));
        assert!(matches!(
            object_literal.members()[1].value(),
            ExpressionSyntax::ArrayLiteral(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_object_literal_expression_skips_trivia() {
        let source = SourceText::new(
            "{ /* comment */ name /* comment */ : /* comment */ 42 /* comment */ }",
        );
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression().unwrap();

        let ExpressionSyntax::ObjectLiteral(object_literal) = expression else {
            panic!("expected object literal expression");
        };

        assert_eq!(object_literal.members().len(), 1);
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_object_literal_expression_returns_none_without_value() {
        let source = SourceText::new("{name:}");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression();

        assert_eq!(expression, None);
        assert_eq!(parser.current_token().kind(), TokenKind::RightBrace);
    }

    #[test]
    fn parser_object_literal_expression_returns_none_without_colon() {
        let source = SourceText::new("{name 42}");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression();

        assert_eq!(expression, None);
        assert_eq!(parser.current_token().kind(), TokenKind::Number);
    }

    #[test]
    fn parser_object_literal_expression_returns_none_without_member_after_comma() {
        let source = SourceText::new("{name: 42,}");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression();

        assert_eq!(expression, None);
        assert_eq!(parser.current_token().kind(), TokenKind::RightBrace);
    }

    #[test]
    fn parser_object_literal_expression_returns_none_without_right_brace() {
        let source = SourceText::new("{name: 42");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression();

        assert_eq!(expression, None);
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_object_literal_expression_returns_none_for_non_identifier_member_name() {
        let source = SourceText::new("{42: value}");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression();

        assert_eq!(expression, None);
        assert_eq!(parser.current_token().kind(), TokenKind::Number);
    }

    #[test]
    fn parser_parses_array_literal_expression_with_multiple_elements() {
        let source = SourceText::new("[1, 2, 3]");
        let mut parser = Parser::new(&source);

        let expression = parser
            .parse_expression()
            .expect("array literal should parse");

        let ExpressionSyntax::ArrayLiteral(array_literal) = expression else {
            panic!("expected array literal expression");
        };

        assert_eq!(array_literal.elements().len(), 3);
        assert!(
            array_literal
                .elements()
                .iter()
                .all(|element| matches!(element, ExpressionSyntax::Literal(_)))
        );
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_array_literal_expression_accepts_expression_elements() {
        let source = SourceText::new("[a + b, foo()]");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression().unwrap();

        let ExpressionSyntax::ArrayLiteral(array_literal) = expression else {
            panic!("expected array literal expression");
        };

        assert_eq!(array_literal.elements().len(), 2);
        assert!(matches!(
            array_literal.elements()[0],
            ExpressionSyntax::Binary(_)
        ));
        assert!(matches!(
            array_literal.elements()[1],
            ExpressionSyntax::Call(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_array_literal_expression_supports_nesting() {
        let source = SourceText::new("[[1, 2], [3, 4]]");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression().unwrap();

        let ExpressionSyntax::ArrayLiteral(array_literal) = expression else {
            panic!("expected outer array literal expression");
        };

        assert_eq!(array_literal.elements().len(), 2);
        assert!(
            array_literal
                .elements()
                .iter()
                .all(|element| matches!(element, ExpressionSyntax::ArrayLiteral(_)))
        );
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_array_literal_expression_supports_index_postfix() {
        let source = SourceText::new("[1, 2][0]");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression().unwrap();

        let ExpressionSyntax::Index(index_expression) = expression else {
            panic!("expected index expression");
        };

        assert!(matches!(
            index_expression.expression(),
            ExpressionSyntax::ArrayLiteral(_)
        ));
        assert!(matches!(
            index_expression.index(),
            ExpressionSyntax::Literal(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_assignment_expression_accepts_array_literal_on_right() {
        let source = SourceText::new("value = [1, 2]");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression().unwrap();

        let ExpressionSyntax::Assignment(assignment_expression) = expression else {
            panic!("expected assignment expression");
        };

        assert!(matches!(
            assignment_expression.expression(),
            ExpressionSyntax::ArrayLiteral(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_array_literal_expression_participates_in_binary_expression() {
        let source = SourceText::new("[1, 2] + other");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression().unwrap();

        let ExpressionSyntax::Binary(binary_expression) = expression else {
            panic!("expected binary expression");
        };

        assert!(matches!(
            binary_expression.left(),
            ExpressionSyntax::ArrayLiteral(_)
        ));
        assert!(matches!(
            binary_expression.right(),
            ExpressionSyntax::Name(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_array_literal_expression_skips_trivia() {
        let source = SourceText::new("[ /* comment */ 1, /* comment */ 2 /* comment */ ]");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression().unwrap();

        let ExpressionSyntax::ArrayLiteral(array_literal) = expression else {
            panic!("expected array literal expression");
        };

        assert_eq!(array_literal.elements().len(), 2);
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_array_literal_expression_returns_none_without_element_after_comma() {
        let source = SourceText::new("[1,]");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression();

        assert_eq!(expression, None);
        assert_eq!(parser.current_token().kind(), TokenKind::RightBracket);
    }

    #[test]
    fn parser_array_literal_expression_returns_none_without_right_bracket() {
        let source = SourceText::new("[1");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression();

        assert_eq!(expression, None);
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_array_literal_expression_returns_none_for_missing_first_element() {
        let source = SourceText::new("[,]");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression();

        assert_eq!(expression, None);
        assert_eq!(parser.current_token().kind(), TokenKind::Comma);
    }

    #[test]
    fn parser_array_literal_expression_returns_none_for_consecutive_commas() {
        let source = SourceText::new("[1,,2]");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression();

        assert_eq!(expression, None);
        assert_eq!(parser.current_token().kind(), TokenKind::Comma);
    }

    #[test]
    fn parser_index_expression_accepts_name_index() {
        let source = SourceText::new("items[index]");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression().unwrap();

        let ExpressionSyntax::Index(index_expression) = expression else {
            panic!("expected index expression");
        };

        assert!(matches!(
            index_expression.index(),
            ExpressionSyntax::Name(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_index_expression_supports_chaining() {
        let source = SourceText::new("matrix[row][column]");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression().unwrap();

        let ExpressionSyntax::Index(outer_index) = expression else {
            panic!("expected outer index expression");
        };

        assert!(matches!(
            outer_index.expression(),
            ExpressionSyntax::Index(_)
        ));
        assert!(matches!(outer_index.index(), ExpressionSyntax::Name(_)));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_index_expression_supports_index_after_member_access() {
        let source = SourceText::new("object.items[0]");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression().unwrap();

        let ExpressionSyntax::Index(index_expression) = expression else {
            panic!("expected index expression");
        };

        assert!(matches!(
            index_expression.expression(),
            ExpressionSyntax::MemberAccess(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_index_expression_supports_index_after_call() {
        let source = SourceText::new("foo()[index]");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression().unwrap();

        let ExpressionSyntax::Index(index_expression) = expression else {
            panic!("expected index expression");
        };

        assert!(matches!(
            index_expression.expression(),
            ExpressionSyntax::Call(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_index_expression_supports_member_access_after_index() {
        let source = SourceText::new("items[0].member");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression().unwrap();

        let ExpressionSyntax::MemberAccess(member_access) = expression else {
            panic!("expected member access expression");
        };

        assert!(matches!(
            member_access.expression(),
            ExpressionSyntax::Index(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_index_expression_supports_call_after_index() {
        let source = SourceText::new("items[0]()");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression().unwrap();

        let ExpressionSyntax::Call(call_expression) = expression else {
            panic!("expected call expression");
        };

        assert!(matches!(
            call_expression.callee(),
            ExpressionSyntax::Index(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_index_expression_accepts_binary_expression_index() {
        let source = SourceText::new("items[a + b]");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression().unwrap();

        let ExpressionSyntax::Index(index_expression) = expression else {
            panic!("expected index expression");
        };

        assert!(matches!(
            index_expression.index(),
            ExpressionSyntax::Binary(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_index_expression_accepts_assignment_expression_index() {
        let source = SourceText::new("items[x = y]");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression().unwrap();

        let ExpressionSyntax::Index(index_expression) = expression else {
            panic!("expected index expression");
        };

        assert!(matches!(
            index_expression.index(),
            ExpressionSyntax::Assignment(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_index_expression_binds_tighter_than_unary_expression() {
        let source = SourceText::new("-items[0]");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression().unwrap();

        let ExpressionSyntax::Unary(unary_expression) = expression else {
            panic!("expected unary expression");
        };

        assert!(matches!(
            unary_expression.operand(),
            ExpressionSyntax::Index(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_index_expression_binds_tighter_than_binary_expression() {
        let source = SourceText::new("items[0] + 1");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression().unwrap();

        let ExpressionSyntax::Binary(binary_expression) = expression else {
            panic!("expected binary expression");
        };

        assert!(matches!(
            binary_expression.left(),
            ExpressionSyntax::Index(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_assignment_expression_accepts_index_expression_on_right() {
        let source = SourceText::new("value = items[0]");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression().unwrap();

        let ExpressionSyntax::Assignment(assignment_expression) = expression else {
            panic!("expected assignment expression");
        };

        assert!(matches!(
            assignment_expression.expression(),
            ExpressionSyntax::Index(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_index_expression_does_not_become_assignment_target() {
        let source = SourceText::new("items[0] = value");
        let mut parser = Parser::new(&source);

        let expression = parser
            .parse_expression()
            .expect("left index should parse as expression");

        assert!(matches!(expression, ExpressionSyntax::Index(_)));
        assert_eq!(parser.current_token().kind(), TokenKind::Assignment);
    }

    #[test]
    fn parser_index_expression_skips_trivia_inside_brackets() {
        let source = SourceText::new("items /* comment */ [ /* comment */ 0 /* comment */ ]");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression().unwrap();

        assert!(matches!(expression, ExpressionSyntax::Index(_)));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_index_expression_returns_none_without_index() {
        let source = SourceText::new("items[]");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression();

        assert_eq!(expression, None);
        assert_eq!(parser.current_token().kind(), TokenKind::RightBracket);
    }

    #[test]
    fn parser_index_expression_returns_none_without_index_at_end_of_file() {
        let source = SourceText::new("items[");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression();

        assert_eq!(expression, None);
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_index_expression_returns_none_without_right_bracket() {
        let source = SourceText::new("items[0");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression();

        assert_eq!(expression, None);
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_parses_binary_literal_expression() {
        let source = SourceText::new("1 + 2");
        let mut parser = Parser::new(&source);

        let expression = parser
            .parse_expression()
            .expect("binary literal expression should parse");

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
    fn parser_binary_expression_respects_multiplicative_precedence() {
        let source = SourceText::new("1 + 2 * 3");
        let mut parser = Parser::new(&source);

        let expression = parser
            .parse_expression()
            .expect("binary expression should parse");

        let ExpressionSyntax::Binary(addition) = expression else {
            panic!("expected addition expression");
        };

        assert_eq!(addition.operator_token().kind(), TokenKind::Plus);
        assert!(matches!(addition.left(), ExpressionSyntax::Literal(_)));

        let ExpressionSyntax::Binary(multiplication) = addition.right() else {
            panic!("expected multiplication on right side");
        };

        assert_eq!(multiplication.operator_token().kind(), TokenKind::Multiply);
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_binary_expression_is_left_associative() {
        let source = SourceText::new("10 - 3 - 2");
        let mut parser = Parser::new(&source);

        let expression = parser
            .parse_expression()
            .expect("binary expression should parse");

        let ExpressionSyntax::Binary(outer) = expression else {
            panic!("expected outer binary expression");
        };

        assert_eq!(outer.operator_token().kind(), TokenKind::Minus);
        assert!(matches!(outer.right(), ExpressionSyntax::Literal(_)));

        let ExpressionSyntax::Binary(inner) = outer.left() else {
            panic!("expected left-associated binary expression");
        };

        assert_eq!(inner.operator_token().kind(), TokenKind::Minus);
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_parses_assignment_expression() {
        let source = SourceText::new("answer = 42");
        let mut parser = Parser::new(&source);

        let expression = parser
            .parse_expression()
            .expect("assignment expression should parse");

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
    fn parser_assignment_expression_accepts_binary_expression_on_right() {
        let source = SourceText::new("answer = 1 + 2");
        let mut parser = Parser::new(&source);

        let expression = parser
            .parse_expression()
            .expect("assignment with binary expression should parse");

        let ExpressionSyntax::Assignment(assignment) = expression else {
            panic!("expected assignment expression");
        };

        let ExpressionSyntax::Binary(binary) = assignment.expression() else {
            panic!("expected binary expression on assignment right side");
        };

        assert_eq!(binary.operator_token().kind(), TokenKind::Plus);
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_assignment_expression_is_right_associative() {
        let source = SourceText::new("a = b = 1");
        let mut parser = Parser::new(&source);

        let expression = parser
            .parse_expression()
            .expect("chained assignment expression should parse");

        let ExpressionSyntax::Assignment(outer_assignment) = expression else {
            panic!("expected outer assignment expression");
        };

        assert_eq!(
            outer_assignment.identifier_token().span(),
            SourceSpan::new(0, 1)
        );

        let ExpressionSyntax::Assignment(inner_assignment) = outer_assignment.expression() else {
            panic!("expected assignment expression on right side");
        };

        assert_eq!(
            inner_assignment.identifier_token().span(),
            SourceSpan::new(4, 1)
        );
        assert!(matches!(
            inner_assignment.expression(),
            ExpressionSyntax::Literal(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_binary_expression_binds_tighter_than_assignment_expression() {
        let source = SourceText::new("a = 1 + 2 * 3");
        let mut parser = Parser::new(&source);

        let expression = parser
            .parse_expression()
            .expect("assignment with binary expression should parse");

        let ExpressionSyntax::Assignment(assignment) = expression else {
            panic!("expected assignment expression");
        };

        let ExpressionSyntax::Binary(addition) = assignment.expression() else {
            panic!("expected addition on assignment right side");
        };

        assert_eq!(addition.operator_token().kind(), TokenKind::Plus);

        let ExpressionSyntax::Binary(multiplication) = addition.right() else {
            panic!("expected multiplication on addition right side");
        };

        assert_eq!(multiplication.operator_token().kind(), TokenKind::Multiply);
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_assignment_expression_requires_identifier_target() {
        let source = SourceText::new("1 = 2");
        let mut parser = Parser::new(&source);

        let expression = parser
            .parse_expression()
            .expect("left literal should parse as expression");

        assert!(matches!(expression, ExpressionSyntax::Literal(_)));
        assert_eq!(parser.current_token().kind(), TokenKind::Assignment);
    }

    #[test]
    fn parser_assignment_expression_rejects_parenthesized_target() {
        let source = SourceText::new("(a) = 1");
        let mut parser = Parser::new(&source);

        let expression = parser
            .parse_expression()
            .expect("parenthesized name should parse as expression");

        assert!(matches!(expression, ExpressionSyntax::Parenthesized(_)));
        assert_eq!(parser.current_token().kind(), TokenKind::Assignment);
    }

    #[test]
    fn parser_unary_expression_binds_tighter_than_binary_expression() {
        let source = SourceText::new("-1 + 2");
        let mut parser = Parser::new(&source);

        let expression = parser
            .parse_expression()
            .expect("unary and binary expression should parse");

        let ExpressionSyntax::Binary(binary_expression) = expression else {
            panic!("expected binary expression");
        };

        assert_eq!(binary_expression.operator_token().kind(), TokenKind::Plus);
        assert!(matches!(
            binary_expression.left(),
            ExpressionSyntax::Unary(_)
        ));
        assert!(matches!(
            binary_expression.right(),
            ExpressionSyntax::Literal(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_parentheses_override_binary_precedence() {
        let source = SourceText::new("(1 + 2) * 3");
        let mut parser = Parser::new(&source);

        let expression = parser
            .parse_expression()
            .expect("parenthesized binary expression should parse");

        let ExpressionSyntax::Binary(multiplication) = expression else {
            panic!("expected multiplication expression");
        };

        assert_eq!(multiplication.operator_token().kind(), TokenKind::Multiply);

        let ExpressionSyntax::Parenthesized(parenthesized) = multiplication.left() else {
            panic!("expected parenthesized left operand");
        };

        let ExpressionSyntax::Binary(addition) = parenthesized.expression() else {
            panic!("expected addition inside parentheses");
        };

        assert_eq!(addition.operator_token().kind(), TokenKind::Plus);
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_comparison_binds_tighter_than_equality() {
        let source = SourceText::new("1 < 2 == true");
        let mut parser = Parser::new(&source);

        let expression = parser
            .parse_expression()
            .expect("comparison and equality expression should parse");

        let ExpressionSyntax::Binary(equality) = expression else {
            panic!("expected equality expression");
        };

        assert_eq!(equality.operator_token().kind(), TokenKind::Equal);

        let ExpressionSyntax::Binary(comparison) = equality.left() else {
            panic!("expected comparison on left side");
        };

        assert_eq!(comparison.operator_token().kind(), TokenKind::LessThan);
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_logical_and_binds_tighter_than_logical_or() {
        let source = SourceText::new("true || false && true");
        let mut parser = Parser::new(&source);

        let expression = parser
            .parse_expression()
            .expect("logical expression should parse");

        let ExpressionSyntax::Binary(logical_or) = expression else {
            panic!("expected logical-or expression");
        };

        assert_eq!(logical_or.operator_token().kind(), TokenKind::LogicalOr);

        let ExpressionSyntax::Binary(logical_and) = logical_or.right() else {
            panic!("expected logical-and expression on right side");
        };

        assert_eq!(logical_and.operator_token().kind(), TokenKind::LogicalAnd);
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_arithmetic_binds_tighter_than_comparison() {
        let source = SourceText::new("1 + 2 < 4");
        let mut parser = Parser::new(&source);

        let expression = parser
            .parse_expression()
            .expect("arithmetic and comparison expression should parse");

        let ExpressionSyntax::Binary(comparison) = expression else {
            panic!("expected comparison expression");
        };

        assert_eq!(comparison.operator_token().kind(), TokenKind::LessThan);

        let ExpressionSyntax::Binary(addition) = comparison.left() else {
            panic!("expected addition on left side");
        };

        assert_eq!(addition.operator_token().kind(), TokenKind::Plus);
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_parses_conditional_expression() {
        let source = SourceText::new("condition ? when_true : when_false");
        let mut parser = Parser::new(&source);

        let expression = parser
            .parse_expression()
            .expect("conditional expression should parse");

        let ExpressionSyntax::Conditional(conditional) = expression else {
            panic!("expected conditional expression");
        };

        assert!(matches!(conditional.condition(), ExpressionSyntax::Name(_)));
        assert_eq!(conditional.question_token().kind(), TokenKind::QuestionMark);
        assert!(matches!(conditional.when_true(), ExpressionSyntax::Name(_)));
        assert_eq!(conditional.colon_token().kind(), TokenKind::Colon);
        assert!(matches!(
            conditional.when_false(),
            ExpressionSyntax::Name(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_binary_expression_binds_tighter_than_conditional_expression() {
        let source = SourceText::new("score > 90 ? 1 : 0");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression().unwrap();

        let ExpressionSyntax::Conditional(conditional) = expression else {
            panic!("expected conditional expression");
        };

        let ExpressionSyntax::Binary(condition) = conditional.condition() else {
            panic!("expected binary condition");
        };

        assert_eq!(condition.operator_token().kind(), TokenKind::GreaterThan);
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_conditional_expression_binds_tighter_than_assignment_expression() {
        let source = SourceText::new("result = condition ? when_true : when_false");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression().unwrap();

        let ExpressionSyntax::Assignment(assignment) = expression else {
            panic!("expected assignment expression");
        };

        assert!(matches!(
            assignment.expression(),
            ExpressionSyntax::Conditional(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_conditional_expression_is_right_associative() {
        let source = SourceText::new("a ? b : c ? d : e");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression().unwrap();

        let ExpressionSyntax::Conditional(outer) = expression else {
            panic!("expected outer conditional expression");
        };

        assert!(matches!(outer.condition(), ExpressionSyntax::Name(_)));
        assert!(matches!(outer.when_true(), ExpressionSyntax::Name(_)));
        assert!(matches!(
            outer.when_false(),
            ExpressionSyntax::Conditional(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_conditional_expression_accepts_assignment_in_true_branch() {
        let source = SourceText::new("condition ? value = 1 : fallback");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression().unwrap();

        let ExpressionSyntax::Conditional(conditional) = expression else {
            panic!("expected conditional expression");
        };

        assert!(matches!(
            conditional.when_true(),
            ExpressionSyntax::Assignment(_)
        ));
        assert!(matches!(
            conditional.when_false(),
            ExpressionSyntax::Name(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_conditional_expression_does_not_become_assignment_target() {
        let source = SourceText::new("condition ? first : second = value");
        let mut parser = Parser::new(&source);

        let expression = parser
            .parse_expression()
            .expect("conditional expression should parse");

        assert!(matches!(expression, ExpressionSyntax::Conditional(_)));
        assert_eq!(parser.current_token().kind(), TokenKind::Assignment);
    }

    #[test]
    fn parser_conditional_expression_skips_trivia() {
        let source =
            SourceText::new("condition /* comment */ ? /* comment */ first /* comment */ : second");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression().unwrap();

        assert!(matches!(expression, ExpressionSyntax::Conditional(_)));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_conditional_expression_returns_none_without_true_branch() {
        let source = SourceText::new("condition ? : fallback");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression();

        assert_eq!(expression, None);
        assert_eq!(parser.current_token().kind(), TokenKind::Colon);
    }

    #[test]
    fn parser_conditional_expression_returns_none_without_colon() {
        let source = SourceText::new("condition ? when_true");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression();

        assert_eq!(expression, None);
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_conditional_expression_returns_none_without_false_branch() {
        let source = SourceText::new("condition ? when_true :");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression();

        assert_eq!(expression, None);
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_parses_expression_statement() {
        let source = SourceText::new("answer;");
        let mut parser = Parser::new(&source);

        let statement = parser
            .parse_statement()
            .expect("expected expression statement");

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
    fn parser_expression_statement_accepts_full_expression() {
        let source = SourceText::new("answer = condition ? first : second;");
        let mut parser = Parser::new(&source);

        let statement = parser
            .parse_statement()
            .expect("expected expression statement");

        let StatementSyntax::Expression(expression_statement) = statement else {
            panic!("expected expression statement");
        };

        assert!(matches!(
            expression_statement.expression(),
            ExpressionSyntax::Assignment(_)
        ));
        assert_eq!(
            expression_statement.semicolon_token().kind(),
            TokenKind::Semicolon
        );
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_expression_statement_skips_trivia_before_semicolon() {
        let source = SourceText::new("answer /* comment */ ;");
        let mut parser = Parser::new(&source);

        let statement = parser
            .parse_statement()
            .expect("expected expression statement");

        let StatementSyntax::Expression(expression_statement) = statement else {
            panic!("expected expression statement");
        };

        assert_eq!(
            expression_statement.semicolon_token().kind(),
            TokenKind::Semicolon
        );
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_expression_statement_returns_none_without_semicolon() {
        let source = SourceText::new("answer");
        let mut parser = Parser::new(&source);

        assert!(parser.parse_statement().is_none());
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_statement_returns_none_for_non_expression() {
        let source = SourceText::new(";");
        let mut parser = Parser::new(&source);

        assert!(parser.parse_statement().is_none());
        assert_eq!(parser.current_token().kind(), TokenKind::Semicolon);
    }

    #[test]
    fn parser_parses_empty_block() {
        let source = SourceText::new("{}");
        let mut parser = Parser::new(&source);

        let block = parser.parse_block().expect("expected block");

        assert_eq!(block.left_brace_token().kind(), TokenKind::LeftBrace);
        assert!(block.statements().is_empty());
        assert_eq!(block.right_brace_token().kind(), TokenKind::RightBrace);
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_parses_block_with_single_statement() {
        let source = SourceText::new("{ answer; }");
        let mut parser = Parser::new(&source);

        let block = parser.parse_block().expect("expected block");

        assert_eq!(block.statements().len(), 1);

        let StatementSyntax::Expression(statement) = &block.statements()[0] else {
            panic!("expected expression statement");
        };

        assert!(matches!(statement.expression(), ExpressionSyntax::Name(_)));
        assert_eq!(statement.semicolon_token().kind(), TokenKind::Semicolon);
    }

    #[test]
    fn parser_parses_block_with_multiple_statements() {
        let source = SourceText::new("{ answer; 1 + 2; }");
        let mut parser = Parser::new(&source);

        let block = parser.parse_block().expect("expected block");

        assert_eq!(block.statements().len(), 2);

        assert!(matches!(
            block.statements()[0],
            StatementSyntax::Expression(_)
        ));
        assert!(matches!(
            block.statements()[1],
            StatementSyntax::Expression(_)
        ));
    }

    #[test]
    fn parser_block_advances_past_right_brace() {
        let source = SourceText::new("{} answer");
        let mut parser = Parser::new(&source);

        parser.parse_block().expect("expected block");

        assert_eq!(parser.current_token().kind(), TokenKind::Identifier);
    }

    #[test]
    fn parser_block_accepts_full_expression_statement() {
        let source = SourceText::new("{ value = condition ? foo(1)[0].bar : {name: 42}; }");
        let mut parser = Parser::new(&source);

        let block = parser.parse_block().expect("expected block");

        assert_eq!(block.statements().len(), 1);

        let StatementSyntax::Expression(statement) = &block.statements()[0] else {
            panic!("expected expression statement");
        };

        assert!(matches!(
            statement.expression(),
            ExpressionSyntax::Assignment(_)
        ));
    }

    #[test]
    fn parser_block_skips_trivia() {
        let source = SourceText::new(
            "{
                // first
                answer;

                /* second */
                1 + 2;
            }",
        );
        let mut parser = Parser::new(&source);

        let block = parser.parse_block().expect("expected block");

        assert_eq!(block.statements().len(), 2);
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_block_returns_none_without_right_brace() {
        let source = SourceText::new("{ answer;");
        let mut parser = Parser::new(&source);

        assert!(parser.parse_block().is_none());
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_block_returns_none_for_incomplete_statement() {
        let source = SourceText::new("{ answer }");
        let mut parser = Parser::new(&source);

        assert!(parser.parse_block().is_none());
        assert_eq!(parser.current_token().kind(), TokenKind::RightBrace);
    }

    #[test]
    fn parser_block_rejects_non_left_brace_without_advancing() {
        let source = SourceText::new("answer;");
        let mut parser = Parser::new(&source);

        assert!(parser.parse_block().is_none());
        assert_eq!(parser.current_token().kind(), TokenKind::Identifier);
    }

    #[test]
    fn parser_expression_still_parses_object_literal() {
        let source = SourceText::new("{name: 42}");
        let mut parser = Parser::new(&source);

        let expression = parser.parse_expression().expect("expected expression");

        assert!(matches!(expression, ExpressionSyntax::ObjectLiteral(_)));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_block_can_contain_object_literal_expression_statement() {
        let source = SourceText::new("{ {name: 42}; }");
        let mut parser = Parser::new(&source);

        let block = parser.parse_block().expect("expected block");

        assert_eq!(block.statements().len(), 1);

        let StatementSyntax::Expression(statement) = &block.statements()[0] else {
            panic!("expected expression statement");
        };

        assert!(matches!(
            statement.expression(),
            ExpressionSyntax::ObjectLiteral(_)
        ));
    }

    #[test]
    fn parser_parses_conditional_statement_without_else() {
        let source = SourceText::new("if condition { answer; }");
        let mut parser = Parser::new(&source);

        let statement = parser
            .parse_statement()
            .expect("expected conditional statement");

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

        assert!(conditional_statement.else_clause().is_none());
        assert_eq!(conditional_statement.then_block().statements().len(), 1);
    }

    #[test]
    fn parser_parses_conditional_statement_with_single_elseif() {
        let source = SourceText::new("if first { answer; } elseif second { fallback; }");
        let mut parser = Parser::new(&source);

        let statement = parser
            .parse_statement()
            .expect("expected conditional statement");

        let StatementSyntax::Conditional(conditional_statement) = statement else {
            panic!("expected conditional statement");
        };

        assert_eq!(conditional_statement.then_block().statements().len(), 1);
        assert_eq!(conditional_statement.elseif_clauses().len(), 1);
        assert!(conditional_statement.else_clause().is_none());

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
    }

    #[test]
    fn parser_parses_conditional_statement_with_multiple_elseif_clauses() {
        let source = SourceText::new("if first {} elseif second {} elseif third { answer; }");
        let mut parser = Parser::new(&source);

        let statement = parser
            .parse_statement()
            .expect("expected conditional statement");

        let StatementSyntax::Conditional(conditional_statement) = statement else {
            panic!("expected conditional statement");
        };

        assert_eq!(conditional_statement.elseif_clauses().len(), 2);
        assert!(conditional_statement.else_clause().is_none());

        let first_elseif_clause = &conditional_statement.elseif_clauses()[0];
        let second_elseif_clause = &conditional_statement.elseif_clauses()[1];

        assert_eq!(
            first_elseif_clause.elseif_keyword().kind(),
            TokenKind::ElseIfKeyword
        );
        assert!(matches!(
            first_elseif_clause.condition(),
            ExpressionSyntax::Name(_)
        ));
        assert!(first_elseif_clause.block().statements().is_empty());

        assert_eq!(
            second_elseif_clause.elseif_keyword().kind(),
            TokenKind::ElseIfKeyword
        );
        assert!(matches!(
            second_elseif_clause.condition(),
            ExpressionSyntax::Name(_)
        ));
        assert_eq!(second_elseif_clause.block().statements().len(), 1);
    }

    #[test]
    fn parser_parses_conditional_statement_with_elseif_and_else() {
        let source =
            SourceText::new("if first {} elseif second {} elseif third {} else { fallback; }");
        let mut parser = Parser::new(&source);

        let statement = parser
            .parse_statement()
            .expect("expected conditional statement");

        let StatementSyntax::Conditional(conditional_statement) = statement else {
            panic!("expected conditional statement");
        };

        assert_eq!(conditional_statement.elseif_clauses().len(), 2);

        let else_clause = conditional_statement
            .else_clause()
            .expect("expected else clause");

        assert_eq!(else_clause.else_keyword().kind(), TokenKind::ElseKeyword);
        assert_eq!(else_clause.block().statements().len(), 1);
    }

    #[test]
    fn parser_conditional_statement_advances_past_elseif_and_else_chain() {
        let source = SourceText::new("if first {} elseif second {} elseif third {} else {} next;");
        let mut parser = Parser::new(&source);

        parser
            .parse_statement()
            .expect("expected conditional statement");

        assert_eq!(parser.current_token().kind(), TokenKind::Identifier);
    }

    #[test]
    fn parser_conditional_statement_advances_past_single_elseif_block() {
        let source = SourceText::new("if first {} elseif second {} next;");
        let mut parser = Parser::new(&source);

        parser
            .parse_statement()
            .expect("expected conditional statement");

        assert_eq!(parser.current_token().kind(), TokenKind::Identifier);
    }

    #[test]
    fn parser_elseif_clause_accepts_full_expression_condition() {
        let source = SourceText::new("if first {} elseif score > 90 && ready { answer; }");
        let mut parser = Parser::new(&source);

        let statement = parser
            .parse_statement()
            .expect("expected conditional statement");

        let StatementSyntax::Conditional(conditional_statement) = statement else {
            panic!("expected conditional statement");
        };

        let elseif_clause = &conditional_statement.elseif_clauses()[0];

        assert!(matches!(
            elseif_clause.condition(),
            ExpressionSyntax::Binary(_)
        ));
        assert_eq!(elseif_clause.block().statements().len(), 1);
    }

    #[test]
    fn parser_elseif_clause_accepts_empty_block() {
        let source = SourceText::new("if first {} elseif second {}");
        let mut parser = Parser::new(&source);

        let statement = parser
            .parse_statement()
            .expect("expected conditional statement");

        let StatementSyntax::Conditional(conditional_statement) = statement else {
            panic!("expected conditional statement");
        };

        let elseif_clause = &conditional_statement.elseif_clauses()[0];

        assert!(elseif_clause.block().statements().is_empty());
    }

    #[test]
    fn parser_elseif_clause_skips_trivia() {
        let source = SourceText::new(
            "if first {} /* elseif */ elseif /* condition */ second /* block */ { answer; }",
        );
        let mut parser = Parser::new(&source);

        let statement = parser
            .parse_statement()
            .expect("expected conditional statement");

        let StatementSyntax::Conditional(conditional_statement) = statement else {
            panic!("expected conditional statement");
        };

        assert_eq!(conditional_statement.elseif_clauses().len(), 1);

        let elseif_clause = &conditional_statement.elseif_clauses()[0];

        assert!(matches!(
            elseif_clause.condition(),
            ExpressionSyntax::Name(_)
        ));
        assert_eq!(elseif_clause.block().statements().len(), 1);
    }

    #[test]
    fn parser_elseif_clause_supports_nested_conditional_statement() {
        let source = SourceText::new("if first {} elseif second { if nested { answer; } }");
        let mut parser = Parser::new(&source);

        let statement = parser
            .parse_statement()
            .expect("expected conditional statement");

        let StatementSyntax::Conditional(conditional_statement) = statement else {
            panic!("expected conditional statement");
        };

        let elseif_clause = &conditional_statement.elseif_clauses()[0];

        assert_eq!(elseif_clause.block().statements().len(), 1);
        assert!(matches!(
            &elseif_clause.block().statements()[0],
            StatementSyntax::Conditional(_)
        ));
    }

    #[test]
    fn parser_conditional_statement_returns_none_when_elseif_has_no_condition() {
        let source = SourceText::new("if first {} elseif");
        let mut parser = Parser::new(&source);

        assert!(parser.parse_statement().is_none());
    }

    #[test]
    fn parser_conditional_statement_returns_none_when_elseif_has_no_block() {
        let source = SourceText::new("if first {} elseif second");
        let mut parser = Parser::new(&source);

        assert!(parser.parse_statement().is_none());
    }

    #[test]
    fn parser_conditional_statement_accepts_full_expression_condition() {
        let source = SourceText::new("if score > 90 && ready { answer; }");
        let mut parser = Parser::new(&source);

        let statement = parser
            .parse_statement()
            .expect("expected conditional statement");

        let StatementSyntax::Conditional(conditional_statement) = statement else {
            panic!("expected conditional statement");
        };

        assert!(matches!(
            conditional_statement.condition(),
            ExpressionSyntax::Binary(_)
        ));

        assert_eq!(conditional_statement.then_block().statements().len(), 1);
    }

    #[test]
    fn parser_conditional_statement_accepts_empty_then_block() {
        let source = SourceText::new("if condition {}");
        let mut parser = Parser::new(&source);

        let statement = parser
            .parse_statement()
            .expect("expected conditional statement");

        let StatementSyntax::Conditional(conditional_statement) = statement else {
            panic!("expected conditional statement");
        };

        assert!(conditional_statement.then_block().statements().is_empty());
        assert!(conditional_statement.else_clause().is_none());
    }

    #[test]
    fn parser_conditional_statement_advances_past_then_block() {
        let source = SourceText::new("if condition {} next;");
        let mut parser = Parser::new(&source);

        parser
            .parse_statement()
            .expect("expected conditional statement");

        assert_eq!(parser.current_token().kind(), TokenKind::Identifier);
    }

    #[test]
    fn parser_parses_conditional_statement_with_else() {
        let source = SourceText::new("if condition { answer; } else { fallback; }");
        let mut parser = Parser::new(&source);

        let statement = parser
            .parse_statement()
            .expect("expected conditional statement");

        let StatementSyntax::Conditional(conditional_statement) = statement else {
            panic!("expected conditional statement");
        };

        assert_eq!(conditional_statement.then_block().statements().len(), 1);

        let else_clause = conditional_statement
            .else_clause()
            .expect("expected else clause");

        assert_eq!(else_clause.else_keyword().kind(), TokenKind::ElseKeyword);
        assert_eq!(else_clause.block().statements().len(), 1);
    }

    #[test]
    fn parser_conditional_statement_accepts_empty_else_block() {
        let source = SourceText::new("if condition {} else {}");
        let mut parser = Parser::new(&source);

        let statement = parser
            .parse_statement()
            .expect("expected conditional statement");

        let StatementSyntax::Conditional(conditional_statement) = statement else {
            panic!("expected conditional statement");
        };

        assert!(conditional_statement.then_block().statements().is_empty());

        let else_clause = conditional_statement
            .else_clause()
            .expect("expected else clause");

        assert!(else_clause.block().statements().is_empty());
    }

    #[test]
    fn parser_conditional_statement_advances_past_else_block() {
        let source = SourceText::new("if condition {} else {} next;");
        let mut parser = Parser::new(&source);

        parser
            .parse_statement()
            .expect("expected conditional statement");

        assert_eq!(parser.current_token().kind(), TokenKind::Identifier);
    }

    #[test]
    fn parser_conditional_statement_supports_nested_conditional_in_then_block() {
        let source = SourceText::new("if outer { if inner { answer; } }");
        let mut parser = Parser::new(&source);

        let statement = parser
            .parse_statement()
            .expect("expected outer conditional statement");

        let StatementSyntax::Conditional(outer_conditional) = statement else {
            panic!("expected outer conditional statement");
        };

        assert_eq!(outer_conditional.then_block().statements().len(), 1);

        let StatementSyntax::Conditional(inner_conditional) =
            &outer_conditional.then_block().statements()[0]
        else {
            panic!("expected nested conditional statement");
        };

        assert!(matches!(
            inner_conditional.condition(),
            ExpressionSyntax::Name(_)
        ));
    }

    #[test]
    fn parser_conditional_statement_supports_nested_conditional_in_else_block() {
        let source = SourceText::new("if outer {} else { if inner { answer; } }");
        let mut parser = Parser::new(&source);

        let statement = parser
            .parse_statement()
            .expect("expected outer conditional statement");

        let StatementSyntax::Conditional(outer_conditional) = statement else {
            panic!("expected outer conditional statement");
        };

        let else_clause = outer_conditional
            .else_clause()
            .expect("expected outer else clause");

        assert_eq!(else_clause.block().statements().len(), 1);

        assert!(matches!(
            &else_clause.block().statements()[0],
            StatementSyntax::Conditional(_)
        ));
    }

    #[test]
    fn parser_conditional_statement_skips_trivia() {
        let source = SourceText::new(
            "if /* condition */ condition /* then */ { answer; } /* else */ else /* block */ { fallback; }",
        );
        let mut parser = Parser::new(&source);

        let statement = parser
            .parse_statement()
            .expect("expected conditional statement");

        let StatementSyntax::Conditional(conditional_statement) = statement else {
            panic!("expected conditional statement");
        };

        assert_eq!(conditional_statement.then_block().statements().len(), 1);
        assert!(conditional_statement.else_clause().is_some());
    }

    #[test]
    fn parser_conditional_statement_returns_none_without_condition() {
        let source = SourceText::new("if");
        let mut parser = Parser::new(&source);

        assert!(parser.parse_statement().is_none());
    }

    #[test]
    fn parser_conditional_statement_returns_none_without_then_block() {
        let source = SourceText::new("if condition");
        let mut parser = Parser::new(&source);

        assert!(parser.parse_statement().is_none());
    }

    #[test]
    fn parser_conditional_statement_returns_none_when_else_has_no_block() {
        let source = SourceText::new("if condition {} else");
        let mut parser = Parser::new(&source);

        assert!(parser.parse_statement().is_none());
    }

    #[test]
    fn parser_conditional_statement_condition_can_be_object_literal() {
        let source = SourceText::new("if { value: 42 } { answer; }");
        let mut parser = Parser::new(&source);

        let statement = parser
            .parse_statement()
            .expect("expected conditional statement");

        let StatementSyntax::Conditional(conditional_statement) = statement else {
            panic!("expected conditional statement");
        };

        assert!(matches!(
            conditional_statement.condition(),
            ExpressionSyntax::ObjectLiteral(_)
        ));

        assert_eq!(conditional_statement.then_block().statements().len(), 1);
    }

    #[test]
    fn parser_conditional_statement_condition_can_be_conditional_expression() {
        let source = SourceText::new("if condition ? when_true : when_false { answer; }");
        let mut parser = Parser::new(&source);

        let statement = parser
            .parse_statement()
            .expect("expected conditional statement");

        let StatementSyntax::Conditional(conditional_statement) = statement else {
            panic!("expected conditional statement");
        };

        assert!(matches!(
            conditional_statement.condition(),
            ExpressionSyntax::Conditional(_)
        ));
    }

    #[test]
    fn parser_block_accepts_conditional_and_expression_statements() {
        let source = SourceText::new("{ if condition { answer; } fallback; }");
        let mut parser = Parser::new(&source);

        let block = parser.parse_block().expect("expected block");

        assert_eq!(block.statements().len(), 2);
        assert!(matches!(
            &block.statements()[0],
            StatementSyntax::Conditional(_)
        ));
        assert!(matches!(
            &block.statements()[1],
            StatementSyntax::Expression(_)
        ));
    }

    #[test]
    fn parser_parses_while_statement() {
        let source = SourceText::new("while ready { process(); }");
        let mut parser = Parser::new(&source);

        let statement = parser.parse_statement().expect("expected while statement");

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
        assert!(matches!(
            &while_statement.block().statements()[0],
            StatementSyntax::Expression(_)
        ));
    }

    #[test]
    fn parser_while_statement_accepts_empty_block() {
        let source = SourceText::new("while ready {}");
        let mut parser = Parser::new(&source);

        let statement = parser.parse_statement().expect("expected while statement");

        let StatementSyntax::While(while_statement) = statement else {
            panic!("expected while statement");
        };

        assert!(matches!(
            while_statement.condition(),
            ExpressionSyntax::Name(_)
        ));
        assert!(while_statement.block().statements().is_empty());
    }

    #[test]
    fn parser_while_statement_advances_past_block() {
        let source = SourceText::new("while ready {} next;");
        let mut parser = Parser::new(&source);

        parser.parse_statement().expect("expected while statement");

        assert_eq!(parser.current_token().kind(), TokenKind::Identifier);
    }

    #[test]
    fn parser_while_statement_accepts_full_expression_condition() {
        let source = SourceText::new("while ready && count < limit { process(); }");
        let mut parser = Parser::new(&source);

        let statement = parser.parse_statement().expect("expected while statement");

        let StatementSyntax::While(while_statement) = statement else {
            panic!("expected while statement");
        };

        let ExpressionSyntax::Binary(logical_and) = while_statement.condition() else {
            panic!("expected logical-and condition");
        };

        assert_eq!(logical_and.operator_token().kind(), TokenKind::LogicalAnd);
        assert!(matches!(logical_and.left(), ExpressionSyntax::Name(_)));

        let ExpressionSyntax::Binary(comparison) = logical_and.right() else {
            panic!("expected comparison on right side");
        };

        assert_eq!(comparison.operator_token().kind(), TokenKind::LessThan);
        assert_eq!(while_statement.block().statements().len(), 1);
    }

    #[test]
    fn parser_while_statement_supports_nested_while_statement() {
        let source = SourceText::new("while outer { while inner { process(); } }");
        let mut parser = Parser::new(&source);

        let statement = parser
            .parse_statement()
            .expect("expected outer while statement");

        let StatementSyntax::While(outer_while) = statement else {
            panic!("expected outer while statement");
        };

        assert_eq!(outer_while.block().statements().len(), 1);

        let StatementSyntax::While(inner_while) = &outer_while.block().statements()[0] else {
            panic!("expected nested while statement");
        };

        assert!(matches!(inner_while.condition(), ExpressionSyntax::Name(_)));
        assert_eq!(inner_while.block().statements().len(), 1);
    }

    #[test]
    fn parser_while_statement_supports_conditional_statement_in_block() {
        let source = SourceText::new("while running { if ready { process(); } }");
        let mut parser = Parser::new(&source);

        let statement = parser.parse_statement().expect("expected while statement");

        let StatementSyntax::While(while_statement) = statement else {
            panic!("expected while statement");
        };

        assert_eq!(while_statement.block().statements().len(), 1);
        assert!(matches!(
            &while_statement.block().statements()[0],
            StatementSyntax::Conditional(_)
        ));
    }

    #[test]
    fn parser_conditional_statement_supports_while_statement_in_then_block() {
        let source = SourceText::new("if enabled { while running { process(); } }");
        let mut parser = Parser::new(&source);

        let statement = parser
            .parse_statement()
            .expect("expected conditional statement");

        let StatementSyntax::Conditional(conditional_statement) = statement else {
            panic!("expected conditional statement");
        };

        assert_eq!(conditional_statement.then_block().statements().len(), 1);
        assert!(matches!(
            &conditional_statement.then_block().statements()[0],
            StatementSyntax::While(_)
        ));
    }

    #[test]
    fn parser_block_accepts_while_conditional_and_expression_statements() {
        let source = SourceText::new("{ while running {} if ready { process(); } fallback; }");
        let mut parser = Parser::new(&source);

        let block = parser.parse_block().expect("expected block");

        assert_eq!(block.statements().len(), 3);
        assert!(matches!(&block.statements()[0], StatementSyntax::While(_)));
        assert!(matches!(
            &block.statements()[1],
            StatementSyntax::Conditional(_)
        ));
        assert!(matches!(
            &block.statements()[2],
            StatementSyntax::Expression(_)
        ));
    }

    #[test]
    fn parser_while_statement_skips_trivia() {
        let source =
            SourceText::new("while /* condition */ ready /* block */ { /* body */ process(); }");
        let mut parser = Parser::new(&source);

        let statement = parser.parse_statement().expect("expected while statement");

        let StatementSyntax::While(while_statement) = statement else {
            panic!("expected while statement");
        };

        assert!(matches!(
            while_statement.condition(),
            ExpressionSyntax::Name(_)
        ));
        assert_eq!(while_statement.block().statements().len(), 1);
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_while_statement_returns_none_without_condition() {
        let source = SourceText::new("while");
        let mut parser = Parser::new(&source);

        assert!(parser.parse_statement().is_none());
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_while_statement_returns_none_without_block() {
        let source = SourceText::new("while ready");
        let mut parser = Parser::new(&source);

        assert!(parser.parse_statement().is_none());
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_while_statement_returns_none_for_missing_condition_before_block() {
        let source = SourceText::new("while {}");
        let mut parser = Parser::new(&source);

        assert!(parser.parse_statement().is_none());
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_while_statement_returns_none_for_incomplete_block() {
        let source = SourceText::new("while ready { process();");
        let mut parser = Parser::new(&source);

        assert!(parser.parse_statement().is_none());
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_parses_let_variable_declaration_statement() {
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
    fn parser_parses_var_variable_declaration_statement() {
        let source = SourceText::new("var total = 10;");
        let mut parser = Parser::new(&source);

        let statement = parser.parse_statement().unwrap();

        let StatementSyntax::VariableDeclaration(declaration) = statement else {
            panic!("expected variable declaration statement");
        };

        assert_eq!(
            declaration.declaration_keyword().kind(),
            TokenKind::VarKeyword
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
    fn parser_parses_for_statement() {
        let source = SourceText::new("for i = 0; i < 10; i = i + 1 {}");
        let mut parser = Parser::new(&source);

        let statement = parser
            .parse_statement()
            .expect("for statement should parse");

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
    fn parser_for_statement_accepts_multiple_statements_in_block() {
        let source = SourceText::new("for i = 0; i < 10; i = i + 1 { first(); second(); }");
        let mut parser = Parser::new(&source);

        let statement = parser
            .parse_statement()
            .expect("for statement should parse");

        let StatementSyntax::For(for_statement) = statement else {
            panic!("expected for statement");
        };

        assert_eq!(for_statement.block().statements().len(), 2);
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_for_statement_supports_nested_for_statement() {
        let source = SourceText::new(
            "for i = 0; i < 10; i = i + 1 {
                for j = 0; j < 10; j = j + 1 {
                    process(i, j);
                }
            }",
        );
        let mut parser = Parser::new(&source);

        let statement = parser
            .parse_statement()
            .expect("outer for statement should parse");

        let StatementSyntax::For(outer_for) = statement else {
            panic!("expected outer for statement");
        };

        assert_eq!(outer_for.block().statements().len(), 1);
        assert!(matches!(
            outer_for.block().statements()[0],
            StatementSyntax::For(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_while_statement_supports_for_statement_in_block() {
        let source = SourceText::new(
            "while running {
                for i = 0; i < limit; i = i + 1 {
                    process(i);
                }
            }",
        );
        let mut parser = Parser::new(&source);

        let statement = parser
            .parse_statement()
            .expect("while statement should parse");

        let StatementSyntax::While(while_statement) = statement else {
            panic!("expected while statement");
        };

        assert_eq!(while_statement.block().statements().len(), 1);
        assert!(matches!(
            while_statement.block().statements()[0],
            StatementSyntax::For(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_for_statement_supports_conditional_statement_in_block() {
        let source = SourceText::new(
            "for i = 0; i < limit; i = i + 1 {
                if ready {
                    process(i);
                }
            }",
        );
        let mut parser = Parser::new(&source);

        let statement = parser
            .parse_statement()
            .expect("for statement should parse");

        let StatementSyntax::For(for_statement) = statement else {
            panic!("expected for statement");
        };

        assert_eq!(for_statement.block().statements().len(), 1);
        assert!(matches!(
            for_statement.block().statements()[0],
            StatementSyntax::Conditional(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_for_statement_accepts_full_expressions() {
        let source =
            SourceText::new("for i = start + offset; i < limit && ready; i = i + step * 2 {}");
        let mut parser = Parser::new(&source);

        let statement = parser
            .parse_statement()
            .expect("for statement should parse");

        let StatementSyntax::For(for_statement) = statement else {
            panic!("expected for statement");
        };

        assert!(matches!(
            for_statement.initializer(),
            ExpressionSyntax::Assignment(_)
        ));
        assert!(matches!(
            for_statement.condition(),
            ExpressionSyntax::Binary(_)
        ));
        assert!(matches!(
            for_statement.update(),
            ExpressionSyntax::Assignment(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_for_statement_skips_trivia() {
        let source = SourceText::new(
            "for /* initializer */ i = 0
                ; // condition follows
                i < 10
                ; /* update */ i = i + 1
                {
                    process(i);
                }",
        );
        let mut parser = Parser::new(&source);

        let statement = parser
            .parse_statement()
            .expect("for statement should parse with trivia");

        let StatementSyntax::For(for_statement) = statement else {
            panic!("expected for statement");
        };

        assert_eq!(for_statement.block().statements().len(), 1);
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_for_statement_returns_none_without_initializer() {
        let source = SourceText::new("for");
        let mut parser = Parser::new(&source);

        assert!(parser.parse_statement().is_none());
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_for_statement_returns_none_for_missing_initializer_before_semicolon() {
        let source = SourceText::new("for ; i < 10; i = i + 1 {}");
        let mut parser = Parser::new(&source);

        assert!(parser.parse_statement().is_none());
        assert_eq!(parser.current_token().kind(), TokenKind::Semicolon);
    }

    #[test]
    fn parser_for_statement_returns_none_without_first_semicolon() {
        let source = SourceText::new("for i = 0 i < 10; i = i + 1 {}");
        let mut parser = Parser::new(&source);

        assert!(parser.parse_statement().is_none());
        assert_eq!(parser.current_token().kind(), TokenKind::Identifier);
    }

    #[test]
    fn parser_for_statement_returns_none_without_condition() {
        let source = SourceText::new("for i = 0; ; i = i + 1 {}");
        let mut parser = Parser::new(&source);

        assert!(parser.parse_statement().is_none());
        assert_eq!(parser.current_token().kind(), TokenKind::Semicolon);
    }

    #[test]
    fn parser_for_statement_returns_none_without_second_semicolon() {
        let source = SourceText::new("for i = 0; i < 10 i = i + 1 {}");
        let mut parser = Parser::new(&source);

        assert!(parser.parse_statement().is_none());
        assert_eq!(parser.current_token().kind(), TokenKind::Identifier);
    }

    #[test]
    fn parser_for_statement_returns_none_without_update() {
        let source = SourceText::new("for i = 0; i < 10; {}");
        let mut parser = Parser::new(&source);

        assert!(parser.parse_statement().is_none());
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_for_statement_returns_none_without_block() {
        let source = SourceText::new("for i = 0; i < 10; i = i + 1");
        let mut parser = Parser::new(&source);

        assert!(parser.parse_statement().is_none());
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_for_statement_returns_none_for_incomplete_block() {
        let source = SourceText::new(
            "for i = 0; i < 10; i = i + 1 {
                process(i);
            ",
        );
        let mut parser = Parser::new(&source);

        assert!(parser.parse_statement().is_none());
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_for_statement_advances_past_block() {
        let source = SourceText::new("for i = 0; i < 10; i = i + 1 {} next;");
        let mut parser = Parser::new(&source);

        let statement = parser
            .parse_statement()
            .expect("for statement should parse");

        assert!(matches!(statement, StatementSyntax::For(_)));
        assert_eq!(parser.current_token().kind(), TokenKind::Identifier);
    }

    #[test]
    fn parser_variable_declaration_accepts_binary_expression_initializer() {
        let source = SourceText::new("let total = first + second * third;");
        let mut parser = Parser::new(&source);

        let statement = parser.parse_statement().unwrap();

        let StatementSyntax::VariableDeclaration(declaration) = statement else {
            panic!("expected variable declaration statement");
        };

        assert!(matches!(
            declaration.initializer(),
            ExpressionSyntax::Binary(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_variable_declaration_accepts_call_expression_initializer() {
        let source = SourceText::new("let result = process(value);");
        let mut parser = Parser::new(&source);

        let statement = parser.parse_statement().unwrap();

        let StatementSyntax::VariableDeclaration(declaration) = statement else {
            panic!("expected variable declaration statement");
        };

        assert!(matches!(
            declaration.initializer(),
            ExpressionSyntax::Call(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_variable_declaration_accepts_conditional_expression_initializer() {
        let source = SourceText::new("var selected = ready ? first : second;");
        let mut parser = Parser::new(&source);

        let statement = parser.parse_statement().unwrap();

        let StatementSyntax::VariableDeclaration(declaration) = statement else {
            panic!("expected variable declaration statement");
        };

        assert!(matches!(
            declaration.initializer(),
            ExpressionSyntax::Conditional(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_variable_declaration_returns_none_without_identifier() {
        let source = SourceText::new("let");
        let mut parser = Parser::new(&source);

        assert_eq!(parser.parse_statement(), None);
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_var_declaration_returns_none_without_identifier() {
        let source = SourceText::new("var");
        let mut parser = Parser::new(&source);

        assert_eq!(parser.parse_statement(), None);
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_variable_declaration_returns_none_when_assignment_follows_keyword() {
        let source = SourceText::new("let = 10;");
        let mut parser = Parser::new(&source);

        assert_eq!(parser.parse_statement(), None);
        assert_eq!(parser.current_token().kind(), TokenKind::Assignment);
    }

    #[test]
    fn parser_variable_declaration_returns_none_without_assignment_operator() {
        let source = SourceText::new("let value 10;");
        let mut parser = Parser::new(&source);

        assert_eq!(parser.parse_statement(), None);
        assert_eq!(parser.current_token().kind(), TokenKind::Number);
    }

    #[test]
    fn parser_variable_declaration_returns_none_without_initializer() {
        let source = SourceText::new("let value = ;");
        let mut parser = Parser::new(&source);

        assert_eq!(parser.parse_statement(), None);
        assert_eq!(parser.current_token().kind(), TokenKind::Semicolon);
    }

    #[test]
    fn parser_variable_declaration_returns_none_without_semicolon() {
        let source = SourceText::new("let value = 10");
        let mut parser = Parser::new(&source);

        assert_eq!(parser.parse_statement(), None);
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_variable_declaration_returns_none_for_incomplete_binary_initializer() {
        let source = SourceText::new("let value = first + ;");
        let mut parser = Parser::new(&source);

        assert_eq!(parser.parse_statement(), None);
        assert_eq!(parser.current_token().kind(), TokenKind::Semicolon);
    }

    #[test]
    fn parser_variable_declaration_skips_trivia() {
        let source =
            SourceText::new("let /* declaration */ value\n=\n/* initializer */ 10 /* end */ ;");
        let mut parser = Parser::new(&source);

        let statement = parser.parse_statement().unwrap();

        assert!(matches!(statement, StatementSyntax::VariableDeclaration(_)));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_block_accepts_variable_declaration_statement() {
        let source = SourceText::new("{ let count = 0; }");
        let mut parser = Parser::new(&source);

        let block = parser.parse_block().unwrap();

        assert_eq!(block.statements().len(), 1);
        assert!(matches!(
            block.statements()[0],
            StatementSyntax::VariableDeclaration(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_block_accepts_multiple_variable_declarations_and_expression_statement() {
        let source = SourceText::new("{ let first = 1; var second = 2; first = second; }");
        let mut parser = Parser::new(&source);

        let block = parser.parse_block().unwrap();

        assert_eq!(block.statements().len(), 3);
        assert!(matches!(
            block.statements()[0],
            StatementSyntax::VariableDeclaration(_)
        ));
        assert!(matches!(
            block.statements()[1],
            StatementSyntax::VariableDeclaration(_)
        ));
        assert!(matches!(
            block.statements()[2],
            StatementSyntax::Expression(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_control_flow_blocks_accept_variable_declaration_statements() {
        let source =
            SourceText::new("if ready { let first = 1; } while active { var second = 2; }");
        let mut parser = Parser::new(&source);

        let first_statement = parser.parse_statement().unwrap();
        assert!(matches!(first_statement, StatementSyntax::Conditional(_)));

        let second_statement = parser.parse_statement().unwrap();
        assert!(matches!(second_statement, StatementSyntax::While(_)));

        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_parses_return_statement() {
        let source = SourceText::new("return value;");
        let mut parser = Parser::new(&source);

        let statement = parser.parse_statement().expect("expected return statement");

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
    fn parser_parses_function_declaration_without_parameters() {
        let source = SourceText::new("function greet() {}");
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
        assert!(declaration.parameters().is_empty());
        assert_eq!(
            declaration.right_parenthesis_token().kind(),
            TokenKind::RightParenthesis
        );
        assert!(declaration.body().statements().is_empty());
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_function_declaration_accepts_statement_in_body() {
        let source = SourceText::new("function identity() { return value; }");
        let mut parser = Parser::new(&source);

        let declaration = parser
            .parse_function_declaration()
            .expect("expected function declaration");

        assert_eq!(declaration.body().statements().len(), 1);
        assert!(matches!(
            declaration.body().statements()[0],
            StatementSyntax::Return(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_function_declaration_skips_trivia() {
        let source = SourceText::new(
            "function /* keyword */ greet /* name */ ( /* open */ ) /* close */ { /* body */ }",
        );
        let mut parser = Parser::new(&source);

        let declaration = parser
            .parse_function_declaration()
            .expect("expected function declaration");

        assert_eq!(declaration.identifier_token().kind(), TokenKind::Identifier);
        assert!(declaration.parameters().is_empty());
        assert!(declaration.body().statements().is_empty());
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_function_declaration_advances_past_body() {
        let source = SourceText::new("function greet() {} next");
        let mut parser = Parser::new(&source);

        parser
            .parse_function_declaration()
            .expect("expected function declaration");

        assert_eq!(parser.current_token().kind(), TokenKind::Identifier);
        assert_eq!(parser.current_token().span(), SourceSpan::new(20, 4));
    }

    #[test]
    fn parser_function_declaration_accepts_single_parameter() {
        let source = SourceText::new("function identity(value) {}");
        let mut parser = Parser::new(&source);

        let declaration = parser
            .parse_function_declaration()
            .expect("expected function declaration");

        assert_eq!(declaration.parameters().len(), 1);
        assert_eq!(
            declaration.parameters()[0].identifier_token().kind(),
            TokenKind::Identifier
        );
        assert_eq!(declaration.parameters()[0].comma_token(), None);
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_function_declaration_accepts_multiple_parameters() {
        let source = SourceText::new("function add(first, second, third) {}");
        let mut parser = Parser::new(&source);

        let declaration = parser
            .parse_function_declaration()
            .expect("expected function declaration");

        assert_eq!(declaration.parameters().len(), 3);

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
        assert_eq!(
            declaration.parameters()[1]
                .comma_token()
                .expect("expected comma")
                .kind(),
            TokenKind::Comma
        );

        assert_eq!(
            declaration.parameters()[2].identifier_token().kind(),
            TokenKind::Identifier
        );
        assert_eq!(declaration.parameters()[2].comma_token(), None);

        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_function_declaration_skips_trivia_between_parameters() {
        let source = SourceText::new(
            "function add( /* first */ first /* comma */, /* second */ second /* close */ ) {}",
        );
        let mut parser = Parser::new(&source);

        let declaration = parser
            .parse_function_declaration()
            .expect("expected function declaration");

        assert_eq!(declaration.parameters().len(), 2);
        assert_eq!(
            declaration.parameters()[0]
                .comma_token()
                .expect("expected comma")
                .kind(),
            TokenKind::Comma
        );
        assert_eq!(declaration.parameters()[1].comma_token(), None);
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_function_declaration_accepts_multiple_body_statements() {
        let source = SourceText::new(
            "function process(value) { let first = value; var second = first; return second; }",
        );
        let mut parser = Parser::new(&source);

        let declaration = parser
            .parse_function_declaration()
            .expect("expected function declaration");

        assert_eq!(declaration.body().statements().len(), 3);

        assert!(matches!(
            declaration.body().statements()[0],
            StatementSyntax::VariableDeclaration(_)
        ));
        assert!(matches!(
            declaration.body().statements()[1],
            StatementSyntax::VariableDeclaration(_)
        ));
        assert!(matches!(
            declaration.body().statements()[2],
            StatementSyntax::Return(_)
        ));

        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_function_declaration_accepts_conditional_in_body() {
        let source = SourceText::new(
            "function choose(value) { if value { return value; } else { return 0; } }",
        );
        let mut parser = Parser::new(&source);

        let declaration = parser
            .parse_function_declaration()
            .expect("expected function declaration");

        assert_eq!(declaration.body().statements().len(), 1);
        assert!(matches!(
            declaration.body().statements()[0],
            StatementSyntax::Conditional(_)
        ));

        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_function_declaration_accepts_while_loop_in_body() {
        let source = SourceText::new(
            "function repeat(value) { while value { value = value - 1; } return value; }",
        );
        let mut parser = Parser::new(&source);

        let declaration = parser
            .parse_function_declaration()
            .expect("expected function declaration");

        assert_eq!(declaration.body().statements().len(), 2);
        assert!(matches!(
            declaration.body().statements()[0],
            StatementSyntax::While(_)
        ));
        assert!(matches!(
            declaration.body().statements()[1],
            StatementSyntax::Return(_)
        ));

        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_function_declaration_accepts_for_loop_in_body() {
        let source = SourceText::new(
            "function iterate(index) { for index = 0; index < 10; index = index + 1 { value = index; } return value; }",
        );
        let mut parser = Parser::new(&source);

        let declaration = parser
            .parse_function_declaration()
            .expect("expected function declaration");

        assert_eq!(declaration.body().statements().len(), 2);
        assert!(matches!(
            declaration.body().statements()[0],
            StatementSyntax::For(_)
        ));
        assert!(matches!(
            declaration.body().statements()[1],
            StatementSyntax::Return(_)
        ));

        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_return_statement_accepts_literal_expression() {
        let source = SourceText::new("return 10;");
        let mut parser = Parser::new(&source);

        let statement = parser.parse_statement().expect("expected return statement");

        let StatementSyntax::Return(return_statement) = statement else {
            panic!("expected return statement");
        };

        assert!(matches!(
            return_statement.expression(),
            ExpressionSyntax::Literal(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_return_statement_advances_to_next_statement() {
        let source = SourceText::new("return value; next;");
        let mut parser = Parser::new(&source);

        let statement = parser.parse_statement().expect("expected return statement");

        assert!(matches!(statement, StatementSyntax::Return(_)));
        assert_eq!(parser.current_token().kind(), TokenKind::Identifier);

        let next_statement = parser
            .parse_statement()
            .expect("expected following expression statement");

        assert!(matches!(next_statement, StatementSyntax::Expression(_)));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_return_statement_accepts_binary_expression() {
        let source = SourceText::new("return first + second * third;");
        let mut parser = Parser::new(&source);

        let statement = parser.parse_statement().expect("expected return statement");

        let StatementSyntax::Return(return_statement) = statement else {
            panic!("expected return statement");
        };

        assert!(matches!(
            return_statement.expression(),
            ExpressionSyntax::Binary(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_return_statement_accepts_call_expression() {
        let source = SourceText::new("return process(value);");
        let mut parser = Parser::new(&source);

        let statement = parser.parse_statement().expect("expected return statement");

        let StatementSyntax::Return(return_statement) = statement else {
            panic!("expected return statement");
        };

        assert!(matches!(
            return_statement.expression(),
            ExpressionSyntax::Call(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_return_statement_accepts_conditional_expression() {
        let source = SourceText::new("return ready ? first : second;");
        let mut parser = Parser::new(&source);

        let statement = parser.parse_statement().expect("expected return statement");

        let StatementSyntax::Return(return_statement) = statement else {
            panic!("expected return statement");
        };

        assert!(matches!(
            return_statement.expression(),
            ExpressionSyntax::Conditional(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_block_accepts_return_statement() {
        let source = SourceText::new("{ return value; }");
        let mut parser = Parser::new(&source);

        let block = parser.parse_block().expect("expected block");

        assert_eq!(block.statements().len(), 1);
        assert!(matches!(block.statements()[0], StatementSyntax::Return(_)));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_block_accepts_multiple_statements_with_return() {
        let source = SourceText::new("{ value; return result; next; }");
        let mut parser = Parser::new(&source);

        let block = parser.parse_block().expect("expected block");

        assert_eq!(block.statements().len(), 3);
        assert!(matches!(
            block.statements()[0],
            StatementSyntax::Expression(_)
        ));
        assert!(matches!(block.statements()[1], StatementSyntax::Return(_)));
        assert!(matches!(
            block.statements()[2],
            StatementSyntax::Expression(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_conditional_statement_accepts_return_statement_in_block() {
        let source = SourceText::new("if ready { return result; }");
        let mut parser = Parser::new(&source);

        let statement = parser
            .parse_statement()
            .expect("expected conditional statement");

        let StatementSyntax::Conditional(conditional) = statement else {
            panic!("expected conditional statement");
        };

        assert_eq!(conditional.then_block().statements().len(), 1);
        assert!(matches!(
            conditional.then_block().statements()[0],
            StatementSyntax::Return(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_while_statement_accepts_return_statement_in_block() {
        let source = SourceText::new("while active { return value; }");
        let mut parser = Parser::new(&source);

        let statement = parser.parse_statement().expect("expected while statement");

        let StatementSyntax::While(while_statement) = statement else {
            panic!("expected while statement");
        };

        assert_eq!(while_statement.block().statements().len(), 1);
        assert!(matches!(
            while_statement.block().statements()[0],
            StatementSyntax::Return(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_for_statement_accepts_return_statement_in_block() {
        let source =
            SourceText::new("for index = 0; index < count; index = index + 1 { return index; }");
        let mut parser = Parser::new(&source);

        let statement = parser.parse_statement().expect("expected for statement");

        let StatementSyntax::For(for_statement) = statement else {
            panic!("expected for statement");
        };

        assert_eq!(for_statement.block().statements().len(), 1);
        assert!(matches!(
            for_statement.block().statements()[0],
            StatementSyntax::Return(_)
        ));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_return_statement_returns_none_without_expression() {
        let source = SourceText::new("return");
        let mut parser = Parser::new(&source);

        assert_eq!(parser.parse_statement(), None);
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_return_statement_rejects_bare_return() {
        let source = SourceText::new("return;");
        let mut parser = Parser::new(&source);

        assert_eq!(parser.parse_statement(), None);
        assert_eq!(parser.current_token().kind(), TokenKind::Semicolon);
    }

    #[test]
    fn parser_return_statement_returns_none_for_incomplete_unary_expression() {
        let source = SourceText::new("return + ;");
        let mut parser = Parser::new(&source);

        assert_eq!(parser.parse_statement(), None);
        assert_eq!(parser.current_token().kind(), TokenKind::Semicolon);
    }

    #[test]
    fn parser_return_statement_returns_none_without_semicolon() {
        let source = SourceText::new("return value");
        let mut parser = Parser::new(&source);

        assert_eq!(parser.parse_statement(), None);
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }

    #[test]
    fn parser_return_statement_returns_none_for_incomplete_binary_expression() {
        let source = SourceText::new("return first + ;");
        let mut parser = Parser::new(&source);

        assert_eq!(parser.parse_statement(), None);
        assert_eq!(parser.current_token().kind(), TokenKind::Semicolon);
    }

    #[test]
    fn parser_return_statement_returns_none_for_incomplete_conditional_expression() {
        let source = SourceText::new("return ready ? : fallback;");
        let mut parser = Parser::new(&source);

        assert_eq!(parser.parse_statement(), None);
        assert_eq!(parser.current_token().kind(), TokenKind::Colon);
    }

    #[test]
    fn parser_return_statement_skips_trivia() {
        let source = SourceText::new("return /* before */ value /* after */ ; /* following */");
        let mut parser = Parser::new(&source);

        let statement = parser.parse_statement().expect("expected return statement");

        assert!(matches!(statement, StatementSyntax::Return(_)));
        assert_eq!(parser.current_token().kind(), TokenKind::EndOfFile);
    }
}
