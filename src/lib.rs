use ethereum_types::U256;
use serde::Deserialize;

pub mod stack;
use crate::stack::Stack;
pub mod memory;
use crate::memory::Memory;
pub mod opcode;
use crate::opcode::Opcode;

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
    code: Vec<u8>,
    stack: Stack,
    memory: Memory,
    pc: usize,
    gas: usize,
    stopped: bool,
}

impl ExecutionContext {
    pub fn new(code: Vec<u8>) -> Self {
        Self {
            code,
            stack: Stack::new(),
            memory: Memory::new(),
            pc: 0,
            gas: 0,
            stopped: false,
        }
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
            let (pc_delta, opcode_success) = opcode.execute(self);
        
            // Update control variables
            self.pc += pc_delta;
            success = opcode_success;
        }

        EvmResult {
            stack: self.stack.deref_items(),
            success,
        }
    }
}