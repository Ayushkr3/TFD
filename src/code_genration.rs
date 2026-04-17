use std::collections::HashMap;
use regex::Regex;

struct CodeGenerator {
    operator_stack: Vec<String>,
    operand_stack: Vec<String>,
    temp_count: i32,
    precedence: HashMap<String, i32>,
}

impl CodeGenerator {
    fn new() -> Self {
        let mut precedence = HashMap::new();
        precedence.insert("+".to_string(), 1);
        precedence.insert("-".to_string(), 1);
        precedence.insert("*".to_string(), 2);
        precedence.insert("/".to_string(), 2);
        precedence.insert("^".to_string(), 3);

        CodeGenerator {
            operator_stack: Vec::new(),
            operand_stack: Vec::new(),
            temp_count: 0,
            precedence,
        }
    }

    fn new_temp(&mut self) -> String {
        self.temp_count += 1;
        format!("t{}", self.temp_count)
    }

    fn generate_instruction(&mut self, op: &str, arg1: &str, arg2: &str) -> String {
        let temp_var = self.new_temp();
        println!("{} = {} {} {}", temp_var, arg1, op, arg2);
        temp_var
    }

    fn process_operator(&mut self) {
        if self.operand_stack.len() < 2 {
            return;
        }
        let op = self.operator_stack.pop().unwrap();
        let right = self.operand_stack.pop().unwrap();
        let left = self.operand_stack.pop().unwrap();
        let result_temp = self.generate_instruction(&op, &left, &right);
        self.operand_stack.push(result_temp);
    }

    fn parse_and_generate(&mut self, expression: &str) {
        println!("\n--- Generating Code for: {} ---", expression);
        println!("Generated Instructions:");

        let (lhs, expr) = if expression.contains('=') {
            let parts: Vec<&str> = expression.splitn(2, '=').collect();
            (Some(parts[0].trim()), parts[1].trim())
        } else {
            (None, expression)
        };

        let re = Regex::new(r"\d+|[a-zA-Z_]\w*|[-+*/^()]").unwrap();
        let tokens: Vec<&str> = re.find_iter(expr).map(|m| m.as_str()).collect();

        for token in tokens {
            if token.chars().all(|c| c.is_alphanumeric() || c == '_') {
                self.operand_stack.push(token.to_string());
            } else if token == "(" {
                self.operator_stack.push(token.to_string());
            } else if token == ")" {
                while !self.operator_stack.is_empty() && self.operator_stack.last().unwrap() != "(" {
                    self.process_operator();
                }
                if !self.operator_stack.is_empty() {
                    self.operator_stack.pop();
                }
            } else if self.precedence.contains_key(token) {
                while !self.operator_stack.is_empty() {
                    let last_op = self.operator_stack.last().unwrap().clone();
                    if self.precedence.contains_key(&last_op) {
                        if self.precedence[&last_op] >= self.precedence[token] {
                            self.process_operator();
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                self.operator_stack.push(token.to_string());
            }
        }

        while !self.operator_stack.is_empty() {
            self.process_operator();
        }

        if let Some(lhs_var) = lhs {
            if !self.operand_stack.is_empty() {
                let final_result = self.operand_stack.pop().unwrap();
                println!("{} = {}", lhs_var, final_result);
            }
        }
        println!("---------------------------------------");
    }

    fn reset(&mut self) {
        self.temp_count = 0;
        self.operator_stack.clear();
        self.operand_stack.clear();
    }
}

fn main() {
    let mut generator = CodeGenerator::new();
    println!("Compiler Design: Simple Code Generator");
    println!("Enter expressions like: x = a + b * c");
    println!("Type 'exit' to quit.\n");

    let stdin = std::io::stdin();
    let mut input = String::new();

    loop {
        input.clear();
        print!("Enter expression: ");
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        stdin.read_line(&mut input).unwrap();
        let user_input = input.trim();

        if user_input.to_lowercase() == "exit" {
            break;
        }

        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            generator.parse_and_generate(user_input);
        })) {
            Ok(_) => {
                generator.reset();
            }
            Err(_) => {
                eprintln!("Error parsing expression");
                generator.reset();
            }
        }
    }
}