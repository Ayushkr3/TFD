use std::collections::HashMap;

#[derive(Clone)]
struct TAC {
    op: String,
    arg1: String,
    arg2: Option<String>,
    result: String,
}

struct SimpleCodeGenerator {
    registers: HashMap<String, Option<String>>,
}

impl SimpleCodeGenerator {
    fn new() -> Self {
        let mut registers = HashMap::new();
        registers.insert("R0".to_string(), None);
        registers.insert("R1".to_string(), None);

        SimpleCodeGenerator { registers }
    }

    fn get_reg(&mut self, var: &str) -> String {
        for (reg, val) in &self.registers {
            if val.as_ref().map(|s| s.as_str()) == Some(var) {
                return reg.clone();
            }
        }

        for (reg, val) in &mut self.registers {
            if val.is_none() {
                *val = Some(var.to_string());
                return reg.clone();
            }
        }

        "R0".to_string()
    }

    fn generate(&mut self, tacs: &[TAC]) {
        for tac in tacs {
            let reg1 = self.get_reg(&tac.arg1);
            println!("MOV {}, {}", tac.arg1, reg1);

            if let Some(ref arg2) = tac.arg2 {
                let reg2 = self.get_reg(arg2);
                if self.registers[&reg2] != Some(arg2.clone()) {
                    println!("MOV {}, {}", arg2, reg2);
                }

                let op_map = [
                    ("+", "ADD"),
                    ("-", "SUB"),
                    ("*", "MUL"),
                    ("/", "DIV"),
                ];
                let op_name = op_map.iter().find(|p| p.0 == tac.op).map(|p| p.1).unwrap_or("UNKNOWN");
                println!("{} {}, {}", op_name, reg2, reg1);
            }

            self.registers.insert(reg1.clone(), Some(tac.result.clone()));
            println!("MOV {}, {}", reg1, tac.result);
        }
    }
}

fn main() {
    let tacs = vec![
        TAC {
            op: "*".to_string(),
            arg1: "b".to_string(),
            arg2: Some("c".to_string()),
            result: "t1".to_string(),
        },
        TAC {
            op: "+".to_string(),
            arg1: "a".to_string(),
            arg2: Some("t1".to_string()),
            result: "t2".to_string(),
        },
    ];

    let mut cg = SimpleCodeGenerator::new();
    cg.generate(&tacs);
}