use std::collections::HashMap;

struct GrammarTransformer {
    grammar: HashMap<String, Vec<String>>,
}

impl GrammarTransformer {
    fn new(grammar: HashMap<String, Vec<String>>) -> Self {
        GrammarTransformer { grammar }
    }

    fn eliminate_left_recursion(&mut self) {
        let mut new_grammar: HashMap<String, Vec<String>> = HashMap::new();

        for (non_term, productions) in &self.grammar {
            let alphas: Vec<String> = productions
                .iter()
                .filter(|p| p.starts_with(non_term))
                .map(|p| p[non_term.len()..].to_string())
                .collect();

            let betas: Vec<String> = productions
                .iter()
                .filter(|p| !p.starts_with(non_term))
                .cloned()
                .collect();

            if alphas.is_empty() {
                new_grammar.insert(non_term.clone(), productions.clone());
                continue;
            }

            let prime = format!("{}\'", non_term);
            for beta in &betas {
                new_grammar
                    .entry(non_term.clone())
                    .or_insert_with(Vec::new)
                    .push(format!("{}{}", beta, prime));
            }

            for alpha in &alphas {
                new_grammar
                    .entry(prime.clone())
                    .or_insert_with(Vec::new)
                    .push(format!("{}{}", alpha, prime));
            }

            new_grammar
                .entry(prime)
                .or_insert_with(Vec::new)
                .push("ε".to_string());
        }

        self.grammar = new_grammar;
    }

    fn left_factor(&mut self) {
        let mut new_grammar: HashMap<String, Vec<String>> = HashMap::new();

        for (non_term, productions) in &self.grammar {
            let mut prefix_map: HashMap<char, Vec<String>> = HashMap::new();

            for p in productions {
                let prefix = if p == "ε" { 'ε' } else { p.chars().next().unwrap() };
                prefix_map.entry(prefix).or_insert_with(Vec::new).push(p.clone());
            }

            for (prefix, prods) in prefix_map {
                if prods.len() > 1 {
                    let prime = format!("{}\'\'", non_term);
                    new_grammar
                        .entry(non_term.clone())
                        .or_insert_with(Vec::new)
                        .push(format!("{}{}", prefix, prime));

                    for p in prods {
                        let remainder = if p.len() > 1 {
                            p[1..].to_string()
                        } else {
                            "ε".to_string()
                        };
                        new_grammar
                            .entry(prime.clone())
                            .or_insert_with(Vec::new)
                            .push(remainder);
                    }
                } else {
                    new_grammar
                        .entry(non_term.clone())
                        .or_insert_with(Vec::new)
                        .extend(prods);
                }
            }
        }

        self.grammar = new_grammar;
    }
}

fn main() {
    let mut grammar = HashMap::new();
    grammar.insert("E".to_string(), vec!["E+T".to_string(), "T".to_string()]);

    let mut gt = GrammarTransformer::new(grammar);
    gt.eliminate_left_recursion();
    println!("{:?}", gt.grammar);
}