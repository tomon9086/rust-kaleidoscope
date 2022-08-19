mod ast;
mod chars;
mod error;
mod io;
mod lexer;
mod parser;

use error::Error;
use io::{read_file_by_chars, read_options};
use lexer::tokenize;
use parser::parse_expr;
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

            parse_expr(&mut chars, curr_token);
        }
    }
}
