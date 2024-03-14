use kml::TokenGenerator;
use kml::def::Token;
use kml::stream;

#[test]
fn string() {
    let data = std::fs::read("./tests/data/string.js".to_string()).unwrap();
    let mut gen = TokenGenerator::new(stream::Stream::new(data));
    assert_eq!(Token::StringLiteral("test1".to_string()), gen.get().token);
    assert_eq!(Token::LineTerminator, gen.get().token);
    assert_eq!(Token::StringLiteral("test2".to_string()), gen.get().token);
    assert_eq!(Token::LineTerminator, gen.get().token);
    assert_eq!(Token::StringLiteral("test3".to_string()), gen.get().token);
    assert_eq!(Token::LineTerminator, gen.get().token);
    assert_eq!(Token::StringLiteral("test4".to_string()), gen.get().token);
}

#[test]
fn punctuator() {
    let data = std::fs::read("./tests/data/punctuator.js".to_string()).unwrap();
    let mut gen = TokenGenerator::new(stream::Stream::new(data));
    assert_eq!(Token::Punctuator("&&".to_string()), gen.get().token);
    assert_eq!(Token::LineTerminator, gen.get().token);
    assert_eq!(Token::Punctuator("||".to_string()), gen.get().token);
    assert_eq!(Token::LineTerminator, gen.get().token);
    assert_eq!(Token::Punctuator("??".to_string()), gen.get().token);
    assert_eq!(Token::LineTerminator, gen.get().token);
    assert_eq!(Token::Punctuator("<<".to_string()), gen.get().token);
    assert_eq!(Token::LineTerminator, gen.get().token);
    assert_eq!(Token::Punctuator(">>".to_string()), gen.get().token);
    assert_eq!(Token::LineTerminator, gen.get().token);
    assert_eq!(Token::Punctuator("--".to_string()), gen.get().token);
    assert_eq!(Token::LineTerminator, gen.get().token);
    assert_eq!(Token::Punctuator("++".to_string()), gen.get().token);
    assert_eq!(Token::LineTerminator, gen.get().token);
    assert_eq!(Token::Punctuator("?.".to_string()), gen.get().token);
    assert_eq!(Token::LineTerminator, gen.get().token);
    assert_eq!(Token::Punctuator("<=".to_string()), gen.get().token);
    assert_eq!(Token::LineTerminator, gen.get().token);
    assert_eq!(Token::Punctuator(">=".to_string()), gen.get().token);
    assert_eq!(Token::LineTerminator, gen.get().token);
    assert_eq!(Token::Punctuator("==".to_string()), gen.get().token);
    assert_eq!(Token::LineTerminator, gen.get().token);
    assert_eq!(Token::Punctuator("===".to_string()), gen.get().token);
    assert_eq!(Token::LineTerminator, gen.get().token);
    assert_eq!(Token::Punctuator("!=".to_string()), gen.get().token);
    assert_eq!(Token::LineTerminator, gen.get().token);
}

#[test]
fn numeric() {
    let data = std::fs::read("./tests/data/numeric.js".to_string()).unwrap();
    let mut gen = TokenGenerator::new(stream::Stream::new(data));
    assert_eq!(Token::Numeric("0".to_string()), gen.get().token);
    assert_eq!(Token::LineTerminator, gen.get().token);
    assert_eq!(Token::Numeric("0.9".to_string()), gen.get().token);
    assert_eq!(Token::LineTerminator, gen.get().token);
    assert_eq!(Token::Numeric("9_999".to_string()), gen.get().token);
    assert_eq!(Token::LineTerminator, gen.get().token);
    assert_eq!(Token::Numeric("1e9".to_string()), gen.get().token);
    assert_eq!(Token::LineTerminator, gen.get().token);
    assert_eq!(Token::Numeric("0.1e-9".to_string()), gen.get().token);
    assert_eq!(Token::LineTerminator, gen.get().token);
    assert_eq!(Token::Numeric("0b11".to_string()), gen.get().token);
    assert_eq!(Token::LineTerminator, gen.get().token);
    assert_eq!(Token::Numeric("0xd".to_string()), gen.get().token);
    assert_eq!(Token::LineTerminator, gen.get().token);
    assert_eq!(Token::Numeric("0007".to_string()), gen.get().token);
    assert_eq!(Token::LineTerminator, gen.get().token);
}

#[test]
fn inline_comment() {
    let data = std::fs::read("./tests/data/inline_comment.js".to_string()).unwrap();
    let mut gen = TokenGenerator::new(stream::Stream::new(data));
    assert_eq!(Token::Comment(" test".to_string()), gen.get().token);
    assert_eq!(Token::LineTerminator, gen.get().token);
    assert_eq!(
        Token::Comment(" \"test\" if let keyword {} /* */ // /////".to_string()),
        gen.get().token);
    assert_eq!(Token::LineTerminator, gen.get().token);
    assert_eq!(Token::Comment("".to_string()), gen.get().token);
}

#[test]
fn identifier() {
    let data = std::fs::read("./tests/data/identifier.js".to_string()).unwrap();
    let mut gen = TokenGenerator::new(stream::Stream::new(data));
    assert_eq!(Token::Identifier("if".to_string()), gen.get().token);
    assert_eq!(Token::LineTerminator, gen.get().token);
    assert_eq!(Token::Identifier("var".to_string()), gen.get().token);
    assert_eq!(Token::LineTerminator, gen.get().token);
    assert_eq!(Token::Identifier("while".to_string()), gen.get().token);
    assert_eq!(Token::LineTerminator, gen.get().token);
    assert_eq!(Token::Identifier("null".to_string()), gen.get().token);
    assert_eq!(Token::LineTerminator, gen.get().token);
    assert_eq!(Token::Identifier("false".to_string()), gen.get().token);
    assert_eq!(Token::LineTerminator, gen.get().token);
    assert_eq!(Token::Identifier("true".to_string()), gen.get().token);
    assert_eq!(Token::LineTerminator, gen.get().token);
    assert_eq!(Token::Identifier("NaN".to_string()), gen.get().token);
    assert_eq!(Token::LineTerminator, gen.get().token);
    assert_eq!(Token::Identifier("function".to_string()), gen.get().token);
    assert_eq!(Token::LineTerminator, gen.get().token);
    assert_eq!(Token::Identifier("variable_name".to_string()), gen.get().token);
    assert_eq!(Token::LineTerminator, gen.get().token);
}

#[test]
fn block_comment() {
    let data = std::fs::read("./tests/data/block_comment.js".to_string()).unwrap();
    let mut gen = TokenGenerator::new(stream::Stream::new(data));
    assert_eq!(Token::Comment(" // ".to_string()), gen.get().token);
    assert_eq!(Token::LineTerminator, gen.get().token);
    assert_eq!(Token::Comment("//".to_string()), gen.get().token);
    assert_eq!(Token::LineTerminator, gen.get().token);
    assert_eq!(Token::Comment("**".to_string()), gen.get().token);
    assert_eq!(Token::LineTerminator, gen.get().token);
    assert_eq!(Token::Comment("/*t".to_string()), gen.get().token);
    assert_eq!(Token::LineTerminator, gen.get().token);
    assert_eq!(Token::Comment("".to_string()), gen.get().token);
    assert_eq!(Token::LineTerminator, gen.get().token);
}
