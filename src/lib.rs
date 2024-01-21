use serde::Deserialize;
use ethereum_types::U256;

pub mod utils;
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

#[derive(Debug, Clone)]
pub struct EvmResult {
    pub stack: Vec<Bytes32>,
    pub success: bool,
    pub result: Bytes,
}

#[derive(Debug, Clone)]
pub struct CallResult {
    pub success: Bytes32,
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
    pc: usize,
    gas: usize,
    pub target: Address,
    return_data: Bytes,
    stopped: bool,
    to_delete: Vec<Address>,
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
            pc: 0,
            gas: 0,
            target,
            return_data: Bytes::new(),
            stopped: false,
            to_delete: Vec::new(),
        }
    }

    pub fn sub_ctx(&self, code: Bytes, call: Call) -> Self {
        let mut sub_ctx = self.clone();
        // Update the execution subcontext for the call
        sub_ctx.target = call.recipient();
        sub_ctx.code = code;
        sub_ctx.call = call;
        sub_ctx.pc = 0;
        sub_ctx
    }

    pub fn code_size(&self) -> usize {
        self.code.len()
    }

    pub fn code(&self) -> Bytes {
        self.code.clone()
    }

    pub fn return_data(&self) -> Bytes {
        self.return_data.clone()
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

        if success {
            self.to_delete.iter().for_each(|address| {
                self.state.delete(&address);
            });
        }

        EvmResult {
            stack: self.stack.deref_items(),
            success,
            result: self.call.result(),
        }
    }

    pub fn execute_call(&mut self, call: Call) -> CallResult {
        match self.state.transfer(&call.originator(), &call.recipient(), call.value()) {
            Err(error) => {
                println!("{:?}\n", error);
                CallResult{success: Bytes32::zero(), result: Bytes::new()}
            },
            _ => {
                let code = self.state.code(&call.code_target());
                if code.is_empty() {
                    return CallResult{success: Bytes32::one(), result: Bytes::new()};
                }
        
                let mut sub_ctx = self.sub_ctx(code, call.clone());
                let call_result = sub_ctx.run();
                match call_result.success {
                    true => {
                        // Update the execution context
                        self.stack = sub_ctx.stack;
                        self.memory = sub_ctx.memory;
                        if !call.is_static() { self.state = sub_ctx.state };
                        self.return_data = call_result.result.clone();
        
                        CallResult {
                            success: Bytes32::one(),
                            result: call_result.result,
                        }
                    },
                    false => {
                        CallResult {
                            success: Bytes32::zero(),
                            result: call_result.result,
                        }
                    },
                }
            },
        }
    }

    pub fn create_call(&mut self, address: Address, value: U256, code: Bytes) -> CallResult {
        match self.state.transfer(&self.call.originator(), &self.call.recipient(), value) {
            Err(error) => {
                println!("{:?}\n", error);
                CallResult{success: Bytes32::zero(), result: Bytes::new()}
            },
            _ => {
                println!("\nCreating contract at address: {:#X}", address);
                println!("with code: {:#X}\n", code);
                if code.is_empty() {
                    self.state.create(address, Bytes::zero(), value);
                    return CallResult{success: Bytes32::one(), result: Bytes::new()};
                }

                let call = Call::new(
                    self.target,
                    address,
                    self.call.originator(),
                    U256::zero(),
                    U256::from(self.gas_left()),
                    address,
                    Bytes::zero(),
                    value,
                    false
                );

                let mut sub_ctx = self.sub_ctx(code, call.clone());
                let call_result = sub_ctx.run();
                match call_result.success {
                    true => {
                        // Update the execution context
                        self.stack = sub_ctx.stack;
                        self.memory = sub_ctx.memory;
                        if !call.is_static() { self.state = sub_ctx.state };
                        self.return_data = call_result.result.clone();
                        self.state.create(address, call_result.result.clone(), value);

                        CallResult {
                            success: Bytes32::one(),
                            result: call_result.result,
                        }
                    },
                    false => {
                        CallResult {
                            success: Bytes32::zero(),
                            result: call_result.result,
                        }
                    },
                }
            },
        }
    }

    pub fn selfdestruct(&mut self) {
        self.to_delete.push(self.target);
    }

    pub fn gas_left(&self) -> usize {
        self.gas
    }
}