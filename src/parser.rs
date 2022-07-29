use crate::{
    ast::NumberExprAst,
    io::Chars,
    lexer::{tokenize, Token},
};

pub fn parseNumberExpr(chars: &mut Chars, n: f64) -> (NumberExprAst, Token) {
    (NumberExprAst { val: n }, tokenize(chars))
}
