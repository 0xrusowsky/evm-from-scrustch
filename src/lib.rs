use serde::Deserialize;
use ethereum_types::U256;

pub mod types;
use crate::types::{Address, Bytes, Bytes32};
pub mod stack;
use crate::stack::Stack;
pub mod memory;
use crate::memory::Memory;
pub mod storage;
use crate::storage::Storage;
pub mod opcode;
use crate::opcode::Opcode;
pub mod call;
pub use crate::call::Call;
pub mod block;
pub use crate::block::Block;
pub mod state;
pub use crate::state::State;

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Code {
    #[serde(default)]
    pub asm: Option<String>,
    #[serde(default)]
    pub bin: String,
}

pub struct EvmResult {
    pub stack: Vec<Bytes32>,
    pub success: bool,
    pub result: Bytes,
}

pub struct CallResult {
    pub success: bool,
    pub result: Bytes,
}

#[derive(Debug, Clone)]
pub struct ExecutionContext {
    call: Call,
    block: Block,
    state: State,
    code: Bytes,
    stack: Stack,
    memory: Memory,
    storage: Storage,
    pc: usize,
    gas: usize,
    target: Address,
    stopped: bool,
}

impl ExecutionContext {
    pub fn new(call: Call, block: Block, state: State, code: Bytes) -> Self {
        let target = call.recipient();

        Self {
            call,
            block,
            state,
            code,
            stack: Stack::new(),
            memory: Memory::new(),
            storage: Storage::new(),
            pc: 0,
            gas: 0,
            target,
            stopped: false,
        }
    }

    pub fn code_size(&self) -> usize {
        self.code.len()
    }

    pub fn code(&self) -> Bytes {
        self.code.clone()
    }

    pub fn run(&mut self) -> EvmResult {
        let mut success = true;
        loop {
            // Check execution conditions
            if !success || self.stopped || self.pc >= self.code.len() {
                break;
            }

            // Process the next opcode
            let opcode: Opcode = self.code[self.pc].try_into().unwrap();
            let opcode_success = opcode.execute(self);

            // Update control variables
            success = opcode_success;
        }

        EvmResult {
            stack: self.stack.deref_items(),
            success,
            result: self.call.result(),
        }
    }

    pub fn execute_call(&mut self, call: Call) -> CallResult {
        let mut success = true;
        let code = self.state.code(&call.code_target());

        if code.is_empty() {
            return CallResult{success, result: Bytes::new()};
        }

        // Cache the current execution context before executing the call
        let cache = self.clone();

        // Update the execution context for the call
        self.target = call.recipient();
        self.code = code;
        self.call = call;
        self.pc = 0;

        loop {
            // Check execution conditions
            if !success || self.stopped || self.pc >= self.code.len() {
                break;
            }

            // Process the next opcode
            let opcode: Opcode = self.code[self.pc].try_into().unwrap();
            let opcode_success = opcode.execute(self);

            // Update control variables
            success = opcode_success;
        }

        let result = self.call.result();
        if success == false {
            // Restore the execution context
            *self = cache;
        };

        CallResult {
            success,
            result,
        }
    }

    pub fn gas_left(&self) -> usize {
        self.gas
    }
}
