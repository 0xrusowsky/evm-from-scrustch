#[derive(Debug)]
pub struct Memory(Vec<u8>);

impl Memory {
    pub fn new() -> Self {
        Memory(Vec::new())
    }

    pub fn load(&self, offset: usize) -> &u8 {
        if offset >= self.0.len() {
            return &0;
        }

        &self.0[offset]
    }

    pub fn store(&mut self, offset: usize, value: &u8) {
        if offset < self.0.len() {
            self.0[offset] = *value;
        } else {
            self.0.resize(offset + 1, 0);
            self.0[offset] = *value;
        }
    }
}