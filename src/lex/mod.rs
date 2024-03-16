mod grammar;
mod assist;

use crate::stream::Stream;
use crate::def::Token;
use crate::def::StringType;
use crate::def::PositionalToken;

pub fn generate_token(stream: &mut Stream) -> PositionalToken {
    let line = stream.line;
    let col = stream.col;
    if stream.is_eof() {
        PositionalToken {
            line,
            col,
            length: 0,
            token: Token::EndOfFile
        }
    } else {
        let symbol = stream.current();
        let token = match symbol {
            b'a'..=b'z' | b'A'..=b'Z' => grammar::identifier_name(stream),
            b'0'..=b'9' => grammar::numeric_literal(stream),
            b'"' => grammar::string_literal(
                stream, StringType::DoubleQuoted 
                ),
            b'\'' => grammar::string_literal(
                stream, StringType::SingleQuoted 
                ),
            b'/' => grammar::potential_comment(stream),
            b'(' => { stream.step(); Token::LeftParen },
            b')' => { stream.step(); Token::RightParen },
            b'!'|
            b'%'|
            b'&'|
            b'+'|
            b','|
            b'-'|
            b'.'|
            b':'|
            b';'|
            b'<'|
            b'='|
            b'>'|
            b'?'|
            b'['|
            b']'|
            b'^'|
            b'{'|
            b'|'|
            b'}'|
            b'~' => grammar::punctuator(stream),
            b' ' => {
                stream.step();
                Token::WhiteSpace
            },
            b'\n'|
            b'\r' => { stream.step(); Token::LineTerminator }
            _ => todo!()
        };
        PositionalToken {
            line,
            col,
            length: 0,
            token
        }
    }
}
