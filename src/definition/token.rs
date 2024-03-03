#[derive(Debug)]
pub struct PositionalToken<'a> {
    pub line: usize,
    pub col: usize,
    pub length: usize,
    pub token: Token<'a>,
}

#[derive(Debug)]
pub enum Token<'a> {
    Literal(&'a str),
    Unknown(u8),
    EndOfFile,
    Function,
    Return,
    Let,
    Var,
    Const,
    RightCurly,
    LeftCurly,
    LeftParen,
    RightParen,
    Semicolon,
    Plus,
    Minus,
}

impl<'a> From<u8> for Token<'a> {
    fn from(c: u8) -> Self {
        match c {
            b'{' => Token::LeftCurly,
            b'(' => Token::LeftParen,
            b'}' => Token::RightCurly,
            b')' => Token::RightParen,
            b'+' => Token::Plus,
            b'-' => Token::Minus,
            b';' => Token::Semicolon,
            _ =>    Token::Unknown(c), 
        }
    }
}

impl<'a> From<&'a str> for Token<'a> {
    fn from(word: &'a str) -> Self {
        match word {
            "function" => Token::Function,
            "return" => Token::Return,
            "let" => Token::Let,
            "var" => Token::Var,
            "const" => Token::Const,
            _ => Token::Literal(word)
        }
    }
}



