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
    // line: usize,
    // column: usize,
    // length: usize,
    pub token: Token<'stream>
}

pub struct Lexer<'stream> {
    stream: Stream<'stream>,
}

impl<'stream> Lexer<'stream> {
    pub fn new(data: &'stream Vec<u8>) -> Self {
        Self {
            stream: Stream::new(data),
        }
    }

    pub fn next_token(&mut self) -> LexerOptionalResult {
        if self.stream.is_eof() {
            Ok(
                Some(
                    Lexer::make_token(Token::EndOfFile)
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
                        Lexer::string_into_keyword(keyword)
                        )
                    )
            },
            b'0'..=b'9' => {
                let literal = std::str::from_utf8(
                    self.get_next_numeric()
                    ).unwrap();
                Ok(
                    Some(
                        Lexer::make_token(Token::Literal(literal))
                        )
                    )
            },
            b'(' => Ok(
                    Some(
                        Lexer::make_token(Token::LeftParen)
                        )
                    ),
            b')' => Ok(
                    Some(
                        Lexer::make_token(Token::RightParen)
                        )
                    ),
            b'{' => Ok(
                    Some(
                        Lexer::make_token(Token::LeftCurly)
                        )
                    ),
            b'}' => Ok(
                    Some(
                        Lexer::make_token(Token::RightCurly)
                        )
                    ),
            b'+' => Ok(
                    Some(
                        Lexer::make_token(Token::Plus)
                        )
                    ),
            b' ' |
            b'\t'|
            b'\n' => {
                Ok(None)
            },
            b';' => Ok(
                    Some(
                        Lexer::make_token(Token::Semicolon)
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

    fn string_into_keyword(keyword: &'stream str) -> PositionalToken {
        match keyword {
            "function" => {
                PositionalToken {
                    token: Token::Function 
                }
            },
            "return" => {
                PositionalToken {
                    token: Token::Return
                }
            },
            _ => {
                PositionalToken {
                    token: Token::Identifier(keyword)
                }
            }
        }
    }

    fn make_token(token: Token) -> PositionalToken {
        PositionalToken { token }
    }
}
