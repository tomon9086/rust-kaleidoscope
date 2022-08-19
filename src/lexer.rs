use crate::{chars::Chars, Error, Result};

const EOF: char = '\0';

#[derive(Debug, PartialEq)]
pub enum Token {
    TokEof,
    TokUnsupported(char),

    // commands
    TokDef,
    TokExtern,
    // primary
    TokIdentifier(String),
    TokNumber(f64),
    TokParenStart,
}

fn get_char(chars: &mut Chars) -> Result<char> {
    let c = chars.next();
    match c {
        Some(c) => Ok(c),
        None => Err(Error::new("eol")),
    }
}

fn peek_char(chars: &Chars) -> char {
    let c = chars.peek();
    match c {
        Some(c) => c,
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

pub fn tokenize(chars: &mut Chars) -> Token {
    let mut identifier_str: String; // Filled in if tok_identifier
    let mut peeked_char = peek_char(chars);

    // Skip any whitespace.
    while is_space(peeked_char) {
        get_char(chars).unwrap_or(EOF);
        peeked_char = peek_char(chars);
    }

    if is_alpha(peeked_char) {
        peeked_char = peek_char(chars);
        identifier_str = "".to_string();

        while is_alnum(peeked_char) {
            identifier_str += &peeked_char.to_string();

            get_char(chars).unwrap_or(EOF);
            peeked_char = peek_char(chars);
        }

        if identifier_str == "def" {
            return Token::TokDef;
        }
        if identifier_str == "extern" {
            return Token::TokExtern;
        }
        return Token::TokIdentifier(identifier_str);
    }

    if is_digit(peeked_char) || peeked_char == '.' {
        // Number: [0-9.]+
        let mut num_str = "".to_string();

        while is_digit(peeked_char) || peeked_char == '.' {
            num_str += &peeked_char.to_string();

            get_char(chars).unwrap_or(EOF);
            peeked_char = peek_char(chars);
        }

        return match num_str.parse::<f64>() {
            Ok(n) => Token::TokNumber(n),
            Err(e) => panic!("{}", e),
        };
    }

    if peeked_char == '#' {
        // Comment until end of line.
        while peeked_char != EOF && peeked_char != '\n' && peeked_char != '\r' {
            get_char(chars).unwrap_or(EOF);
            peeked_char = peek_char(chars);
        }

        if peeked_char != EOF {
            return tokenize(chars);
        }
    }

    // Check for end of file.  Don't eat the EOF.
    if peeked_char == EOF {
        return Token::TokEof;
    }

    // Otherwise, just return the character as its ascii value.
    let this_char = peeked_char;
    get_char(chars).unwrap_or(EOF);

    if this_char == '(' {
        return Token::TokParenStart;
    }

    Token::TokUnsupported(this_char)
}
