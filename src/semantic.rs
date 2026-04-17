use std::collections::HashSet;
use std::io::{self, Write};

fn run_semantic() {
    println!("\n--- PHASE 3: SEMANTIC ANALYSIS ---\n");
    println!("Enter statements (type 'exit' to stop)\n");

    let mut symbol_table: HashSet<String> = HashSet::new();
    let stdin = io::stdin();
    let mut line = String::new();

    loop {
        line.clear();
        print!(">> ");
        io::stdout().flush().unwrap();
        stdin.read_line(&mut line).unwrap();
        let line = line.trim();

        if line.to_lowercase() == "exit" {
            break;
        }

        if line.is_empty() {
            continue;
        }

        let line = if line.ends_with(';') {
            &line[..line.len() - 1]
        } else {
            line
        };

        if line.starts_with("int") {
            let parts: Vec<&str> = line.split_whitespace().collect();

            if parts.len() >= 2 {
                let variables: Vec<&str> = parts[1].split(',').collect();

                for var in variables {
                    let var = var.trim();

                    if symbol_table.contains(var) {
                        println!("ERROR: {} already declared", var);
                    } else {
                        symbol_table.insert(var.to_string());
                        println!("Declared: {}", var);
                    }
                }
            } else {
                println!("Invalid declaration");
            }
        } else if line.contains('=') {
            if let Some(idx) = line.find('=') {
                let lhs = line[..idx].trim();
                let _rhs = line[idx + 1..].trim();

                if !symbol_table.contains(lhs) {
                    println!("ERROR: {} not declared", lhs);
                } else {
                    println!("OK: {}", line);
                }
            }
        } else {
            println!("Invalid statement");
        }
    }

    println!("\nFinal Symbol Table: {:?}", symbol_table);
}

fn main() {
    run_semantic();
}