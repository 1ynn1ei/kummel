#![allow(dead_code, unused)]
mod definition;
mod arena;
mod stream;
mod lexer;
use std::fs;
use lexer::Lexer;

fn main() {
    let data = fs::read("./sample.js").unwrap();
    let mut lexer = Lexer::new(&data);
    loop {
        let token = lexer.next_token();
        if let Ok(Some(token)) = token {
            println!("{:?}", token);
            match token.token {
                definition::Token::EndOfFile => {
                    return;
                },
                _ => { continue }
            }
        }
    }
}
