use std::io::{self, Write};

fn run_intermediate() {
    println!("\n--- PHASE 4: INTERMEDIATE CODE GENERATION ---\n");
    println!("Enter statements (type 'exit' to stop)\n");

    let mut temp = 1;
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

        if line.contains('=') && line.contains('+') {
            if let Some(eq_idx) = line.find('=') {
                let lhs = line[..eq_idx].trim();
                let rhs = line[eq_idx + 1..].trim();

                let parts: Vec<&str> = rhs.splitn(2, '+').collect();
                if parts.len() == 2 {
                    let a = parts[0].trim();
                    let b = parts[1].trim();

                    println!("t{} = {} + {}", temp, a, b);
                    println!("{} = t{}", lhs, temp);
                    temp += 1;
                }
            }
        } else if line.contains('=') {
            println!("{}", line);
        } else {
            println!("Invalid statement");
        }
    }

    println!("\nIntermediate Code Generation Finished.");
}

fn main() {
    run_intermediate();
}