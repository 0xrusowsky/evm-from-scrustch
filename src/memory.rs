#[derive(Debug)]
pub struct Memory(Vec<u8>);

impl Memory {
    pub fn new() -> Self {
        Memory(Vec::new())
    }

    pub fn load(&mut self, offset: usize) -> &[u8] {
        // if out of bounds, expand the memory
        if offset + 32 > self.0.len() {
            self.0.resize(offset + 32, 0);
        }
        &self.0[offset..offset + 32]
    }

    pub fn store(&mut self, offset: usize, value: &[u8]) {
        let words = value.len() / 32;
        if offset + words * 32 > self.0.len() {
            self.0.resize(offset + words * 32, 0);
        }
        self.0[offset..offset + words * 32].copy_from_slice(value);
    }

    pub fn store8(&mut self, offset: usize, value: u8) {
        if offset + 1 > self.0.len() {
            self.0.resize(offset + 1, 0);
        }
        self.0[offset] = value;
    }

    pub fn size(&self) -> usize {
        let len = self.0.len();
        if len == 0 {
            return 0;
        }
        if len % 32 != 0 {
            (len / 32 + 1) * 32
        } else {
            len / 32 * 32
        }
    }

    pub fn expansion(&self, offset: usize, size: usize) -> usize {
        let len = self.0.len();
        if offset + size > len {
            offset + size - len
        } else {
            0
        }
    }
}
