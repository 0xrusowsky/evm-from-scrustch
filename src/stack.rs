use ethereum_types::U256;

#[derive(Debug)]
pub struct Stack {
    items: Vec<U256>,
    max_depth: usize,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            max_depth: 1024,
        }
    }

    // Stack Operations

    pub fn push(&mut self, value: U256) {
        println!("Pushing value: {:#X}", value);
        if self.items.len() == self.max_depth {
            panic!("Stack overflow");
        }

        self.items.push(value);
    }

    pub fn pop(&mut self) -> U256 {
        if self.items.is_empty() {
            panic!("Stack underflow");
        }

        self.items.pop().unwrap()
    }

    // Stack Getters

    pub fn items(&self) -> &Vec<U256> {
        &self.items
    }

    pub fn deref_items(&self) -> Vec<U256> {
        let mut items = self.items.clone();
        items.reverse();
        items
    }

    pub fn max_depth(&self) -> usize {
        self.max_depth
    }

    pub fn depth(&self) -> usize {
        self.items.len()
    }
}