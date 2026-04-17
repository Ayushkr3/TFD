use std::collections::HashMap;

struct AllocationSystem {
    memory: Vec<i32>,
    stack_ptr: usize,
    heap_ptr: usize,
    static_ptr: usize,
    symbol_table: HashMap<String, usize>,
}

impl AllocationSystem {
    fn new(memory_size: usize) -> Self {
        AllocationSystem {
            memory: vec![0; memory_size],
            stack_ptr: memory_size - 1,
            heap_ptr: 0,
            static_ptr: 0,
            symbol_table: HashMap::new(),
        }
    }

    fn alloc_static(&mut self, name: &str, size: usize) -> usize {
        let addr = self.static_ptr;
        self.static_ptr += size;
        self.symbol_table.insert(name.to_string(), addr);
        addr
    }

    fn alloc_stack(&mut self, size: usize) -> Result<usize, String> {
        if self.stack_ptr < size {
            return Err("Stack Overflow".to_string());
        }
        self.stack_ptr -= size;
        if self.stack_ptr <= self.heap_ptr {
            return Err("Stack Overflow".to_string());
        }
        Ok(self.stack_ptr)
    }

    fn pop_stack(&mut self, size: usize) {
        self.stack_ptr += size;
    }

    fn alloc_heap(&mut self, size: usize) -> Result<usize, String> {
        let addr = self.heap_ptr;
        self.heap_ptr += size;
        if self.heap_ptr >= self.stack_ptr {
            return Err("Out of Heap Memory".to_string());
        }
        Ok(addr)
    }
}

fn main() {
    let mut mem = AllocationSystem::new(1024);
    println!("Static var at: {}", mem.alloc_static("global_x", 4));
    match mem.alloc_heap(16) {
        Ok(addr) => println!("Heap var at: {}", addr),
        Err(e) => println!("Error: {}", e),
    }
    match mem.alloc_stack(32) {
        Ok(addr) => println!("Stack frame at: {}", addr),
        Err(e) => println!("Error: {}", e),
    }
}