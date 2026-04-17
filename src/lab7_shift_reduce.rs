use std::io::{self, Write};

struct ShiftReduceParser {
    rules: Vec<(String, String)>,
}

impl ShiftReduceParser {
    fn new(rules: Vec<(String, String)>) -> Self {
        ShiftReduceParser { rules }
    }

    fn parse(&self, tokens: Vec<&str>) {
        let mut stack: Vec<&str> = Vec::new();
        let mut buffer: Vec<&str> = tokens;
        buffer.push("$");

        println!("{:<20} | {:<20} | {}", "STACK", "BUFFER", "ACTION");

        loop {
            let stack_str = stack.join("");
            let buffer_str = buffer.join("");
            print!("{:<20} | {:<20} | ", stack_str, buffer_str);
            io::stdout().flush().unwrap();

            let mut reduced = false;
            for (lhs, rhs) in &self.rules {
                if stack_str.ends_with(rhs) {
                    for _ in 0..rhs.len() {
                        stack.pop();
                    }
                    stack.push(lhs);
                    println!("REDUCE {} -> {}", lhs, rhs);
                    reduced = true;
                    break;
                }
            }

            if !reduced {
                if buffer[0] == "$" {
                    if stack.len() == 1 && stack[0] == self.rules[0].0 {
                        println!("ACCEPT");
                        return;
                    } else {
                        println!("REJECT");
                        return;
                    }
                }
                stack.push(buffer.remove(0));
                println!("SHIFT");
            }
        }
    }
}

fn main() {
    let rules = vec![
        ("E".to_string(), "E+E".to_string()),
        ("E".to_string(), "E*E".to_string()),
        ("E".to_string(), "id".to_string()),
    ];

    let sr = ShiftReduceParser::new(rules);
    sr.parse(vec!["id", "+", "id", "*", "id"]);
}