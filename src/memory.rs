#[derive(Debug)]
pub struct Memory(Vec<u8>);

impl Memory {
    pub fn new() -> Self {
        Memory(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn size(&self) -> usize {
        ((self.len() + 31 ) / 32) * 32
    }

    pub fn expansion(&self, offset: usize, size: usize) -> usize {
        if offset + size > self.len() {
            offset + size - self.len()
        } else {
            0
        }
    }

    pub fn load(&mut self, offset: usize, size: usize) -> &[u8] {
        // if out of bounds, expand the memory
        if offset + size > self.0.len() {
            self.0.resize(offset + size, 0);
        }
        &self.0[offset..offset + size]
    }

    pub fn store(&mut self, offset: usize, data: &[u8]) {
        // if out of bounds, expand the memory
        let end = offset + data.len();
        if end > self.len() {
            self.0.resize(((end + 31) / 32) * 32, 0);
        }
        self.0[offset..offset + data.len()].copy_from_slice(data);
    }
}
