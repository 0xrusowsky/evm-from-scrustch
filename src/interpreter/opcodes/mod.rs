use std::convert::TryFrom;
use sha3::{Digest, Keccak256};
use std::ops::{BitAnd, BitOr, BitXor, Not, Shl, Shr};

use crate::types::{Bytes, Bytes32, Address, U256};
use crate::utils::rlp_encode;
use crate::call::Call;
use crate::logs::Log;
use crate::host::Host;
use crate::interpreter::{Interpreter, ControlFlow};

#[derive(Debug)]
pub enum Opcode {
    STOP,
    ADD,
    MUL,
    SUB,
    DIV,
    SDIV,
    MOD,
    SMOD,
    ADDMOD,
    MULMOD,
    EXP,
    SIGNEXTEND,
    LT,
    GT,
    SLT,
    SGT,
    EQ,
    ISZERO,
    AND,
    OR,
    XOR,
    NOT,
    BYTE,
    SHL,
    SHR,
    SAR,
    SHA3,
    ADDRESS,
    BALANCE,
    ORIGIN,
    CALLER,
    CALLVALUE,
    CALLDATALOAD,
    CALLDATASIZE,
    CALLDATACOPY,
    CODESIZE,
    CODECOPY,
    GASPRICE,
    EXTCODESIZE,
    EXTCODECOPY,
    RETURNDATASIZE,
    RETURNDATACOPY,
    EXTCODEHASH,
    BLOCKHASH,
    COINBASE,
    TIMESTAMP,
    NUMBER,
    PREVRANDAO,
    GASLIMIT,
    CHAINID,
    SELFBALANCE,
    BASEFEE,
    POP,
    MLOAD,
    MSTORE,
    MSTORE8,
    SLOAD,
    SSTORE,
    JUMP,
    JUMPI,
    PC,
    MSIZE,
    GAS,
    JUMPDEST,
    PUSH1,
    PUSH2,
    PUSH3,
    PUSH4,
    PUSH5,
    PUSH6,
    PUSH7,
    PUSH8,
    PUSH9,
    PUSH10,
    PUSH11,
    PUSH12,
    PUSH13,
    PUSH14,
    PUSH15,
    PUSH16,
    PUSH17,
    PUSH18,
    PUSH19,
    PUSH20,
    PUSH21,
    PUSH22,
    PUSH23,
    PUSH24,
    PUSH25,
    PUSH26,
    PUSH27,
    PUSH28,
    PUSH29,
    PUSH30,
    PUSH31,
    PUSH32,
    DUP1,
    DUP2,
    DUP3,
    DUP4,
    DUP5,
    DUP6,
    DUP7,
    DUP8,
    DUP9,
    DUP10,
    DUP11,
    DUP12,
    DUP13,
    DUP14,
    DUP15,
    DUP16,
    SWAP1,
    SWAP2,
    SWAP3,
    SWAP4,
    SWAP5,
    SWAP6,
    SWAP7,
    SWAP8,
    SWAP9,
    SWAP10,
    SWAP11,
    SWAP12,
    SWAP13,
    SWAP14,
    SWAP15,
    SWAP16,
    LOG0,
    LOG1,
    LOG2,
    LOG3,
    LOG4,
    CREATE,
    CALL,
    CALLCODE,
    RETURN,
    DELEGATECALL,
    CREATE2,
    STATICCALL,
    REVERT,
    INVALID,
    SELFDESTRUCT,
}

