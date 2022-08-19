use crate::{
    ast::{ExprAst, NumberExprAst},
    chars::Chars,
    lexer::{tokenize, Token},
};

pub fn parse_number_expr(chars: &mut Chars, n: f64) -> (NumberExprAst, Token) {
    (NumberExprAst { val: n }, tokenize(chars))
}

pub fn parse_paren_expr(chars: &mut Chars) -> impl ExprAst {
    loop {
        let token = tokenize(chars);

        if token == Token::TokUnsupported(')') {
            println!(")");
            return NumberExprAst { val: 1. };
        }

        println!("    {:?}", token);
    }
}

pub fn parse_primary(chars: &mut Chars, curr_token: Token) {
    match curr_token {
        Token::TokEof => {}
        Token::TokUnsupported(ch) => {
            // TODO: LogError
            println!("unsupported: {}", ch);
        }
        Token::TokDef => {
            println!("def");
        }
        Token::TokExtern => {
            println!("extern");
        }
        Token::TokIdentifier(identifier) => {
            println!("identifier: {}", identifier);
        }
        Token::TokNumber(n) => {
            println!("number: {}", n);
            parse_number_expr(chars, n);
        }
        Token::TokParenStart => {
            println!("(");
            parse_paren_expr(chars);
        }
    }
}

pub fn parse_expr(chars: &mut Chars, curr_token: Token) {
    // borrowing?
    parse_primary(chars, curr_token);
}
