use regex::Regex;
use std::collections::HashSet;
use std::io::{self, Write};

fn main() {
    let keywords: HashSet<&str> = [
        "int", "float", "if", "else", "while", "return", "for", "switch", "elif",
    ]
    .iter()
    .copied()
    .collect();

    let operators: HashSet<&str> = ["+", "-", "*", "/", "=", ">", "<"]
        .iter()
        .copied()
        .collect();

    let delimiters: HashSet<&str> = [";", ",", "(", ")", "{", "}"]
        .iter()
        .copied()
        .collect();

    println!("Enter the source code (end input with an empty line):");

    let mut lines = Vec::new();
    let stdin = io::stdin();
    let mut line = String::new();

    loop {
        line.clear();
        stdin.read_line(&mut line).unwrap();
        if line.trim().is_empty() {
            break;
        }
        lines.push(line.clone());
    }

    let code = lines.join("\n");

    println!("\nLEXICAL ANALYSIS OUTPUT:\n");

    let re = Regex::new(r"[A-Za-z_]\w*|\d+|\S").unwrap();
    let tokens: Vec<&str> = re.find_iter(&code).map(|m| m.as_str()).collect();

    for token in tokens {
        if keywords.contains(token) {
            println!("{:<10} → KEYWORD", token);
        } else if operators.contains(token) {
            println!("{:<10} → OPERATOR", token);
        } else if delimiters.contains(token) {
            println!("{:<10} → DELIMITER", token);
        } else if token.chars().all(|c| c.is_numeric()) {
            println!("{:<10} → NUMBER", token);
        } else {
            println!("{:<10} → IDENTIFIER", token);
        }
    }
}