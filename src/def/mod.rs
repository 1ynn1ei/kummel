mod token;

pub mod pattern;
pub use token::Token;
pub use token::PositionalToken;

pub enum StringType {
    SingleQuoted,
    DoubleQuoted
}

pub enum CommentType {
    SingleLine,
    MultiLine
}
