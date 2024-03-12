use super::pattern;

#[derive(Debug)]
pub struct PositionalToken {
    pub line: usize,
    pub col: usize,
    pub length: usize,
    pub token: Token,
}

impl PositionalToken {
    pub fn new(line: usize, col: usize, length: usize, token: Token) -> Self {
        Self {
            line, col, length, token
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Token {
    And,
    Apply,
    Arguments,
    LineTerminator,
    Asterisk,
    Call,
    Colon,
    Comma,
    Comment(String),
    Punctuator(String),
    Const,
    Emit,
    EndOfFile,
    Equal,
    Equality,
    For,
    ForwardSlash,
    Function,
    LeftBracket,
    LeftCarat,
    LeftCurly,
    LeftParen,
    Length,
    Let,
    Identifier(String),
    StringLiteral(String),
    Numeric(String),
    Minus,
    Period,
    Pipe,
    Plus,
    Push,
    Question,
    Return,
    RightBracket,
    RightCarat,
    RightCurly,
    RightParen,
    Semicolon,
    Slice,
    Splice,
    StrictEquality,
    StrictInequality,
    This,
    Unknown(u8),
    Var,
    WhiteSpace,
}

impl From<u8> for Token {
    fn from(c: u8) -> Self {
        match c {
            b'&' => Token::And,
            b'(' => Token::LeftParen,
            b')' => Token::RightParen,
            b'*' => Token::Asterisk,
            b'+' => Token::Plus,
            b',' => Token::Comma,
            b'-' => Token::Minus,
            b'.' => Token::Period,
            b'/' => Token::ForwardSlash,
            b':' => Token::Colon,
            b';' => Token::Semicolon,
            b'<' => Token::LeftCarat,
            b'=' => Token::Equal,
            b'>' => Token::RightCarat,
            b'?' => Token::Question,
            b'[' => Token::LeftBracket,
            b']' => Token::RightBracket,
            b'{' => Token::LeftCurly,
            b'|' => Token::Pipe,
            b'}' => Token::RightCurly,
            _ => panic!("{}", std::str::from_utf8(&[c]).unwrap())
        }
    }
}

impl From<&str> for Token {
    fn from(word: &str) -> Self {
        match word {
            "apply" => Token::Apply,
            "arguments" => Token::Arguments,
            "call" => Token::Call,
            "const" => Token::Const,
            "emit" => Token::Emit,
            "for" => Token::For,
            "function" => Token::Function,
            "length" => Token::Length,
            "let" => Token::Let,
            "push" => Token::Push,
            "return" => Token::Return,
            "slice" => Token::Slice,
            "splice" => Token::Splice,
            "this" => Token::This,
            "==" => Token::Equality,
            "===" => Token::StrictEquality,
            "!==" => Token::StrictInequality,
            "var" => Token::Var,
            _ => {
                let str_check = word.as_bytes();
                if pattern::is_numeric(&str_check[0]) {
                    Token::Numeric(word.to_string())
                } else if pattern::is_literal(&str_check[0]) {
                    Token::StringLiteral(word.to_string())
                } else {
                    Token::from(str_check[0])
                }
            }
        }
    }
}



