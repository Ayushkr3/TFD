#![allow(warnings)]
mod lexer;

use lexer::Lexer;
use std::fs;
use std::process;

fn main() {
    let filename = "source/main.jpp";
    let source = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", filename, e);
            process::exit(1);
        }
    };
    let mut lexer = Lexer::new(source);
    lexer.tokenize().unwrap_or_else(|e|{
    eprintln!("Error: {}", e);
    process::exit(1);
    });
    let tokens = lexer.get_tokens();
    
}