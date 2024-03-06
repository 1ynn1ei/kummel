use crate::def::CommentType;
use crate::def::StringType;
use crate::def::Token;
use crate::def::pattern;
use crate::lex::assist::*;
use crate::stream::Stream;

pub fn string_literal<'s>(
    stream: &'s mut Stream,
    string_type: StringType) -> Token<'s> {
    let slice_start = match string_type {
        StringType::DoubleQuoted => walk_until_expect_or_terminate(stream, b'"'),
        StringType::SingleQuoted => walk_until_expect_or_terminate(stream, b'\'')
    };
    Token::StringLiteral(
        slice_into_str(
            stream.get_slice(slice_start)
            )
        )
}

pub fn identifier_name<'s>(
    stream: &'s mut Stream) -> Token<'s> {
    let slice_start = walk_until_not_matches(stream, &pattern::is_literal);
    let identifier = slice_into_str(
        stream.get_slice(slice_start)
        );
    Token::from(identifier)
}

pub fn potential_comment<'s>(
    stream: &'s mut Stream) -> Token<'s> {
    if let Some(next_symbol) = stream.peek() {
        match next_symbol {
            b'/' => comment(stream, CommentType::SingleLine),
            b'*' => comment(stream, CommentType::MultiLine),
            _ => todo!()
        }
    } else {
        Token::from("/")
    }
}

fn comment<'s>(
    stream: &'s mut Stream,
    comment_type: CommentType) -> Token<'s> {
    match comment_type {
        CommentType::SingleLine => {
            let slice_start = walk_until_terminate(stream);
            Token::Comment(
                slice_into_str(
                    stream.get_slice(slice_start)
                    )
                )
        },
        CommentType::MultiLine => {
            let slice_start = walk_until_expect_expect(stream,b'*',b'/');
            Token::Comment(
                slice_into_str(
                    stream.get_slice(slice_start)
                    )
                )
        }
    }
}
