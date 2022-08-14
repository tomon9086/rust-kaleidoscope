use crate::{chars::Chars, Error, Result};
use std::{
    env, fs,
    io::{self, BufRead},
    path,
};

type Lines = io::Lines<io::BufReader<fs::File>>;

pub struct CmdOption {
    pub filepath: String,
}

pub fn read_options() -> Result<CmdOption> {
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

pub fn read_file_by_chars<P>(path: P) -> io::Result<Chars>
where
    P: AsRef<path::Path>,
{
    read_file_by_lines(path).and_then(|lines| {
        Ok(Chars::new(
            lines
                .map(|l| l.unwrap_or(String::new()).chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>(),
        ))
    })
}
