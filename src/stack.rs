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

    pub fn swap(&mut self, depth: usize) {
        let stack_depth = self.depth();
        if depth >= stack_depth {
            panic!("Stack underflow");
        }

        let index = stack_depth - depth - 1;
        self.items.swap(index, stack_depth - 1);

    }

    // Stack Getters

    pub fn items(&self) -> &Vec<U256> {
        &self.items
    }

    pub fn get_item(&self, depth: usize) -> Option<U256> {
        if depth >= self.items.len() {
            None
        } else {
            Some(self.items[depth])
        }
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