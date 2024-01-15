use crate::types::Bytes;

#[derive(Debug, Default)]
pub struct Memory(Bytes);

impl Memory {
    pub fn new() -> Self {
        Memory(Bytes::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.len() == 0
    }

    pub fn size(&self) -> usize {
        ((self.len() + 31) / 32) * 32
    }

    pub fn expansion(&self, offset: usize, size: usize) -> usize {
        if offset + size > self.len() {
            offset + size - self.len()
        } else {
            0
        }
    }

    pub fn load(&mut self, offset: usize, size: usize) -> Bytes {
        // if out of bounds, expand the memory
        if offset + size > self.0.len() {
            self.0.resize(offset + size, 0);
        }
        Bytes::from_slice(&self.0[offset..offset + size])
    }

    pub fn store(&mut self, offset: usize, data: Bytes) {
        // if out of bounds, expand the memory
        let end = offset + data.len();
        if end > self.len() {
            self.0.resize(((end + 31) / 32) * 32, 0);
        }
        self.0[offset..offset + data.len()].copy_from_slice(data.as_slice());
    }
}
