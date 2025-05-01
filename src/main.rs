mod lexer;
mod parser;
mod token;
mod utils;
mod vm;

use std::env;
use std::fs;
use lexer::Lexer;

fn main() {
    println!("this runsssssssss");

    // gets the C source file from command line args
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <source.c>", args[0]);
        return;
    }

    // reads our source codes file
    let source_path = &args[1];
    let source_code = match fs::read_to_string(source_path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Failed to read file '{}': {}", source_path, err);
            return;
        }
    };

    // initializing our lexer
    let mut lexer = Lexer::new(&source_code);

    println!("📜 Starting tokenization...");
    while let Some(token) = lexer.next_token() {
        println!("{:?}", token);
        if token == token::Token::Eof {
            break;
        }
    }

}
