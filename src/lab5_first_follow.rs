use std::collections::{HashMap, HashSet};

struct FirstFollow {
    grammar: HashMap<String, Vec<String>>,
    start: String,
    first: HashMap<String, HashSet<String>>,
    follow: HashMap<String, HashSet<String>>,
}

impl FirstFollow {
    fn new(grammar: HashMap<String, Vec<String>>, start_symbol: String) -> Self {
        let mut ff = FirstFollow {
            grammar,
            start: start_symbol,
            first: HashMap::new(),
            follow: HashMap::new(),
        };
        ff.compute_first();
        ff.compute_follow();
        ff
    }

    fn compute_first(&mut self) {
        for non_term in self.grammar.keys() {
            self.first.insert(non_term.clone(), HashSet::new());
        }

        let mut changed = true;
        while changed {
            changed = false;
            for (non_term, productions) in &self.grammar {
                let before_len = self.first[non_term].len();

                for prod in productions {
                    if prod == "ε" {
                        self.first.get_mut(non_term).unwrap().insert("ε".to_string());
                    } else {
                        let mut all_have_eps = true;
                        for ch in prod.chars() {
                            if let Some(first_set) = self.first.get(&ch.to_string()) {
                                for item in first_set {
                                    if item != "ε" {
                                        self.first.get_mut(non_term).unwrap().insert(item.clone());
                                    }
                                }
                                if !first_set.contains("ε") {
                                    all_have_eps = false;
                                    break;
                                }
                            } else {
                                self.first.get_mut(non_term).unwrap().insert(ch.to_string());
                                all_have_eps = false;
                                break;
                            }
                        }
                        if all_have_eps {
                            self.first.get_mut(non_term).unwrap().insert("ε".to_string());
                        }
                    }
                }

                if self.first[non_term].len() > before_len {
                    changed = true;
                }
            }
        }
    }

    fn compute_follow(&mut self) {
        for _ in self.grammar.keys() {
            self.follow.insert(self.start.clone(), HashSet::new());
        }
        self.follow.get_mut(&self.start).unwrap().insert("$".to_string());

        let mut changed = true;
        while changed {
            changed = false;
            for (head, productions) in &self.grammar {
                for prod in productions {
                    for (i, ch) in prod.chars().enumerate() {
                        if ch.is_uppercase() {
                            let s = ch.to_string();
                            let before_len = self.follow.get(&s).map(|s| s.len()).unwrap_or(0);

                            if i + 1 < prod.len() {
                                let next_ch = prod.chars().nth(i + 1).unwrap();
                                if let Some(next_first) = self.first.get(&next_ch.to_string()) {
                                    for item in next_first {
                                        if item != "ε" {
                                            self.follow
                                                .entry(s.clone())
                                                .or_insert_with(HashSet::new)
                                                .insert(item.clone());
                                        }
                                    }
                                    if next_first.contains("ε") {
                                        if let Some(head_follow) = self.follow.get(head) {
                                            for item in head_follow {
                                                self.follow
                                                    .entry(s.clone())
                                                    .or_insert_with(HashSet::new)
                                                    .insert(item.clone());
                                            }
                                        }
                                    }
                                }
                            } else {
                                if let Some(head_follow) = self.follow.get(head) {
                                    for item in head_follow {
                                        self.follow
                                            .entry(s.clone())
                                            .or_insert_with(HashSet::new)
                                            .insert(item.clone());
                                    }
                                }
                            }

                            let after_len = self.follow.get(&s).map(|s| s.len()).unwrap_or(0);
                            if after_len > before_len {
                                changed = true;
                            }
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    let mut grammar = HashMap::new();
    grammar.insert("S".to_string(), vec!["aABb".to_string()]);
    grammar.insert("A".to_string(), vec!["c".to_string(), "ε".to_string()]);
    grammar.insert("B".to_string(), vec!["d".to_string()]);

    let ff = FirstFollow::new(grammar, "S".to_string());
    println!("FIRST: {:?}", ff.first);
    println!("FOLLOW: {:?}", ff.follow);
}