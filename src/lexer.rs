use crate::stream::Stream;
use crate::definition::Token;
use crate::definition::PositionalToken;
use crate::definition::pattern;

pub enum JSError {

}

type LexerOptionalResult<'stream> = Result<Option<PositionalToken<'stream>>,JSError>;

pub struct Lexer<'stream> {
    stream: Stream<'stream>,
    current_line: usize,
}

impl<'stream> Lexer<'stream> {
    pub fn new(data: &'stream Vec<u8>) -> Self {
        Self {
            stream: Stream::new(data),
            current_line: 1
        }
    }

    pub fn next_token(&mut self) -> LexerOptionalResult {
        if self.stream.is_eof() {
            Ok(
                Some(
                    PositionalToken {
                        line: self.current_line,
                        col: 0,
                        length: 0,
                        token: Token::EndOfFile,
                    }
                )
            )
        } else {
            let char = self.stream.current();
            self.stream.step();
            self.ruleset(char)
        }
    }

    pub fn ruleset(&mut self, char: u8) -> LexerOptionalResult {
        let cur_line = self.current_line;
        let cur_col = 0;
        match char {
            b'a'..=b'z' | b'A'..=b'Z' => {
                let keyword = std::str::from_utf8(
                    self.get_next_literal()
                    ).unwrap();
                Ok(Some(PositionalToken {
                    line: cur_line,
                    col: cur_col,
                    length: 0,
                    token: Token::from(keyword)
                }))
            },
            b'0'..=b'9' => {
                let literal = std::str::from_utf8(
                    self.get_next_numeric()
                    ).unwrap();
                Ok(Some(PositionalToken {
                    line: cur_line,
                    col: cur_col,
                    length: 0,
                    token: Token::from(literal)
                }))
            },
            b' '|
            b'\t' => Ok(None),
            b'\n' => {
                self.current_line += 1;
                Ok(None)
            },
            b'(' => Ok(Some(PositionalToken {
                        line: cur_line,
                        col: cur_col,
                        length: 0,
                        token: Token::LeftParen
                    })),
            b')' => Ok(Some(PositionalToken {
                        line: cur_line,
                        col: cur_col,
                        length: 0,
                        token: Token::RightParen
                    })),
            _ => {
                if pattern::is_operator_candidate(&char) {
                    let operator = std::str::from_utf8(
                        self.get_next_operator()
                        ).unwrap();
                    Ok(Some(PositionalToken {
                    line: cur_line,
                    col: cur_col,
                    length: 0,
                    token: Token::from(operator)
                    }))
                } else {
                    println!("{}", char);
                    todo!()
                }
            },
        }
    }


    fn get_next_literal(&mut self) -> &[u8] {
        self.stream.restep();
        let start_idx = self.stream.cursor();
        while pattern::is_literal(
            &self.stream.current()
            ) {
            self.stream.step();
        }
        self.stream.get_slice(start_idx)
    }

    fn get_next_numeric(&mut self) -> &[u8] {
        self.stream.restep();
        let start_idx = self.stream.cursor();
        while pattern::is_numeric(
            &self.stream.current()
            ) {
            self.stream.step();
        }
        self.stream.get_slice(start_idx)
    }

    fn get_next_operator(&mut self) -> &[u8] {
        self.stream.restep();
        let start_idx = self.stream.cursor();
        while pattern::is_operator_candidate(
            &self.stream.current()
            ) {
            self.stream.step();
        }
        self.stream.get_slice(start_idx)
    }

    fn get_next_comment(&mut self) -> &[u8] {
        self.stream.restep();
        let start_idx = self.stream.cursor();
        while !pattern::is_line_terminator(
            &self.stream.current()
            ) {
            self.stream.step();
        }
        self.stream.get_slice(start_idx)
    }
}
