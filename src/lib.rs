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

pub enum ExecutionStatus {
    Active,
    Stop,
    Invalid,
}

pub struct ExecutionContext {
    code: Vec<u8>,
    stack: Stack,
    memory: Memory,
    gas: usize,
    pc: usize,
    status: ExecutionStatus,
}

impl ExecutionContext {
    pub fn new(code: Vec<u8>) -> Self {
        Self {
            code,
            stack: Stack::new(),
            memory: Memory::new(),
            gas: 0,
            pc: 0,
            status: ExecutionStatus::Active,
        }
    }

    pub fn run(&mut self) -> EvmResult {
        while self.pc < self.code.len(){
            match self.status {
                ExecutionStatus::Active => {
                    let opcode: Opcode = self.code[self.pc].try_into().unwrap();
                    let pc_delta = opcode.execute(self);
                    self.pc += pc_delta;
                },
                ExecutionStatus::Stop => break,
                ExecutionStatus::Invalid => break,
            }
        }

        let success = match self.status {
            ExecutionStatus::Active => true,
            ExecutionStatus::Stop => true,
            ExecutionStatus::Invalid => false,
        };

        EvmResult {
            stack: self.stack.deref_items(),
            success,
        }
    }
}