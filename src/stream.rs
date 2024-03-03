pub struct Stream<'a> {
    pub data: &'a Vec<u8>,
    idx: usize,
}

impl<'a> Stream<'a> {
    pub fn new(data: &'a Vec<u8>) -> Self {
        Stream {
            data,
            idx: 0
        }
    }
    pub fn step(&mut self) {
        self.idx += 1;
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

    pub fn get_slice(&self, start: usize) -> &[u8] {
        &self.data[start..self.idx]
    }

    pub fn expect(&self, chars: &str) -> bool {
        if chars.len() + self.idx > self.data.len() {
            false
        } else {
            for (offset, byte) in chars.as_bytes().iter().enumerate() {
                if &self.data[self.idx+offset] == byte {
                    continue;
                } else {
                    return false;
                }
            }
            true
        }

    }

    pub fn expect_insensitive(&self, chars: &str) -> bool {
        // TODO: obviously needs to be modified for case insensitivity
        if chars.len() + self.idx > self.data.len() {
            false
        } else {
            for (offset, byte) in chars.as_bytes().iter().enumerate() {
                if &self.data[self.idx+offset] == byte {
                    continue;
                } else {
                    return false;
                }
            }
            true
        }

    }

    pub fn consume(&mut self, chars: &str) {
        self.idx += chars.len() - 1
    }

    pub fn reconsume(&mut self, chars: &str) {
        self.idx -= chars.len()
    }
}
