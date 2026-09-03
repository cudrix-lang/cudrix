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
pub struct BinaryExpressionSyntax {
    left: Box<ExpressionSyntax>,
    operator_token: Token,
    right: Box<ExpressionSyntax>,
}

impl BinaryExpressionSyntax {
    pub fn new(left: ExpressionSyntax, operator_token: Token, right: ExpressionSyntax) -> Self {
        Self {
            left: Box::new(left),
            operator_token,
            right: Box::new(right),
        }
    }

    pub fn left(&self) -> &ExpressionSyntax {
        &self.left
    }

    pub fn operator_token(&self) -> Token {
        self.operator_token
    }

    pub fn right(&self) -> &ExpressionSyntax {
        &self.right
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssignmentExpressionSyntax {
    identifier_token: Token,
    assignment_token: Token,
    expression: Box<ExpressionSyntax>,
}

impl AssignmentExpressionSyntax {
    pub fn new(
        identifier_token: Token,
        assignment_token: Token,
        expression: ExpressionSyntax,
    ) -> Self {
        Self {
            identifier_token,
            assignment_token,
            expression: Box::new(expression),
        }
    }

    pub fn identifier_token(&self) -> Token {
        self.identifier_token
    }

    pub fn assignment_token(&self) -> Token {
        self.assignment_token
    }

    pub fn expression(&self) -> &ExpressionSyntax {
        &self.expression
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CallExpressionSyntax {
    callee: Box<ExpressionSyntax>,
    left_parenthesis_token: Token,
    arguments: Vec<ExpressionSyntax>,
    right_parenthesis_token: Token,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemberAccessExpressionSyntax {
    expression: Box<ExpressionSyntax>,
    dot_token: Token,
    member_name_token: Token,
}

impl MemberAccessExpressionSyntax {
    pub fn new(expression: ExpressionSyntax, dot_token: Token, member_name_token: Token) -> Self {
        Self {
            expression: Box::new(expression),
            dot_token,
            member_name_token,
        }
    }

    pub fn expression(&self) -> &ExpressionSyntax {
        &self.expression
    }

    pub fn dot_token(&self) -> &Token {
        &self.dot_token
    }

    pub fn member_name_token(&self) -> &Token {
        &self.member_name_token
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexExpressionSyntax {
    expression: Box<ExpressionSyntax>,
    left_bracket_token: Token,
    index: Box<ExpressionSyntax>,
    right_bracket_token: Token,
}

impl IndexExpressionSyntax {
    pub fn new(
        expression: ExpressionSyntax,
        left_bracket_token: Token,
        index: ExpressionSyntax,
        right_bracket_token: Token,
    ) -> Self {
        Self {
            expression: Box::new(expression),
            left_bracket_token,
            index: Box::new(index),
            right_bracket_token,
        }
    }

    pub fn expression(&self) -> &ExpressionSyntax {
        &self.expression
    }

    pub fn left_bracket_token(&self) -> Token {
        self.left_bracket_token
    }

    pub fn index(&self) -> &ExpressionSyntax {
        &self.index
    }

    pub fn right_bracket_token(&self) -> Token {
        self.right_bracket_token
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArrayLiteralExpressionSyntax {
    left_bracket_token: Token,
    elements: Vec<ExpressionSyntax>,
    right_bracket_token: Token,
}

impl ArrayLiteralExpressionSyntax {
    pub fn new(
        left_bracket_token: Token,
        elements: Vec<ExpressionSyntax>,
        right_bracket_token: Token,
    ) -> Self {
        Self {
            left_bracket_token,
            elements,
            right_bracket_token,
        }
    }

    pub fn left_bracket_token(&self) -> Token {
        self.left_bracket_token
    }

    pub fn elements(&self) -> &[ExpressionSyntax] {
        &self.elements
    }

    pub fn right_bracket_token(&self) -> Token {
        self.right_bracket_token
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectMemberSyntax {
    name_token: Token,
    colon_token: Token,
    value: ExpressionSyntax,
}

impl ObjectMemberSyntax {
    pub fn new(name_token: Token, colon_token: Token, value: ExpressionSyntax) -> Self {
        Self {
            name_token,
            colon_token,
            value,
        }
    }

    pub fn name_token(&self) -> Token {
        self.name_token
    }

    pub fn colon_token(&self) -> Token {
        self.colon_token
    }

    pub fn value(&self) -> &ExpressionSyntax {
        &self.value
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectLiteralExpressionSyntax {
    left_brace_token: Token,
    members: Vec<ObjectMemberSyntax>,
    right_brace_token: Token,
}

impl ObjectLiteralExpressionSyntax {
    pub fn new(
        left_brace_token: Token,
        members: Vec<ObjectMemberSyntax>,
        right_brace_token: Token,
    ) -> Self {
        Self {
            left_brace_token,
            members,
            right_brace_token,
        }
    }

    pub fn left_brace_token(&self) -> Token {
        self.left_brace_token
    }

    pub fn members(&self) -> &[ObjectMemberSyntax] {
        &self.members
    }

    pub fn right_brace_token(&self) -> Token {
        self.right_brace_token
    }
}

impl CallExpressionSyntax {
    pub fn new(
        callee: ExpressionSyntax,
        left_parenthesis_token: Token,
        arguments: Vec<ExpressionSyntax>,
        right_parenthesis_token: Token,
    ) -> Self {
        Self {
            callee: Box::new(callee),
            left_parenthesis_token,
            arguments,
            right_parenthesis_token,
        }
    }

    pub fn callee(&self) -> &ExpressionSyntax {
        &self.callee
    }

    pub fn left_parenthesis_token(&self) -> Token {
        self.left_parenthesis_token
    }

    pub fn arguments(&self) -> &[ExpressionSyntax] {
        &self.arguments
    }

    pub fn right_parenthesis_token(&self) -> Token {
        self.right_parenthesis_token
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
    Binary(BinaryExpressionSyntax),
    Assignment(AssignmentExpressionSyntax),
    MemberAccess(MemberAccessExpressionSyntax),
    Index(IndexExpressionSyntax),
    ArrayLiteral(ArrayLiteralExpressionSyntax),
    ObjectLiteral(ObjectLiteralExpressionSyntax),
    Call(CallExpressionSyntax),
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
    fn binary_expression_preserves_operands_and_operator() {
        let left_token = Token::new(TokenKind::Number, SourceSpan::new(0, 1));
        let operator_token = Token::new(TokenKind::Plus, SourceSpan::new(2, 1));
        let right_token = Token::new(TokenKind::Number, SourceSpan::new(4, 1));

        let left = ExpressionSyntax::Literal(LiteralExpressionSyntax::new(left_token));
        let right = ExpressionSyntax::Literal(LiteralExpressionSyntax::new(right_token));

        let binary_expression =
            BinaryExpressionSyntax::new(left.clone(), operator_token, right.clone());

        assert_eq!(binary_expression.left(), &left);
        assert_eq!(binary_expression.operator_token(), operator_token);
        assert_eq!(binary_expression.right(), &right);
    }

    #[test]
    fn expression_syntax_preserves_binary_expression() {
        let left_token = Token::new(TokenKind::Identifier, SourceSpan::new(0, 4));
        let operator_token = Token::new(TokenKind::Plus, SourceSpan::new(5, 1));
        let right_token = Token::new(TokenKind::Number, SourceSpan::new(7, 1));

        let left = ExpressionSyntax::Name(NameExpressionSyntax::new(left_token));
        let right = ExpressionSyntax::Literal(LiteralExpressionSyntax::new(right_token));

        let binary_expression = BinaryExpressionSyntax::new(left, operator_token, right);

        let expression = ExpressionSyntax::Binary(binary_expression.clone());

        assert_eq!(expression, ExpressionSyntax::Binary(binary_expression));
    }

    #[test]
    fn assignment_expression_preserves_identifier_assignment_token_and_expression() {
        let identifier_token = Token::new(TokenKind::Identifier, SourceSpan::new(0, 6));
        let assignment_token = Token::new(TokenKind::Assignment, SourceSpan::new(7, 1));
        let literal_token = Token::new(TokenKind::Number, SourceSpan::new(9, 2));

        let expression = ExpressionSyntax::Literal(LiteralExpressionSyntax::new(literal_token));

        let assignment_expression =
            AssignmentExpressionSyntax::new(identifier_token, assignment_token, expression.clone());

        assert_eq!(assignment_expression.identifier_token(), identifier_token);
        assert_eq!(assignment_expression.assignment_token(), assignment_token);
        assert_eq!(assignment_expression.expression(), &expression);
    }

    #[test]
    fn expression_syntax_preserves_assignment_expression() {
        let identifier_token = Token::new(TokenKind::Identifier, SourceSpan::new(0, 6));
        let assignment_token = Token::new(TokenKind::Assignment, SourceSpan::new(7, 1));
        let literal_token = Token::new(TokenKind::Number, SourceSpan::new(9, 2));

        let expression = ExpressionSyntax::Literal(LiteralExpressionSyntax::new(literal_token));

        let assignment_expression =
            AssignmentExpressionSyntax::new(identifier_token, assignment_token, expression);

        let expression = ExpressionSyntax::Assignment(assignment_expression.clone());

        assert_eq!(
            expression,
            ExpressionSyntax::Assignment(assignment_expression)
        );
    }

    #[test]
    fn member_access_expression_preserves_expression_dot_and_member_name() {
        let identifier_token = Token::new(TokenKind::Identifier, SourceSpan::new(0, 6));
        let dot_token = Token::new(TokenKind::Dot, SourceSpan::new(6, 1));
        let member_name_token = Token::new(TokenKind::Identifier, SourceSpan::new(7, 6));

        let expression = ExpressionSyntax::Name(NameExpressionSyntax::new(identifier_token));

        let member_access_expression =
            MemberAccessExpressionSyntax::new(expression, dot_token, member_name_token);

        assert_eq!(
            member_access_expression.expression(),
            &ExpressionSyntax::Name(NameExpressionSyntax::new(identifier_token))
        );
        assert_eq!(member_access_expression.dot_token(), &dot_token);
        assert_eq!(
            member_access_expression.member_name_token(),
            &member_name_token
        );
    }

    #[test]
    fn expression_syntax_preserves_member_access_expression() {
        let identifier_token = Token::new(TokenKind::Identifier, SourceSpan::new(0, 6));
        let dot_token = Token::new(TokenKind::Dot, SourceSpan::new(6, 1));
        let member_name_token = Token::new(TokenKind::Identifier, SourceSpan::new(7, 6));

        let member_access_expression = MemberAccessExpressionSyntax::new(
            ExpressionSyntax::Name(NameExpressionSyntax::new(identifier_token)),
            dot_token,
            member_name_token,
        );

        let expression = ExpressionSyntax::MemberAccess(member_access_expression.clone());

        assert_eq!(
            expression,
            ExpressionSyntax::MemberAccess(member_access_expression)
        );
    }

    #[test]
    fn index_expression_preserves_expression_brackets_and_index() {
        let identifier_token = Token::new(TokenKind::Identifier, SourceSpan::new(0, 5));
        let left_bracket_token = Token::new(TokenKind::LeftBracket, SourceSpan::new(5, 1));
        let index_token = Token::new(TokenKind::Number, SourceSpan::new(6, 1));
        let right_bracket_token = Token::new(TokenKind::RightBracket, SourceSpan::new(7, 1));

        let expression = ExpressionSyntax::Name(NameExpressionSyntax::new(identifier_token));
        let index = ExpressionSyntax::Literal(LiteralExpressionSyntax::new(index_token));

        let index_expression = IndexExpressionSyntax::new(
            expression.clone(),
            left_bracket_token,
            index.clone(),
            right_bracket_token,
        );

        assert_eq!(index_expression.expression(), &expression);
        assert_eq!(index_expression.left_bracket_token(), left_bracket_token);
        assert_eq!(index_expression.index(), &index);
        assert_eq!(index_expression.right_bracket_token(), right_bracket_token);
    }

    #[test]
    fn expression_syntax_preserves_index_expression() {
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

        let expression = ExpressionSyntax::Index(index_expression.clone());

        assert_eq!(expression, ExpressionSyntax::Index(index_expression));
    }

    #[test]
    fn array_literal_expression_preserves_brackets_and_elements() {
        let left_bracket_token = Token::new(TokenKind::LeftBracket, SourceSpan::new(0, 1));
        let first_element_token = Token::new(TokenKind::Number, SourceSpan::new(1, 1));
        let second_element_token = Token::new(TokenKind::Identifier, SourceSpan::new(4, 1));
        let right_bracket_token = Token::new(TokenKind::RightBracket, SourceSpan::new(5, 1));

        let first_element =
            ExpressionSyntax::Literal(LiteralExpressionSyntax::new(first_element_token));
        let second_element =
            ExpressionSyntax::Name(NameExpressionSyntax::new(second_element_token));

        let array_literal_expression = ArrayLiteralExpressionSyntax::new(
            left_bracket_token,
            vec![first_element.clone(), second_element.clone()],
            right_bracket_token,
        );

        assert_eq!(
            array_literal_expression.left_bracket_token(),
            left_bracket_token
        );
        assert_eq!(
            array_literal_expression.elements(),
            &[first_element, second_element]
        );
        assert_eq!(
            array_literal_expression.right_bracket_token(),
            right_bracket_token
        );
    }

    #[test]
    fn expression_syntax_preserves_array_literal_expression() {
        let left_bracket_token = Token::new(TokenKind::LeftBracket, SourceSpan::new(0, 1));
        let element_token = Token::new(TokenKind::Number, SourceSpan::new(1, 1));
        let right_bracket_token = Token::new(TokenKind::RightBracket, SourceSpan::new(2, 1));

        let array_literal_expression = ArrayLiteralExpressionSyntax::new(
            left_bracket_token,
            vec![ExpressionSyntax::Literal(LiteralExpressionSyntax::new(
                element_token,
            ))],
            right_bracket_token,
        );

        let expression = ExpressionSyntax::ArrayLiteral(array_literal_expression.clone());

        assert_eq!(
            expression,
            ExpressionSyntax::ArrayLiteral(array_literal_expression)
        );
    }

    #[test]
    fn object_member_preserves_name_colon_and_value() {
        let name_token = Token::new(TokenKind::Identifier, SourceSpan::new(1, 4));
        let colon_token = Token::new(TokenKind::Colon, SourceSpan::new(5, 1));
        let value_token = Token::new(TokenKind::Number, SourceSpan::new(7, 2));

        let value = ExpressionSyntax::Literal(LiteralExpressionSyntax::new(value_token));

        let object_member = ObjectMemberSyntax::new(name_token, colon_token, value.clone());

        assert_eq!(object_member.name_token(), name_token);
        assert_eq!(object_member.colon_token(), colon_token);
        assert_eq!(object_member.value(), &value);
    }

    #[test]
    fn object_literal_expression_preserves_braces_and_members() {
        let left_brace_token = Token::new(TokenKind::LeftBrace, SourceSpan::new(0, 1));
        let name_token = Token::new(TokenKind::Identifier, SourceSpan::new(1, 4));
        let colon_token = Token::new(TokenKind::Colon, SourceSpan::new(5, 1));
        let value_token = Token::new(TokenKind::Number, SourceSpan::new(7, 1));
        let right_brace_token = Token::new(TokenKind::RightBrace, SourceSpan::new(8, 1));

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
    fn call_expression_preserves_callee_parentheses_and_arguments() {
        let identifier_token = Token::new(TokenKind::Identifier, SourceSpan::new(0, 3));
        let left_parenthesis_token = Token::new(TokenKind::LeftParenthesis, SourceSpan::new(3, 1));
        let argument_token = Token::new(TokenKind::Number, SourceSpan::new(4, 2));
        let right_parenthesis_token =
            Token::new(TokenKind::RightParenthesis, SourceSpan::new(6, 1));

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
    }

    #[test]
    fn expression_syntax_preserves_call_expression() {
        let identifier_token = Token::new(TokenKind::Identifier, SourceSpan::new(0, 3));
        let left_parenthesis_token = Token::new(TokenKind::LeftParenthesis, SourceSpan::new(3, 1));
        let right_parenthesis_token =
            Token::new(TokenKind::RightParenthesis, SourceSpan::new(4, 1));

        let callee = ExpressionSyntax::Name(NameExpressionSyntax::new(identifier_token));

        let call_expression = CallExpressionSyntax::new(
            callee,
            left_parenthesis_token,
            Vec::new(),
            right_parenthesis_token,
        );

        let expression = ExpressionSyntax::Call(call_expression.clone());

        assert_eq!(expression, ExpressionSyntax::Call(call_expression));
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
