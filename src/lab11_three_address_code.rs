#[derive(Clone)]
struct TAC {
    op: String,
    arg1: String,
    arg2: Option<String>,
    result: String,
}

struct TACRepresentations {
    code: Vec<TAC>,
}

impl TACRepresentations {
    fn new(code: Vec<TAC>) -> Self {
        TACRepresentations { code }
    }

    fn print_quadruple(&self) {
        println!("{:<6} {:<6} {:<6} {:<6}", "OP", "ARG1", "ARG2", "RESULT");
        for c in &self.code {
            let arg2_str = c.arg2.as_ref().map(|s| s.as_str()).unwrap_or("-");
            println!("{:<6} {:<6} {:<6} {:<6}", c.op, c.arg1, arg2_str, c.result);
        }
    }

    fn print_triple(&self) {
        println!("{:<4} {:<6} {:<6} {:<6}", "ID", "OP", "ARG1", "ARG2");
        for (i, c) in self.code.iter().enumerate() {
            let arg1 = if self.is_temp(&c.arg1) {
                format!("({})", self.find_res(&c.arg1))
            } else {
                c.arg1.clone()
            };
            let arg2 = if let Some(ref a2) = c.arg2 {
                if self.is_temp(a2) {
                    format!("({})", self.find_res(a2))
                } else {
                    a2.clone()
                }
            } else {
                "-".to_string()
            };
            println!("({:<2}) {:<6} {:<6} {:<6}", i, c.op, arg1, arg2);
        }
    }

    fn is_temp(&self, var: &str) -> bool {
        var.starts_with('t')
    }

    fn find_res(&self, res: &str) -> i32 {
        for (i, c) in self.code.iter().enumerate() {
            if c.result == res {
                return i as i32;
            }
        }
        -1
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

    let rep = TACRepresentations::new(tacs);
    println!("Quadruples:");
    rep.print_quadruple();
    println!("\nTriples:");
    rep.print_triple();
}