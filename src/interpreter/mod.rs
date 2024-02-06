pub mod contract;
pub mod opcodes;
pub mod memory;
pub mod stack;
pub mod call;
pub mod host;

pub use crate::contract::*;
pub use crate::opcodes::*;
pub use crate::memory::*;
pub use crate::stack::*;
pub use crate::call::*;
pub use crate::host::*;

use crate::primitives::{Address, Bytes, Bytes32, Log};

#[derive(Debug, Clone)]
pub struct Interpreter {
    // Contract information and invoking data
    pub contract: Contract,
    // Program counter of the current execution
    pub pc: usize,
    // Stack of the current execution
    pub stack: Stack,
    // EVM Memory
    pub memory: Memory,
    // Gas consumed by the current execution
    pub gas: usize,
    // Whether it is a view only call or not
    pub is_static: bool,
    // Execution control flag. If not set to `Continue`, the interpreter will stop
    pub control_flow: ControlFlow,
    // Return data resulting from the execution
    pub return_data: Bytes,
}

// The result of interpreting an opcode.
#[derive(Debug, Clone)]
enum ControlFlow {
    Continue,
    Stop,
    Return,
    // TODO Handle different error codes
    Revert,
}

// The result of an interpreter execution.
#[derive(Debug, Clone)]
pub struct InterpreterResult {
    // Whether the interpreter execution was successful or not.
    pub success: bool,
    // The output of the instruction execution.
    pub output: Bytes,
    // The gas usage information.
    pub gas: usize,
}

#[derive(Debug, Clone)]
pub struct EvmResult {
    // Resulting stack after the EVM execution
    pub stack: Vec<Bytes32>,
    // Resulting logs after the EVM execution
    pub logs: Vec<Log>,
    // Whether the transaction was successful or not
    pub success: bool,
    // Result of the transaction execution
    pub result: Bytes,
}

impl Interpreter {
    /// Create new interpreter
    pub fn new(pc: usize, contract: Contract, gas: usize, is_static: bool) -> Self {
        Self {
            contract,
            pc,
            stack: Stack::new(),
            memory: Memory::new(),
            gas,
            is_static,
            control_flow: ControlFlow::Continue,
            return_data: Bytes::new(),
            logs: Vec::new(),
            to_delete: Vec::new(),
        }
    }

    // Get current opcode and execute it.
    fn step<H: Host>(&mut self, host: &mut H) {
        let opcode: Opcode = self.contract.code[self.pc].try_into().unwrap();
        self.control_flow = opcode.execute(self, host);
    }

    // Run the interpreter.
    pub fn run<H: Host>(&mut self, host: &mut H) -> EvmResult {
        loop {
            // If the program counter is out of bounds, stop the execution
            if self.pc >= self.contract.code.len() { break; }

            // Check execution conditions
            match self.control_flow {
                ControlFlow::Continue => self.step(host),
                ControlFlow::Stop => break,
                ControlFlow::Revert => {
                    return EvmResult {
                        stack: self.stack.deref_items(),
                        logs: self.logs,
                        success: false,
                        result: self.return_data,
                    };
                },
                ControlFlow::Return => {
                    // TODO move to the EVM after interpreter execution is over
                    // Delete the contracts marked for selfdestruction
                    // self.to_delete.iter().for_each(|address| {
                    //     state.delete(&address);
                    // });
                    return EvmResult {
                        stack: self.stack.deref_items(),
                        logs: self.logs,
                        success: true,
                        result: self.return_data,
                    }
                }
            }
        }

        EvmResult {
            stack: self.stack.deref_items(),
            logs: self.logs,
            success: true,
            result: Bytes::zero(),
        }
    }

    pub fn gas_left(&self) -> usize {
        self.gas
    }
}