use std::collections::HashMap;
use sha3::{Digest, Keccak256};
use ethereum_types::U256;
use serde::Deserialize;

use super::Code;
use crate::utils::{
    Address,
    hex_string_to_bytes,
    hex_string_to_address,
};

#[derive(Debug, Default, Deserialize, Clone)]
#[serde(default)]
pub struct State(HashMap<Address, AccountState>);

impl State {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn get(&self, address: &Address) -> Option<&AccountState> {
        self.0.get(address)
    }

    pub fn insert(&mut self, address: Address, account_state: AccountState) {
        self.0.insert(address, account_state);
    }

    pub fn remove(&mut self, address: &Address) {
        self.0.remove(address);
    }

    pub fn balance(&self, address: &Address) -> U256 {
        match self.get(address) {
            Some(account_state) => {
                account_state.balance()
            },
            None => U256::zero(),
        }
    }

    pub fn code(&self, address: &Address) -> Vec<u8> {
        match self.get(address) {
            Some(account_state) => {
                account_state.code()
            },
            None => Vec::new(),
        }
    }

    pub fn code_size(&self, address: &Address) -> usize {
        self.code(address).len()
    }

    pub fn code_hash(&self, address: &Address) -> Vec<u8> {
        let code = self.code(address);
        if code.len() == 0 {
            vec![0]
        } else {
            Keccak256::digest(self.code(address)).to_vec()
        }
    }
}

#[derive(Debug, Default, Deserialize, Clone)]
pub struct AccountState {
    #[serde(default, deserialize_with = "hex_string_to_address")]
    address: Address,
    #[serde(default)]
    balance: U256,
    #[serde(default)]
    nonce: U256,
    #[serde(default, deserialize_with = "hex_string_to_bytes")]
    code_bytes: Vec<u8>,
    #[serde(default, rename = "code")]
    code_test: Code,
    #[serde(default, rename = "storageRoot", deserialize_with = "hex_string_to_bytes")]
    storage_root: Vec<u8>,
}

impl AccountState {
    pub fn new(address: Address) -> Self {
        Self {
            address,
            balance: U256::zero(),
            nonce: U256::zero(),
            code_bytes: Vec::new(),
            code_test: Code::default(),
            storage_root: Vec::new(),
        }
    }

    pub fn address(&self) -> Address {
        self.address
    }

    pub fn balance(&self) -> U256 {
        self.balance
    }

    pub fn nonce(&self) -> U256 {
        self.nonce
    }

    pub fn code(&self) -> Vec<u8> {
        if self.code_bytes.len() > 0 {
            self.code_bytes.clone()
        } else {
            hex::decode(&self.code_test.bin).unwrap()
        }
    }

    pub fn storage_root(&self) -> &Vec<u8> {
        &self.storage_root
    }
}