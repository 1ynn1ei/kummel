use crate::stream::Stream;
use crate::definition::Token;
use crate::definition::PositionalToken;
use crate::definition::pattern;

pub enum JSError {

}

pub enum Mode {
    Data,
    BeforeComment,
    Comment,
    BlockComment,
    BeforeString,
    StringDoubleQuoted,
    StringSingleQuoted,
}

type LexerOptionalResult<'stream> = Result<Option<PositionalToken<'stream>>,JSError>;

pub struct Lexer<'stream> {
    mode: Mode,
    stream: Stream<'stream>,
    current_line: usize,
}

impl<'stream> Lexer<'stream> {
    pub fn new(data: &'stream Vec<u8>) -> Self {
        Self {
            mode: Mode::Data,
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
            match self.mode {
                Mode::Data => self.data_ruleset(char),
                Mode::BeforeComment => self.before_comment_ruleset(char),
                Mode::StringDoubleQuoted => self.string_double_quoted_ruleset(char),
                Mode::StringSingleQuoted => self.string_single_quoted_ruleset(char),
                _ => todo!()
            }
        }
    }

    pub fn string_double_quoted_ruleset(&mut self, char: u8) -> LexerOptionalResult {
        let cur_line = self.current_line;
        let cur_col = 0;
        
    }

    pub fn string_single_quoted_ruleset(&mut self, char: u8) -> LexerOptionalResult {
        let cur_line = self.current_line;
        let cur_col = 0;
    }

    pub fn before_comment_ruleset(&mut self, char: u8) -> LexerOptionalResult {
        let cur_line = self.current_line;
        let cur_col = 0;
        /* we've already consumed a / */
        match char {
            b'/' => { 
                self.mode = Mode::Comment;
                Ok(None)
            },
            b'*' => {
                self.mode = Mode::BlockComment;
                Ok(None)
            },
            _ => {
                self.mode = Mode::Data;
                // we're 1-step from the /, re consume it later
                self.stream.restep();
                Ok(Some(PositionalToken {
                    line: cur_line,
                    col: cur_col,
                    length: 0,
                    token: Token::from("/")
                }))
            }
        }
    }

    pub fn data_ruleset(&mut self, char: u8) -> LexerOptionalResult {
        let cur_line = self.current_line;
        let cur_col = 0;
        match char {
            b'/' => {
                self.mode = Mode::BeforeComment;
                todo!("Before Comment State")
            },
            b'\''|
            b'"' => {
                self.mode = Mode::BeforeString;
                todo!("Before String State")
            },
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


    fn get_single_quote_string(&mut self) -> &[u8] {
        
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
