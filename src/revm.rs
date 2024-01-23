pub mod primitives;
pub mod interpreter;

use crate::primitives::*;
use crate::interpreter::*;

pub struct Evm {
    pub context: ExecutionContext,
    pub interpreter: Interpreter,
}

pub struct ExecutionContext {
    pub env: Env,
    pub state: State,
}

impl ExecutionContext {
    pub fn new(env: Env, state: State) -> Self {
        Self { env, state }
    }

    pub fn block_hash(&mut self, number: U256) -> Option<Bytes32> {
        self.env.block_hash(number)
    }

    pub fn load_account(&mut self, address: Address) -> (bool, bool) {
        self.state.load_account(address)
    }

    pub fn balance(&mut self, address: Address) -> Option<U256> {
        self.state.balance(address)
    }

    pub fn nonce(&mut self, address: Address) -> Option<U256> {
        self.state.nonce(address)
    }

    pub fn code(&mut self, address: Address) -> Option<U256> {
        self.state.code(address)
    }

    pub fn code_size(&mut self, address: Address) -> Option<usize> {
        self.state.code_size(address)
    }

    pub fn code_hash(&mut self, address: Address) -> Option<Bytes32> {
        self.state.code_hash(address)
    }

    pub fn sload(&mut self, address: Address, key: U256) -> Option<U256> {
        self.state.sload(address, key)
    }

    pub fn sstore(&mut self, address: Address, key: U256, value: U256){
        self.state.sstore(address, key, value);
    }
}