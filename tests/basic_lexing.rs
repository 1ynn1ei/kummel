use kml::TokenGenerator;
use kml::def::Token;
use kml::stream;

#[test]
fn string() {
    let data = std::fs::read("./tests/data/string.js").unwrap();
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
fn punctuator() {
    let data = std::fs::read("./tests/data/punctuator.js").unwrap();
    let mut gen = TokenGenerator::new(stream::Stream::new(&data));
    assert_eq!(Token::Punctuator("&&"), gen.get().token);
    assert_eq!(Token::LineTerminator, gen.get().token);
    assert_eq!(Token::Punctuator("||"), gen.get().token);
    assert_eq!(Token::LineTerminator, gen.get().token);
    assert_eq!(Token::Punctuator("??"), gen.get().token);
    assert_eq!(Token::LineTerminator, gen.get().token);
    assert_eq!(Token::Punctuator("<<"), gen.get().token);
    assert_eq!(Token::LineTerminator, gen.get().token);
    assert_eq!(Token::Punctuator(">>"), gen.get().token);
    assert_eq!(Token::LineTerminator, gen.get().token);
    assert_eq!(Token::Punctuator("--"), gen.get().token);
    assert_eq!(Token::LineTerminator, gen.get().token);
    assert_eq!(Token::Punctuator("++"), gen.get().token);
    assert_eq!(Token::LineTerminator, gen.get().token);
    assert_eq!(Token::Punctuator("?."), gen.get().token);
    assert_eq!(Token::LineTerminator, gen.get().token);
    assert_eq!(Token::Punctuator("<="), gen.get().token);
    assert_eq!(Token::LineTerminator, gen.get().token);
    assert_eq!(Token::Punctuator(">="), gen.get().token);
    assert_eq!(Token::LineTerminator, gen.get().token);
    assert_eq!(Token::Punctuator("=="), gen.get().token);
    assert_eq!(Token::LineTerminator, gen.get().token);
    assert_eq!(Token::Punctuator("==="), gen.get().token);
    assert_eq!(Token::LineTerminator, gen.get().token);
    assert_eq!(Token::Punctuator("!="), gen.get().token);
    assert_eq!(Token::LineTerminator, gen.get().token);
}

#[test]
fn numeric() {
    let data = std::fs::read("./tests/data/numeric.js").unwrap();
    let mut gen = TokenGenerator::new(stream::Stream::new(&data));
    assert_eq!(Token::Numeric("0"), gen.get().token);
    assert_eq!(Token::LineTerminator, gen.get().token);
    assert_eq!(Token::Numeric("0.9"), gen.get().token);
    assert_eq!(Token::LineTerminator, gen.get().token);
    assert_eq!(Token::Numeric("9_999"), gen.get().token);
    assert_eq!(Token::LineTerminator, gen.get().token);
    assert_eq!(Token::Numeric("1e9"), gen.get().token);
    assert_eq!(Token::LineTerminator, gen.get().token);
    assert_eq!(Token::Numeric("0.1e-9"), gen.get().token);
    assert_eq!(Token::LineTerminator, gen.get().token);
    assert_eq!(Token::Numeric("0b11"), gen.get().token);
    assert_eq!(Token::LineTerminator, gen.get().token);
    assert_eq!(Token::Numeric("0xd"), gen.get().token);
    assert_eq!(Token::LineTerminator, gen.get().token);
    assert_eq!(Token::Numeric("0007"), gen.get().token);
    assert_eq!(Token::LineTerminator, gen.get().token);
}

#[test]
fn inline_comment() {
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

#[test]
fn identifier() {
    let data = std::fs::read("./tests/data/identifier.js").unwrap();
    let mut gen = TokenGenerator::new(stream::Stream::new(&data));
    assert_eq!(Token::Identifier("if"), gen.get().token);
    assert_eq!(Token::LineTerminator, gen.get().token);
    assert_eq!(Token::Identifier("var"), gen.get().token);
    assert_eq!(Token::LineTerminator, gen.get().token);
    assert_eq!(Token::Identifier("while"), gen.get().token);
    assert_eq!(Token::LineTerminator, gen.get().token);
    assert_eq!(Token::Identifier("null"), gen.get().token);
    assert_eq!(Token::LineTerminator, gen.get().token);
    assert_eq!(Token::Identifier("false"), gen.get().token);
    assert_eq!(Token::LineTerminator, gen.get().token);
    assert_eq!(Token::Identifier("true"), gen.get().token);
    assert_eq!(Token::LineTerminator, gen.get().token);
    assert_eq!(Token::Identifier("NaN"), gen.get().token);
    assert_eq!(Token::LineTerminator, gen.get().token);
    assert_eq!(Token::Identifier("function"), gen.get().token);
    assert_eq!(Token::LineTerminator, gen.get().token);
    assert_eq!(Token::Identifier("variable_name"), gen.get().token);
    assert_eq!(Token::LineTerminator, gen.get().token);
}

#[test]
fn block_comment() {
    let data = std::fs::read("./tests/data/block_comment.js").unwrap();
    let mut gen = TokenGenerator::new(stream::Stream::new(&data));
    assert_eq!(Token::Comment(" // "), gen.get().token);
    assert_eq!(Token::LineTerminator, gen.get().token);
    assert_eq!(Token::Comment("//"), gen.get().token);
    assert_eq!(Token::LineTerminator, gen.get().token);
    assert_eq!(Token::Comment("**"), gen.get().token);
    assert_eq!(Token::LineTerminator, gen.get().token);
    assert_eq!(Token::Comment("/*t"), gen.get().token);
    assert_eq!(Token::LineTerminator, gen.get().token);
    assert_eq!(Token::Comment(""), gen.get().token);
    assert_eq!(Token::LineTerminator, gen.get().token);
}
