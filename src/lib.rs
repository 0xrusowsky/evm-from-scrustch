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
    stopped: bool,
}

impl ExecutionContext {
    pub fn new(code: Vec<u8>) -> Self {
        Self {
            code,
            stack: Stack::new(),
            memory: Memory::new(),
            pc: 0,
            stopped: false,
        }
    }

    pub fn run(&mut self) -> EvmResult {
        while !self.stopped && self.pc < self.code.len(){
            // println!("PC: {:#?}/{:#?}", self.pc, self.code.len()-1);
            let opcode: Opcode = self.code[self.pc].try_into().unwrap();
            let pc_delta = opcode.execute(self);
            self.pc += pc_delta;
        }

        EvmResult {
            stack: self.stack.deref_items(),
            success: true,
        }
    }
}