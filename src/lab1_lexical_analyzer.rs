use regex::Regex;

#[derive(Debug, Clone)]
struct Token {
    token_type: String,
    value: String,
    line: usize,
    column: usize,
}

struct LexicalAnalyzer {
    rules: Vec<(String, String)>,
    regex: Regex,
}

impl LexicalAnalyzer {
    fn new() -> Self {
        let rules = vec![
            ("COMMENT".to_string(), r"//.*|/\*[\s\S]*?\*/".to_string()),
            ("KEYWORD".to_string(), r"\b(if|else|while|return|int|float|void)\b".to_string()),
            ("ID".to_string(), r"[a-zA-Z_]\w*".to_string()),
            ("FLOAT".to_string(), r"\d+\.\d+".to_string()),
            ("INT".to_string(), r"\d+".to_string()),
            ("OP".to_string(), r"[+\-*/%=<>!&|]+".to_string()),
            ("PUNC".to_string(), r"[;,\{\}\(\)\[\]]".to_string()),
            ("SPACE".to_string(), r"[ \t]+".to_string()),
            ("NEWLINE".to_string(), r"\n".to_string()),
            ("MISMATCH".to_string(), r".".to_string()),
        ];

        let pattern = rules
            .iter()
            .map(|(name, pat)| format!("(?P<{}>{}", name, pat) + ")")
            .collect::<Vec<_>>()
            .join("|");

        let regex = Regex::new(&pattern).unwrap();

        LexicalAnalyzer { rules, regex }
    }

    fn tokenize(&self, code: &str) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        let mut line_num = 1;
        let mut line_start = 0;

        for mat in self.regex.captures_iter(code) {
            if let Some(comment_match) = mat.name("COMMENT") {
                line_num += code[line_start..comment_match.end()].matches('\n').count();
                line_start = comment_match.end();
                continue;
            }

            if let Some(newline_match) = mat.name("NEWLINE") {
                line_start = newline_match.end();
                line_num += 1;
                continue;
            }

            if mat.name("SPACE").is_some() {
                continue;
            }

            if let Some(mismatch) = mat.name("MISMATCH") {
                return Err(format!(
                    "Unexpected character {:?} at line {}",
                    mismatch.as_str(),
                    line_num
                ));
            }

            for (name, _) in &self.rules {
                if let Some(m) = mat.name(name) {
                    tokens.push(Token {
                        token_type: name.clone(),
                        value: m.as_str().to_string(),
                        line: line_num,
                        column: m.start() - line_start,
                    });
                    break;
                }
            }
        }

        Ok(tokens)
    }
}

fn main() {
    let lexer = LexicalAnalyzer::new();
    let code = "int main() { return 0; }";
    match lexer.tokenize(code) {
        Ok(tokens) => {
            for token in tokens {
                println!("{:?}", token);
            }
        }
        Err(e) => println!("Error: {}", e),
    }
}