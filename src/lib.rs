use serde::Deserialize;

pub mod types;
use crate::types::{Bytes, Bytes32};
pub mod stack;
use crate::stack::Stack;
pub mod memory;
use crate::memory::Memory;
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
}

pub struct ExecutionContext {
    call: Call,
    block: Block,
    state: State,
    code: Bytes,
    stack: Stack,
    memory: Memory,
    pc: usize,
    gas: usize,
    stopped: bool,
}

impl ExecutionContext {
    pub fn new(call: Call, block: Block, state: State, code: Bytes) -> Self {
        Self {
            call,
            block,
            state,
            code,
            stack: Stack::new(),
            memory: Memory::new(),
            pc: 0,
            gas: 0,
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
        }
    }
}
