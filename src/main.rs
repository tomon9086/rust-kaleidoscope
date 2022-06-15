use std::{
    env, error, fmt, fs,
    io::{self, BufRead},
    path, result,
};

type Result<T> = result::Result<T, Error>;

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

fn read_file_by_lines<P>(path: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
where
    P: AsRef<path::Path>,
{
    let file = fs::File::open(path)?;
    Ok(io::BufReader::new(file).lines())
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
        for (i, line) in lines.enumerate() {
            if let Ok(ip) = line {
                println!("{}: {}", i, ip);
            }
        }
    }
}
