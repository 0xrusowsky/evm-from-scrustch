use core::result::Result::Err;
use ethereum_types::U256;
use serde::Deserialize;
use sha3::{Digest, Keccak256};
use std::collections::HashMap;

use crate::types::{hex_string_to_address, hex_string_to_bytes, Address, Bytes, Bytes32};

pub const KECCAK_EMPTY: Bytes32 = Bytes32::from_slice(&Keccak256::digest(&[]));

// EVM State is a mapping from addresses to accounts.
#[derive(Debug, Default, Deserialize, Clone)]
#[serde(default)]
pub struct State(HashMap<Address, AccountState>);

// An account's Storage is a mapping from 256-bit integer keys to `StorageSlot`s.
#[derive(Debug, Default, Deserialize, Clone)]
pub struct Storage {
    // Storage map.
    map: HashMap<U256, Bytes32>,
    // Slots accessed during the current execution.
    warm_slots: Vec<U256>,
}

// State of an account.
#[derive(Debug, Default, Deserialize, Clone)]
pub struct AccountState {
    // Address of the account.
    #[serde(default, deserialize_with = "hex_string_to_address")]
    pub address: Address,
    // Balance of the account.
    #[serde(default)]
    pub balance: U256,
    // Nonce of the account.
    #[serde(default)]
    pub nonce: U256,
    // Hash of the account's code.
    #[serde(default, deserialize_with = "hex_string_to_bytes")]
    pub code_hash: Bytes,
    // Code of the account.
    #[serde(default, deserialize_with = "hex_string_to_bytes")]
    pub bytecode: Bytes,
    // Code of the account (input from json tests)
    #[serde(default, rename = "code")]
    code_test: Code,
    // Storage layout of the account.
    #[serde(default)]
    pub storage: Storage,
    // Storage layout of the account.
    #[serde(default)]
    pub warm: bool,
}

// Contract code representation for tests
#[derive(Debug, Deserialize, Default, Clone)]
pub struct Code {
    #[serde(default)]
    pub asm: Option<String>,
    #[serde(default)]
    pub bin: String,
}

impl State {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn get(&self, address: &Address) -> Option<&AccountState> {
        self.0.get(address)
    }

    pub fn load_account(&self, address: &Address) ->  Result<(&mut AccountState, bool), String> {
        let account = self.0.get_mut(address);
        match account {
            Some(account) => {
                match account.warm {
                    true => Ok((account, true)),
                    false => Ok((account, false)),
                }
            }
            None => {
                self.new_not_existing(address);
                let account = self.0.get_mut(address);
                match account {
                    Some(account) => Ok((account, false)),
                    None => Err("Error creating account".to_string()),
                }
            }
        }
    }

    pub fn get_mut(&mut self, address: &Address) -> Option<&mut AccountState> {
        self.0.get_mut(address)
    }

    pub fn insert(&mut self, address: Address, account_state: AccountState) {
        self.0.insert(address, account_state);
    }

    pub fn new_not_existing(&mut self, address: &Address) {
        self.0.insert(address.clone(), AccountState::new(address.clone()));
    }

    pub fn delete(&mut self, address: &Address) {
        self.0.remove(address);
    }

    pub fn create(&mut self, address: Address, code: Bytes, balance: U256) {
        let account_state = AccountState {
            address: address.clone(),
            bytecode: code,
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

    pub fn balance(&self, address: &Address) -> Option<U256> {
        self.load_account(address).ok().map(|(acc, _)| acc.balance)
    }
    
    pub fn nonce(&self, address: &Address) -> Option<U256> {
        self.load_account(address).ok().map(|(acc, _)| acc.nonce)
    }
    
    pub fn code(&self, address: &Address) -> Option<Bytes> {
        self.load_account(address).ok().map(|(acc, _)| acc.code())
    }
    
    pub fn code_size(&self, address: &Address) -> Option<usize> {
        self.load_account(address).ok().map(|(acc, _)| acc.code().len())
    }
    
    pub fn code_hash(&self, address: &Address) -> Option<Bytes32> {
        match self.code(address) {
            Some(code) => {
                if code.is_empty() {
                    Some(KECCAK_EMPTY)
                } else {
                    Some(Bytes32::from_vec(Keccak256::digest(code.as_slice()).to_vec()))
                }
            }
            None => None,
        }
    }
    
    pub fn storage_load(&self, address: &Address, key: U256) -> Option<Bytes32> {
        self.load_account(address).ok().map(|(acc, _)| acc.storage.load(key))
    }

    pub fn storage_store(&mut self, address: &Address, key: U256, value: Bytes32) {
        self.load_account(address).ok().map(|(acc, _)| acc.storage.store(key, value));
    }
}

impl AccountState {
    pub fn new(address: Address) -> Self {
        Self {
            address,
            balance: U256::zero(),
            nonce: U256::zero(),
            code_hash: Bytes::zero(),
            bytecode: Bytes::zero(),
            code_test: Code::default(),
            storage: Storage::default(),
            warm: false,
        }
    }

    pub fn code(&self) -> Bytes {
        if !self.bytecode.is_empty() {
            self.bytecode.clone()
        } else {
            Bytes::from_vec(hex::decode(&self.code_test.bin).unwrap())
        }
    }
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

