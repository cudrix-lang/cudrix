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
}
