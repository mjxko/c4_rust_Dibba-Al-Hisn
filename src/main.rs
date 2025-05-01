// main.rs

mod lexer;
mod parser;
mod token;
mod vm;

use std::env;
use std::fs;
use lexer::Lexer;
use parser::Parser;
use vm::{VM, OpCode};

fn main() {
    println!("Rewriting the C4 Compiler in Rust");

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <source.c>", args[0]);
        return;
    }

    let source_path = &args[1];
    let source_code = match fs::read_to_string(source_path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Failed to read file '{}': {}", source_path, err);
            return;
        }
    };

    let lexer = Lexer::new(&source_code);
    let mut parser = Parser::new(lexer);

    println!("parsing expression:");
    parser.parse_expression(1);

    println!("running the virtual machine:");
    let program = vec![OpCode::Imm, OpCode::Imm, OpCode::Add, OpCode::Exit];
    let mut vm = VM::new(program);
    vm.run();
}
