use crate::{Error, Result};
use std::str;

const EOF: char = '\0';

#[derive(Debug)]
enum Token {
    TokEof = -1,

    // commands
    TokDef = -2,
    TokExtern = -3,
    // primary
    TokIdentifier = -4,
    TokNumber = -5,
}

fn get_char(chars: &mut str::Chars) -> Result<char> {
    let c = chars.next();
    match c {
        Some(c) => Ok(c),
        None => Err(Error::new("eol")),
    }
}

fn is_space(c: char) -> bool {
    c.is_whitespace()
}

fn is_alpha(c: char) -> bool {
    c.is_alphabetic()
}

fn is_alnum(c: char) -> bool {
    c.is_alphanumeric()
}

fn is_digit(c: char) -> bool {
    c.is_numeric()
}

// TODO: return type を char or Token にしたい
pub fn tokenize(chars: &mut str::Chars) -> i32 {
    let mut identifier_str: String; // Filled in if tok_identifier
    let num_val: f64; // Filled in if tok_number
    let mut last_char = ' ';

    let mut gc = || get_char(chars).unwrap_or(EOF);

    // Skip any whitespace.
    while is_space(last_char) {
        last_char = gc();
    }

    if is_alpha(last_char) {
        identifier_str = last_char.to_string();

        last_char = gc();
        while is_alnum(last_char) {
            identifier_str += &last_char.to_string();
            last_char = gc();
        }

        if identifier_str == "def" {
            return Token::TokDef as i32;
        }
        if identifier_str == "extern" {
            return Token::TokExtern as i32;
        }
        println!("identifier: \"{}\"", identifier_str);
        return Token::TokIdentifier as i32;
    }

    if is_digit(last_char) || last_char == '.' {
        // Number: [0-9.]+
        let mut num_str = "".to_string();
        while is_digit(last_char) || last_char == '.' {
            num_str += &last_char.to_string();
            last_char = gc();
        }

        num_val = match num_str.parse::<f64>() {
            Ok(n) => n,
            Err(e) => panic!("{}", e),
        };
        println!("number: {}", num_val);
        return Token::TokNumber as i32;
    }

    if last_char == '#' {
        // Comment until end of line.
        while last_char != EOF && last_char != '\n' && last_char != '\r' {
            last_char = gc();
        }

        if last_char != EOF {
            // ここに入ることある？
            return tokenize(chars);
        }
    }

    // Check for end of file.  Don't eat the EOF.
    if last_char == EOF {
        return Token::TokEof as i32;
    }

    // Otherwise, just return the character as its ascii value.
    let this_char = last_char;
    gc();

    println!("token: \"{}\"", this_char);
    this_char.to_digit(10).unwrap_or_default() as i32
}
