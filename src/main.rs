use std::{
    env, error, fmt, fs,
    io::{self, BufRead},
    path, result, str,
};

type Result<T> = result::Result<T, Error>;
type Lines = io::Lines<io::BufReader<fs::File>>;

#[derive(Debug, Clone)]
struct Error {
    message: String,
}

impl Error {
    fn new(msg: &str) -> Error {
        Error {
            message: msg.to_string(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

struct CmdOption {
    filepath: String,
}

fn read_options() -> Result<CmdOption> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args[1].len() == 0 {
        return Err(Error::new("source file path is not specified"));
    }
    let filepath = args[1].clone();

    Ok(CmdOption { filepath: filepath })
}

fn read_file_by_lines<P>(path: P) -> io::Result<Lines>
where
    P: AsRef<path::Path>,
{
    let file = fs::File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}

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

fn tokenize(chars: &mut str::Chars) -> Token {
    let mut identifier_str: String; // Filled in if tok_identifier
    let num_val: f64; // Filled in if tok_number
    let mut last_char = ' ';

    let mut gc = || match get_char(chars) {
        Ok(c) => c,
        Err(_) => '\0',
    };

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
            return Token::TokDef;
        }
        if identifier_str == "extern" {
            return Token::TokExtern;
        }
        println!("\"{}\"", identifier_str);
        return Token::TokIdentifier;
    }

    Token::TokEof
}

fn main() {
    let options = match read_options() {
        Ok(options) => options,
        Err(err) => {
            eprintln!("invalid options: {}", err);
            panic!();
        }
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
                let token = tokenize(&mut chars);
                println!("{:?}", token);
            }
        }
    }
}
