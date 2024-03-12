#![allow(dead_code, unused)]
mod lex;
mod parse;
pub mod arena;
pub mod def;
pub mod stream;

pub struct TokenGenerator {
    stream: stream::Stream
}

impl TokenGenerator {
    pub fn new(stream: stream::Stream) -> Self {
        Self { stream }
    }

    pub fn get(&mut self) -> def::PositionalToken {
        lex::generate_token(&mut self.stream)
    }
}

