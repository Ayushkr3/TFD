use std::collections::{HashSet, HashMap};

#[derive(Clone)]
struct BasicBlock {
    id: i32,
    gen: HashSet<i32>,
    kill: HashSet<i32>,
    preds: Vec<i32>,
    in_set: HashSet<i32>,
    out_set: HashSet<i32>,
}

impl BasicBlock {
    fn new(id: i32, gen: HashSet<i32>, kill: HashSet<i32>, preds: Vec<i32>) -> Self {
        let out_set = gen.clone();
        BasicBlock {
            id,
            gen,
            kill,
            preds,
            in_set: HashSet::new(),
            out_set,
        }
    }
}

struct ReachingDefinitions {
    blocks: Vec<BasicBlock>,
}

impl ReachingDefinitions {
    fn new(blocks: Vec<BasicBlock>) -> Self {
        ReachingDefinitions { blocks }
    }

    fn analyze(&mut self) {
        let mut changed = true;
        while changed {
            changed = false;
            for i in 0..self.blocks.len() {
                let mut new_in = HashSet::new();
                let preds = self.blocks[i].preds.clone();
                for p_id in preds {
                    if let Some(p) = self.blocks.iter().find(|b| b.id == p_id) {
                        new_in.extend(&p.out_set);
                    }
                }

                self.blocks[i].in_set = new_in;
                let mut new_out = self.blocks[i].gen.clone();
                let kill_ref = self.blocks[i].kill.clone();
                let in_ref = self.blocks[i].in_set.clone();
                new_out.extend(in_ref.difference(&kill_ref).copied());

                if new_out != self.blocks[i].out_set {
                    self.blocks[i].out_set = new_out;
                    changed = true;
                }
            }
        }
    }
}

fn main() {
    let mut b1_gen = HashSet::new();
    b1_gen.insert(1);
    b1_gen.insert(2);
    let mut b1_kill = HashSet::new();
    b1_kill.insert(3);

    let b1 = BasicBlock::new(1, b1_gen, b1_kill, vec![]);

    let mut b2_gen = HashSet::new();
    b2_gen.insert(3);
    let mut b2_kill = HashSet::new();
    b2_kill.insert(1);

    let b2 = BasicBlock::new(2, b2_gen, b2_kill, vec![1]);

    let mut rd = ReachingDefinitions::new(vec![b1, b2]);
    rd.analyze();
    println!("Block 2 IN: {:?}", rd.blocks[1].in_set);
}