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

impl Host for Evm {
    fn env(&mut self) -> &mut Env {
        self.context.env()
    }

    fn block_hash(&mut self, number: U256) -> Option<Bytes32> {
        self.context.block_hash(number)
    }

    fn load_account(&mut self, address: Address) -> Option<(bool, bool)> {
        self.context.load_account(address)
    }

    fn balance(&mut self, address: Address) -> Option<(U256, bool)> {
        self.context.balance(address)
    }

    fn code(&mut self, address: Address) -> Option<(Bytes, bool)> {
        self.context.code(address)
    }

    fn code_hash(&mut self, address: Address) -> Option<(Bytes32, bool)> {
        self.context.code_hash(address)
    }

    fn sload(&mut self, address: Address, index: U256) -> Option<(U256, bool)> {
        self.context.sload(address, index)
    }

    fn sstore(
        &mut self,
        address: Address,
        index: U256,
        value: U256,
    ) -> Option<(U256, U256, U256, bool)> {
        self.context.sstore(address, index, value)
    }

    fn log(&mut self, log: Log) {
        self.context.journaled_state.log(log);
    }

    fn selfdestruct(&mut self, address: Address, target: Address) {}

    // fn selfdestruct(&mut self, address: Address, target: Address) -> Option<SelfDestructResult> {
    //     self.context
    //         .evm
    //         .journaled_state
    //         .selfdestruct(address, target, &mut self.context.db)
    //         .map_err(|e| self.context.error = Some(e))
    //         .ok()
    // }
}