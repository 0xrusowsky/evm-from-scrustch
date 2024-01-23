pub mod host;
pub mod stack;
pub mod memory;
pub mod contract;
pub mod call;

pub use stack::Stack;
pub use memory::Memory;
pub use contract::Contract;
pub use call::CallContext;

pub struct Interpreter {
    pub stack: Stack,
    pub memory: Memory,
    pub contract: Contract,
    pub call: CallContext,
    pub pc: usize,
    pub gas: usize,
    pub stopped: bool,
    pub return_data: Vec<u8>,
    // pub logs: Vec<Log>,
}