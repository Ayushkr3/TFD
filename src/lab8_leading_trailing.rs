use std::collections::{HashMap, HashSet};

struct OperatorPrecedenceSets {
    grammar: HashMap<String, Vec<String>>,
    leading: HashMap<String, HashSet<String>>,
    trailing: HashMap<String, HashSet<String>>,
}

impl OperatorPrecedenceSets {
    fn new(grammar: HashMap<String, Vec<String>>) -> Self {
        let mut ops = OperatorPrecedenceSets {
            grammar,
            leading: HashMap::new(),
            trailing: HashMap::new(),
        };
        ops.compute_leading();
        ops.compute_trailing();
        ops
    }

    fn compute_leading(&mut self) {
        for non_term in self.grammar.keys() {
            self.leading.insert(non_term.clone(), HashSet::new());
        }

        let mut changed = true;
        while changed {
            changed = false;
            for (lhs, rhs_list) in &self.grammar {
                let before = self.leading[lhs].len();
                for rhs in rhs_list {
                    if !rhs.is_empty() {
                        let first_char = rhs.chars().next().unwrap();
                        if !first_char.is_uppercase() {
                            self.leading.get_mut(lhs).unwrap().insert(first_char.to_string());
                        } else {
                            if let Some(set) = self.leading.get(&first_char.to_string()) {
                                for item in set.clone() {
                                    self.leading.get_mut(lhs).unwrap().insert(item);
                                }
                            }
                            if rhs.len() > 1 {
                                let second_char = rhs.chars().nth(1).unwrap();
                                if !second_char.is_uppercase() {
                                    self.leading.get_mut(lhs).unwrap().insert(second_char.to_string());
                                }
                            }
                        }
                    }
                }
                if self.leading[lhs].len() > before {
                    changed = true;
                }
            }
        }
    }

    fn compute_trailing(&mut self) {
        for non_term in self.grammar.keys() {
            self.trailing.insert(non_term.clone(), HashSet::new());
        }

        let mut changed = true;
        while changed {
            changed = false;
            for (lhs, rhs_list) in &self.grammar {
                let before = self.trailing[lhs].len();
                for rhs in rhs_list {
                    if !rhs.is_empty() {
                        let last_char = rhs.chars().last().unwrap();
                        if !last_char.is_uppercase() {
                            self.trailing.get_mut(lhs).unwrap().insert(last_char.to_string());
                        } else {
                            if let Some(set) = self.trailing.get(&last_char.to_string()) {
                                for item in set.clone() {
                                    self.trailing.get_mut(lhs).unwrap().insert(item);
                                }
                            }
                            if rhs.len() > 1 {
                                let second_last = rhs.chars().rev().nth(1).unwrap();
                                if !second_last.is_uppercase() {
                                    self.trailing.get_mut(lhs).unwrap().insert(second_last.to_string());
                                }
                            }
                        }
                    }
                }
                if self.trailing[lhs].len() > before {
                    changed = true;
                }
            }
        }
    }
}

fn main() {
    let mut grammar = HashMap::new();
    grammar.insert("E".to_string(), vec!["E+T".to_string(), "T".to_string()]);
    grammar.insert("T".to_string(), vec!["T*F".to_string(), "F".to_string()]);
    grammar.insert("F".to_string(), vec!["(E)".to_string(), "id".to_string()]);

    let ops = OperatorPrecedenceSets::new(grammar);
    println!("LEADING: {:?}", ops.leading);
    println!("TRAILING: {:?}", ops.trailing);
}