use ethereum_types::U256;
use sha3::{Digest, Keccak256};
use std::convert::TryFrom;
use std::ops::{BitAnd, BitOr, BitXor, Not, Shl, Shr};

use crate::types::{Bytes, Bytes32, Address};
use crate::utils::rlp_encode;
use crate::call::Call;
use crate::logs::Log;

use super::ExecutionContext;

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
    pub fn execute(&self, ctx: &mut ExecutionContext) -> bool {
        match self {
            Opcode::STOP => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                ctx.stopped = true;
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::ADD => {
                // STACK
                let a = ctx.stack.pop().to_u256();
                let b = ctx.stack.pop().to_u256();
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                // rely on U256 overflowing_add to handle overflow
                let (result, _) = a.overflowing_add(b);
                ctx.stack.push_u256(result);
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::MUL => {
                // STACK
                let a = ctx.stack.pop().to_u256();
                let b = ctx.stack.pop().to_u256();
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                // rely on U256 overflowing_mul to handle overflow
                let (result, _) = a.overflowing_mul(b);
                ctx.stack.push_u256(result);
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::SUB => {
                // STACK
                let a = ctx.stack.pop().to_u256();
                let b = ctx.stack.pop().to_u256();
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                // rely on U256 overflowing_sub to handle underflow
                let (result, _) = a.overflowing_sub(b);
                ctx.stack.push_u256(result);
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::DIV => {
                // STACK
                let a = ctx.stack.pop().to_u256();
                let b = ctx.stack.pop().to_u256();
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let result = if b.is_zero() { U256::zero() } else { a / b };
                ctx.stack.push_u256(result);
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::SDIV => {
                // STACK
                let a = ctx.stack.pop().to_u256();
                let b = ctx.stack.pop().to_u256();
                // GAS
                ctx.gas += self.fix_gas();
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
                ctx.stack.push_u256(result);
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::MOD => {
                // STACK
                let a = ctx.stack.pop().to_u256();
                let b = ctx.stack.pop().to_u256();
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let result = if b.is_zero() { U256::zero() } else { a % b };
                ctx.stack.push_u256(result);
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::SMOD => {
                // STACK
                let a = ctx.stack.pop().to_u256();
                let b = ctx.stack.pop().to_u256();
                // GAS
                ctx.gas += self.fix_gas();
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
                ctx.stack.push_u256(result);
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::ADDMOD => {
                // STACK
                let a = ctx.stack.pop().to_u512();
                let b = ctx.stack.pop().to_u512();
                let c = ctx.stack.pop().to_u256();
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let result = if c.is_zero() {
                    U256::zero()
                } else {
                    ((a + b) % c).try_into().unwrap()
                };
                ctx.stack.push_u256(result);
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::MULMOD => {
                // STACK
                let a = ctx.stack.pop().to_u512();
                let b = ctx.stack.pop().to_u512();
                let c = ctx.stack.pop().to_u256();
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let result = if c.is_zero() {
                    U256::zero()
                } else {
                    ((a * b) % c).try_into().unwrap()
                };
                ctx.stack.push_u256(result);
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::EXP => {
                // STACK
                let a = ctx.stack.pop().to_u256();
                let b = ctx.stack.pop().to_u256();
                // GAS
                let var_gas = if b != U256::zero() {
                    50 * (b.bits() + 7)
                } else {
                    0
                };
                ctx.gas += self.fix_gas() + var_gas;
                // OPERATION
                let (result, _) = a.overflowing_pow(b);
                ctx.stack.push_u256(result);
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::SIGNEXTEND => {
                // STACK
                let exp = ctx.stack.pop().as_usize();
                let num = ctx.stack.pop().to_u256();
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let id = (exp + 1) * 8;
                let result = if num.bit(id - 1) {
                    U256::MAX.shl(id).bitor(num)
                } else {
                    num
                };
                ctx.stack.push_u256(result);
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::LT => {
                // STACK
                let a = ctx.stack.pop().to_u256();
                let b = ctx.stack.pop().to_u256();
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let result = if a < b { U256::one() } else { U256::zero() };
                ctx.stack.push_u256(result);
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::GT => {
                // STACK
                let a = ctx.stack.pop().to_u256();
                let b = ctx.stack.pop().to_u256();
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let result = if a > b { U256::one() } else { U256::zero() };
                ctx.stack.push_u256(result);
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::SLT => {
                // STACK
                let a = ctx.stack.pop().to_u256();
                let b = ctx.stack.pop().to_u256();
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let (a_twos, _) = a.not().overflowing_add(U256::one());
                let (b_twos, _) = b.not().overflowing_add(U256::one());
                let result = if a_twos > b_twos {
                    U256::one()
                } else {
                    U256::zero()
                };
                ctx.stack.push_u256(result);
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::SGT => {
                // STACK
                let a = ctx.stack.pop().to_u256();
                let b = ctx.stack.pop().to_u256();
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let (a_twos, _) = a.not().overflowing_add(U256::one());
                let (b_twos, _) = b.not().overflowing_add(U256::one());
                let result = if a_twos < b_twos {
                    U256::one()
                } else {
                    U256::zero()
                };
                ctx.stack.push_u256(result);
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::EQ => {
                // STACK
                let a = ctx.stack.pop();
                let b = ctx.stack.pop();
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                ctx.stack.push(if a == b {
                    Bytes32::one()
                } else {
                    Bytes32::zero()
                });
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::ISZERO => {
                // STACK
                let a = ctx.stack.pop();
                ctx.stack.push(if a.is_zero() {
                    Bytes32::one()
                } else {
                    Bytes32::zero()
                });
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::AND => {
                // STACK
                let a = ctx.stack.pop();
                let b = ctx.stack.pop();
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let result = a.bitand(b);
                ctx.stack.push(result);
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::OR => {
                // STACK
                let a = ctx.stack.pop();
                let b = ctx.stack.pop();
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let result = a.bitor(b);
                ctx.stack.push(result);
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::XOR => {
                // STACK
                let a = ctx.stack.pop();
                let b = ctx.stack.pop();
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let result = a.bitxor(b);
                ctx.stack.push(result);
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::NOT => {
                // STACK
                let a = ctx.stack.pop();
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let result = a.not();
                ctx.stack.push(result);
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::BYTE => {
                // STACK
                let index = ctx.stack.pop().as_usize();
                let word = ctx.stack.pop();
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let result = Bytes32::from_vec(vec![word.get_byte(index)]);
                ctx.stack.push(result);
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::SHL => {
                // STACK
                // let index = ctx.stack.pop().as_usize();
                let index = ctx.stack.pop();
                println!("index.as_usize(): {:#X}", index.as_usize());
                println!("index.to_u256(): {:#X}", index.to_u256());
                println!(
                    "index.to_u256.as_usize(): {:#X}",
                    index.to_u256().as_usize()
                );
                let word = ctx.stack.pop().to_u256();
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let result = word.shl(index.as_usize());
                ctx.stack.push_u256(result);
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::SHR => {
                // STACK
                // let index = ctx.stack.pop().as_usize();
                let index = ctx.stack.pop();
                println!("index.as_usize(): {:#X}", index.as_usize());
                println!("index.to_u256(): {:#X}", index.to_u256());
                println!(
                    "index.to_u256.as_usize(): {:#X}",
                    index.to_u256().as_usize()
                );
                let word = ctx.stack.pop().to_u256();
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let result = word.shr(index.as_usize());
                ctx.stack.push_u256(result);
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::SAR => {
                // STACK
                let index = ctx.stack.pop().as_usize();
                let word = ctx.stack.pop().to_u256();
                // GAS
                ctx.gas += self.fix_gas();
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
                ctx.stack.push_u256(result);
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::SHA3 => {
                // STACK
                let offset = ctx.stack.pop().as_usize();
                let size = ctx.stack.pop().as_usize();
                // GAS
                ctx.gas += self.fix_gas() + 6 * (size + 31) / 32;
                // // OPERATION
                let result = Bytes32::from_slice(
                    Keccak256::digest(ctx.memory.load(offset, size).as_slice()).as_slice(),
                );
                ctx.stack.push(result);
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::ADDRESS => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                ctx.stack.push_address(ctx.call.recipient());
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::BALANCE => {
                // STACK
                let address = ctx.stack.pop().to_address();
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                ctx.stack.push_u256(ctx.state.balance(&address));
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::ORIGIN => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                ctx.stack.push_address(ctx.call.originator());
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::CALLER => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                ctx.stack.push_address(ctx.call.sender());
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::CALLVALUE => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                ctx.stack.push_u256(ctx.call.value());
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::CALLDATALOAD => {
                // STACK
                let offset = ctx.stack.pop().as_usize();
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let mut result = [0u8; 32];
                let calldata = ctx.call.data();
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
                ctx.stack.push(Bytes32::from_slice(&result));
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::CALLDATASIZE => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let result = ctx.call.data_size();
                ctx.stack.push_usize(result);
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::CALLDATACOPY => {
                // STACK
                let memory_offset = ctx.stack.pop().as_usize();
                let offset = ctx.stack.pop().as_usize();
                let size = ctx.stack.pop().as_usize();
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let mut result = vec![0u8; size];
                let calldata = ctx.call.data();
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
                ctx.memory.store(memory_offset, Bytes::from_vec(result));
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::CODESIZE => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                ctx.stack.push_usize(ctx.code_size());
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::CODECOPY => {
                // STACK
                let memory_offset = ctx.stack.pop().as_usize();
                let offset = ctx.stack.pop().as_usize();
                let mut size = ctx.stack.pop().as_usize();
                if size > ctx.code_size() {
                    size = ctx.code_size()
                }
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let mut result = vec![0u8; size];
                let code = ctx.code();
                let (end, len) = if offset + size > ctx.code_size() {
                    (size, size - offset)
                } else {
                    (offset + size, size)
                };
                if len == size {
                    result.copy_from_slice(&code[offset..end]);
                } else {
                    result[..len].copy_from_slice(&code[offset..end]);
                }
                ctx.memory.store(memory_offset, Bytes::from_vec(result));
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::GASPRICE => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                ctx.stack.push_u256(ctx.call.gas_price());
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::EXTCODESIZE => {
                // STACK
                let address = ctx.stack.pop().to_address();
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                ctx.stack.push_usize(ctx.state.code_size(&address));
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::EXTCODECOPY => {
                // STACK
                let address = ctx.stack.pop().to_address();
                let memory_offset = ctx.stack.pop().as_usize();
                let offset = ctx.stack.pop().as_usize();
                let size = ctx.stack.pop().as_usize();
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let code = ctx.state.code(&address);
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
                ctx.memory.store(memory_offset, Bytes::from_vec(result));
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::RETURNDATASIZE => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let data = ctx.return_data();
                ctx.stack.push_usize(data.len());
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::RETURNDATACOPY => {
                // STACK
                let memory_offset = ctx.stack.pop().as_usize();
                let offset = ctx.stack.pop().as_usize();
                let mut size = ctx.stack.pop().as_usize();
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let data = ctx.return_data();
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
                ctx.memory.store(memory_offset, Bytes::from_vec(result));
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::EXTCODEHASH => {
                // STACK
                let address = ctx.stack.pop().to_address();
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                ctx.stack.push(ctx.state.code_hash(&address));
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::BLOCKHASH => {
                // STACK
                let _block_number = ctx.stack.pop();
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let result = Bytes32::zero();
                // let result = match ctx.block.block_hash(block_number) {
                //     Some(hash) => hash,
                //     None => U256::zero(),
                // };
                ctx.stack.push(result);
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::COINBASE => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let result = match ctx.block.beneficiary() {
                    Some(coinbase) => coinbase.to_u256(),
                    None => U256::zero(),
                };
                ctx.stack.push_u256(result);
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::TIMESTAMP => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                ctx.stack.push_u256(ctx.block.timestamp());
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::NUMBER => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let result = match ctx.block.number() {
                    Some(number) => Bytes32::from_u64(number).to_u256(),
                    None => U256::zero(),
                };
                ctx.stack.push_u256(result);
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::PREVRANDAO => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let result = match ctx.block.prev_randao() {
                    Some(number) => number,
                    // If block.prev_randao is None, use block.difficulty instead
                    None => match ctx.block.difficulty() {
                        Some(number) => number,
                        None => U256::zero(),
                    },
                };
                ctx.stack.push_u256(result);
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::GASLIMIT => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                ctx.stack.push_u256(ctx.block.gas_limit());
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::CHAINID => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let chain_id = Bytes32::from_u64(ctx.block.chain_id()).to_u256();
                ctx.stack.push_u256(chain_id);
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::SELFBALANCE => {
                // STACK
                let address = ctx.call.recipient();
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                ctx.stack.push_u256(ctx.state.balance(&address));
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::BASEFEE => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let base_fee = match ctx.block.base_fee() {
                    Some(base_fee) => base_fee,
                    None => U256::zero(),
                };
                ctx.stack.push_u256(base_fee);
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::POP => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                ctx.stack.pop();
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::MLOAD => {
                // STACK
                let offset = ctx.stack.pop();
                // GAS
                ctx.gas += self.fix_gas() * ctx.memory.expansion(offset.as_usize(), 32);
                // OPERATION
                let value = ctx.memory.load(offset.as_usize(), 32);
                ctx.stack.push(value.as_bytes32());
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::MSTORE => {
                // STACK
                let offset = ctx.stack.pop();
                let value = ctx.stack.pop();
                println!(" > MSTORE\n   - offset: {:#X}\n   -  value: {:#X}", offset, value);
                // GAS
                ctx.gas += self.fix_gas() * ctx.memory.expansion(offset.as_usize(), 32);
                // OPERATION
                ctx.memory
                    .store(offset.as_usize(), Bytes::from_bytes32(value));
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::MSTORE8 => {
                // STACK
                let offset = ctx.stack.pop();
                let value = ctx.stack.pop();
                // GAS
                ctx.gas += self.fix_gas() * ctx.memory.expansion(offset.as_usize(), 1);
                // OPERATION
                ctx.memory
                    .store(offset.as_usize(), Bytes::from_byte(value.get_byte(31)));
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::SLOAD => {
                // STACK
                let key = ctx.stack.pop().to_u256();
                // GAS
                ctx.gas += self.fix_gas(); //+ self.state_access_gas(key);
                // OPERATION
                let value = ctx.state.storage_load(&ctx.target, key);
                ctx.stack.push(value);
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::SSTORE => {
                // CHECK REVERT CONDITION
                if ctx.call.is_static() { return false; }
                // STACK
                let key = ctx.stack.pop().to_u256();
                let value = ctx.stack.pop();
                // GAS
                ctx.gas += self.fix_gas(); //+ self.state_access_gas(key);
                // OPERATION
                ctx.state.storage_store(&ctx.target, key, value);
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::JUMP => {
                // STACK
                let jumpdest = ctx.stack.pop().as_usize();
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                match validate_jumpdest(&ctx.code, jumpdest) {
                    true => {
                        // PC
                        ctx.pc = jumpdest;
                        // SUCCESS
                        true
                    }
                    false => false,
                }
            },
            Opcode::JUMPI => {
                // STACK
                let jumpdest = ctx.stack.pop().as_usize();
                let condition = ctx.stack.pop().to_u256();
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                match condition.is_zero() {
                    true => {
                        // PC
                        ctx.pc += 1;
                        // SUCCESS
                        true
                    }
                    false => {
                        match validate_jumpdest(&ctx.code, jumpdest) {
                            true => {
                                // PC
                                ctx.pc = jumpdest;
                                // SUCCESS
                                true
                            }
                            false => false,
                        }
                    }
                }
            },
            Opcode::PC => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                ctx.stack.push_usize(ctx.pc);
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::MSIZE => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                ctx.stack.push_usize(ctx.memory.size());
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::GAS => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                // TODO
                // ctx.stack.push(U256::from(ctx.gas));
                ctx.stack.push_u256(U256::max_value());
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::JUMPDEST => {
                // GAS
                ctx.gas += self.fix_gas();
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::PUSH1 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let value = &[ctx.code[ctx.pc + 1]];
                ctx.stack.push(Bytes32::from_slice(value));
                // PC
                ctx.pc += 2;
                // SUCCESS
                true
            },
            Opcode::PUSH2 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 3];
                ctx.stack.push(Bytes32::from_slice(value));
                // PC
                ctx.pc += 3;
                // SUCCESS
                true
            },
            Opcode::PUSH3 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 4];
                ctx.stack.push(Bytes32::from_slice(value));
                // PC
                ctx.pc += 4;
                // SUCCESS
                true
            },
            Opcode::PUSH4 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 5];
                ctx.stack.push(Bytes32::from_slice(value));
                // PC
                ctx.pc += 5;
                // SUCCESS
                true
            },
            Opcode::PUSH5 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 6];
                ctx.stack.push(Bytes32::from_slice(value));
                // PC
                ctx.pc += 6;
                // SUCCESS
                true
            },
            Opcode::PUSH6 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 7];
                ctx.stack.push(Bytes32::from_slice(value));
                // PC
                ctx.pc += 7;
                // SUCCESS
                true
            },
            Opcode::PUSH7 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 8];
                ctx.stack.push(Bytes32::from_slice(value));
                // PC
                ctx.pc += 8;
                // SUCCESS
                true
            },
            Opcode::PUSH8 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 9];
                ctx.stack.push(Bytes32::from_slice(value));
                // PC
                ctx.pc += 9;
                // SUCCESS
                true
            },
            Opcode::PUSH9 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 10];
                ctx.stack.push(Bytes32::from_slice(value));
                // PC
                ctx.pc += 10;
                // SUCCESS
                true
            },
            Opcode::PUSH10 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 11];
                ctx.stack.push(Bytes32::from_slice(value));
                // PC
                ctx.pc += 11;
                // SUCCESS
                true
            },
            Opcode::PUSH11 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 12];
                ctx.stack.push(Bytes32::from_slice(value));
                // PC
                ctx.pc += 12;
                // SUCCESS
                true
            },
            Opcode::PUSH12 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 13];
                ctx.stack.push(Bytes32::from_slice(value));
                // PC
                ctx.pc += 13;
                // SUCCESS
                true
            },
            Opcode::PUSH13 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 14];
                ctx.stack.push(Bytes32::from_slice(value));
                // PC
                ctx.pc += 14;
                // SUCCESS
                true
            },
            Opcode::PUSH14 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 15];
                ctx.stack.push(Bytes32::from_slice(value));
                // PC
                ctx.pc += 15;
                // SUCCESS
                true
            },
            Opcode::PUSH15 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 16];
                ctx.stack.push(Bytes32::from_slice(value));
                // PC
                ctx.pc += 16;
                // SUCCESS
                true
            },
            Opcode::PUSH16 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 17];
                ctx.stack.push(Bytes32::from_slice(value));
                // PC
                ctx.pc += 17;
                // SUCCESS
                true
            },
            Opcode::PUSH17 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 18];
                ctx.stack.push(Bytes32::from_slice(value));
                // PC
                ctx.pc += 18;
                // SUCCESS
                true
            },
            Opcode::PUSH18 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 19];
                ctx.stack.push(Bytes32::from_slice(value));
                // PC
                ctx.pc += 19;
                // SUCCESS
                true
            },
            Opcode::PUSH19 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 20];
                ctx.stack.push(Bytes32::from_slice(value));
                // PC
                ctx.pc += 20;
                // SUCCESS
                true
            },
            Opcode::PUSH20 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 21];
                ctx.stack.push(Bytes32::from_slice(value));
                // PC
                ctx.pc += 21;
                // SUCCESS
                true
            },
            Opcode::PUSH21 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 22];
                ctx.stack.push(Bytes32::from_slice(value));
                // PC
                ctx.pc += 22;
                // SUCCESS
                true
            },
            Opcode::PUSH22 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 23];
                ctx.stack.push(Bytes32::from_slice(value));
                // PC
                ctx.pc += 23;
                // SUCCESS
                true
            },
            Opcode::PUSH23 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 24];
                ctx.stack.push(Bytes32::from_slice(value));
                // PC
                ctx.pc += 24;
                // SUCCESS
                true
            },
            Opcode::PUSH24 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 25];
                ctx.stack.push(Bytes32::from_slice(value));
                // PC
                ctx.pc += 25;
                // SUCCESS
                true
            },
            Opcode::PUSH25 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 26];
                ctx.stack.push(Bytes32::from_slice(value));
                // PC
                ctx.pc += 26;
                // SUCCESS
                true
            },
            Opcode::PUSH26 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 27];
                ctx.stack.push(Bytes32::from_slice(value));
                // PC
                ctx.pc += 27;
                // SUCCESS
                true
            },
            Opcode::PUSH27 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 28];
                ctx.stack.push(Bytes32::from_slice(value));
                // PC
                ctx.pc += 28;
                // SUCCESS
                true
            },
            Opcode::PUSH28 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 29];
                ctx.stack.push(Bytes32::from_slice(value));
                // PC
                ctx.pc += 29;
                // SUCCESS
                true
            },
            Opcode::PUSH29 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 30];
                ctx.stack.push(Bytes32::from_slice(value));
                // PC
                ctx.pc += 30;
                // SUCCESS
                true
            },
            Opcode::PUSH30 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 31];
                ctx.stack.push(Bytes32::from_slice(value));
                // PC
                ctx.pc += 31;
                // SUCCESS
                true
            },
            Opcode::PUSH31 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 32];
                ctx.stack.push(Bytes32::from_slice(value));
                // PC
                ctx.pc += 32;
                // SUCCESS
                true
            },
            Opcode::PUSH32 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 33];
                ctx.stack.push(Bytes32::from_slice(value));
                // PC
                ctx.pc += 33;
                // SUCCESS
                true
            },
            Opcode::DUP1 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let value = ctx.stack.get_item(ctx.stack.depth() - 1);
                match value {
                    Some(value) => ctx.stack.push(value),
                    None => panic!("Stack underflow"),
                };
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::DUP2 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let value = ctx.stack.get_item(ctx.stack.depth() - 2);
                match value {
                    Some(value) => ctx.stack.push(value),
                    None => panic!("Stack underflow"),
                };
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::DUP3 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let value = ctx.stack.get_item(ctx.stack.depth() - 3);
                match value {
                    Some(value) => ctx.stack.push(value),
                    None => panic!("Stack underflow"),
                };
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::DUP4 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let value = ctx.stack.get_item(ctx.stack.depth() - 4);
                match value {
                    Some(value) => ctx.stack.push(value),
                    None => panic!("Stack underflow"),
                };
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::DUP5 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let value = ctx.stack.get_item(ctx.stack.depth() - 5);
                match value {
                    Some(value) => ctx.stack.push(value),
                    None => panic!("Stack underflow"),
                };
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::DUP6 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let value = ctx.stack.get_item(ctx.stack.depth() - 6);
                match value {
                    Some(value) => ctx.stack.push(value),
                    None => panic!("Stack underflow"),
                };
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::DUP7 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let value = ctx.stack.get_item(ctx.stack.depth() - 7);
                match value {
                    Some(value) => ctx.stack.push(value),
                    None => panic!("Stack underflow"),
                };
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::DUP8 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let value = ctx.stack.get_item(ctx.stack.depth() - 8);
                match value {
                    Some(value) => ctx.stack.push(value),
                    None => panic!("Stack underflow"),
                };
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::DUP9 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let value = ctx.stack.get_item(ctx.stack.depth() - 9);
                match value {
                    Some(value) => ctx.stack.push(value),
                    None => panic!("Stack underflow"),
                };
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::DUP10 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let value = ctx.stack.get_item(ctx.stack.depth() - 10);
                match value {
                    Some(value) => ctx.stack.push(value),
                    None => panic!("Stack underflow"),
                };
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::DUP11 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let value = ctx.stack.get_item(ctx.stack.depth() - 11);
                match value {
                    Some(value) => ctx.stack.push(value),
                    None => panic!("Stack underflow"),
                };
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::DUP12 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let value = ctx.stack.get_item(ctx.stack.depth() - 12);
                match value {
                    Some(value) => ctx.stack.push(value),
                    None => panic!("Stack underflow"),
                };
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::DUP13 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let value = ctx.stack.get_item(ctx.stack.depth() - 13);
                match value {
                    Some(value) => ctx.stack.push(value),
                    None => panic!("Stack underflow"),
                };
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::DUP14 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let value = ctx.stack.get_item(ctx.stack.depth() - 14);
                match value {
                    Some(value) => ctx.stack.push(value),
                    None => panic!("Stack underflow"),
                };
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::DUP15 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let value = ctx.stack.get_item(ctx.stack.depth() - 15);
                match value {
                    Some(value) => ctx.stack.push(value),
                    None => panic!("Stack underflow"),
                };
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::DUP16 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let value = ctx.stack.get_item(ctx.stack.depth() - 16);
                match value {
                    Some(value) => ctx.stack.push(value),
                    None => panic!("Stack underflow"),
                };
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::SWAP1 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                ctx.stack.swap(1);
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::SWAP2 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                ctx.stack.swap(2);
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::SWAP3 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                ctx.stack.swap(3);
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::SWAP4 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                ctx.stack.swap(4);
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::SWAP5 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                ctx.stack.swap(5);
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::SWAP6 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                ctx.stack.swap(6);
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::SWAP7 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                ctx.stack.swap(7);
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::SWAP8 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                ctx.stack.swap(8);
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::SWAP9 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                ctx.stack.swap(9);
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::SWAP10 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                ctx.stack.swap(10);
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::SWAP11 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                ctx.stack.swap(11);
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::SWAP12 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                ctx.stack.swap(12);
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::SWAP13 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                ctx.stack.swap(13);
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::SWAP14 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                ctx.stack.swap(14);
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::SWAP15 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                ctx.stack.swap(15);
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::SWAP16 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                ctx.stack.swap(16);
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::LOG0 => {
                // STACK
                let offset = ctx.stack.pop().as_usize();
                let size = ctx.stack.pop().as_usize();
                // GAS
                ctx.gas += self.fix_gas();// + self.log_gas(offset, size);
                // OPERATION
                let data = ctx.memory.load(offset, size);
                let log = Log::new(ctx.target, data);
                ctx.add_log(log);
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::LOG1 => {
                // STACK
                let offset = ctx.stack.pop().as_usize();
                let size = ctx.stack.pop().as_usize();
                let topic1 = ctx.stack.pop();
                // GAS
                ctx.gas += self.fix_gas();// + self.log_gas(offset, size);
                // OPERATION
                let data = ctx.memory.load(offset, size);
                let mut log = Log::new(ctx.target, data);
                log.add_topic(topic1);
                ctx.add_log(log);
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::LOG2 => {
                // STACK
                let offset = ctx.stack.pop().as_usize();
                let size = ctx.stack.pop().as_usize();
                let topic1 = ctx.stack.pop();
                let topic2 = ctx.stack.pop();
                // GAS
                ctx.gas += self.fix_gas();// + self.log_gas(offset, size);
                // OPERATION
                let data = ctx.memory.load(offset, size);
                let mut log = Log::new(ctx.target, data);
                log.add_topics(vec![topic1, topic2]);
                ctx.add_log(log);
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::LOG3 => {
                // STACK
                let offset = ctx.stack.pop().as_usize();
                let size = ctx.stack.pop().as_usize();
                let topic1 = ctx.stack.pop();
                let topic2 = ctx.stack.pop();
                let topic3 = ctx.stack.pop();
                // GAS
                ctx.gas += self.fix_gas();// + self.log_gas(offset, size);
                // OPERATION
                let data = ctx.memory.load(offset, size);
                let mut log = Log::new(ctx.target, data);
                log.add_topics(vec![topic1, topic2, topic3]);
                ctx.add_log(log);
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::LOG4 => {
                // STACK
                let offset = ctx.stack.pop().as_usize();
                let size = ctx.stack.pop().as_usize();
                let topic1 = ctx.stack.pop();
                let topic2 = ctx.stack.pop();
                let topic3 = ctx.stack.pop();
                let topic4 = ctx.stack.pop();
                // GAS
                ctx.gas += self.fix_gas();// + self.log_gas(offset, size);
                // OPERATION
                let data = ctx.memory.load(offset, size);
                let mut log = Log::new(ctx.target, data);
                log.add_topics(vec![topic1, topic2, topic3, topic4]);
                ctx.add_log(log);
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::CREATE => {
                // STACK
                let value = ctx.stack.pop().to_u256();
                let offset = ctx.stack.pop().as_usize();
                let size = ctx.stack.pop().as_usize();
                // CHECK REVERT CONDITION
                if ctx.call.is_static() & !value.is_zero() {
                    return false;
                }
                if !ctx.call.is_static() & (ctx.state.balance(&ctx.call.originator()) < value) {
                    return false;
                }
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let data = ctx.memory.load(offset, size);
                let mut rlp_encoded1 = rlp_encode(ctx.target.as_slice());
                let mut rlp_encoded2 = rlp_encode(Bytes32::from_u256(ctx.state.nonce(&ctx.target)).as_slice());
                rlp_encoded1.append(&mut rlp_encoded2);
                let address = Address::from_slice(Keccak256::digest(rlp_encoded1).as_slice());
                let call_result = ctx.create_call(address, value, data);
                if !call_result.success.is_zero() {
                    ctx.stack.push_address(address);
                } else {
                    ctx.stack.push(Bytes32::zero());
                }
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            }
            Opcode::CALL => {
                // STACK
                let gas = ctx.stack.pop().to_u256();
                let address = ctx.stack.pop().to_address();
                let value = ctx.stack.pop().to_u256();
                let args_offset = ctx.stack.pop().as_usize();
                let args_size = ctx.stack.pop().as_usize();
                let ret_offset = ctx.stack.pop().as_usize();
                let ret_size = ctx.stack.pop().as_usize();
                // CHECK REVERT CONDITION
                if ctx.call.is_static() & !value.is_zero() {
                    return false;
                }
                if !ctx.call.is_static() & (ctx.state.balance(&ctx.call.originator()) < value) {
                    return false;
                }
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let data = ctx.memory.load(args_offset, args_size);
                let call = Call::new(
                    ctx.target,
                    address,
                    ctx.call.originator(),
                    gas,
                    U256::from(ctx.gas_left()),
                    address,
                    data,
                    value,
                    false
                );
                let call_result = ctx.execute_call(call);
                let mut data = vec![0u8; ret_size];
                data.copy_from_slice(&call_result.result[0..ret_size]);
                ctx.memory.store(ret_offset, Bytes::from_vec(data));
                ctx.stack.push(call_result.success);
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::CALLCODE => {
                // STACK
                let gas = ctx.stack.pop().to_u256();
                let address = ctx.stack.pop().to_address();
                let value = ctx.stack.pop().to_u256();
                let args_offset = ctx.stack.pop().as_usize();
                let args_size = ctx.stack.pop().as_usize();
                let ret_offset = ctx.stack.pop().as_usize();
                let ret_size = ctx.stack.pop().as_usize();
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let data = ctx.memory.load(args_offset, args_size);
                let call = Call::new(
                    ctx.target,
                    address,
                    ctx.call.originator(),
                    gas,
                    U256::from(ctx.gas_left()),
                    address,
                    data,
                    value,
                    false
                );
                let call_result = ctx.execute_call(call);
                let mut data = vec![0u8; ret_size];
                data.copy_from_slice(&call_result.result[0..ret_size]);
                ctx.memory.store(ret_offset, Bytes::from_vec(data));
                ctx.stack.push(call_result.success);
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::RETURN => {
                // STACK
                let offset = ctx.stack.pop().as_usize();
                let size = ctx.stack.pop().as_usize();
                // GAS
                ctx.gas += self.fix_gas() * ctx.memory.expansion(offset, size);
                // OPERATION
                let value = ctx.memory.load(offset, size);
                ctx.call.set_result(value.clone());
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::DELEGATECALL => {
                // STACK
                let gas = ctx.stack.pop().to_u256();
                let address = ctx.stack.pop().to_address();
                let args_offset = ctx.stack.pop().as_usize();
                let args_size = ctx.stack.pop().as_usize();
                let ret_offset = ctx.stack.pop().as_usize();
                let ret_size = ctx.stack.pop().as_usize();
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let data = ctx.memory.load(args_offset, args_size);
                let call = Call::new(
                    ctx.target,
                    ctx.target,
                    ctx.call.originator(),
                    gas,
                    U256::from(ctx.gas_left()),
                    address,
                    data,
                    U256::zero(),
                    false
                );
                let call_result = ctx.execute_call(call);
                if !call_result.success.is_zero() {
                    let mut data = vec![0u8; ret_size];
                    data.copy_from_slice(&call_result.result[0..ret_size]);
                    ctx.memory.store(ret_offset, Bytes::from_vec(data));
                }
                ctx.stack.push(call_result.success);
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::CREATE2 => {
                todo!()
            },
            Opcode::STATICCALL => {
                // STACK
                let gas = ctx.stack.pop().to_u256();
                let address = ctx.stack.pop().to_address();
                let args_offset = ctx.stack.pop().as_usize();
                let args_size = ctx.stack.pop().as_usize();
                let ret_offset = ctx.stack.pop().as_usize();
                let ret_size = ctx.stack.pop().as_usize();
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let data = ctx.memory.load(args_offset, args_size);
                let call = Call::new(
                    ctx.target,
                    address,
                    ctx.call.originator(),
                    gas,
                    U256::from(ctx.gas_left()),
                    address,
                    data,
                    U256::zero(),
                    true
                );
                let call_result = ctx.execute_call(call);
                if !call_result.success.is_zero() {
                    let mut data = vec![0u8; ret_size];
                    data.copy_from_slice(&call_result.result[0..ret_size]);
                    ctx.memory.store(ret_offset, Bytes::from_vec(data));
                }
                ctx.stack.push(call_result.success);
                // PC
                ctx.pc += 1;
                // SUCCESS
                true
            },
            Opcode::REVERT => {
                // STACK
                let offset = ctx.stack.pop().as_usize();
                let size = ctx.stack.pop().as_usize();
                // GAS
                ctx.gas += self.fix_gas() * ctx.memory.expansion(offset, size);
                // OPERATION
                let value = ctx.memory.load(offset, size);
                ctx.call.set_result(value);
                // PC
                ctx.pc += 1;
                // SUCCESS
                false
            },
            Opcode::INVALID => {
                // PC
                ctx.pc += 1;
                // SUCCESS
                false
            },
            Opcode::SELFDESTRUCT => {
                // STACK
                let address = ctx.stack.pop().to_address();
                // CHECK REVERT CONDITION
                if ctx.call.is_static() {
                    return false;
                }
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                match ctx.state.transfer(&ctx.target, &address, ctx.state.balance(&ctx.target)) {
                    Ok(_) => {
                        ctx.selfdestruct();
                        // PC
                        ctx.pc += 1;
                        // SUCCESS
                        true
                    }
                    Err(_) => return false,
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
