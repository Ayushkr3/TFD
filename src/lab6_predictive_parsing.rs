use std::collections::{HashMap, HashSet};

struct PredictiveParser {
    grammar: HashMap<String, Vec<String>>,
    ff: FirstFollow,
    table: HashMap<String, HashMap<String, String>>,
}

struct FirstFollow {
    first: HashMap<String, HashSet<String>>,
    follow: HashMap<String, HashSet<String>>,
}

impl PredictiveParser {
    fn new(grammar: HashMap<String, Vec<String>>, start_symbol: String) -> Self {
        let ff = FirstFollow::new(&grammar, start_symbol);
        let mut parser = PredictiveParser {
            grammar,
            ff,
            table: HashMap::new(),
        };
        parser.build_table();
        parser
    }

    fn build_table(&mut self) {
        for (non_term, productions) in &self.grammar {
            self.table.insert(non_term.clone(), HashMap::new());
            for prod in productions {
                let first_alpha = self.get_first_of_string(prod);
                for terminal in &first_alpha {
                    if terminal != "ε" {
                        self.table
                            .get_mut(non_term)
                            .unwrap()
                            .insert(terminal.clone(), prod.clone());
                    }
                }
                if first_alpha.contains(&"ε".to_string()) {
                    if let Some(follow_set) = self.ff.follow.get(non_term) {
                        for terminal in follow_set {
                            self.table
                                .get_mut(non_term)
                                .unwrap()
                                .insert(terminal.clone(), prod.clone());
                        }
                    }
                }
            }
        }
    }

    fn get_first_of_string(&self, string: &str) -> HashSet<String> {
        let mut res = HashSet::new();
        if string == "ε" {
            res.insert("ε".to_string());
            return res;
        }

        let mut all_have_eps = true;
        for ch in string.chars() {
            if let Some(first_set) = self.ff.first.get(&ch.to_string()) {
                for item in first_set {
                    if item != "ε" {
                        res.insert(item.clone());
                    }
                }
                if !first_set.contains(&"ε".to_string()) {
                    all_have_eps = false;
                    break;
                }
            } else {
                res.insert(ch.to_string());
                all_have_eps = false;
                break;
            }
        }

        if all_have_eps {
            res.insert("ε".to_string());
        }

        res
    }
}

impl FirstFollow {
    fn new(grammar: &HashMap<String, Vec<String>>, start: String) -> Self {
        let mut ff = FirstFollow {
            first: HashMap::new(),
            follow: HashMap::new(),
        };

        for non_term in grammar.keys() {
            ff.first.insert(non_term.clone(), HashSet::new());
            ff.follow.insert(non_term.clone(), HashSet::new());
        }

        ff.follow.insert(start.clone(), {
            let mut s = HashSet::new();
            s.insert("$".to_string());
            s
        });

        ff
    }
}

fn main() {
    let mut grammar = HashMap::new();
    grammar.insert("S".to_string(), vec!["aABb".to_string()]);
    grammar.insert("A".to_string(), vec!["c".to_string(), "ε".to_string()]);
    grammar.insert("B".to_string(), vec!["d".to_string()]);

    let parser = PredictiveParser::new(grammar, "S".to_string());
    println!("Parse Table: {:?}", parser.table);
}