mod ast;
mod error;
mod io;
mod lexer;

use error::Error;
use io::{read_file_by_lines, read_options};
use lexer::{tokenize, Token};
use std::result;

type Result<T> = result::Result<T, Error>;

fn main() {
    let options = match read_options() {
        Ok(options) => options,
        Err(err) => panic!("invalid options: {}", err),
    };
    let filepath = options.filepath;

    if let Ok(lines) = read_file_by_lines(filepath) {
        for l in lines {
            // let mut chars = line.expect("lines failed").chars();
            let line = match l {
                Ok(line) => line,
                Err(_) => String::new(),
            };
            let mut chars = line.chars();

            while !chars.as_str().is_empty() {
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
}
