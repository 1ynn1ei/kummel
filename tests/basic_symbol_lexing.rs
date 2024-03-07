use kml::TokenGenerator;
use kml::def::Token;
use kml::stream;

#[test]
fn basic_string_literal() {
    let data = std::fs::read("./tests/data/string_literal.js").unwrap();
    let mut gen = TokenGenerator::new(stream::Stream::new(&data));
    assert_eq!(Token::StringLiteral("test1"), gen.get().token);
    assert_eq!(Token::LineTerminator, gen.get().token);
    assert_eq!(Token::StringLiteral("test2"), gen.get().token);
    assert_eq!(Token::LineTerminator, gen.get().token);
    assert_eq!(Token::StringLiteral("test3"), gen.get().token);
    assert_eq!(Token::LineTerminator, gen.get().token);
    assert_eq!(Token::StringLiteral("test4"), gen.get().token);
}

#[test]
fn basic_inline_comment() {
    let data = std::fs::read("./tests/data/inline_comment.js").unwrap();
    let mut gen = TokenGenerator::new(stream::Stream::new(&data));
    assert_eq!(Token::Comment(" test"), gen.get().token);
    assert_eq!(Token::LineTerminator, gen.get().token);
    assert_eq!(
        Token::Comment(" \"test\" if let keyword {} /* */ // /////"),
        gen.get().token);
    assert_eq!(Token::LineTerminator, gen.get().token);
    assert_eq!(Token::Comment(""), gen.get().token);
}
