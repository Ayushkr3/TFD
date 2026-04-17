use std::collections::{HashSet, HashMap};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Item {
    lhs: String,
    rhs: String,
    dot_pos: usize,
}

struct LR0 {
    grammar: HashMap<String, Vec<String>>,
    start: String,
}

impl LR0 {
    fn new(mut grammar: HashMap<String, Vec<String>>, start_symbol: String) -> Self {
        let augmented_start = format!("{}\'", start_symbol);
        grammar.insert(augmented_start.clone(), vec![start_symbol.clone()]);

        LR0 {
            grammar,
            start: augmented_start,
        }
    }

    fn closure(&self, items: HashSet<Item>) -> HashSet<Item> {
        let mut closure_set = items;
        let mut changed = true;

        while changed {
            changed = false;
            let mut new_items = HashSet::new();

            for item in &closure_set {
                if item.dot_pos < item.rhs.len() {
                    if let Some(ch) = item.rhs.chars().nth(item.dot_pos) {
                        if ch.is_uppercase() {
                            let s = ch.to_string();
                            if let Some(productions) = self.grammar.get(&s) {
                                for prod in productions {
                                    new_items.insert(Item {
                                        lhs: s.clone(),
                                        rhs: prod.clone(),
                                        dot_pos: 0,
                                    });
                                }
                            }
                        }
                    }
                }
            }

            for item in new_items {
                if !closure_set.contains(&item) {
                    closure_set.insert(item);
                    changed = true;
                }
            }
        }

        closure_set
    }

    fn goto(&self, items: &HashSet<Item>, symbol: &str) -> HashSet<Item> {
        let mut moved = HashSet::new();
        for item in items {
            if item.dot_pos < item.rhs.len() {
                if let Some(ch) = item.rhs.chars().nth(item.dot_pos) {
                    if ch.to_string() == symbol {
                        moved.insert(Item {
                            lhs: item.lhs.clone(),
                            rhs: item.rhs.clone(),
                            dot_pos: item.dot_pos + 1,
                        });
                    }
                }
            }
        }
        self.closure(moved)
    }
}

fn main() {
    let mut grammar = HashMap::new();
    grammar.insert("E".to_string(), vec!["E+T".to_string(), "T".to_string()]);
    grammar.insert("T".to_string(), vec!["id".to_string()]);

    let lr0 = LR0::new(grammar, "E".to_string());
    let mut i0 = HashSet::new();
    i0.insert(Item {
        lhs: "E\'".to_string(),
        rhs: "E".to_string(),
        dot_pos: 0,
    });

    let closure = lr0.closure(i0);
    println!("I0: {:?}", closure);
}