use crate::compiler::Token;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LiteralExpressionSyntax {
    literal_token: Token,
}

impl LiteralExpressionSyntax {
    pub fn new(literal_token: Token) -> Self {
        Self { literal_token }
    }

    pub fn literal_token(&self) -> Token {
        self.literal_token
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NameExpressionSyntax {
    identifier_token: Token,
}

impl NameExpressionSyntax {
    pub fn new(identifier_token: Token) -> Self {
        Self { identifier_token }
    }

    pub fn identifier_token(&self) -> Token {
        self.identifier_token
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnaryExpressionSyntax {
    operator_token: Token,
    operand: Box<ExpressionSyntax>,
}

impl UnaryExpressionSyntax {
    pub fn new(operator_token: Token, operand: ExpressionSyntax) -> Self {
        Self {
            operator_token,
            operand: Box::new(operand),
        }
    }

    pub fn operator_token(&self) -> Token {
        self.operator_token
    }

    pub fn operand(&self) -> &ExpressionSyntax {
        &self.operand
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParenthesizedExpressionSyntax {
    left_parenthesis_token: Token,
    expression: Box<ExpressionSyntax>,
    right_parenthesis_token: Token,
}

impl ParenthesizedExpressionSyntax {
    pub fn new(
        left_parenthesis_token: Token,
        expression: ExpressionSyntax,
        right_parenthesis_token: Token,
    ) -> Self {
        Self {
            left_parenthesis_token,
            expression: Box::new(expression),
            right_parenthesis_token,
        }
    }

    pub fn left_parenthesis_token(&self) -> Token {
        self.left_parenthesis_token
    }

    pub fn expression(&self) -> &ExpressionSyntax {
        &self.expression
    }

    pub fn right_parenthesis_token(&self) -> Token {
        self.right_parenthesis_token
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExpressionSyntax {
    Literal(LiteralExpressionSyntax),
    Name(NameExpressionSyntax),
    Unary(UnaryExpressionSyntax),
    Parenthesized(ParenthesizedExpressionSyntax),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::TokenKind;
    use crate::source::SourceSpan;

    #[test]
    fn name_expression_preserves_identifier_token() {
        let token = Token::new(TokenKind::Identifier, SourceSpan::new(0, 6));
        let name_expression = NameExpressionSyntax::new(token);

        assert_eq!(name_expression.identifier_token(), token);
    }

    #[test]
    fn expression_syntax_preserves_literal_expression() {
        let token = Token::new(TokenKind::Number, SourceSpan::new(0, 2));
        let literal_expression = LiteralExpressionSyntax::new(token);
        let expression = ExpressionSyntax::Literal(literal_expression);

        assert_eq!(expression, ExpressionSyntax::Literal(literal_expression));
    }

    #[test]
    fn expression_syntax_preserves_name_expression() {
        let token = Token::new(TokenKind::Identifier, SourceSpan::new(0, 6));
        let name_expression = NameExpressionSyntax::new(token);
        let expression = ExpressionSyntax::Name(name_expression);

        assert_eq!(expression, ExpressionSyntax::Name(name_expression));
    }

    #[test]
    fn unary_expression_preserves_operator_and_operand() {
        let operator_token = Token::new(TokenKind::Minus, SourceSpan::new(0, 1));
        let literal_token = Token::new(TokenKind::Number, SourceSpan::new(1, 2));

        let operand = ExpressionSyntax::Literal(LiteralExpressionSyntax::new(literal_token));

        let unary_expression = UnaryExpressionSyntax::new(operator_token, operand.clone());

        assert_eq!(unary_expression.operator_token(), operator_token);
        assert_eq!(unary_expression.operand(), &operand);
    }

    #[test]
    fn expression_syntax_preserves_unary_expression() {
        let operator_token = Token::new(TokenKind::LogicalNot, SourceSpan::new(0, 1));
        let identifier_token = Token::new(TokenKind::Identifier, SourceSpan::new(1, 6));

        let operand = ExpressionSyntax::Name(NameExpressionSyntax::new(identifier_token));

        let unary_expression = UnaryExpressionSyntax::new(operator_token, operand);

        let expression = ExpressionSyntax::Unary(unary_expression.clone());

        assert_eq!(expression, ExpressionSyntax::Unary(unary_expression));
    }

    #[test]
    fn parenthesized_expression_preserves_tokens_and_expression() {
        let left_parenthesis_token = Token::new(TokenKind::LeftParenthesis, SourceSpan::new(0, 1));
        let literal_token = Token::new(TokenKind::Number, SourceSpan::new(1, 2));
        let right_parenthesis_token =
            Token::new(TokenKind::RightParenthesis, SourceSpan::new(3, 1));

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
    fn expression_syntax_preserves_parenthesized_expression() {
        let left_parenthesis_token = Token::new(TokenKind::LeftParenthesis, SourceSpan::new(0, 1));
        let literal_token = Token::new(TokenKind::Number, SourceSpan::new(1, 2));
        let right_parenthesis_token =
            Token::new(TokenKind::RightParenthesis, SourceSpan::new(3, 1));

        let expression = ExpressionSyntax::Literal(LiteralExpressionSyntax::new(literal_token));

        let parenthesized_expression = ParenthesizedExpressionSyntax::new(
            left_parenthesis_token,
            expression,
            right_parenthesis_token,
        );

        let expression = ExpressionSyntax::Parenthesized(parenthesized_expression.clone());

        assert_eq!(
            expression,
            ExpressionSyntax::Parenthesized(parenthesized_expression)
        );
    }
}
