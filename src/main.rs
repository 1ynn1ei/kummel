#![allow(dead_code, unused)]
mod def;
mod arena;
mod stream;
mod lex;
mod parse;
use std::fs;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    path: String,
    #[arg(short, long)]
    output: Option<String>
}

fn main() {
    let cli = Cli::parse();
    let file = cli.path;
    match fs::read(file) {
        Ok(data) => {
            let mut stream = stream::Stream::new(data);
            let mut tokens : Vec<def::PositionalToken> = Vec::new();
            loop {
                let token = lex::generate_token(&mut stream);
                match token.token {
                    // def::Token::WhiteSpace => { },
                    _ => println!("[TOKEN GENERATED: {:?}]", token.token)
                }
                if let def::Token::EndOfFile = token.token {
                    break;
                }
                tokens.push(token);
            }
            println!("here");
            println!("{:?}", parse::make_tree(tokens));
        },
        Err(e) => {
            println!("{}", e);
        }
    }
}
