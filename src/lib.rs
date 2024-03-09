mod lex;
mod parse;
pub mod arena;
pub mod def;
pub mod stream;

pub struct TokenGenerator<'s> {
    stream: stream::Stream<'s>
}

impl<'s> TokenGenerator<'s> {
    pub fn new(stream: stream::Stream<'s>) -> Self {
        Self { stream }
    }

    pub fn get(&mut self) -> def::PositionalToken {
        lex::generate_token(&mut self.stream)
    }
}

