use std::collections::HashMap;

#[derive(Clone, Debug)]
struct State {
    id: usize,
    is_end: bool,
    transitions: HashMap<String, Vec<usize>>,
    epsilon_transitions: Vec<usize>,
}

impl State {
    fn new(id: usize) -> Self {
        State {
            id,
            is_end: false,
            transitions: HashMap::new(),
            epsilon_transitions: Vec::new(),
        }
    }
}

#[derive(Clone)]
struct NFA {
    start: usize,
    end: usize,
    states: Vec<State>,
}

struct ThompsonNFA {
    state_counter: usize,
    states: Vec<State>,
}

impl ThompsonNFA {
    fn new() -> Self {
        ThompsonNFA {
            state_counter: 0,
            states: Vec::new(),
        }
    }

    fn new_state(&mut self) -> usize {
        self.state_counter += 1;
        let id = self.state_counter;
        self.states.push(State::new(id));
        id
    }

    fn create_basic_nfa(&mut self, symbol: &str) -> NFA {
        let start = self.new_state();
        let end = self.new_state();
        self.states[start - 1].transitions.insert(
            symbol.to_string(),
            vec![end],
        );
        NFA {
            start,
            end,
            states: self.states.clone(),
        }
    }

    fn concat(&mut self, nfa1: NFA, nfa2: NFA) -> NFA {
        let mut combined_states = nfa1.states.clone();
        combined_states.extend(nfa2.states);
        combined_states[nfa1.end - 1].epsilon_transitions.push(nfa2.start);
        
        self.states = combined_states.clone();
        NFA {
            start: nfa1.start,
            end: nfa2.end,
            states: combined_states,
        }
    }

    fn union(&mut self, nfa1: NFA, nfa2: NFA) -> NFA {
        let start = self.new_state();
        let end = self.new_state();
        
        let mut combined_states = nfa1.states.clone();
        combined_states.extend(nfa2.states);
        combined_states[start - 1].epsilon_transitions.extend(vec![nfa1.start, nfa2.start]);
        combined_states[nfa1.end - 1].epsilon_transitions.push(end);
        combined_states[nfa2.end - 1].epsilon_transitions.push(end);
        
        self.states = combined_states.clone();
        NFA {
            start,
            end,
            states: combined_states,
        }
    }

    fn kleene_star(&mut self, nfa: NFA) -> NFA {
        let start = self.new_state();
        let end = self.new_state();
        
        let mut combined_states = nfa.states.clone();
        combined_states[start - 1].epsilon_transitions.extend(vec![nfa.start, end]);
        combined_states[nfa.end - 1].epsilon_transitions.extend(vec![nfa.start, end]);
        
        self.states = combined_states.clone();
        NFA {
            start,
            end,
            states: combined_states,
        }
    }
}

fn main() {
    let mut builder = ThompsonNFA::new();
    let nfa_a = builder.create_basic_nfa("a");
    let nfa_b = builder.create_basic_nfa("b");
    let nfa_ab = builder.concat(nfa_a, nfa_b);
    println!("NFA start state: {}, end state: {}", nfa_ab.start, nfa_ab.end);
}