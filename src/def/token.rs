use super::pattern;

#[derive(Debug)]
pub struct PositionalToken<'a> {
    pub line: usize,
    pub col: usize,
    pub length: usize,
    pub token: Token<'a>,
}

impl<'a> PositionalToken<'a> {
    pub fn new(line: usize, col: usize, length: usize, token: Token<'a>) -> Self {
        Self {
            line, col, length, token
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    And,
    Apply,
    Arguments,
    Assignment,
    AssignmentAdd,
    AssignmentBitwiseAnd,
    AssignmentBitwiseOr,
    LineTerminator,
    AssignmentBitwiseXor,
    AssignmentDivide,
    AssignmentExponate,
    AssignmentLeftShift,
    AssignmentLogicalAnd,
    AssignmentLogicalOr,
    AssignmentMultiply,
    AssignmentNullishCoalesce,
    AssignmentRemainder,
    AssignmentRightShift,
    AssignmentSubtract,
    AssignmentUnsignedRightShift,
    Asterisk,
    BitwiseAnd,
    BitwiseLeftShift,
    BitwiseOr,
    BitwiseRightShift,
    BitwiseUnsignedRightShift,
    BitwiseXor,
    Break,
    Call,
    Colon,
    Comma,
    Comment(&'a str),
    Punctuator(&'a str),
    Const,
    Emit,
    EndOfFile,
    Equal,
    Equality,
    For,
    ForwardSlash,
    Function,
    Inequality,
    LeftBracket,
    LeftCarat,
    LeftCurly,
    LeftParen,
    Length,
    Let,
    Identifier(&'a str),
    StringLiteral(&'a str),
    Numeric(&'a str),
    LogicalAnd,
    LogicalOr,
    Minus,
    NullishCoalesce,
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
    Spread,
    StrictEquality,
    StrictInequality,
    This,
    Unknown(u8),
    Var,
    Yield,
    WhiteSpace,
}

impl<'a> From<u8> for Token<'a> {
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

impl<'a> From<&'a str> for Token<'a> {
    fn from(word: &'a str) -> Self {
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
                    Token::Numeric(word)
                } else if pattern::is_literal(&str_check[0]) {
                    Token::StringLiteral(word)
                } else {
                    Token::from(str_check[0])
                }
            }
        }
    }
}



