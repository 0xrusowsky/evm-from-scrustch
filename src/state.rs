use ethereum_types::U256;
use sha3::{Digest, Keccak256};

#[derive(Debug)]
pub struct State {
    address: Address,
    balance: U256,
    nonce: U256,
    code: Vec<u8>,
    storage_root: Vec<u8>,
}

impl State {
    pub fn new(address: Address) -> Self {
        Self {
            address,
            balance: U256::zero(),
            nonce: U256::zero(),
            code: Vec::new(),
            storage_root: Vec::new(),
        }
    }

    pub fn address(&self) -> &Address {
        &self.address
    }

    pub fn balance(&self) -> &U256 {
        &self.balance
    }

    pub fn nonce(&self) -> &U256 {
        &self.nonce
    }

    pub fn code(&self) -> &Vec<u8> {
        &self.code
    }

    pub fn code_hash(&self) -> Vec<u8> {
        Keccak256::digest(&self.code)
    }

    pub fn storage_root(&self) -> &Vec<u8> {
        &self.storage_root
    }

    pub fn set_balance(&mut self, balance: U256) {
        self.balance = balance;
    }

    pub fn set_nonce(&mut self, nonce: U256) {
        self.nonce = nonce;
    }

    pub fn set_code(&mut self, code: Vec<u8>) {
        self.code = code;
    }

    pub fn set_storage_root(&mut self, storage_root: Vec<u8>) {
        self.storage_root = storage_root;
    }
}