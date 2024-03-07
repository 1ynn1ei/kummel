#![allow(dead_code, unused)]
mod def;
mod arena;
mod stream;
mod lex;
use std::fs;
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
            let mut stream = stream::Stream::new(&data);
            loop {
                let token = lex::generate_token(&mut stream);
                println!("[TOKEN GENERATED: {:?}]", token);
                if let def::Token::EndOfFile = token.token {
                    return;
                }
            }
        },
        Err(e) => {
            println!("{}", e);
        }
    }
}
