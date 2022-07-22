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

fn peek_char(chars: &str::Chars) -> char {
    let mut peekable = chars.clone().peekable();
    let c = peekable.peek();
    match c {
        Some(c) => *c,
        None => EOF,
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
    let mut last_char;
    let mut peeked_char = peek_char(chars);

    // Skip any whitespace.
    while is_space(peeked_char) {
        get_char(chars).unwrap_or(EOF);
        peeked_char = peek_char(chars);
    }

    if is_alpha(peeked_char) {
        last_char = get_char(chars).unwrap_or(EOF);
        peeked_char = peek_char(chars);
        identifier_str = last_char.to_string();

        while is_alnum(peeked_char) {
            last_char = get_char(chars).unwrap_or(EOF);
            peeked_char = peek_char(chars);
            identifier_str += &last_char.to_string();
        }

        println!("identifier: \"{}\"", identifier_str);
        if identifier_str == "def" {
            return Token::TokDef as i32;
        }
        if identifier_str == "extern" {
            return Token::TokExtern as i32;
        }
        return Token::TokIdentifier as i32;
    }

    if is_digit(peeked_char) || peeked_char == '.' {
        // Number: [0-9.]+
        let mut num_str = "".to_string();
        while is_digit(peeked_char) || peeked_char == '.' {
            last_char = get_char(chars).unwrap_or(EOF);
            peeked_char = peek_char(chars);
            num_str += &last_char.to_string();
        }

        num_val = match num_str.parse::<f64>() {
            Ok(n) => n,
            Err(e) => panic!("{}", e),
        };
        println!("number: {}", num_val);
        return Token::TokNumber as i32;
    }

    if peeked_char == '#' {
        // Comment until end of line.
        while peeked_char != EOF && peeked_char != '\n' && peeked_char != '\r' {
            get_char(chars).unwrap_or(EOF);
            peeked_char = peek_char(chars);
        }

        if peeked_char != EOF {
            // ここに入ることある？
            return tokenize(chars);
        }
    }

    // Check for end of file.  Don't eat the EOF.
    if peeked_char == EOF {
        return Token::TokEof as i32;
    }

    // Otherwise, just return the character as its ascii value.
    let this_char = peeked_char;
    get_char(chars).unwrap_or(EOF);

    println!("token: \"{}\"", this_char);
    this_char.to_digit(10).unwrap_or_default() as i32
}
