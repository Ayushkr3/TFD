#[derive(Clone)]
struct ExprTree {
    op: String,
    left: Option<Box<ExprTree>>,
    right: Option<Box<ExprTree>>,
}

impl ExprTree {
    fn new(op: &str, left: Option<Box<ExprTree>>, right: Option<Box<ExprTree>>) -> Self {
        ExprTree {
            op: op.to_string(),
            left,
            right,
        }
    }

    fn new_leaf(op: &str) -> Self {
        ExprTree {
            op: op.to_string(),
            left: None,
            right: None,
        }
    }
}

struct IntermediateCodeConvertor;

impl IntermediateCodeConvertor {
    fn to_postfix(node: &Option<Box<ExprTree>>) -> String {
        match node {
            None => String::new(),
            Some(n) => {
                if n.left.is_none() && n.right.is_none() {
                    n.op.clone()
                } else {
                    let left_str = Self::to_postfix(&n.left);
                    let right_str = Self::to_postfix(&n.right);
                    format!("{} {} {}", left_str, right_str, n.op).trim().to_string()
                }
            }
        }
    }

    fn to_prefix(node: &Option<Box<ExprTree>>) -> String {
        match node {
            None => String::new(),
            Some(n) => {
                if n.left.is_none() && n.right.is_none() {
                    n.op.clone()
                } else {
                    let left_str = Self::to_prefix(&n.left);
                    let right_str = Self::to_prefix(&n.right);
                    format!("{} {} {}", n.op, left_str, right_str).trim().to_string()
                }
            }
        }
    }
}

fn main() {
    // Representing: a + b * c
    let ast = Box::new(ExprTree::new(
        "+",
        Some(Box::new(ExprTree::new_leaf("a"))),
        Some(Box::new(ExprTree::new(
            "*",
            Some(Box::new(ExprTree::new_leaf("b"))),
            Some(Box::new(ExprTree::new_leaf("c"))),
        ))),
    ));

    println!("Postfix: {}", IntermediateCodeConvertor::to_postfix(&Some(ast.clone())));
    println!("Prefix: {}", IntermediateCodeConvertor::to_prefix(&Some(ast)));
}