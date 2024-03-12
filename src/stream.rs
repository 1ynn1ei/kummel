use crate::def::pattern;

pub struct Stream {
    pub data: Vec<u8>,
    idx: usize,
    pub col: usize,
    pub line: usize,
}

impl Stream {
    pub fn new(data: Vec<u8>) -> Self {
        Stream {
            data,
            idx: 0,
            col: 0,
            line: 0
        }
    }
    pub fn step(&mut self) {
        self.idx += 1;
        if !self.is_eof() {
            if pattern::is_line_terminator(&self.data[self.idx]) {
                self.line += 1;
                self.col = 0;
            } else {
                self.col += 1;
            }
        }
    }

    pub fn restep(&mut self) {
        self.idx -= 1;
    }

    pub fn peek(&self) -> Option<u8> {
        if self.idx + 1 >= self.data.len() {
            None
        } else {
            Some(self.data[self.idx + 1])
        }
    }

    pub fn is_eof(&self) -> bool {
        self.idx >= self.data.len()
    }

    pub fn current(&self) -> u8 {
        self.data[self.idx]
    }

    pub fn cursor(&self) -> usize {
        self.idx
    }

    pub fn get_slice(&self, start: usize, end: usize) -> &[u8] {
        &self.data[start..end]
    }
}
