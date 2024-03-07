use crate::def::CommentType;
use crate::def::StringType;
use crate::def::Token;
use crate::def::pattern;
use crate::lex::assist::*;
use crate::stream::Stream;

pub fn numeric_literal<'s>(
    stream: &'s mut Stream) -> Token<'s> {
    let (start, end) = walk_until_not_matches(stream, &pattern::is_numeric);
    let number = slice_into_str(
        stream.get_slice(start, end)
        );
    Token::Numeric(number)
}

pub fn string_literal<'s>(
    stream: &'s mut Stream,
    string_type: StringType) -> Token<'s> {
    let (start, end) = match string_type {
        StringType::DoubleQuoted => walk_until_expect_or_terminate(stream, b'"'),
        StringType::SingleQuoted => walk_until_expect_or_terminate(stream, b'\'')
    };
    Token::StringLiteral(
        slice_into_str(
            stream.get_slice(start, end)
            )
        )
}

pub fn identifier_name<'s>(
    stream: &'s mut Stream) -> Token<'s> {
    let (start, end) = walk_until_not_matches(stream, &pattern::is_literal);
    let identifier = slice_into_str(
        stream.get_slice(start, end)
        );
    Token::Identifier(identifier)
}

pub fn potential_comment<'s>(
    stream: &'s mut Stream) -> Token<'s> {
    if let Some(next_symbol) = stream.peek() {
        match next_symbol {
            b'/' => comment(stream, CommentType::SingleLine),
            b'*' => comment(stream, CommentType::MultiLine),
            _ => punctuator(stream)
        }
    } else {
        Token::from("/")
    }
}

pub fn punctuator<'s>(
    stream: &'s mut Stream) -> Token<'s> {
    let (start, end) = walk_until_not_matches(stream, &pattern::is_operator_candidate);
    let punctuator = slice_into_str(
        stream.get_slice(start, end)
        );
    Token::from(punctuator)
}

fn comment<'s>(
    stream: &'s mut Stream,
    comment_type: CommentType) -> Token<'s> {
    // eat the two comment identifiers
    stream.step();
    stream.step();
    match comment_type {
        CommentType::SingleLine => {
            let (start, end) = walk_until_terminate(stream);
            Token::Comment(
                slice_into_str(
                    stream.get_slice(start, end)
                    )
                )
        },
        CommentType::MultiLine => {
            let (start, end) = walk_until_expect_expect(stream,b'*',b'/');

            Token::Comment(
                slice_into_str(
                    stream.get_slice(start, end)
                    )
                )
        }
    }
}
