mod ast;
mod chars;
mod error;
mod io;
mod lexer;

use error::Error;
use io::{read_file_by_chars, read_options};
use lexer::{tokenize, Token};
use std::result;

type Result<T> = result::Result<T, Error>;

fn main() {
    let options = match read_options() {
        Ok(options) => options,
        Err(err) => panic!("invalid options: {}", err),
    };
    let filepath = options.filepath;

    if let Ok(mut chars) = read_file_by_chars(filepath) {
        while !chars.is_empty() {
            let curr_token = tokenize(&mut chars);

            match curr_token {
                Token::TokEof => {}
                Token::TokUnsupported(ch) => {
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
                }
            }
        }
    }
}
