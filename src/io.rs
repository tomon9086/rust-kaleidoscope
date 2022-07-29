use crate::{Error, Result};
use std::{
    env, fs,
    io::{self, BufRead, Read},
    iter::Peekable,
    path, str,
};

type Lines = io::Lines<io::BufReader<fs::File>>;

pub struct Chars<'c> {
    lines: &'c mut Lines,
    curr_line: String,
    offset: usize,
}

impl<'c> Chars<'c> {
    fn new(lines: &'c mut Lines) -> Self {
        let mut chars = Chars {
            lines,
            curr_line: "".to_string(),
            offset: 0,
        };
        chars.curr_line = chars
            .lines
            .next()
            .unwrap_or(Ok("".to_string()))
            .ok()
            .unwrap_or_default();
        chars
    }

    pub fn chars(&self) -> str::Chars {
        self.curr_line.chars()
    }

    pub fn is_empty(&mut self) -> bool {
        println!(
            "is_empty: {} - {}",
            self.curr_line.as_str().is_empty(),
            match self.lines.peekable().peek() {
                Some(_) => false,
                None => true,
            }
        );

        self.curr_line.as_str().is_empty()
            && match self.lines.peekable().peek() {
                Some(_) => false,
                None => true,
            }
    }

    pub fn peek(self) -> Option<&'c char> {
        match self.curr_line[self.offset..].chars().peekable().peek() {
            Some(c) => Some(c),
            None => match self.lines.peekable().peek() {
                Some(Ok(l)) => l.chars().peekable().peek(),
                Some(Err(_)) => None,
                None => None,
            },
        }
    }
}

impl Iterator for Chars<'_> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        match self.curr_line[self.offset..].chars().peekable().peek() {
            Some(_) => {}
            None => {
                self.curr_line = self
                    .lines
                    .next()
                    .unwrap_or(Ok("".to_string()))
                    .ok()
                    .unwrap_or_default();
                self.offset = 0;
            }
        }

        match self.curr_line[self.offset..].chars().next() {
            Some(c) => {
                self.offset += c.len_utf8();
                Some(c)
            }
            None => None,
        }
    }
}

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

pub fn read_file_by_lines<'rfbl, P>(path: P) -> io::Result<Lines>
where
    P: AsRef<path::Path>,
{
    let file = fs::File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_chars_from_lines<'rcfl>(lines: &'rcfl mut Lines) -> Chars<'rcfl> {
    Chars::new(lines)
}
