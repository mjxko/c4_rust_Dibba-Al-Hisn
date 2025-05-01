mod lexer;
mod parser;
mod vm;

use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::vm::VM;
use std::env;
use std::fs;



fn main() {
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
    parser.parse_program();

    println!("\n Instructions:");
    for inst in &parser.instructions {
        println!("{}", inst);
    }

    let mut vm = VM::new(parser.instructions);
    vm.run();
}
