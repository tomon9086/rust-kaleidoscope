#[derive(Clone)]
pub struct Chars {
    lines: Vec<char>,
    offset: usize,
}

impl Iterator for Chars {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO: peek() と共通化できない？
        if self.lines.len() <= self.offset {
            return None;
        } else {
            self.offset += 1;
        }

        if self.lines.len() != 0 && self.lines.len() > self.offset {
            Some(self.lines[self.offset])
        } else {
            None
        }
    }
}

impl Chars {
    pub fn new(lines: Vec<Vec<char>>) -> Self {
        Self {
            lines: lines.join(&'\n'),
            offset: 0,
        }
    }

    pub fn peek(&self) -> Option<char> {
        if self.lines.len() <= self.offset {
            return None;
        }

        Some(self.lines[self.offset])
    }

    pub fn is_empty(&self) -> bool {
        self.lines.len() <= self.offset
    }
}
