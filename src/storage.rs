use std::collections::HashMap;
use ethereum_types::U256;

use crate::types::Bytes32;

#[derive(Debug, Clone)]
pub struct Storage {
    map: HashMap<U256, Bytes32>,
    warm_slots: Vec<U256>,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            warm_slots: Vec::new(),
        }
    }

    pub fn load(&mut self, key: U256) -> Bytes32 {
        match self.map.get(&key) {
            Some(value) => value.clone(),
            None => Bytes32::zero(),
        }
    }

    pub fn store(&mut self, key: U256, value: Bytes32) {
        self.map.insert(key, value);
    }

    pub fn delete(&mut self, key: U256) {
        self.map.remove(&key);
    }

    pub fn warm_slots( self) -> Vec<U256> {
        self.warm_slots
    }

    pub fn clear_warm_slots(&mut self) {
        self.warm_slots.clear();
    }

    pub fn access_slot(&mut self, key: U256) {
        self.warm_slots.push(key);
    }
}