impl TryFrom<u8> for Opcode {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(Opcode::STOP),
            0x01 => Ok(Opcode::ADD),
            0x02 => Ok(Opcode::MUL),
            0x03 => Ok(Opcode::SUB),
            0x04 => Ok(Opcode::DIV),
            0x05 => Ok(Opcode::SDIV),
            0x06 => Ok(Opcode::MOD),
            0x07 => Ok(Opcode::SMOD),
            0x08 => Ok(Opcode::ADDMOD),
            0x09 => Ok(Opcode::MULMOD),
            0x0A => Ok(Opcode::EXP),
            0x0B => Ok(Opcode::SIGNEXTEND),
            0x10 => Ok(Opcode::LT),
            0x11 => Ok(Opcode::GT),
            0x12 => Ok(Opcode::SLT),
            0x13 => Ok(Opcode::SGT),
            0x14 => Ok(Opcode::EQ),
            0x15 => Ok(Opcode::ISZERO),
            0x16 => Ok(Opcode::AND),
            0x17 => Ok(Opcode::OR),
            0x18 => Ok(Opcode::XOR),
            0x19 => Ok(Opcode::NOT),
            0x1A => Ok(Opcode::BYTE),
            0x1B => Ok(Opcode::SHL),
            0x1C => Ok(Opcode::SHR),
            0x1D => Ok(Opcode::SAR),
            0x20 => Ok(Opcode::SHA3),
            0x30 => Ok(Opcode::ADDRESS),
            0x31 => Ok(Opcode::BALANCE),
            0x32 => Ok(Opcode::ORIGIN),
            0x33 => Ok(Opcode::CALLER),
            0x34 => Ok(Opcode::CALLVALUE),
            0x35 => Ok(Opcode::CALLDATALOAD),
            0x36 => Ok(Opcode::CALLDATASIZE),
            0x37 => Ok(Opcode::CALLDATACOPY),
            0x38 => Ok(Opcode::CODESIZE),
            0x39 => Ok(Opcode::CODECOPY),
            0x3A => Ok(Opcode::GASPRICE),
            0x3B => Ok(Opcode::EXTCODESIZE),
            0x3C => Ok(Opcode::EXTCODECOPY),
            0x3D => Ok(Opcode::RETURNDATASIZE),
            0x3E => Ok(Opcode::RETURNDATACOPY),
            0x3F => Ok(Opcode::EXTCODEHASH),
            0x40 => Ok(Opcode::BLOCKHASH),
            0x41 => Ok(Opcode::COINBASE),
            0x42 => Ok(Opcode::TIMESTAMP),
            0x43 => Ok(Opcode::NUMBER),
            0x44 => Ok(Opcode::PREVRANDAO),
            0x45 => Ok(Opcode::GASLIMIT),
            0x46 => Ok(Opcode::CHAINID),
            0x47 => Ok(Opcode::SELFBALANCE),
            0x48 => Ok(Opcode::BASEFEE),
            0x50 => Ok(Opcode::POP),
            0x51 => Ok(Opcode::MLOAD),
            0x52 => Ok(Opcode::MSTORE),
            0x53 => Ok(Opcode::MSTORE8),
            0x54 => Ok(Opcode::SLOAD),
            0x55 => Ok(Opcode::SSTORE),
            0x56 => Ok(Opcode::JUMP),
            0x57 => Ok(Opcode::JUMPI),
            0x58 => Ok(Opcode::PC),
            0x59 => Ok(Opcode::MSIZE),
            0x5A => Ok(Opcode::GAS),
            0x5B => Ok(Opcode::JUMPDEST),
            0x60 => Ok(Opcode::PUSH1),
            0x61 => Ok(Opcode::PUSH2),
            0x62 => Ok(Opcode::PUSH3),
            0x63 => Ok(Opcode::PUSH4),
            0x64 => Ok(Opcode::PUSH5),
            0x65 => Ok(Opcode::PUSH6),
            0x66 => Ok(Opcode::PUSH7),
            0x67 => Ok(Opcode::PUSH8),
            0x68 => Ok(Opcode::PUSH9),
            0x69 => Ok(Opcode::PUSH10),
            0x6A => Ok(Opcode::PUSH11),
            0x6B => Ok(Opcode::PUSH12),
            0x6C => Ok(Opcode::PUSH13),
            0x6D => Ok(Opcode::PUSH14),
            0x6E => Ok(Opcode::PUSH15),
            0x6F => Ok(Opcode::PUSH16),
            0x70 => Ok(Opcode::PUSH17),
            0x71 => Ok(Opcode::PUSH18),
            0x72 => Ok(Opcode::PUSH19),
            0x73 => Ok(Opcode::PUSH20),
            0x74 => Ok(Opcode::PUSH21),
            0x75 => Ok(Opcode::PUSH22),
            0x76 => Ok(Opcode::PUSH23),
            0x77 => Ok(Opcode::PUSH24),
            0x78 => Ok(Opcode::PUSH25),
            0x79 => Ok(Opcode::PUSH26),
            0x7A => Ok(Opcode::PUSH27),
            0x7B => Ok(Opcode::PUSH28),
            0x7C => Ok(Opcode::PUSH29),
            0x7D => Ok(Opcode::PUSH30),
            0x7E => Ok(Opcode::PUSH31),
            0x7F => Ok(Opcode::PUSH32),
            0x80 => Ok(Opcode::DUP1),
            0x81 => Ok(Opcode::DUP2),
            0x82 => Ok(Opcode::DUP3),
            0x83 => Ok(Opcode::DUP4),
            0x84 => Ok(Opcode::DUP5),
            0x85 => Ok(Opcode::DUP6),
            0x86 => Ok(Opcode::DUP7),
            0x87 => Ok(Opcode::DUP8),
            0x88 => Ok(Opcode::DUP9),
            0x89 => Ok(Opcode::DUP10),
            0x8A => Ok(Opcode::DUP11),
            0x8B => Ok(Opcode::DUP12),
            0x8C => Ok(Opcode::DUP13),
            0x8D => Ok(Opcode::DUP14),
            0x8E => Ok(Opcode::DUP15),
            0x8F => Ok(Opcode::DUP16),
            0x90 => Ok(Opcode::SWAP1),
            0x91 => Ok(Opcode::SWAP2),
            0x92 => Ok(Opcode::SWAP3),
            0x93 => Ok(Opcode::SWAP4),
            0x94 => Ok(Opcode::SWAP5),
            0x95 => Ok(Opcode::SWAP6),
            0x96 => Ok(Opcode::SWAP7),
            0x97 => Ok(Opcode::SWAP8),
            0x98 => Ok(Opcode::SWAP9),
            0x99 => Ok(Opcode::SWAP10),
            0x9A => Ok(Opcode::SWAP11),
            0x9B => Ok(Opcode::SWAP12),
            0x9C => Ok(Opcode::SWAP13),
            0x9D => Ok(Opcode::SWAP14),
            0x9E => Ok(Opcode::SWAP15),
            0x9F => Ok(Opcode::SWAP16),
            0xA0 => Ok(Opcode::LOG0),
            0xA1 => Ok(Opcode::LOG1),
            0xA2 => Ok(Opcode::LOG2),
            0xA3 => Ok(Opcode::LOG3),
            0xA4 => Ok(Opcode::LOG4),
            0xF0 => Ok(Opcode::CREATE),
            0xF1 => Ok(Opcode::CALL),
            0xF2 => Ok(Opcode::CALLCODE),
            0xF3 => Ok(Opcode::RETURN),
            0xF4 => Ok(Opcode::DELEGATECALL),
            0xF5 => Ok(Opcode::CREATE2),
            0xFA => Ok(Opcode::STATICCALL),
            0xFD => Ok(Opcode::REVERT),
            0xFE => Ok(Opcode::INVALID),
            0xFF => Ok(Opcode::SELFDESTRUCT),
            // ... other opcodes
            _ => Err(format!("Invalid opcode: {}", value)),
        }
    }
}

