use ethereum_types::U256;
use serde::Deserialize;

pub mod utils;
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

#[derive(Debug, Deserialize, Clone)]
pub struct Code {
    #[serde(default)]
    pub asm: Option<String>,
    #[serde(default)]
    pub bin: String,
}

pub struct EvmResult {
    pub stack: Vec<U256>,
    pub success: bool,
}

pub struct ExecutionContext {
    call: Call,
    block: Block,
    code: Vec<u8>,
    stack: Stack,
    memory: Memory,
    pc: usize,
    gas: usize,
    stopped: bool,
}

impl ExecutionContext {
    pub fn new(call: Call, block: Block, code: Vec<u8>) -> Self {
        Self {
            call,
            block,
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

    pub fn code(&self) -> &[u8] {
        &self.code
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