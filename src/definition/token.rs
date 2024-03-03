#[derive(Debug)]
pub struct PositionalToken<'a> {
    pub line: usize,
    pub col: usize,
    pub length: usize,
    pub token: Token<'a>,
}

#[derive(Debug)]
pub enum Token<'a> {
    Push,
    Break,
    Const,
    EndOfFile,
    Function,
    LeftCurly,
    LeftParen,
    LeftBracket,
    RightBracket,
    Let,
    Literal(&'a str),
    Minus,
    Equal,
    Plus,
    Comma,
    Period,
    RightCarat,
    LeftCarat,
    Question,
    Pipe,
    Return,
    RightCurly,
    RightParen,
    Semicolon,
    Colon,
    And,
    Unknown(u8),
    Var,
    Arguments,
    Slice,
    Call,
    Apply,
    Splice,
    Length,
    This,
    Emit,
    For,
}

impl<'a> From<u8> for Token<'a> {
    fn from(c: u8) -> Self {
        match c {
            b'&' => Token::And,
            b'(' => Token::LeftParen,
            b')' => Token::RightParen,
            b'+' => Token::Plus,
            b',' => Token::Comma,
            b'-' => Token::Minus,
            b'.' => Token::Period,
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

impl<'a> From<&'a str> for Token<'a> {
    fn from(word: &'a str) -> Self {
        match word {
            "arguments" => Token::Arguments,
            "const" => Token::Const,
            "function" => Token::Function,
            "let" => Token::Let,
            "return" => Token::Return,
            "var" => Token::Var,
            "emit" => Token::Emit,
            "this" => Token::This,
            "call" => Token::Call,
            "apply" => Token::Apply,
            "length" => Token::Length,
            "slice" => Token::Slice,
            "splice" => Token::Splice,
            "for" => Token::For,
            "push" => Token::Push,
            _ => Token::Literal(word)
        }
    }
}



