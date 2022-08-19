use crate::{
    ast::NumberExprAst,
    chars::Chars,
    lexer::{tokenize, Token},
};

pub fn parse_number_expr<'c>(
    _chars: &'c mut Chars,
    curr_token: &'c Token,
    n: f64,
) -> (NumberExprAst, &'c Token) {
    (NumberExprAst { val: n }, curr_token)
}

pub fn parse_paren_expr(chars: &mut Chars) {
    loop {
        let token = tokenize(chars);

        if token == Token::TokParenEnd {
            println!(")");
            return;
        }

        parse_expr(chars, &token);
        println!("    {:?}", token);
    }
}

pub fn parse_primary(chars: &mut Chars, curr_token: &Token) {
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
            parse_number_expr(chars, curr_token, *n);
        }
        Token::TokParenStart => {
            println!("(");
            parse_paren_expr(chars);
        }
        Token::TokParenEnd => {
            panic!("unexpected closing paren");
        }
    }
}

pub fn parse_expr(chars: &mut Chars, curr_token: &Token) {
    parse_primary(chars, curr_token);
}
