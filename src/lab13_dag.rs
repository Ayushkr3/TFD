use std::collections::HashMap;

#[derive(Clone, Debug)]
struct DAGNode {
    value: String,
    left: Option<Box<DAGNode>>,
    right: Option<Box<DAGNode>>,
}

impl DAGNode {
    fn new(value: &str, left: Option<Box<DAGNode>>, right: Option<Box<DAGNode>>) -> Self {
        DAGNode {
            value: value.to_string(),
            left,
            right,
        }
    }
}

struct DAGBuilder {
    node_map: HashMap<String, DAGNode>,
}

impl DAGBuilder {
    fn new() -> Self {
        DAGBuilder {
            node_map: HashMap::new(),
        }
    }

    fn add_node(&mut self, op: &str, left_val: &str, right_val: Option<&str>) -> DAGNode {
        let signature = format!(
            "{}_{}_{}",
            op,
            left_val,
            right_val.unwrap_or("-")
        );

        if let Some(node) = self.node_map.get(&signature) {
            return node.clone();
        }

        let left_node = self.node_map.get(left_val).cloned()
            .unwrap_or_else(|| DAGNode::new(left_val, None, None));
        
        let right_node = right_val.and_then(|rv| {
            self.node_map.get(rv).cloned()
                .or_else(|| Some(DAGNode::new(rv, None, None)))
        });

        let new_node = DAGNode::new(op, Some(Box::new(left_node)), right_node.map(Box::new));
        self.node_map.insert(signature, new_node.clone());
        self.node_map.insert(new_node.value.clone(), new_node.clone());
        new_node
    }
}

fn main() {
    let mut dag = DAGBuilder::new();
    // a + a * (b - c) + (b - c) * d
    let n1 = dag.add_node("-", "b", Some("c")); // common
    let _n2 = dag.add_node("*", "a", Some(&n1.value));
    println!("DAG Nodes tracked for reuse: {}", dag.node_map.len());
}