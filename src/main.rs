#![allow(dead_code, unused)]
mod definition;
mod arena;
mod stream;
mod lexer;
use std::fs;
use lexer::Lexer;
use clap::Parser;
use clap::Subcommand;

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
        },
        Err(e) => {
            println!("{}", e);
        }
    }
}
