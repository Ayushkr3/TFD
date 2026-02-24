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
    
    match lexer.tokenize() {
        Ok(()) => {
            // Print token stream
            lexer.print_token_stream();
            
            // Print symbol table
            lexer.get_symbol_table().print();
            
            // Generate and save JSON output
            let json_output = lexer.to_json();
            let json_filename = filename.replace(".mcpp", "_tokens.json");
            match fs::write(&json_filename, &json_output) {
                Ok(_) => println!("\nJSON output saved to: {}", json_filename),
                Err(e) => eprintln!("Warning: Could not write JSON file: {}", e),
            }
            
            println!("\n=== Lexical Analysis Complete ===");
            println!("Total tokens: {}", lexer.get_tokens().len());
        }
        Err(e) => {
            eprintln!("\n{}", e);
            process::exit(1);
        }
    }
}