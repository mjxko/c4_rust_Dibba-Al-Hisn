// src/main.rs

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

    // Get the C source file from command line args
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <source.c>", args[0]);
        return;
    }

    // Read the source code file
    let source_path = &args[1];
    let source_code = match fs::read_to_string(source_path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Failed to read file '{}': {}", source_path, err);
            return;
        }
    };

    // Initialize the lexer
    let mut lexer = Lexer::new(&source_code);

    println!("📜 Starting tokenization...");
    while let Some(token) = lexer.next_token() {
        println!("{:?}", token);
        if token == token::Token::Eof {
            break;
        }
    }

    // (parser and vm to be added in future steps)
}
