use kml::TokenGenerator;
use kml::def::Token;
use kml::stream;

#[test]
fn mathmatical_expressions() {
    let data = std::fs::read(
        "./tests/data/mathematical_expression.js"
        ).unwrap();
    let mut gen = TokenGenerator::new(stream::Stream::new(data));
}
