mod rule;
mod assist;

use crate::stream::Stream;
use crate::def::Token;
use crate::def::StringType;
use crate::def::PositionalToken;

pub fn generate_token<'s>(stream: &'s mut Stream) -> PositionalToken<'s> {
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
        println!("[LEXING SYMBOL: {:?}]", std::str::from_utf8(&[symbol]).unwrap());
        let token = match symbol {
            b'a'..=b'z' | b'A'..=b'Z' => rule::identifier_name(stream),
            b'0'..=b'9' => rule::numeric_literal(stream),
            b'"' => rule::string_literal(
                stream, StringType::DoubleQuoted 
                ),
            b'\'' => rule::string_literal(
                stream, StringType::SingleQuoted 
                ),
            b'/' => rule::potential_comment(stream),
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
            b'~' => rule::punctuator(stream),
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
