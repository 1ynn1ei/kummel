use crate::def::pattern;

pub struct Stream<'a> {
    pub data: &'a Vec<u8>,
    idx: usize,
    col: usize,
    line: usize,
}

impl<'a> Stream<'a> {
    pub fn new(data: &'a Vec<u8>) -> Self {
        Stream {
            data,
            idx: 0,
            col: 0,
            line: 0
        }
    }
    pub fn step(&mut self) -> Option<u8> {
        if self.idx + 1 > self.data.len() {
            None
        } else {
            self.idx += 1;
            if pattern::is_line_terminator(&self.data[self.idx]) {
                self.line += 1;
                self.col = 0;
            } else {
                self.col += 1;
            }
            Some(self.data[self.idx])
        }
    }

    pub fn restep(&mut self) {
        self.idx -= 1;
    }

    pub fn peek(&self) -> Option<u8> {
        if self.idx == 0 {
            Some(self.data[self.idx])
        } else if self.idx + 1 >= self.data.len() {
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

    pub fn get_slice(&self, start: usize) -> &[u8] {
        &self.data[start..self.idx]
    }
}
