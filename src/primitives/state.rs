use core::result::Result::Err;
use serde::Deserialize;
use sha3::{Digest, Keccak256};
use std::collections::HashMap;

use crate::types::{hex_string_to_address, hex_string_to_bytes, Address, Bytes, Bytes32, U256, Code};

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

    pub fn get_mut(&mut self, address: &Address) -> Option<&mut AccountState> {
        self.0.get_mut(address)
    }

    pub fn insert(&mut self, address: Address, account_state: AccountState) {
        self.0.insert(address, account_state);
    }

    pub fn delete(&mut self, address: &Address) {
        self.0.remove(address);
    }

    pub fn create(&mut self, address: Address, code: Bytes, balance: U256) {
        let account_state = AccountState {
            address: address.clone(),
            code_bytes: code,
            balance,
            ..Default::default()
        };
        self.0.insert(address, account_state);
    }

    pub fn transfer(&mut self, from: &Address, to: &Address, value: U256) -> Result<(), String>{
        if value.is_zero() {return Ok(())};

        let state_from = self.get_mut(from);
        match state_from {
            Some(state_from) => {
                if state_from.balance < value {
                    return Err(format!("InsufficientBalance({:#X}): {:#X} < {:#X}",
                        from,
                        state_from.balance,
                        value
                    ));
                }
                state_from.balance -= value;
            },
            _ => return Err(format!("InsufficientBalance({:#X}): {:#X} < {:#X}",
                    from,
                    U256::zero(),
                    value
                )),
        }

        let state_to = self.get_mut(to);
        match state_to {
            Some(state_to) => {
                state_to.balance += value;
            },
            _ => {
                let account_state = AccountState {
                    address: to.clone(),
                    balance: value,
                    ..Default::default()
                };
                self.0.insert(to.clone(), account_state);
            },
        }

        Ok(())
    }

    pub fn balance(&self, address: &Address) -> U256 {
        match self.get(address) {
            Some(account_state) => account_state.balance(),
            None => U256::zero(),
        }
    }

    pub fn nonce(&self, address: &Address) -> U256 {
        match self.get(address) {
            Some(account_state) => account_state.nonce(),
            None => U256::zero(),
        }
    }

    pub fn code(&self, address: &Address) -> Bytes {
        match self.get(address) {
            Some(account_state) => account_state.code(),
            None => Bytes::new(),
        }
    }

    pub fn code_size(&self, address: &Address) -> usize {
        self.code(address).len()
    }

    pub fn code_hash(&self, address: &Address) -> Bytes32 {
        let code = self.code(address);
        if code.is_empty() {
            Bytes32::from_vec(vec![0])
        } else {
            Bytes32::from_vec(Keccak256::digest(self.code(address).as_slice()).to_vec())
        }
    }

    pub fn storage_load(&self, address: &Address, key: U256) -> Bytes32 {
        match self.get(address) {
            Some(account_state) => account_state.storage().load(key),
            None => Bytes32::zero(),
        }
    }

    pub fn storage_store(&mut self, address: &Address, key: U256, value: Bytes32) {
        match self.get_mut(address) {
            Some(account_state) => account_state.storage_mut().store(key, value),
            None => {
                self.insert(
                    address.clone(),
                    AccountState::new(address.clone()),
                );
                self.storage_store(address, key, value);
            }
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
    code_bytes: Bytes,
    #[serde(default, rename = "code")]
    code_test: Code,
    #[serde(
        default,
        rename = "storageRoot",
        deserialize_with = "hex_string_to_bytes"
    )]
    storage_root: Bytes,
    #[serde(default)]
    storage: Storage,
}

impl AccountState {
    pub fn new(address: Address) -> Self {
        Self {
            address,
            balance: U256::zero(),
            nonce: U256::zero(),
            code_bytes: Bytes::new(),
            code_test: Code::default(),
            storage_root: Bytes::new(),
            storage: Storage::new(),
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

    pub fn code(&self) -> Bytes {
        if !self.code_bytes.is_empty() {
            self.code_bytes.clone()
        } else {
            Bytes::from_vec(hex::decode(&self.code_test.bin).unwrap())
        }
    }

    pub fn storage_root(&self) -> &Bytes {
        &self.storage_root
    }

    pub fn storage(&self) -> &Storage {
        &self.storage
    }

    pub fn storage_mut(&mut self) -> &mut Storage {
        &mut self.storage
    }
}

#[derive(Debug, Default, Deserialize, Clone)]
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

    pub fn load(&self, key: U256) -> Bytes32 {
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

    pub fn warm_slots(&self) -> &Vec<U256> {
        &self.warm_slots
    }

    pub fn clear_warm_slots(&mut self) {
        self.warm_slots.clear();
    }

    pub fn access_slot(&mut self, key: U256) {
        self.warm_slots.push(key);
    }
}