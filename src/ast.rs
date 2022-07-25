/// ExprAST - Base class for all expression nodes.
pub trait ExprAst {}

/// NumberExprAST - Expression class for numeric literals like "1.0".
pub struct NumberExprAst {
    pub val: f64,
}

impl ExprAst for NumberExprAst {}

/// VariableExprAST - Expression class for referencing a variable, like "a".
pub struct VariableExprAst {
    pub name: String,
}

impl ExprAst for VariableExprAst {}

/// BinaryExprAST - Expression class for a binary operator.
pub struct BinaryExprAst<'ea> {
    pub op: char,
    pub lhs: &'ea dyn ExprAst,
    pub rhs: &'ea dyn ExprAst,
}

impl ExprAst for BinaryExprAst<'_> {}

/// CallExprAST - Expression class for function calls.
pub struct CallExprAst<'ea> {
    pub calle: String,
    pub args: std::vec::Vec<&'ea dyn ExprAst>,
}

impl ExprAst for CallExprAst<'_> {}