impl Opcode {
    pub fn execute<H: Host>(&self, interp: &mut Interpreter, host: &mut H) -> ControlFlow {
        match self {
            Opcode::STOP => {
                // GAS
                interp.gas += self.fix_gas();
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::ADD => {
                // STACK
                let a = interp.stack.pop().to_u256();
                let b = interp.stack.pop().to_u256();
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                // rely on U256 overflowing_add to handle overflow
                let (result, _) = a.overflowing_add(b);
                interp.stack.push_u256(result);
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::MUL => {
                // STACK
                let a = interp.stack.pop().to_u256();
                let b = interp.stack.pop().to_u256();
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                // rely on U256 overflowing_mul to handle overflow
                let (result, _) = a.overflowing_mul(b);
                interp.stack.push_u256(result);
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::SUB => {
                // STACK
                let a = interp.stack.pop().to_u256();
                let b = interp.stack.pop().to_u256();
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                // rely on U256 overflowing_sub to handle underflow
                let (result, _) = a.overflowing_sub(b);
                interp.stack.push_u256(result);
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::DIV => {
                // STACK
                let a = interp.stack.pop().to_u256();
                let b = interp.stack.pop().to_u256();
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let result = if b.is_zero() { U256::zero() } else { a / b };
                interp.stack.push_u256(result);
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::SDIV => {
                // STACK
                let a = interp.stack.pop().to_u256();
                let b = interp.stack.pop().to_u256();
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let result = if b.is_zero() {
                    U256::zero()
                } else {
                    let (a_neg, a_twos) = (a.bit(255), a.not().overflowing_add(U256::one()).0);
                    let (b_neg, b_twos) = (b.bit(255), b.not().overflowing_add(U256::one()).0);
                    let div = if a_neg { a_twos } else { a } / if b_neg { b_twos } else { b };
                    if a_neg ^ b_neg {
                        div.not().overflowing_add(U256::one()).0
                    } else {
                        div
                    }
                };
                interp.stack.push_u256(result);
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::MOD => {
                // STACK
                let a = interp.stack.pop().to_u256();
                let b = interp.stack.pop().to_u256();
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let result = if b.is_zero() { U256::zero() } else { a % b };
                interp.stack.push_u256(result);
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::SMOD => {
                // STACK
                let a = interp.stack.pop().to_u256();
                let b = interp.stack.pop().to_u256();
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let result = if b.is_zero() {
                    U256::zero()
                } else {
                    let (a_neg, a_twos) = (a.bit(255), a.not().overflowing_add(U256::one()).0);
                    let (b_neg, b_twos) = (b.bit(255), b.not().overflowing_add(U256::one()).0);
                    let div = if a_neg { a_twos } else { a } % if b_neg { b_twos } else { b };
                    if a_neg | b_neg {
                        div.not().overflowing_add(U256::one()).0
                    } else {
                        div
                    }
                };
                interp.stack.push_u256(result);
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::ADDMOD => {
                // STACK
                let a = interp.stack.pop().to_u512();
                let b = interp.stack.pop().to_u512();
                let c = interp.stack.pop().to_u256();
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let result = if c.is_zero() {
                    U256::zero()
                } else {
                    ((a + b) % c).try_into().unwrap()
                };
                interp.stack.push_u256(result);
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::MULMOD => {
                // STACK
                let a = interp.stack.pop().to_u512();
                let b = interp.stack.pop().to_u512();
                let c = interp.stack.pop().to_u256();
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let result = if c.is_zero() {
                    U256::zero()
                } else {
                    ((a * b) % c).try_into().unwrap()
                };
                interp.stack.push_u256(result);
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::EXP => {
                // STACK
                let a = interp.stack.pop().to_u256();
                let b = interp.stack.pop().to_u256();
                // GAS
                let var_gas = if b != U256::zero() {
                    50 * (b.bits() + 7)
                } else {
                    0
                };
                interp.gas += self.fix_gas() + var_gas;
                // OPERATION
                let (result, _) = a.overflowing_pow(b);
                interp.stack.push_u256(result);
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::SIGNEXTEND => {
                // STACK
                let exp = interp.stack.pop().as_usize();
                let num = interp.stack.pop().to_u256();
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let id = (exp + 1) * 8;
                let result = if num.bit(id - 1) {
                    U256::MAX.shl(id).bitor(num)
                } else {
                    num
                };
                interp.stack.push_u256(result);
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::LT => {
                // STACK
                let a = interp.stack.pop().to_u256();
                let b = interp.stack.pop().to_u256();
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let result = if a < b { U256::one() } else { U256::zero() };
                interp.stack.push_u256(result);
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::GT => {
                // STACK
                let a = interp.stack.pop().to_u256();
                let b = interp.stack.pop().to_u256();
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let result = if a > b { U256::one() } else { U256::zero() };
                interp.stack.push_u256(result);
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::SLT => {
                // STACK
                let a = interp.stack.pop().to_u256();
                let b = interp.stack.pop().to_u256();
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let (a_twos, _) = a.not().overflowing_add(U256::one());
                let (b_twos, _) = b.not().overflowing_add(U256::one());
                let result = if a_twos > b_twos {
                    U256::one()
                } else {
                    U256::zero()
                };
                interp.stack.push_u256(result);
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::SGT => {
                // STACK
                let a = interp.stack.pop().to_u256();
                let b = interp.stack.pop().to_u256();
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let (a_twos, _) = a.not().overflowing_add(U256::one());
                let (b_twos, _) = b.not().overflowing_add(U256::one());
                let result = if a_twos < b_twos {
                    U256::one()
                } else {
                    U256::zero()
                };
                interp.stack.push_u256(result);
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::EQ => {
                // STACK
                let a = interp.stack.pop();
                let b = interp.stack.pop();
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                interp.stack.push(if a == b {
                    Bytes32::one()
                } else {
                    Bytes32::zero()
                });
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::ISZERO => {
                // STACK
                let a = interp.stack.pop();
                interp.stack.push(if a.is_zero() {
                    Bytes32::one()
                } else {
                    Bytes32::zero()
                });
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::AND => {
                // STACK
                let a = interp.stack.pop();
                let b = interp.stack.pop();
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let result = a.bitand(b);
                interp.stack.push(result);
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::OR => {
                // STACK
                let a = interp.stack.pop();
                let b = interp.stack.pop();
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let result = a.bitor(b);
                interp.stack.push(result);
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::XOR => {
                // STACK
                let a = interp.stack.pop();
                let b = interp.stack.pop();
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let result = a.bitxor(b);
                interp.stack.push(result);
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::NOT => {
                // STACK
                let a = interp.stack.pop();
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let result = a.not();
                interp.stack.push(result);
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::BYTE => {
                // STACK
                let index = interp.stack.pop().as_usize();
                let word = interp.stack.pop();
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let result = Bytes32::from_vec(vec![word.get_byte(index)]);
                interp.stack.push(result);
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::SHL => {
                // STACK
                // let index = interp.stack.pop().as_usize();
                let index = interp.stack.pop();
                let word = interp.stack.pop().to_u256();
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let result = word.shl(index.as_usize());
                interp.stack.push_u256(result);
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::SHR => {
                // STACK
                // let index = interp.stack.pop().as_usize();
                let index = interp.stack.pop();
                let word = interp.stack.pop().to_u256();
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let result = word.shr(index.as_usize());
                interp.stack.push_u256(result);
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::SAR => {
                // STACK
                let index = interp.stack.pop().as_usize();
                let word = interp.stack.pop().to_u256();
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let result = if word.bit(255) {
                    if index > 255 {
                        U256::MAX
                    } else {
                        word.shr(index)
                            .bitor(U256::MAX.shl(U256::from(255) - index))
                    }
                } else {
                    word.shr(index)
                };
                interp.stack.push_u256(result);
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::SHA3 => {
                // STACK
                let offset = interp.stack.pop().as_usize();
                let size = interp.stack.pop().as_usize();
                // GAS
                interp.gas += self.fix_gas() + 6 * (size + 31) / 32;
                // // OPERATION
                let result = Bytes32::from_slice(
                    Keccak256::digest(interp.memory.load(offset, size).as_slice()).as_slice(),
                );
                interp.stack.push(result);
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::ADDRESS => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                interp.stack.push_address(interp.contract.address);
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::BALANCE => {
                // STACK
                let address = interp.stack.pop().to_address();
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                interp.stack.push_u256(host.balance(&address));
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::ORIGIN => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                interp.stack.push_address(host.env().tx.originator);
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::CALLER => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                interp.stack.push_address(interp.contract.caller);
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::CALLVALUE => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                interp.stack.push_u256(interp.contract.value);
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::CALLDATALOAD => {
                // STACK
                let offset = interp.stack.pop().as_usize();
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let mut result = [0u8; 32];
                let calldata = interp.contract.calldata;
                let (end, len) = if offset + 32 > calldata.len() {
                    (32, 32 - offset)
                } else {
                    (offset + 32, 32)
                };
                if len == 32 {
                    result.copy_from_slice(&calldata[offset..end]);
                } else {
                    result[..len].copy_from_slice(&calldata[offset..end]);
                }
                interp.stack.push(Bytes32::from_slice(&result));
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::CALLDATASIZE => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let result = interp.contract.data_size();
                interp.stack.push_usize(result);
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::CALLDATACOPY => {
                // STACK
                let memory_offset = interp.stack.pop().as_usize();
                let offset = interp.stack.pop().as_usize();
                let size = interp.stack.pop().as_usize();
                // GASs
                interp.gas += self.fix_gas();
                // OPERATION
                let mut result = vec![0u8; size];
                let calldata = interp.contract.calldata.clone();
                let (end, len) = if offset + size > calldata.len() {
                    (size, size - offset)
                } else {
                    (offset + size, size)
                };
                if len == size {
                    result.copy_from_slice(&calldata[offset..end]);
                } else {
                    result[..len].copy_from_slice(&calldata[offset..end]);
                }
                interp.memory.store(memory_offset, Bytes::from_vec(result));
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::CODESIZE => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                interp.stack.push_usize(host.code_size(&interp.contract.address));
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::CODECOPY => {
                // STACK
                let memory_offset = interp.stack.pop().as_usize();
                let offset = interp.stack.pop().as_usize();
                let mut size = interp.stack.pop().as_usize();
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let code = host.code(&interp.contract.address);
                let code_size = host.code_size(&interp.contract.address);
                if size > code_size { size = code_size };
                let mut result = vec![0u8; size];
                let (end, len) = if offset + size > code_size {
                    (size, size - offset)
                } else {
                    (offset + size, size)
                };
                if len == size {
                    result.copy_from_slice(&code[offset..end]);
                } else {
                    result[..len].copy_from_slice(&code[offset..end]);
                }
                interp.memory.store(memory_offset, Bytes::from_vec(result));
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::GASPRICE => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                interp.stack.push_u256(host.env().tx.gas_price);
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::EXTCODESIZE => {
                // STACK
                let address = interp.stack.pop().to_address();
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                interp.stack.push_usize(host.code_size(&address));
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::EXTCODECOPY => {
                // STACK
                let address = interp.stack.pop().to_address();
                let memory_offset = interp.stack.pop().as_usize();
                let offset = interp.stack.pop().as_usize();
                let size = interp.stack.pop().as_usize();
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let code = host.code(&address);
                let mut result = vec![0u8; size];
                let (end, len) = if size > code.len() {
                    (code.len(), code.len() - offset)
                } else {
                    if offset + size > code.len() {
                        (size, size - offset)
                    } else {
                        (offset + size, size)
                    }
                };
                if len == size {
                    result.copy_from_slice(&code[offset..end]);
                } else {
                    result[..len].copy_from_slice(&code[offset..end]);
                };
                interp.memory.store(memory_offset, Bytes::from_vec(result));
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::RETURNDATASIZE => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let data = &interp.return_data;
                interp.stack.push_usize(data.len());
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::RETURNDATACOPY => {
                // STACK
                let memory_offset = interp.stack.pop().as_usize();
                let offset = interp.stack.pop().as_usize();
                let mut size = interp.stack.pop().as_usize();
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let data = interp.return_data.clone();
                if size > data.len() {
                    size = data.len()
                }
                let mut result = vec![0u8; size];
                let (end, len) = if offset + size > data.len() {
                    (size, size - offset)
                } else {
                    (offset + size, size)
                };
                if len == size {
                    result.copy_from_slice(&data[offset..end]);
                } else {
                    result[..len].copy_from_slice(&data[offset..end]);
                }
                interp.memory.store(memory_offset, Bytes::from_vec(result));
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::EXTCODEHASH => {
                // STACK
                let address = interp.stack.pop().to_address();
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                interp.stack.push(host.code_hash(&address));
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::BLOCKHASH => {
                // STACK
                let _block_number = interp.stack.pop();
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let result = Bytes32::zero();
                // let result = match host.env().block.block_hash(block_number) {
                //     Some(hash) => hash,
                //     None => U256::zero(),
                // };
                interp.stack.push(result);
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::COINBASE => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let result = match host.env().block.beneficiary {
                    Some(coinbase) => coinbase.to_u256(),
                    None => U256::zero(),
                };
                interp.stack.push_u256(result);
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::TIMESTAMP => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                interp.stack.push_u256(host.env().block.timestamp);
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::NUMBER => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let result = match host.env().block.number {
                    Some(number) => Bytes32::from_u64(number).to_u256(),
                    None => U256::zero(),
                };
                interp.stack.push_u256(result);
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::PREVRANDAO => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let result = match host.env().block.prev_randao {
                    Some(number) => number,
                    // If block.prev_randao is None, use block.difficulty instead
                    None => match host.env().block.difficulty {
                        Some(number) => number,
                        None => U256::zero(),
                    },
                };
                interp.stack.push_u256(result);
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::GASLIMIT => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                interp.stack.push_u256(host.env().block.gas_limit);
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::CHAINID => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let chain_id = Bytes32::from_u64(host.env().block.chain_id).to_u256();
                interp.stack.push_u256(chain_id);
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::SELFBALANCE => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                interp.stack.push_u256(host.balance(&interp.contract.address));
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::BASEFEE => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let base_fee = match host.env().block.base_fee {
                    Some(base_fee) => base_fee,
                    None => U256::zero(),
                };
                interp.stack.push_u256(base_fee);
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::POP => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                interp.stack.pop();
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::MLOAD => {
                // STACK
                let offset = interp.stack.pop();
                // GAS
                interp.gas += self.fix_gas() * interp.memory.expansion(offset.as_usize(), 32);
                // OPERATION
                let value = interp.memory.load(offset.as_usize(), 32);
                interp.stack.push(value.as_bytes32());
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::MSTORE => {
                // STACK
                let offset = interp.stack.pop();
                let value = interp.stack.pop();
                println!(" > MSTORE\n   - offset: {:#X}\n   -  value: {:#X}", offset, value);
                // GAS
                interp.gas += self.fix_gas() * interp.memory.expansion(offset.as_usize(), 32);
                // OPERATION
                interp.memory
                    .store(offset.as_usize(), Bytes::from_bytes32(value));
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::MSTORE8 => {
                // STACK
                let offset = interp.stack.pop();
                let value = interp.stack.pop();
                // GAS
                interp.gas += self.fix_gas() * interp.memory.expansion(offset.as_usize(), 1);
                // OPERATION
                interp.memory
                    .store(offset.as_usize(), Bytes::from_byte(value.get_byte(31)));
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::SLOAD => {
                // STACK
                let key = interp.stack.pop().to_u256();
                // GAS
                interp.gas += self.fix_gas(); //+ self.state_access_gas(key);
                // OPERATION
                let value = host.sload(&interp.contract.address, key);
                interp.stack.push(value);
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::SSTORE => {
                // CHECK REVERT CONDITION
                if interp.is_static { return ControlFlow::Revert; }
                // STACK
                let key = interp.stack.pop().to_u256();
                let value = interp.stack.pop();
                // GASs
                interp.gas += self.fix_gas(); //+ self.state_access_gas(key);
                // OPERATION
                host.sstore(&interp.contract.address, key, value);
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::JUMP => {
                // STACK
                let jumpdest = interp.stack.pop().as_usize();
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                match validate_jumpdest(&interp.contract.code, jumpdest) {
                    true => {
                        // PC
                        interp.pc = jumpdest;
                        // SUCCESS
                        ControlFlow::Continue
                    }
                    false => ControlFlow::Revert,
                }
            },
            Opcode::JUMPI => {
                // STACK
                let jumpdest = interp.stack.pop().as_usize();
                let condition = interp.stack.pop().to_u256();
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                match condition.is_zero() {
                    true => {
                        // PC
                        interp.pc += 1;
                        // SUCCESS
                        ControlFlow::Continue
                    }
                    false => {
                        match validate_jumpdest(&interp.contract.code, jumpdest) {
                            true => {
                                // PC
                                interp.pc = jumpdest;
                                // SUCCESS
                                ControlFlow::Continue
                            }
                            false => ControlFlow::Revert,
                        }
                    }
                }
            },
            Opcode::PC => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                interp.stack.push_usize(interp.pc);
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::MSIZE => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                interp.stack.push_usize(interp.memory.size());
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::GAS => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                // TODO
                // interp.stack.push(U256::from(interp.gas));
                interp.stack.push_u256(U256::max_value());
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::JUMPDEST => {
                // GAS
                interp.gas += self.fix_gas();
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::PUSH1 => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let value = &[interp.contract.code[interp.pc + 1]];
                interp.stack.push(Bytes32::from_slice(value));
                // PC
                interp.pc += 2;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::PUSH2 => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let value = &interp.contract.code[interp.pc + 1..interp.pc + 3];
                interp.stack.push(Bytes32::from_slice(value));
                // PC
                interp.pc += 3;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::PUSH3 => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let value = &interp.contract.code[interp.pc + 1..interp.pc + 4];
                interp.stack.push(Bytes32::from_slice(value));
                // PC
                interp.pc += 4;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::PUSH4 => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let value = &interp.contract.code[interp.pc + 1..interp.pc + 5];
                interp.stack.push(Bytes32::from_slice(value));
                // PC
                interp.pc += 5;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::PUSH5 => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let value = &interp.contract.code[interp.pc + 1..interp.pc + 6];
                interp.stack.push(Bytes32::from_slice(value));
                // PC
                interp.pc += 6;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::PUSH6 => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let value = &interp.contract.code[interp.pc + 1..interp.pc + 7];
                interp.stack.push(Bytes32::from_slice(value));
                // PC
                interp.pc += 7;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::PUSH7 => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let value = &interp.contract.code[interp.pc + 1..interp.pc + 8];
                interp.stack.push(Bytes32::from_slice(value));
                // PC
                interp.pc += 8;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::PUSH8 => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let value = &interp.contract.code[interp.pc + 1..interp.pc + 9];
                interp.stack.push(Bytes32::from_slice(value));
                // PC
                interp.pc += 9;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::PUSH9 => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let value = &interp.contract.code[interp.pc + 1..interp.pc + 10];
                interp.stack.push(Bytes32::from_slice(value));
                // PC
                interp.pc += 10;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::PUSH10 => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let value = &interp.contract.code[interp.pc + 1..interp.pc + 11];
                interp.stack.push(Bytes32::from_slice(value));
                // PC
                interp.pc += 11;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::PUSH11 => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let value = &interp.contract.code[interp.pc + 1..interp.pc + 12];
                interp.stack.push(Bytes32::from_slice(value));
                // PC
                interp.pc += 12;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::PUSH12 => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let value = &interp.contract.code[interp.pc + 1..interp.pc + 13];
                interp.stack.push(Bytes32::from_slice(value));
                // PC
                interp.pc += 13;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::PUSH13 => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let value = &interp.contract.code[interp.pc + 1..interp.pc + 14];
                interp.stack.push(Bytes32::from_slice(value));
                // PC
                interp.pc += 14;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::PUSH14 => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let value = &interp.contract.code[interp.pc + 1..interp.pc + 15];
                interp.stack.push(Bytes32::from_slice(value));
                // PC
                interp.pc += 15;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::PUSH15 => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let value = &interp.contract.code[interp.pc + 1..interp.pc + 16];
                interp.stack.push(Bytes32::from_slice(value));
                // PC
                interp.pc += 16;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::PUSH16 => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let value = &interp.contract.code[interp.pc + 1..interp.pc + 17];
                interp.stack.push(Bytes32::from_slice(value));
                // PC
                interp.pc += 17;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::PUSH17 => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let value = &interp.contract.code[interp.pc + 1..interp.pc + 18];
                interp.stack.push(Bytes32::from_slice(value));
                // PC
                interp.pc += 18;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::PUSH18 => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let value = &interp.contract.code[interp.pc + 1..interp.pc + 19];
                interp.stack.push(Bytes32::from_slice(value));
                // PC
                interp.pc += 19;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::PUSH19 => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let value = &interp.contract.code[interp.pc + 1..interp.pc + 20];
                interp.stack.push(Bytes32::from_slice(value));
                // PC
                interp.pc += 20;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::PUSH20 => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let value = &interp.contract.code[interp.pc + 1..interp.pc + 21];
                interp.stack.push(Bytes32::from_slice(value));
                // PC
                interp.pc += 21;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::PUSH21 => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let value = &interp.contract.code[interp.pc + 1..interp.pc + 22];
                interp.stack.push(Bytes32::from_slice(value));
                // PC
                interp.pc += 22;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::PUSH22 => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let value = &interp.contract.code[interp.pc + 1..interp.pc + 23];
                interp.stack.push(Bytes32::from_slice(value));
                // PC
                interp.pc += 23;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::PUSH23 => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let value = &interp.contract.code[interp.pc + 1..interp.pc + 24];
                interp.stack.push(Bytes32::from_slice(value));
                // PC
                interp.pc += 24;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::PUSH24 => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let value = &interp.contract.code[interp.pc + 1..interp.pc + 25];
                interp.stack.push(Bytes32::from_slice(value));
                // PC
                interp.pc += 25;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::PUSH25 => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let value = &interp.contract.code[interp.pc + 1..interp.pc + 26];
                interp.stack.push(Bytes32::from_slice(value));
                // PC
                interp.pc += 26;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::PUSH26 => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let value = &interp.contract.code[interp.pc + 1..interp.pc + 27];
                interp.stack.push(Bytes32::from_slice(value));
                // PC
                interp.pc += 27;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::PUSH27 => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let value = &interp.contract.code[interp.pc + 1..interp.pc + 28];
                interp.stack.push(Bytes32::from_slice(value));
                // PC
                interp.pc += 28;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::PUSH28 => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let value = &interp.contract.code[interp.pc + 1..interp.pc + 29];
                interp.stack.push(Bytes32::from_slice(value));
                // PC
                interp.pc += 29;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::PUSH29 => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let value = &interp.contract.code[interp.pc + 1..interp.pc + 30];
                interp.stack.push(Bytes32::from_slice(value));
                // PC
                interp.pc += 30;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::PUSH30 => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let value = &interp.contract.code[interp.pc + 1..interp.pc + 31];
                interp.stack.push(Bytes32::from_slice(value));
                // PC
                interp.pc += 31;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::PUSH31 => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let value = &interp.contract.code[interp.pc + 1..interp.pc + 32];
                interp.stack.push(Bytes32::from_slice(value));
                // PC
                interp.pc += 32;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::PUSH32 => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let value = &interp.contract.code[interp.pc + 1..interp.pc + 33];
                interp.stack.push(Bytes32::from_slice(value));
                // PC
                interp.pc += 33;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::DUP1 => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let value = interp.stack.get_item(interp.stack.depth() - 1);
                match value {
                    Some(value) => interp.stack.push(value),
                    None => panic!("Stack underflow"),
                };
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::DUP2 => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let value = interp.stack.get_item(interp.stack.depth() - 2);
                match value {
                    Some(value) => interp.stack.push(value),
                    None => panic!("Stack underflow"),
                };
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::DUP3 => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let value = interp.stack.get_item(interp.stack.depth() - 3);
                match value {
                    Some(value) => interp.stack.push(value),
                    None => panic!("Stack underflow"),
                };
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::DUP4 => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let value = interp.stack.get_item(interp.stack.depth() - 4);
                match value {
                    Some(value) => interp.stack.push(value),
                    None => panic!("Stack underflow"),
                };
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::DUP5 => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let value = interp.stack.get_item(interp.stack.depth() - 5);
                match value {
                    Some(value) => interp.stack.push(value),
                    None => panic!("Stack underflow"),
                };
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::DUP6 => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let value = interp.stack.get_item(interp.stack.depth() - 6);
                match value {
                    Some(value) => interp.stack.push(value),
                    None => panic!("Stack underflow"),
                };
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::DUP7 => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let value = interp.stack.get_item(interp.stack.depth() - 7);
                match value {
                    Some(value) => interp.stack.push(value),
                    None => panic!("Stack underflow"),
                };
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::DUP8 => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let value = interp.stack.get_item(interp.stack.depth() - 8);
                match value {
                    Some(value) => interp.stack.push(value),
                    None => panic!("Stack underflow"),
                };
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::DUP9 => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let value = interp.stack.get_item(interp.stack.depth() - 9);
                match value {
                    Some(value) => interp.stack.push(value),
                    None => panic!("Stack underflow"),
                };
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::DUP10 => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let value = interp.stack.get_item(interp.stack.depth() - 10);
                match value {
                    Some(value) => interp.stack.push(value),
                    None => panic!("Stack underflow"),
                };
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::DUP11 => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let value = interp.stack.get_item(interp.stack.depth() - 11);
                match value {
                    Some(value) => interp.stack.push(value),
                    None => panic!("Stack underflow"),
                };
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::DUP12 => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let value = interp.stack.get_item(interp.stack.depth() - 12);
                match value {
                    Some(value) => interp.stack.push(value),
                    None => panic!("Stack underflow"),
                };
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::DUP13 => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let value = interp.stack.get_item(interp.stack.depth() - 13);
                match value {
                    Some(value) => interp.stack.push(value),
                    None => panic!("Stack underflow"),
                };
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::DUP14 => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let value = interp.stack.get_item(interp.stack.depth() - 14);
                match value {
                    Some(value) => interp.stack.push(value),
                    None => panic!("Stack underflow"),
                };
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::DUP15 => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let value = interp.stack.get_item(interp.stack.depth() - 15);
                match value {
                    Some(value) => interp.stack.push(value),
                    None => panic!("Stack underflow"),
                };
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::DUP16 => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let value = interp.stack.get_item(interp.stack.depth() - 16);
                match value {
                    Some(value) => interp.stack.push(value),
                    None => panic!("Stack underflow"),
                };
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::SWAP1 => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                interp.stack.swap(1);
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::SWAP2 => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                interp.stack.swap(2);
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::SWAP3 => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                interp.stack.swap(3);
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::SWAP4 => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                interp.stack.swap(4);
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::SWAP5 => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                interp.stack.swap(5);
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::SWAP6 => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                interp.stack.swap(6);
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::SWAP7 => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                interp.stack.swap(7);
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::SWAP8 => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                interp.stack.swap(8);
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::SWAP9 => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                interp.stack.swap(9);
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::SWAP10 => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                interp.stack.swap(10);
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::SWAP11 => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                interp.stack.swap(11);
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::SWAP12 => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                interp.stack.swap(12);
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::SWAP13 => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                interp.stack.swap(13);
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::SWAP14 => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                interp.stack.swap(14);
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::SWAP15 => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                interp.stack.swap(15);
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::SWAP16 => {
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                interp.stack.swap(16);
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::LOG0 => {
                // STACK
                let offset = interp.stack.pop().as_usize();
                let size = interp.stack.pop().as_usize();
                // GAS
                interp.gas += self.fix_gas();// + self.log_gas(offset, size);
                // OPERATION
                let data = interp.memory.load(offset, size);
                let log = Log::new(interp.contract.address, data);
                host.log(log);
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::LOG1 => {
                // STACK
                let offset = interp.stack.pop().as_usize();
                let size = interp.stack.pop().as_usize();
                let topic1 = interp.stack.pop();
                // GAS
                interp.gas += self.fix_gas();// + self.log_gas(offset, size);
                // OPERATION
                let data = interp.memory.load(offset, size);
                let mut log = Log::new(interp.contract.address, data);
                log.add_topic(topic1);
                host.log(log);
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::LOG2 => {
                // STACK
                let offset = interp.stack.pop().as_usize();
                let size = interp.stack.pop().as_usize();
                let topic1 = interp.stack.pop();
                let topic2 = interp.stack.pop();
                // GAS
                interp.gas += self.fix_gas();// + self.log_gas(offset, size);
                // OPERATION
                let data = interp.memory.load(offset, size);
                let mut log = Log::new(interp.contract.address, data);
                log.add_topics(vec![topic1, topic2]);
                host.log(log);
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::LOG3 => {
                // STACK
                let offset = interp.stack.pop().as_usize();
                let size = interp.stack.pop().as_usize();
                let topic1 = interp.stack.pop();
                let topic2 = interp.stack.pop();
                let topic3 = interp.stack.pop();
                // GAS
                interp.gas += self.fix_gas();// + self.log_gas(offset, size);
                // OPERATION
                let data = interp.memory.load(offset, size);
                let mut log = Log::new(interp.contract.address, data);
                log.add_topics(vec![topic1, topic2, topic3]);
                host.log(log);
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::LOG4 => {
                // STACK
                let offset = interp.stack.pop().as_usize();
                let size = interp.stack.pop().as_usize();
                let topic1 = interp.stack.pop();
                let topic2 = interp.stack.pop();
                let topic3 = interp.stack.pop();
                let topic4 = interp.stack.pop();
                // GAS
                interp.gas += self.fix_gas();// + self.log_gas(offset, size);
                // OPERATION
                let data = interp.memory.load(offset, size);
                let mut log = Log::new(interp.contract.address, data);
                log.add_topics(vec![topic1, topic2, topic3, topic4]);
                host.log(log);
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::CREATE => {
                // STACK
                let value = interp.stack.pop().to_u256();
                let offset = interp.stack.pop().as_usize();
                let size = interp.stack.pop().as_usize();
                // CHECK REVERT CONDITION
                if interp.is_static & !value.is_zero() {
                    return ControlFlow::Revert;
                }
                if !interp.is_static & (host.balance(&host.env().tx.originator) < value) {
                    return ControlFlow::Revert;
                }
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let data = interp.memory.load(offset, size);
                let mut rlp_encoded1 = rlp_encode(interp.contract.address.as_slice());
                let mut rlp_encoded2 = rlp_encode(Bytes32::from_u256(host.nonce(&interp.contract.address)).as_slice());
                rlp_encoded1.append(&mut rlp_encoded2);
                let address = Address::from_slice(Keccak256::digest(rlp_encoded1).as_slice());
                let call_result = host.create_call(address, value, data);
                if !call_result.success.is_zero() {
                    interp.stack.push_address(address);
                } else {
                    interp.stack.push(Bytes32::zero());
                }
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            }
            Opcode::CALL => {
                // STACK
                let gas = interp.stack.pop().to_u256();
                let address = interp.stack.pop().to_address();
                let value = interp.stack.pop().to_u256();
                let args_offset = interp.stack.pop().as_usize();
                let args_size = interp.stack.pop().as_usize();
                let ret_offset = interp.stack.pop().as_usize();
                let ret_size = interp.stack.pop().as_usize();
                // CHECK REVERT CONDITION
                if interp.is_static & !value.is_zero() {
                    return ControlFlow::Revert;
                }
                if !interp.is_static & (host.balance(&host.env().tx.originator) < value) {
                    return ControlFlow::Revert;
                }
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let data = interp.memory.load(args_offset, args_size);
                let call = Call::new(
                    interp.contract.address,
                    address,
                    host.env().tx.originator,
                    gas,
                    U256::from(interp.gas_left()),
                    address,
                    data,
                    value,
                    false
                );
                let call_result = host.execute_call(call);
                let mut data = vec![0u8; ret_size];
                data.copy_from_slice(&call_result.result[0..ret_size]);
                interp.memory.store(ret_offset, Bytes::from_vec(data));
                interp.stack.push(call_result.success);
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::CALLCODE => {
                // STACK
                let gas = interp.stack.pop().to_u256();
                let address = interp.stack.pop().to_address();
                let value = interp.stack.pop().to_u256();
                let args_offset = interp.stack.pop().as_usize();
                let args_size = interp.stack.pop().as_usize();
                let ret_offset = interp.stack.pop().as_usize();
                let ret_size = interp.stack.pop().as_usize();
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let data = interp.memory.load(args_offset, args_size);
                let call = Call::new(
                    interp.contract.address,
                    address,
                    host.env().tx.originator,
                    gas,
                    U256::from(interp.gas_left()),
                    address,
                    data,
                    value,
                    false
                );
                let call_result = host.execute_call(call);
                let mut data = vec![0u8; ret_size];
                data.copy_from_slice(&call_result.result[0..ret_size]);
                interp.memory.store(ret_offset, Bytes::from_vec(data));
                interp.stack.push(call_result.success);
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::RETURN => {
                // STACK
                let offset = interp.stack.pop().as_usize();
                let size = interp.stack.pop().as_usize();
                // GAS
                interp.gas += self.fix_gas() * interp.memory.expansion(offset, size);
                // OPERATION
                let value = interp.memory.load(offset, size);
                interp.return_data = value;
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Return
            },
            Opcode::DELEGATECALL => {
                // STACK
                let gas = interp.stack.pop().to_u256();
                let address = interp.stack.pop().to_address();
                let args_offset = interp.stack.pop().as_usize();
                let args_size = interp.stack.pop().as_usize();
                let ret_offset = interp.stack.pop().as_usize();
                let ret_size = interp.stack.pop().as_usize();
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let data = interp.memory.load(args_offset, args_size);
                let call = Call::new(
                    interp.contract.address,
                    interp.contract.address,
                    host.env().tx.originator,
                    gas,
                    U256::from(interp.gas_left()),
                    address,
                    data,
                    U256::zero(),
                    false
                );
                let call_result = host.execute_call(call);
                if !call_result.success.is_zero() {
                    let mut data = vec![0u8; ret_size];
                    data.copy_from_slice(&call_result.result[0..ret_size]);
                    interp.memory.store(ret_offset, Bytes::from_vec(data));
                }
                interp.stack.push(call_result.success);
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::CREATE2 => {
                todo!()
            },
            Opcode::STATICCALL => {
                // STACK
                let gas = interp.stack.pop().to_u256();
                let address = interp.stack.pop().to_address();
                let args_offset = interp.stack.pop().as_usize();
                let args_size = interp.stack.pop().as_usize();
                let ret_offset = interp.stack.pop().as_usize();
                let ret_size = interp.stack.pop().as_usize();
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                let data = interp.memory.load(args_offset, args_size);
                let call = Call::new(
                    interp.contract.address,
                    address,
                    host.env().tx.originator,
                    gas,
                    U256::from(interp.gas_left()),
                    address,
                    data,
                    U256::zero(),
                    true
                );
                let call_result = host.execute_call(call);
                if !call_result.success.is_zero() {
                    let mut data = vec![0u8; ret_size];
                    data.copy_from_slice(&call_result.result[0..ret_size]);
                    interp.memory.store(ret_offset, Bytes::from_vec(data));
                }
                interp.stack.push(call_result.success);
                // PC
                interp.pc += 1;
                // CONTROL FLOW
                ControlFlow::Continue
            },
            Opcode::REVERT => {
                // STACK
                let offset = interp.stack.pop().as_usize();
                let size = interp.stack.pop().as_usize();
                // GAS
                interp.gas += self.fix_gas() * interp.memory.expansion(offset, size);
                // OPERATION
                let value = interp.memory.load(offset, size);
                interp.return_data = value;
                // PC
                interp.pc += 1;
                // SUCCESS
                ControlFlow::Revert
            },
            Opcode::INVALID => {
                // PC
                interp.pc += 1;
                // SUCCESS
                ControlFlow::Revert
            },
            Opcode::SELFDESTRUCT => {
                // STACK
                let address = interp.stack.pop().to_address();
                // CHECK REVERT CONDITION
                if interp.is_static {
                    return ControlFlow::Revert;
                }
                // GAS
                interp.gas += self.fix_gas();
                // OPERATION
                match host.selfdestruct(&interp.contract.address, &address, host.balance(&interp.contract.address)) {
                    Ok(_) => {
                        // PC
                        interp.pc += 1;
                        // SUCCESS
                        ControlFlow::Continue
                    }
                    Err(_) => ControlFlow::Revert,
                }
            }
        }
    }
}

impl Opcode {
    pub fn fix_gas(&self) -> usize {
        match self {
            // Gas: Zero
            Opcode::STOP => 0,
            Opcode::INVALID => 0,
            // Gas: Jumpdest
            Opcode::JUMPDEST => 1,
            // Gas: Base
            Opcode::ADDRESS => 2,
            Opcode::POP => 2,
            Opcode::PC => 2,
            Opcode::MSIZE => 2,
            Opcode::GAS => 2,
            // Gas: Verylow
            Opcode::MLOAD => 3,
            Opcode::MSTORE => 3,
            Opcode::MSTORE8 => 3,
            Opcode::ADD => 3,
            Opcode::SUB => 3,
            Opcode::LT => 3,
            Opcode::GT => 3,
            Opcode::SLT => 3,
            Opcode::SGT => 3,
            Opcode::EQ => 3,
            Opcode::ISZERO => 3,
            Opcode::AND => 3,
            Opcode::OR => 3,
            Opcode::XOR => 3,
            Opcode::NOT => 3,
            Opcode::BYTE => 3,
            Opcode::SHL => 3,
            Opcode::SHR => 3,
            Opcode::SAR => 3,
            Opcode::PUSH1 => 3,
            Opcode::PUSH2 => 3,
            Opcode::PUSH3 => 3,
            Opcode::PUSH4 => 3,
            Opcode::PUSH5 => 3,
            Opcode::PUSH6 => 3,
            Opcode::PUSH7 => 3,
            Opcode::PUSH8 => 3,
            Opcode::PUSH9 => 3,
            Opcode::PUSH10 => 3,
            Opcode::PUSH11 => 3,
            Opcode::PUSH12 => 3,
            Opcode::PUSH13 => 3,
            Opcode::PUSH14 => 3,
            Opcode::PUSH15 => 3,
            Opcode::PUSH16 => 3,
            Opcode::PUSH17 => 3,
            Opcode::PUSH18 => 3,
            Opcode::PUSH19 => 3,
            Opcode::PUSH20 => 3,
            Opcode::PUSH21 => 3,
            Opcode::PUSH22 => 3,
            Opcode::PUSH23 => 3,
            Opcode::PUSH24 => 3,
            Opcode::PUSH25 => 3,
            Opcode::PUSH26 => 3,
            Opcode::PUSH27 => 3,
            Opcode::PUSH28 => 3,
            Opcode::PUSH29 => 3,
            Opcode::PUSH30 => 3,
            Opcode::PUSH31 => 3,
            Opcode::PUSH32 => 3,
            Opcode::DUP1 => 3,
            Opcode::DUP2 => 3,
            Opcode::DUP3 => 3,
            Opcode::DUP4 => 3,
            Opcode::DUP5 => 3,
            Opcode::DUP6 => 3,
            Opcode::DUP7 => 3,
            Opcode::DUP8 => 3,
            Opcode::DUP9 => 3,
            Opcode::DUP10 => 3,
            Opcode::DUP11 => 3,
            Opcode::DUP12 => 3,
            Opcode::DUP13 => 3,
            Opcode::DUP14 => 3,
            Opcode::DUP15 => 3,
            Opcode::DUP16 => 3,
            Opcode::SWAP1 => 3,
            Opcode::SWAP2 => 3,
            Opcode::SWAP3 => 3,
            Opcode::SWAP4 => 3,
            Opcode::SWAP5 => 3,
            Opcode::SWAP6 => 3,
            Opcode::SWAP7 => 3,
            Opcode::SWAP8 => 3,
            Opcode::SWAP9 => 3,
            Opcode::SWAP10 => 3,
            Opcode::SWAP11 => 3,
            Opcode::SWAP12 => 3,
            Opcode::SWAP13 => 3,
            Opcode::SWAP14 => 3,
            Opcode::SWAP15 => 3,
            Opcode::SWAP16 => 3,
            // Gas: Low
            Opcode::MUL => 5,
            Opcode::DIV => 5,
            Opcode::SDIV => 5,
            Opcode::MOD => 5,
            Opcode::SMOD => 5,
            Opcode::SIGNEXTEND => 5,
            // Gas: Mid
            Opcode::ADDMOD => 8,
            Opcode::MULMOD => 8,
            Opcode::JUMP => 8,
            // Gas: High
            Opcode::EXP => 10,
            Opcode::JUMPI => 10,
            // Gas: Copy
            // Gas: Call
            // Gas: Extaccount
            // Gas: Keccak
            Opcode::SHA3 => 30,
            // TODO:
            _ => 0,
        }
    }
}

fn validate_jumpdest(code: &Bytes, pc_new: usize) -> bool {
    // Ensure informed jump destination
    match code[pc_new].try_into().unwrap() {
        Opcode::JUMPDEST => {
            // Ensure valid jump destination
            !matches!(
                code[pc_new - 1].try_into().unwrap(),
                Opcode::PUSH1
                    | Opcode::PUSH2
                    | Opcode::PUSH3
                    | Opcode::PUSH4
                    | Opcode::PUSH5
                    | Opcode::PUSH6
                    | Opcode::PUSH7
                    | Opcode::PUSH8
                    | Opcode::PUSH9
                    | Opcode::PUSH10
                    | Opcode::PUSH11
                    | Opcode::PUSH12
                    | Opcode::PUSH13
                    | Opcode::PUSH14
                    | Opcode::PUSH15
                    | Opcode::PUSH16
                    | Opcode::PUSH17
                    | Opcode::PUSH18
                    | Opcode::PUSH19
                    | Opcode::PUSH20
                    | Opcode::PUSH21
                    | Opcode::PUSH22
                    | Opcode::PUSH23
                    | Opcode::PUSH24
                    | Opcode::PUSH25
                    | Opcode::PUSH26
                    | Opcode::PUSH27
                    | Opcode::PUSH28
                    | Opcode::PUSH29
                    | Opcode::PUSH30
                    | Opcode::PUSH31
                    | Opcode::PUSH32
            )
        }
        _ => false,
    }
}
