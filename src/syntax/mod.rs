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
pub struct ConditionalExpressionSyntax {
    condition: Box<ExpressionSyntax>,
    question_token: Token,
    when_true: Box<ExpressionSyntax>,
    colon_token: Token,
    when_false: Box<ExpressionSyntax>,
}

impl ConditionalExpressionSyntax {
    pub fn new(
        condition: ExpressionSyntax,
        question_token: Token,
        when_true: ExpressionSyntax,
        colon_token: Token,
        when_false: ExpressionSyntax,
    ) -> Self {
        Self {
            condition: Box::new(condition),
            question_token,
            when_true: Box::new(when_true),
            colon_token,
            when_false: Box::new(when_false),
        }
    }

    pub fn condition(&self) -> &ExpressionSyntax {
        &self.condition
    }

    pub fn question_token(&self) -> Token {
        self.question_token
    }

    pub fn when_true(&self) -> &ExpressionSyntax {
        &self.when_true
    }

    pub fn colon_token(&self) -> Token {
        self.colon_token
    }

    pub fn when_false(&self) -> &ExpressionSyntax {
        &self.when_false
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
    Conditional(ConditionalExpressionSyntax),
    Assignment(AssignmentExpressionSyntax),
    MemberAccess(MemberAccessExpressionSyntax),
    Index(IndexExpressionSyntax),
    ArrayLiteral(ArrayLiteralExpressionSyntax),
    ObjectLiteral(ObjectLiteralExpressionSyntax),
    Call(CallExpressionSyntax),
    Parenthesized(ParenthesizedExpressionSyntax),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpressionStatementSyntax {
    expression: ExpressionSyntax,
    semicolon_token: Token,
}

impl ExpressionStatementSyntax {
    pub fn new(expression: ExpressionSyntax, semicolon_token: Token) -> Self {
        Self {
            expression,
            semicolon_token,
        }
    }

    pub fn expression(&self) -> &ExpressionSyntax {
        &self.expression
    }

    pub fn semicolon_token(&self) -> Token {
        self.semicolon_token
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariableDeclarationStatementSyntax {
    declaration_keyword: Token,
    identifier_token: Token,
    assignment_token: Token,
    initializer: ExpressionSyntax,
    semicolon_token: Token,
}

impl VariableDeclarationStatementSyntax {
    pub fn new(
        declaration_keyword: Token,
        identifier_token: Token,
        assignment_token: Token,
        initializer: ExpressionSyntax,
        semicolon_token: Token,
    ) -> Self {
        Self {
            declaration_keyword,
            identifier_token,
            assignment_token,
            initializer,
            semicolon_token,
        }
    }

    pub fn declaration_keyword(&self) -> Token {
        self.declaration_keyword
    }

    pub fn identifier_token(&self) -> Token {
        self.identifier_token
    }

    pub fn assignment_token(&self) -> Token {
        self.assignment_token
    }

    pub fn initializer(&self) -> &ExpressionSyntax {
        &self.initializer
    }

    pub fn semicolon_token(&self) -> Token {
        self.semicolon_token
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReturnStatementSyntax {
    return_keyword: Token,
    expression: ExpressionSyntax,
    semicolon_token: Token,
}

impl ReturnStatementSyntax {
    pub fn new(
        return_keyword: Token,
        expression: ExpressionSyntax,
        semicolon_token: Token,
    ) -> Self {
        Self {
            return_keyword,
            expression,
            semicolon_token,
        }
    }

    pub fn return_keyword(&self) -> Token {
        self.return_keyword
    }

    pub fn expression(&self) -> &ExpressionSyntax {
        &self.expression
    }

    pub fn semicolon_token(&self) -> Token {
        self.semicolon_token
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElseIfClauseSyntax {
    elseif_keyword: Token,
    condition: ExpressionSyntax,
    block: BlockSyntax,
}

impl ElseIfClauseSyntax {
    pub fn new(elseif_keyword: Token, condition: ExpressionSyntax, block: BlockSyntax) -> Self {
        Self {
            elseif_keyword,
            condition,
            block,
        }
    }

    pub fn elseif_keyword(&self) -> Token {
        self.elseif_keyword
    }

    pub fn condition(&self) -> &ExpressionSyntax {
        &self.condition
    }

    pub fn block(&self) -> &BlockSyntax {
        &self.block
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElseClauseSyntax {
    else_keyword: Token,
    block: BlockSyntax,
}

impl ElseClauseSyntax {
    pub fn new(else_keyword: Token, block: BlockSyntax) -> Self {
        Self {
            else_keyword,
            block,
        }
    }

    pub fn else_keyword(&self) -> Token {
        self.else_keyword
    }

    pub fn block(&self) -> &BlockSyntax {
        &self.block
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConditionalStatementSyntax {
    if_keyword: Token,
    condition: ExpressionSyntax,
    then_block: BlockSyntax,
    elseif_clauses: Vec<ElseIfClauseSyntax>,
    else_clause: Option<ElseClauseSyntax>,
}

impl ConditionalStatementSyntax {
    pub fn new(
        if_keyword: Token,
        condition: ExpressionSyntax,
        then_block: BlockSyntax,
        elseif_clauses: Vec<ElseIfClauseSyntax>,
        else_clause: Option<ElseClauseSyntax>,
    ) -> Self {
        Self {
            if_keyword,
            condition,
            then_block,
            elseif_clauses,
            else_clause,
        }
    }

    pub fn if_keyword(&self) -> Token {
        self.if_keyword
    }

    pub fn condition(&self) -> &ExpressionSyntax {
        &self.condition
    }

    pub fn then_block(&self) -> &BlockSyntax {
        &self.then_block
    }

    pub fn elseif_clauses(&self) -> &[ElseIfClauseSyntax] {
        &self.elseif_clauses
    }

    pub fn else_clause(&self) -> Option<&ElseClauseSyntax> {
        self.else_clause.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhileStatementSyntax {
    while_keyword: Token,
    condition: ExpressionSyntax,
    block: BlockSyntax,
}

impl WhileStatementSyntax {
    pub fn new(while_keyword: Token, condition: ExpressionSyntax, block: BlockSyntax) -> Self {
        Self {
            while_keyword,
            condition,
            block,
        }
    }

    pub fn while_keyword(&self) -> Token {
        self.while_keyword
    }

    pub fn condition(&self) -> &ExpressionSyntax {
        &self.condition
    }

    pub fn block(&self) -> &BlockSyntax {
        &self.block
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForStatementSyntax {
    for_keyword: Token,
    initializer: ExpressionSyntax,
    first_semicolon_token: Token,
    condition: ExpressionSyntax,
    second_semicolon_token: Token,
    update: ExpressionSyntax,
    block: BlockSyntax,
}

impl ForStatementSyntax {
    pub fn new(
        for_keyword: Token,
        initializer: ExpressionSyntax,
        first_semicolon_token: Token,
        condition: ExpressionSyntax,
        second_semicolon_token: Token,
        update: ExpressionSyntax,
        block: BlockSyntax,
    ) -> Self {
        Self {
            for_keyword,
            initializer,
            first_semicolon_token,
            condition,
            second_semicolon_token,
            update,
            block,
        }
    }

    pub fn for_keyword(&self) -> Token {
        self.for_keyword
    }

    pub fn initializer(&self) -> &ExpressionSyntax {
        &self.initializer
    }

    pub fn first_semicolon_token(&self) -> Token {
        self.first_semicolon_token
    }

    pub fn condition(&self) -> &ExpressionSyntax {
        &self.condition
    }

    pub fn second_semicolon_token(&self) -> Token {
        self.second_semicolon_token
    }

    pub fn update(&self) -> &ExpressionSyntax {
        &self.update
    }

    pub fn block(&self) -> &BlockSyntax {
        &self.block
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StatementSyntax {
    Expression(ExpressionStatementSyntax),
    VariableDeclaration(VariableDeclarationStatementSyntax),
    Return(ReturnStatementSyntax),
    Conditional(ConditionalStatementSyntax),
    While(WhileStatementSyntax),
    For(ForStatementSyntax),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockSyntax {
    left_brace_token: Token,
    statements: Vec<StatementSyntax>,
    right_brace_token: Token,
}

impl BlockSyntax {
    pub fn new(
        left_brace_token: Token,
        statements: Vec<StatementSyntax>,
        right_brace_token: Token,
    ) -> Self {
        Self {
            left_brace_token,
            statements,
            right_brace_token,
        }
    }

    pub fn left_brace_token(&self) -> Token {
        self.left_brace_token
    }

    pub fn statements(&self) -> &[StatementSyntax] {
        &self.statements
    }

    pub fn right_brace_token(&self) -> Token {
        self.right_brace_token
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionParameterSyntax {
    identifier_token: Token,
    comma_token: Option<Token>,
}

impl FunctionParameterSyntax {
    pub fn new(identifier_token: Token, comma_token: Option<Token>) -> Self {
        Self {
            identifier_token,
            comma_token,
        }
    }

    pub fn identifier_token(&self) -> Token {
        self.identifier_token
    }

    pub fn comma_token(&self) -> Option<Token> {
        self.comma_token
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModuleDeclarationSyntax {
    module_keyword: Token,
    identifier_token: Token,
    semicolon_token: Token,
}

impl ModuleDeclarationSyntax {
    pub fn new(module_keyword: Token, identifier_token: Token, semicolon_token: Token) -> Self {
        Self {
            module_keyword,
            identifier_token,
            semicolon_token,
        }
    }

    pub fn module_keyword(&self) -> Token {
        self.module_keyword
    }

    pub fn identifier_token(&self) -> Token {
        self.identifier_token
    }

    pub fn semicolon_token(&self) -> Token {
        self.semicolon_token
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QualifiedImportNameSegmentSyntax {
    dot_token: Token,
    identifier_token: Token,
}

impl QualifiedImportNameSegmentSyntax {
    pub fn new(dot_token: Token, identifier_token: Token) -> Self {
        Self {
            dot_token,
            identifier_token,
        }
    }

    pub fn dot_token(&self) -> Token {
        self.dot_token
    }

    pub fn identifier_token(&self) -> Token {
        self.identifier_token
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectiveImportMemberSyntax {
    identifier_token: Token,
    comma_token: Option<Token>,
}

impl SelectiveImportMemberSyntax {
    pub fn new(identifier_token: Token, comma_token: Option<Token>) -> Self {
        Self {
            identifier_token,
            comma_token,
        }
    }

    pub fn identifier_token(&self) -> Token {
        self.identifier_token
    }

    pub fn comma_token(&self) -> Option<Token> {
        self.comma_token
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectiveImportClauseSyntax {
    dot_token: Token,
    left_brace_token: Token,
    members: Vec<SelectiveImportMemberSyntax>,
    right_brace_token: Token,
}

impl SelectiveImportClauseSyntax {
    pub fn new(
        dot_token: Token,
        left_brace_token: Token,
        members: Vec<SelectiveImportMemberSyntax>,
        right_brace_token: Token,
    ) -> Self {
        Self {
            dot_token,
            left_brace_token,
            members,
            right_brace_token,
        }
    }

    pub fn dot_token(&self) -> Token {
        self.dot_token
    }

    pub fn left_brace_token(&self) -> Token {
        self.left_brace_token
    }

    pub fn members(&self) -> &[SelectiveImportMemberSyntax] {
        &self.members
    }

    pub fn right_brace_token(&self) -> Token {
        self.right_brace_token
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportAliasSyntax {
    as_keyword: Token,
    identifier_token: Token,
}

impl ImportAliasSyntax {
    pub fn new(as_keyword: Token, identifier_token: Token) -> Self {
        Self {
            as_keyword,
            identifier_token,
        }
    }

    pub fn as_keyword(&self) -> Token {
        self.as_keyword
    }

    pub fn identifier_token(&self) -> Token {
        self.identifier_token
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportDeclarationSyntax {
    import_keyword: Token,
    identifier_token: Token,
    qualified_name_segments: Vec<QualifiedImportNameSegmentSyntax>,
    alias: Option<ImportAliasSyntax>,
    selective_import_clause: Option<SelectiveImportClauseSyntax>,
    semicolon_token: Token,
}

impl ImportDeclarationSyntax {
    pub fn new(
        import_keyword: Token,
        identifier_token: Token,
        qualified_name_segments: Vec<QualifiedImportNameSegmentSyntax>,
        alias: Option<ImportAliasSyntax>,
        selective_import_clause: Option<SelectiveImportClauseSyntax>,
        semicolon_token: Token,
    ) -> Self {
        Self {
            import_keyword,
            identifier_token,
            qualified_name_segments,
            alias,
            selective_import_clause,
            semicolon_token,
        }
    }

    pub fn import_keyword(&self) -> Token {
        self.import_keyword
    }

    pub fn identifier_token(&self) -> Token {
        self.identifier_token
    }

    pub fn qualified_name_segments(&self) -> &[QualifiedImportNameSegmentSyntax] {
        &self.qualified_name_segments
    }

    pub fn alias(&self) -> Option<&ImportAliasSyntax> {
        self.alias.as_ref()
    }

    pub fn selective_import_clause(&self) -> Option<&SelectiveImportClauseSyntax> {
        self.selective_import_clause.as_ref()
    }

    pub fn semicolon_token(&self) -> Token {
        self.semicolon_token
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionDeclarationSyntax {
    function_keyword: Token,
    identifier_token: Token,
    left_parenthesis_token: Token,
    parameters: Vec<FunctionParameterSyntax>,
    right_parenthesis_token: Token,
    body: BlockSyntax,
}

impl FunctionDeclarationSyntax {
    pub fn new(
        function_keyword: Token,
        identifier_token: Token,
        left_parenthesis_token: Token,
        parameters: Vec<FunctionParameterSyntax>,
        right_parenthesis_token: Token,
        body: BlockSyntax,
    ) -> Self {
        Self {
            function_keyword,
            identifier_token,
            left_parenthesis_token,
            parameters,
            right_parenthesis_token,
            body,
        }
    }

    pub fn function_keyword(&self) -> Token {
        self.function_keyword
    }

    pub fn identifier_token(&self) -> Token {
        self.identifier_token
    }

    pub fn left_parenthesis_token(&self) -> Token {
        self.left_parenthesis_token
    }

    pub fn parameters(&self) -> &[FunctionParameterSyntax] {
        &self.parameters
    }

    pub fn right_parenthesis_token(&self) -> Token {
        self.right_parenthesis_token
    }

    pub fn body(&self) -> &BlockSyntax {
        &self.body
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompilationUnitSyntax {
    module_declaration: Option<ModuleDeclarationSyntax>,
    import_declarations: Vec<ImportDeclarationSyntax>,
    function_declarations: Vec<FunctionDeclarationSyntax>,
    end_of_file_token: Token,
}

impl CompilationUnitSyntax {
    pub fn new(
        module_declaration: Option<ModuleDeclarationSyntax>,
        import_declarations: Vec<ImportDeclarationSyntax>,
        function_declarations: Vec<FunctionDeclarationSyntax>,
        end_of_file_token: Token,
    ) -> Self {
        Self {
            module_declaration,
            import_declarations,
            function_declarations,
            end_of_file_token,
        }
    }

    pub fn module_declaration(&self) -> Option<&ModuleDeclarationSyntax> {
        self.module_declaration.as_ref()
    }

    pub fn import_declarations(&self) -> &[ImportDeclarationSyntax] {
        &self.import_declarations
    }

    pub fn function_declarations(&self) -> &[FunctionDeclarationSyntax] {
        &self.function_declarations
    }

    pub fn end_of_file_token(&self) -> Token {
        self.end_of_file_token
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
    fn conditional_expression_preserves_condition_tokens_and_branches() {
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
    }

    #[test]
    fn expression_syntax_preserves_conditional_expression() {
        let condition_token = Token::new(TokenKind::Identifier, SourceSpan::new(0, 5));
        let question_token = Token::new(TokenKind::QuestionMark, SourceSpan::new(6, 1));
        let when_true_token = Token::new(TokenKind::Number, SourceSpan::new(8, 1));
        let colon_token = Token::new(TokenKind::Colon, SourceSpan::new(10, 1));
        let when_false_token = Token::new(TokenKind::Number, SourceSpan::new(12, 1));

        let conditional_expression = ConditionalExpressionSyntax::new(
            ExpressionSyntax::Name(NameExpressionSyntax::new(condition_token)),
            question_token,
            ExpressionSyntax::Literal(LiteralExpressionSyntax::new(when_true_token)),
            colon_token,
            ExpressionSyntax::Literal(LiteralExpressionSyntax::new(when_false_token)),
        );

        let expression = ExpressionSyntax::Conditional(conditional_expression.clone());

        assert_eq!(
            expression,
            ExpressionSyntax::Conditional(conditional_expression)
        );
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

    #[test]
    fn expression_statement_preserves_expression_and_semicolon() {
        let literal_token = Token::new(TokenKind::Number, SourceSpan::new(0, 2));
        let semicolon_token = Token::new(TokenKind::Semicolon, SourceSpan::new(2, 1));

        let expression = ExpressionSyntax::Literal(LiteralExpressionSyntax::new(literal_token));

        let expression_statement =
            ExpressionStatementSyntax::new(expression.clone(), semicolon_token);

        assert_eq!(expression_statement.expression(), &expression);
        assert_eq!(expression_statement.semicolon_token(), semicolon_token);
    }

    #[test]
    fn statement_syntax_preserves_expression_statement() {
        let identifier_token = Token::new(TokenKind::Identifier, SourceSpan::new(0, 6));
        let semicolon_token = Token::new(TokenKind::Semicolon, SourceSpan::new(6, 1));

        let expression = ExpressionSyntax::Name(NameExpressionSyntax::new(identifier_token));
        let expression_statement = ExpressionStatementSyntax::new(expression, semicolon_token);

        let statement = StatementSyntax::Expression(expression_statement.clone());

        assert_eq!(statement, StatementSyntax::Expression(expression_statement));
    }

    #[test]
    fn function_parameter_preserves_identifier_and_comma_tokens() {
        let identifier_token = Token::new(TokenKind::Identifier, SourceSpan::new(0, 5));
        let comma_token = Token::new(TokenKind::Comma, SourceSpan::new(5, 1));

        let parameter = FunctionParameterSyntax::new(identifier_token, Some(comma_token));

        assert_eq!(parameter.identifier_token(), identifier_token);
        assert_eq!(parameter.comma_token(), Some(comma_token));

        let final_identifier_token = Token::new(TokenKind::Identifier, SourceSpan::new(7, 6));
        let final_parameter = FunctionParameterSyntax::new(final_identifier_token, None);

        assert_eq!(final_parameter.identifier_token(), final_identifier_token);
        assert_eq!(final_parameter.comma_token(), None);
    }

    #[test]
    fn module_declaration_preserves_tokens() {
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
    fn import_alias_preserves_as_keyword_and_identifier_token() {
        let as_keyword = Token::new(TokenKind::AsKeyword, SourceSpan::new(19, 2));
        let identifier_token = Token::new(TokenKind::Identifier, SourceSpan::new(22, 2));

        let alias = ImportAliasSyntax::new(as_keyword, identifier_token);

        assert_eq!(alias.as_keyword(), as_keyword);
        assert_eq!(alias.identifier_token(), identifier_token);
    }

    #[test]
    fn import_declaration_preserves_alias() {
        let import_keyword = Token::new(TokenKind::ImportKeyword, SourceSpan::new(0, 6));
        let identifier_token = Token::new(TokenKind::Identifier, SourceSpan::new(7, 8));
        let dot_token = Token::new(TokenKind::Dot, SourceSpan::new(15, 1));
        let qualified_identifier_token = Token::new(TokenKind::Identifier, SourceSpan::new(16, 2));
        let as_keyword = Token::new(TokenKind::AsKeyword, SourceSpan::new(19, 2));
        let alias_identifier_token = Token::new(TokenKind::Identifier, SourceSpan::new(22, 2));
        let semicolon_token = Token::new(TokenKind::Semicolon, SourceSpan::new(24, 1));

        let qualified_name_segments = vec![QualifiedImportNameSegmentSyntax::new(
            dot_token,
            qualified_identifier_token,
        )];

        let alias = ImportAliasSyntax::new(as_keyword, alias_identifier_token);

        let declaration = ImportDeclarationSyntax::new(
            import_keyword,
            identifier_token,
            qualified_name_segments,
            Some(alias),
            None,
            semicolon_token,
        );

        let alias = declaration.alias().expect("expected import alias");

        assert_eq!(alias.as_keyword(), as_keyword);
        assert_eq!(alias.identifier_token(), alias_identifier_token);
    }

    #[test]
    fn import_declaration_preserves_tokens() {
        let import_keyword = Token::new(TokenKind::ImportKeyword, SourceSpan::new(0, 6));
        let identifier_token = Token::new(TokenKind::Identifier, SourceSpan::new(7, 9));
        let semicolon_token = Token::new(TokenKind::Semicolon, SourceSpan::new(16, 1));

        let declaration = ImportDeclarationSyntax::new(
            import_keyword,
            identifier_token,
            Vec::new(),
            None,
            None,
            semicolon_token,
        );

        assert_eq!(declaration.import_keyword(), import_keyword);
        assert_eq!(declaration.identifier_token(), identifier_token);
        assert!(declaration.alias().is_none());
        assert_eq!(declaration.semicolon_token(), semicolon_token);
    }

    #[test]
    fn function_declaration_preserves_tokens_parameters_and_body() {
        let function_keyword = Token::new(TokenKind::FunctionKeyword, SourceSpan::new(0, 8));
        let identifier_token = Token::new(TokenKind::Identifier, SourceSpan::new(9, 3));
        let left_parenthesis_token = Token::new(TokenKind::LeftParenthesis, SourceSpan::new(12, 1));
        let first_parameter_token = Token::new(TokenKind::Identifier, SourceSpan::new(13, 5));
        let comma_token = Token::new(TokenKind::Comma, SourceSpan::new(18, 1));
        let second_parameter_token = Token::new(TokenKind::Identifier, SourceSpan::new(20, 6));
        let right_parenthesis_token =
            Token::new(TokenKind::RightParenthesis, SourceSpan::new(26, 1));
        let left_brace_token = Token::new(TokenKind::LeftBrace, SourceSpan::new(28, 1));
        let right_brace_token = Token::new(TokenKind::RightBrace, SourceSpan::new(29, 1));

        let parameters = vec![
            FunctionParameterSyntax::new(first_parameter_token, Some(comma_token)),
            FunctionParameterSyntax::new(second_parameter_token, None),
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
            first_parameter_token
        );
        assert_eq!(declaration.parameters()[0].comma_token(), Some(comma_token));
        assert_eq!(
            declaration.parameters()[1].identifier_token(),
            second_parameter_token
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
    fn variable_declaration_statement_preserves_declaration_tokens_and_initializer() {
        let declaration_keyword = Token::new(TokenKind::LetKeyword, SourceSpan::new(0, 3));
        let identifier_token = Token::new(TokenKind::Identifier, SourceSpan::new(4, 5));
        let assignment_token = Token::new(TokenKind::Assignment, SourceSpan::new(10, 1));
        let initializer_token = Token::new(TokenKind::Number, SourceSpan::new(12, 1));
        let semicolon_token = Token::new(TokenKind::Semicolon, SourceSpan::new(13, 1));

        let initializer =
            ExpressionSyntax::Literal(LiteralExpressionSyntax::new(initializer_token));

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
    fn statement_syntax_preserves_variable_declaration_statement() {
        let declaration_keyword = Token::new(TokenKind::VarKeyword, SourceSpan::new(0, 3));
        let identifier_token = Token::new(TokenKind::Identifier, SourceSpan::new(4, 5));
        let assignment_token = Token::new(TokenKind::Assignment, SourceSpan::new(10, 1));
        let initializer_token = Token::new(TokenKind::Number, SourceSpan::new(12, 2));
        let semicolon_token = Token::new(TokenKind::Semicolon, SourceSpan::new(14, 1));

        let initializer =
            ExpressionSyntax::Literal(LiteralExpressionSyntax::new(initializer_token));

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
    fn return_statement_preserves_tokens_and_expression() {
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
    fn statement_syntax_preserves_return_statement() {
        let return_keyword = Token::new(TokenKind::ReturnKeyword, SourceSpan::new(0, 6));
        let expression_token = Token::new(TokenKind::Identifier, SourceSpan::new(7, 5));
        let semicolon_token = Token::new(TokenKind::Semicolon, SourceSpan::new(12, 1));

        let expression = ExpressionSyntax::Name(NameExpressionSyntax::new(expression_token));

        let return_statement =
            ReturnStatementSyntax::new(return_keyword, expression, semicolon_token);

        let statement = StatementSyntax::Return(return_statement.clone());

        assert_eq!(statement, StatementSyntax::Return(return_statement));
    }

    #[test]
    fn block_syntax_preserves_braces_and_statements() {
        let left_brace_token = Token::new(TokenKind::LeftBrace, SourceSpan::new(0, 1));
        let identifier_token = Token::new(TokenKind::Identifier, SourceSpan::new(2, 6));
        let semicolon_token = Token::new(TokenKind::Semicolon, SourceSpan::new(8, 1));
        let right_brace_token = Token::new(TokenKind::RightBrace, SourceSpan::new(10, 1));

        let expression = ExpressionSyntax::Name(NameExpressionSyntax::new(identifier_token));
        let statement = StatementSyntax::Expression(ExpressionStatementSyntax::new(
            expression,
            semicolon_token,
        ));

        let block = BlockSyntax::new(left_brace_token, vec![statement.clone()], right_brace_token);

        assert_eq!(block.left_brace_token(), left_brace_token);
        assert_eq!(block.statements(), &[statement]);
        assert_eq!(block.right_brace_token(), right_brace_token);
    }

    #[test]
    fn block_syntax_preserves_empty_statement_list() {
        let left_brace_token = Token::new(TokenKind::LeftBrace, SourceSpan::new(0, 1));
        let right_brace_token = Token::new(TokenKind::RightBrace, SourceSpan::new(1, 1));

        let block = BlockSyntax::new(left_brace_token, Vec::new(), right_brace_token);

        assert_eq!(block.left_brace_token(), left_brace_token);
        assert!(block.statements().is_empty());
        assert_eq!(block.right_brace_token(), right_brace_token);
    }

    #[test]
    fn else_clause_preserves_keyword_and_block() {
        let else_keyword = Token::new(TokenKind::ElseKeyword, SourceSpan::new(0, 4));
        let left_brace_token = Token::new(TokenKind::LeftBrace, SourceSpan::new(5, 1));
        let right_brace_token = Token::new(TokenKind::RightBrace, SourceSpan::new(6, 1));

        let block = BlockSyntax::new(left_brace_token, Vec::new(), right_brace_token);
        let else_clause = ElseClauseSyntax::new(else_keyword, block.clone());

        assert_eq!(else_clause.else_keyword(), else_keyword);
        assert_eq!(else_clause.block(), &block);
    }

    #[test]
    fn elseif_clause_preserves_keyword_condition_and_block() {
        let elseif_keyword = Token::new(TokenKind::ElseIfKeyword, SourceSpan::new(0, 6));
        let condition_token = Token::new(TokenKind::Identifier, SourceSpan::new(7, 9));
        let left_brace_token = Token::new(TokenKind::LeftBrace, SourceSpan::new(17, 1));
        let right_brace_token = Token::new(TokenKind::RightBrace, SourceSpan::new(18, 1));

        let condition = ExpressionSyntax::Name(NameExpressionSyntax::new(condition_token));
        let block = BlockSyntax::new(left_brace_token, Vec::new(), right_brace_token);

        let elseif_clause =
            ElseIfClauseSyntax::new(elseif_keyword, condition.clone(), block.clone());

        assert_eq!(elseif_clause.elseif_keyword(), elseif_keyword);
        assert_eq!(elseif_clause.condition(), &condition);
        assert_eq!(elseif_clause.block(), &block);
    }

    #[test]
    fn conditional_statement_preserves_ordered_elseif_clauses() {
        let if_keyword = Token::new(TokenKind::IfKeyword, SourceSpan::new(0, 2));
        let condition_token = Token::new(TokenKind::Identifier, SourceSpan::new(3, 5));
        let then_left_brace = Token::new(TokenKind::LeftBrace, SourceSpan::new(9, 1));
        let then_right_brace = Token::new(TokenKind::RightBrace, SourceSpan::new(10, 1));

        let first_elseif_keyword = Token::new(TokenKind::ElseIfKeyword, SourceSpan::new(12, 6));
        let first_condition_token = Token::new(TokenKind::Identifier, SourceSpan::new(19, 5));
        let first_left_brace = Token::new(TokenKind::LeftBrace, SourceSpan::new(25, 1));
        let first_right_brace = Token::new(TokenKind::RightBrace, SourceSpan::new(26, 1));

        let second_elseif_keyword = Token::new(TokenKind::ElseIfKeyword, SourceSpan::new(28, 6));
        let second_condition_token = Token::new(TokenKind::Identifier, SourceSpan::new(35, 6));
        let second_left_brace = Token::new(TokenKind::LeftBrace, SourceSpan::new(42, 1));
        let second_right_brace = Token::new(TokenKind::RightBrace, SourceSpan::new(43, 1));

        let condition = ExpressionSyntax::Name(NameExpressionSyntax::new(condition_token));
        let then_block = BlockSyntax::new(then_left_brace, Vec::new(), then_right_brace);

        let first_elseif_clause = ElseIfClauseSyntax::new(
            first_elseif_keyword,
            ExpressionSyntax::Name(NameExpressionSyntax::new(first_condition_token)),
            BlockSyntax::new(first_left_brace, Vec::new(), first_right_brace),
        );

        let second_elseif_clause = ElseIfClauseSyntax::new(
            second_elseif_keyword,
            ExpressionSyntax::Name(NameExpressionSyntax::new(second_condition_token)),
            BlockSyntax::new(second_left_brace, Vec::new(), second_right_brace),
        );

        let conditional_statement = ConditionalStatementSyntax::new(
            if_keyword,
            condition,
            then_block,
            vec![first_elseif_clause.clone(), second_elseif_clause.clone()],
            None,
        );

        assert_eq!(
            conditional_statement.elseif_clauses(),
            &[first_elseif_clause, second_elseif_clause]
        );
    }

    #[test]
    fn conditional_statement_preserves_if_condition_blocks_and_else_clause() {
        let if_keyword = Token::new(TokenKind::IfKeyword, SourceSpan::new(0, 2));
        let condition_token = Token::new(TokenKind::Identifier, SourceSpan::new(3, 5));

        let then_left_brace = Token::new(TokenKind::LeftBrace, SourceSpan::new(9, 1));
        let then_right_brace = Token::new(TokenKind::RightBrace, SourceSpan::new(10, 1));

        let else_keyword = Token::new(TokenKind::ElseKeyword, SourceSpan::new(12, 4));
        let else_left_brace = Token::new(TokenKind::LeftBrace, SourceSpan::new(17, 1));
        let else_right_brace = Token::new(TokenKind::RightBrace, SourceSpan::new(18, 1));

        let condition = ExpressionSyntax::Name(NameExpressionSyntax::new(condition_token));

        let then_block = BlockSyntax::new(then_left_brace, Vec::new(), then_right_brace);
        let else_block = BlockSyntax::new(else_left_brace, Vec::new(), else_right_brace);
        let else_clause = ElseClauseSyntax::new(else_keyword, else_block);

        let conditional_statement = ConditionalStatementSyntax::new(
            if_keyword,
            condition.clone(),
            then_block.clone(),
            Vec::new(),
            Some(else_clause.clone()),
        );

        assert_eq!(conditional_statement.if_keyword(), if_keyword);
        assert_eq!(conditional_statement.condition(), &condition);
        assert_eq!(conditional_statement.then_block(), &then_block);
        assert!(conditional_statement.elseif_clauses().is_empty());
        assert_eq!(conditional_statement.else_clause(), Some(&else_clause));
    }

    #[test]
    fn statement_syntax_preserves_conditional_statement() {
        let if_keyword = Token::new(TokenKind::IfKeyword, SourceSpan::new(0, 2));
        let condition_token = Token::new(TokenKind::Identifier, SourceSpan::new(3, 5));
        let left_brace_token = Token::new(TokenKind::LeftBrace, SourceSpan::new(9, 1));
        let right_brace_token = Token::new(TokenKind::RightBrace, SourceSpan::new(10, 1));

        let condition = ExpressionSyntax::Name(NameExpressionSyntax::new(condition_token));
        let then_block = BlockSyntax::new(left_brace_token, Vec::new(), right_brace_token);

        let conditional_statement =
            ConditionalStatementSyntax::new(if_keyword, condition, then_block, Vec::new(), None);

        let statement = StatementSyntax::Conditional(conditional_statement.clone());

        assert_eq!(
            statement,
            StatementSyntax::Conditional(conditional_statement)
        );
    }

    #[test]
    fn while_statement_preserves_keyword_condition_and_block() {
        let while_keyword = Token::new(TokenKind::WhileKeyword, SourceSpan::new(0, 5));
        let condition_token = Token::new(TokenKind::Identifier, SourceSpan::new(6, 7));
        let left_brace_token = Token::new(TokenKind::LeftBrace, SourceSpan::new(14, 1));
        let right_brace_token = Token::new(TokenKind::RightBrace, SourceSpan::new(15, 1));

        let condition = ExpressionSyntax::Name(NameExpressionSyntax::new(condition_token));
        let block = BlockSyntax::new(left_brace_token, Vec::new(), right_brace_token);

        let while_statement =
            WhileStatementSyntax::new(while_keyword, condition.clone(), block.clone());

        assert_eq!(while_statement.while_keyword(), while_keyword);
        assert_eq!(while_statement.condition(), &condition);
        assert_eq!(while_statement.block(), &block);
    }

    #[test]
    fn statement_syntax_preserves_while_statement() {
        let while_keyword = Token::new(TokenKind::WhileKeyword, SourceSpan::new(0, 5));
        let condition_token = Token::new(TokenKind::Identifier, SourceSpan::new(6, 7));
        let left_brace_token = Token::new(TokenKind::LeftBrace, SourceSpan::new(14, 1));
        let right_brace_token = Token::new(TokenKind::RightBrace, SourceSpan::new(15, 1));

        let condition = ExpressionSyntax::Name(NameExpressionSyntax::new(condition_token));
        let block = BlockSyntax::new(left_brace_token, Vec::new(), right_brace_token);

        let while_statement = WhileStatementSyntax::new(while_keyword, condition, block);

        let statement = StatementSyntax::While(while_statement.clone());

        assert_eq!(statement, StatementSyntax::While(while_statement));
    }

    #[test]
    fn for_statement_preserves_keyword_expressions_semicolons_and_block() {
        let for_keyword = Token::new(TokenKind::ForKeyword, SourceSpan::new(0, 3));
        let initializer_token = Token::new(TokenKind::Identifier, SourceSpan::new(4, 1));
        let first_semicolon_token = Token::new(TokenKind::Semicolon, SourceSpan::new(9, 1));
        let condition_token = Token::new(TokenKind::Identifier, SourceSpan::new(11, 1));
        let second_semicolon_token = Token::new(TokenKind::Semicolon, SourceSpan::new(17, 1));
        let update_token = Token::new(TokenKind::Identifier, SourceSpan::new(19, 1));
        let left_brace_token = Token::new(TokenKind::LeftBrace, SourceSpan::new(29, 1));
        let right_brace_token = Token::new(TokenKind::RightBrace, SourceSpan::new(30, 1));

        let initializer = ExpressionSyntax::Name(NameExpressionSyntax::new(initializer_token));
        let condition = ExpressionSyntax::Name(NameExpressionSyntax::new(condition_token));
        let update = ExpressionSyntax::Name(NameExpressionSyntax::new(update_token));
        let block = BlockSyntax::new(left_brace_token, Vec::new(), right_brace_token);

        let for_statement = ForStatementSyntax::new(
            for_keyword,
            initializer.clone(),
            first_semicolon_token,
            condition.clone(),
            second_semicolon_token,
            update.clone(),
            block.clone(),
        );

        assert_eq!(for_statement.for_keyword(), for_keyword);
        assert_eq!(for_statement.initializer(), &initializer);
        assert_eq!(for_statement.first_semicolon_token(), first_semicolon_token);
        assert_eq!(for_statement.condition(), &condition);
        assert_eq!(
            for_statement.second_semicolon_token(),
            second_semicolon_token
        );
        assert_eq!(for_statement.update(), &update);
        assert_eq!(for_statement.block(), &block);
    }

    #[test]
    fn statement_syntax_preserves_for_statement() {
        let for_keyword = Token::new(TokenKind::ForKeyword, SourceSpan::new(0, 3));
        let initializer_token = Token::new(TokenKind::Identifier, SourceSpan::new(4, 1));
        let first_semicolon_token = Token::new(TokenKind::Semicolon, SourceSpan::new(5, 1));
        let condition_token = Token::new(TokenKind::Identifier, SourceSpan::new(7, 1));
        let second_semicolon_token = Token::new(TokenKind::Semicolon, SourceSpan::new(8, 1));
        let update_token = Token::new(TokenKind::Identifier, SourceSpan::new(10, 1));
        let left_brace_token = Token::new(TokenKind::LeftBrace, SourceSpan::new(12, 1));
        let right_brace_token = Token::new(TokenKind::RightBrace, SourceSpan::new(13, 1));

        let initializer = ExpressionSyntax::Name(NameExpressionSyntax::new(initializer_token));
        let condition = ExpressionSyntax::Name(NameExpressionSyntax::new(condition_token));
        let update = ExpressionSyntax::Name(NameExpressionSyntax::new(update_token));
        let block = BlockSyntax::new(left_brace_token, Vec::new(), right_brace_token);

        let for_statement = ForStatementSyntax::new(
            for_keyword,
            initializer,
            first_semicolon_token,
            condition,
            second_semicolon_token,
            update,
            block,
        );

        let statement = StatementSyntax::For(for_statement.clone());

        assert_eq!(statement, StatementSyntax::For(for_statement));
    }
}
