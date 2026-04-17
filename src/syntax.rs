use regex::Regex;
use std::io::{self, Write};

fn syntax_analyzer(statement: &str) {
    let statement = statement.trim();

    let pattern = r"^[a-zA-Z_]\w*\s*=\s*([a-zA-Z_]\w*|\d+)(\s*[+\-*/]\s*([a-zA-Z_]\w*|\d+))?$";
    let re = Regex::new(pattern).unwrap();

    if re.is_match(statement) {
        println!("VALID SYNTAX   : {}", statement);
    } else {
        println!("INVALID SYNTAX : {}", statement);
    }
}

fn main() {
    println!("SYNTAX ANALYSIS RESULT:\n");
    println!("Type 'exit' to stop.\n");

    let stdin = io::stdin();
    let mut line = String::new();

    loop {
        line.clear();
        print!("Enter a statement: ");
        io::stdout().flush().unwrap();
        stdin.read_line(&mut line).unwrap();
        let statement = line.trim();

        if statement.to_lowercase() == "exit" {
            println!("Program terminated.");
            break;
        }

        if !statement.is_empty() {
            syntax_analyzer(statement);
        }
    }
}