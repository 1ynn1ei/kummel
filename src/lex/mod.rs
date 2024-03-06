mod rule;
mod assist;

use crate::stream::Stream;
use crate::def::Token;
use crate::def::StringType;
use crate::def::CommentType;
use crate::def::PositionalToken;
use crate::def::pattern;

pub fn generate_token<'s>(stream: &'s mut Stream) -> Token<'s> {
    if stream.is_eof() {
        Token::EndOfFile
    } else {
        let symbol = stream.current();
        println!("[LEXING SYMBOL: {:?}]", std::str::from_utf8(&[symbol]).unwrap());
        match symbol {
            b'a'..=b'z' | b'A'..=b'Z' => rule::identifier_name(stream),
            b'"' => rule::string_literal(
                stream, StringType::DoubleQuoted 
                ),
            b'\'' => rule::string_literal(
                stream, StringType::SingleQuoted 
                ),
            b'/' => rule::potential_comment(stream),
            b' ' => {
                stream.step();
                Token::WhiteSpace
            },
            _ => todo!()
        }
    }
    
}

pub enum JSError {

}

pub enum Mode {
    Data,
    BeforeComment,
    Comment,
    BlockComment,
    BeforeString,
    FunctionDelcaration,
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
                Mode::Comment => self.comment_ruleset(char),
                Mode::BlockComment => self.block_comment_ruleset(char),
                Mode::StringDoubleQuoted => self.string_double_quoted_ruleset(char),
                Mode::StringSingleQuoted => self.string_single_quoted_ruleset(char),
                _ => todo!()
            }
        }
    }

    pub fn string_double_quoted_ruleset(&mut self, char: u8) -> LexerOptionalResult {
        let cur_line = self.current_line;
        let cur_col = 0;
        todo!()
        
    }

    pub fn string_single_quoted_ruleset(&mut self, char: u8) -> LexerOptionalResult {
        let cur_line = self.current_line;
        let cur_col = 0;
        todo!()
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

    pub fn block_comment_ruleset(&mut self, char:u8) -> LexerOptionalResult {
        todo!()
    }

    pub fn comment_ruleset(&mut self, char: u8) -> LexerOptionalResult {
        self.mode = Mode::Data;
        let cur_line = self.current_line;
        let cur_col = 0;
        let comment_text = std::str::from_utf8(
            self.get_next_comment()
            ).unwrap();
        Ok(Some(PositionalToken {
            line: cur_line,
            col: cur_col,
            length: comment_text.len(),
            token: Token::Comment(comment_text)
        }))
    }

    pub fn data_ruleset(&mut self, char: u8) -> LexerOptionalResult {
        let cur_line = self.current_line;
        let cur_col = 0;
        match char {
            b'/' => {
                self.mode = Mode::BeforeComment;
                Ok(None)
            },
            b'\''=> {
                self.mode = Mode::StringSingleQuoted;
                Ok(None)
            },
            b'"' => {
                self.mode = Mode::StringDoubleQuoted;
                Ok(None)
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
        self.stream.restep();
        let start_idx = self.stream.cursor();
        while !pattern::is_line_terminator(
            &self.stream.current()
            ) {
            self.stream.step();
        }
        self.stream.get_slice(start_idx)
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
