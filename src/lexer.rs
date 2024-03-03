use crate::stream::Stream;
use crate::definition::Token;
use crate::definition::Operator;
use crate::definition::pattern;

pub enum JSError {

}

type LexerOptionalResult<'stream> = Result<
                                        Option<
                                            PositionalToken<'stream>
                                            >,
                                    JSError>;

#[derive(Debug)]
pub struct PositionalToken<'stream> {
    line: usize,
    // column: usize,
    // length: usize,
    pub token: Token<'stream>
}

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
                    Lexer::make_token(Token::EndOfFile, self.current_line)
                    )
                )
        } else {
            let char = self.stream.current();
            self.stream.step();
            self.ruleset(char)
        }
    }

    pub fn ruleset(&mut self, char: u8) -> LexerOptionalResult {
        match char {
            b'a'..=b'z' | b'A'..=b'Z' => {
                let keyword = std::str::from_utf8(
                    self.get_next_literal()
                    ).unwrap();
                Ok(
                    Some(
                        self.string_into_keyword(keyword)
                        )
                    )
            },
            b'0'..=b'9' => {
                let literal = std::str::from_utf8(
                    self.get_next_numeric()
                    ).unwrap();
                Ok(
                    Some(
                        Lexer::make_token(
                            Token::Literal(literal), 
                            self.current_line)
                        )
                    )
            },
            b'(' => Ok(
                    Some(
                        Lexer::make_token(Token::LeftParen, self.current_line)
                        )
                    ),
            b')' => Ok(
                    Some(
                        Lexer::make_token(Token::RightParen, self.current_line)
                        )
                    ),
            b'{' => Ok(
                    Some(
                        Lexer::make_token(Token::LeftCurly, self.current_line)
                        )
                    ),
            b'}' => Ok(
                    Some(
                        Lexer::make_token(Token::RightCurly, self.current_line)
                        )
                    ),
            b'+' => Ok(
                    Some(
                        Lexer::make_token(Token::Plus, self.current_line)
                        )
                    ),
            b' ' |
            b'\t' => Ok(None),
            b'\n' => {
                self.current_line += 1;
                Ok(None)
            },
            b';' => Ok(
                    Some(
                        Lexer::make_token(Token::Semicolon, self.current_line)
                        )
                    ),
            _ => todo!()
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

    fn string_into_keyword(&self, keyword: &'stream str) -> PositionalToken {
        match keyword {
            "function" => {
                Lexer::make_token(
                    Token::Function,
                    self.current_line
                    )
            },
            "return" => {
                Lexer::make_token(
                    Token::Return,
                    self.current_line
                    )
            },
            _ => {
                Lexer::make_token(
                    Token::Identifier(keyword),
                    self.current_line
                    )
            }
        }
    }

    fn make_token(token: Token, line: usize) -> PositionalToken {
        PositionalToken { token, line }
    }
}
