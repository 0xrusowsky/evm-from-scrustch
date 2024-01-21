use serde::Deserialize;
use ethereum_types::U256;

pub mod utils;
pub mod opcode;
pub mod primitives;
pub mod interpreter;
pub use crate::utils::*;
pub use crate::opcode::*;
pub use crate::primitives::*;
pub use crate::interpreter::*;

#[derive(Debug, Clone)]
pub struct EvmResult {
    pub stack: Vec<Bytes32>,
    pub logs: Vec<Log>,
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
    call: CallContext,
    block: BlockEnv,
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
    logs: Vec<Log>,
}

impl ExecutionContext {
    pub fn new(call: CallContext, block: BlockEnv, tx: TxEnv, state: State, code: Bytes) -> Self {
        let target = tx.recipient;

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
            logs: Vec::new(),
        }
    }

    pub fn sub_ctx(&self, code: Bytes, call: CallContext) -> Self {
        let mut sub_ctx = self.clone();
        // Update the execution subcontext for the call
        sub_ctx.target = call.recipient();
        sub_ctx.code = code;
        sub_ctx.call = call;
        sub_ctx.pc = 0;
        sub_ctx
    }

    pub fn add_log(&mut self, log: Log) {
        self.logs.push(log);
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
            logs: self.logs.clone(),
            success,
            result: self.call.result,
        }
    }

    pub fn execute_call(&mut self, call: CallContext) -> CallResult {
        match self.state.transfer(&call.caller, &call.recipient(), call.value) {
            Err(error) => {
                println!("{:?}\n", error);
                CallResult{success: Bytes32::zero(), result: Bytes::new()}
            },
            _ => {
                let code = self.state.code(&call.code_target);
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
                        if !call.is_static { self.state = sub_ctx.state };
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
        match self.state.transfer(&self.call.caller, &self.call.recipient(), value) {
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

                let call = CallContext{
                    sender: self.target,
                    caller: self.call.caller,
                    code_target: address,
                    value,
                    is_static: false,
                    result: Bytes::zero(),
                };

                let mut sub_ctx = self.sub_ctx(code, call.clone());
                let call_result = sub_ctx.run();
                match call_result.success {
                    true => {
                        // Update the execution context
                        self.stack = sub_ctx.stack;
                        self.memory = sub_ctx.memory;
                        if !call.is_static { self.state = sub_ctx.state };
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