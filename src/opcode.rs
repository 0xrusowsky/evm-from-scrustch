use std::convert::TryFrom;
use ethereum_types::{U256, U512};
use std::ops::{BitAnd, BitOr, BitXor, Not, Shl, Shr};

use super::ExecutionContext;

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
    POP,
    PC,
    GAS,
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
    INVALID,
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
            0x50 => Ok(Opcode::POP),
            0x58 => Ok(Opcode::PC),
            0x5A => Ok(Opcode::GAS),
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
            0xFE => Ok(Opcode::INVALID),
            // ... other opcodes
            _ => Err(format!("Invalid opcode: {}", value)),
        }
    }
}

impl Opcode {
    pub fn execute(&self, ctx: &mut ExecutionContext) -> (usize, bool) {
        match self {
            Opcode::STOP => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                ctx.stopped = true;
                // PC + SUCCESS
                (1, true)
            },
            Opcode::ADD => {
                // STACK
                let a = ctx.stack.pop();
                let b = ctx.stack.pop();
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                // rely on U256 overflowing_add to handle overflow
                let (result, _) = a.overflowing_add(b);
                ctx.stack.push(result);
                // PC + SUCCESS
                (1, true)
            },
            Opcode::MUL => {
                // STACK
                let a = ctx.stack.pop();
                let b = ctx.stack.pop();
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                // rely on U256 overflowing_mul to handle overflow
                let (result, _) = a.overflowing_mul(b);
                ctx.stack.push(result);
                // PC + SUCCESS
                (1, true)
            },
            Opcode::SUB => {
                // STACK
                let a = ctx.stack.pop();
                let b = ctx.stack.pop();
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                // rely on U256 overflowing_sub to handle underflow
                let (result, _) = a.overflowing_sub(b);
                ctx.stack.push(result);
                // PC + SUCCESS
                (1, true)
            },
            Opcode::DIV => {
                // STACK
                let a = ctx.stack.pop();
                let b = ctx.stack.pop();
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let result = if b == U256::zero() {
                    U256::zero()
                } else {
                    a / b
                };
                ctx.stack.push(result);
                // PC + SUCCESS
                (1, true)
            },
            Opcode::SDIV => {
                // STACK
                let a = ctx.stack.pop();
                let b = ctx.stack.pop();
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let result = if b == U256::zero() {
                    U256::zero()
                } else {
                    let ( a_neg, a_twos ) = (a.bit(255), a.not().overflowing_add(U256::one()).0);
                    let ( b_neg, b_twos ) = (b.bit(255), b.not().overflowing_add(U256::one()).0);
                    let div = if a_neg { a_twos } else { a } / if b_neg { b_twos } else { b };
                    if a_neg ^ b_neg { div.not().overflowing_add(U256::one()).0 } else { div }
                };
                ctx.stack.push(result);
                // PC + SUCCESS
                (1, true)
            },
            Opcode::MOD => {
                // STACK
                let a = ctx.stack.pop();
                let b = ctx.stack.pop();
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let result = if b == U256::zero() {
                    U256::zero()
                } else {
                    a % b
                };
                ctx.stack.push(result);
                // PC + SUCCESS
                (1, true)
            },
            Opcode::SMOD => {
                // STACK
                let a = ctx.stack.pop();
                let b = ctx.stack.pop();
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let result = if b == U256::zero() {
                    U256::zero()
                } else {
                    let ( a_neg, a_twos ) = (a.bit(255), a.not().overflowing_add(U256::one()).0);
                    let ( b_neg, b_twos ) = (b.bit(255), b.not().overflowing_add(U256::one()).0);
                    let div = if a_neg { a_twos } else { a } % if b_neg { b_twos } else { b };
                    if a_neg | b_neg { div.not().overflowing_add(U256::one()).0 } else { div }
                };
                ctx.stack.push(result);
                // PC + SUCCESS
                (1, true)
            },
            Opcode::ADDMOD => {
                // STACK
                let a: U512 = ctx.stack.pop().try_into().unwrap();
                let b: U512 = ctx.stack.pop().try_into().unwrap();
                let c: U256 = ctx.stack.pop();
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let result = if c == U256::zero() {
                    U256::zero()
                } else {
                    ((a + b) % c).try_into().unwrap()
                };
                ctx.stack.push(result);
                // PC + SUCCESS
                (1, true)
            },
            Opcode::MULMOD => {
                // STACK
                let a: U512 = ctx.stack.pop().try_into().unwrap();
                let b: U512 = ctx.stack.pop().try_into().unwrap();
                let c: U256 = ctx.stack.pop();
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let result = if c == U256::zero() {
                    U256::zero()
                } else {
                    ((a * b) % c).try_into().unwrap()
                };
                ctx.stack.push(result);
                // PC + SUCCESS
                (1, true)
            },
            Opcode::EXP => {
                // STACK
                let a = ctx.stack.pop();
                let b = ctx.stack.pop();
                // GAS
                let var_gas = if b != U256::zero() { 50 * (b.bits() + 7) } else { 0 };
                ctx.gas += self.fix_gas() + var_gas;
                // OPERATION
                let (result, _) = a.overflowing_pow(b);
                ctx.stack.push(result);
                // PC + SUCCESS
                (1, true)
            },
            Opcode::SIGNEXTEND => {
                // STACK
                let a = ctx.stack.pop();
                let b = ctx.stack.pop();
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let id_b = (a.as_usize() + 1) * 8;
                let result = if b.bit(id_b - 1) {
                    U256::MAX.shl(id_b).bitor(b)
                } else { b };
                ctx.stack.push(result);
                // PC + SUCCESS
                (1, true)
            },
            Opcode::LT => {
                // STACK
                let a = ctx.stack.pop();
                let b = ctx.stack.pop();
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let result = if a < b { U256::one() } else { U256::zero() };
                ctx.stack.push(result);
                // PC + SUCCESS
                (1, true)
            },
            Opcode::GT => {
                // STACK
                let a = ctx.stack.pop();
                let b = ctx.stack.pop();
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let result = if a > b { U256::one() } else { U256::zero() };
                ctx.stack.push(result);
                // PC + SUCCESS
                (1, true)
            },
            Opcode::SLT => {
                // STACK
                let a = ctx.stack.pop();
                let b = ctx.stack.pop();
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let (a_twos, _) = a.not().overflowing_add(U256::one());
                let (b_twos, _) = b.not().overflowing_add(U256::one());
                let result = if a_twos > b_twos { U256::one() } else { U256::zero() };
                ctx.stack.push(result);
                // PC + SUCCESS
                (1, true)
            },
            Opcode::SGT => {
                // STACK
                let a = ctx.stack.pop();
                let b = ctx.stack.pop();
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let (a_twos, _) = a.not().overflowing_add(U256::one());
                let (b_twos, _) = b.not().overflowing_add(U256::one());
                let result = if a_twos < b_twos { U256::one() } else { U256::zero() };
                ctx.stack.push(result);
                // PC + SUCCESS
                (1, true)
            },
            Opcode::EQ => {
                // STACK
                let a = ctx.stack.pop();
                let b = ctx.stack.pop();
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let result = if a == b { U256::one() } else { U256::zero() };
                ctx.stack.push(result);
                // PC + SUCCESS
                (1, true)
            },
            Opcode::ISZERO => {
                // STACK
                let a = ctx.stack.pop();
                let result = if a == U256::zero() { U256::one() } else { U256::zero() };
                ctx.stack.push(result);
                // PC + SUCCESS
                (1, true)
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
                // PC + SUCCESS
                (1, true)
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
                // PC + SUCCESS
                (1, true)
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
                // PC + SUCCESS
                (1, true)
            },
            Opcode::NOT => {
                // STACK
                let a = ctx.stack.pop();
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let result = a.not();
                ctx.stack.push(result);
                // PC + SUCCESS
                (1, true)
            },
            Opcode::BYTE => {
                // STACK
                let a = ctx.stack.pop();
                let b = ctx.stack.pop();
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let a = a.as_usize();
                let result = U256::from(if a < 32 {b.byte(31 - a)} else { 0 });
                ctx.stack.push(result);
                // PC + SUCCESS
                (1, true)
            },
            Opcode::SHL => {
                // STACK
                let a = ctx.stack.pop();
                let b = ctx.stack.pop();
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let result = b.shl(a.as_usize());
                ctx.stack.push(result);
                // PC + SUCCESS
                (1, true)
            },
            Opcode::SHR => {
                // STACK
                let a = ctx.stack.pop();
                let b = ctx.stack.pop();
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let result = b.shr(a.as_usize());
                ctx.stack.push(result);
                // PC + SUCCESS
                (1, true)
            },
            Opcode::SAR => {
                // STACK
                let a = ctx.stack.pop();
                let b = ctx.stack.pop();
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let result = if b.bit(255) { 
                    if a.gt( &U256::from(255) ) {
                        U256::MAX
                    } else {
                        b.shr(a.as_usize()).bitor(U256::MAX.shl(U256::from(255) - a))
                    }
                } else {
                    b.shr(a.as_usize())
                };
                ctx.stack.push(result);
                // PC + SUCCESS
                (1, true)
            },
            // TODO
            Opcode::POP => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                ctx.stack.pop();
                // PC + SUCCESS
                (1, true)
            },
            Opcode::PC => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                ctx.stack.push(U256::from(ctx.pc));
                // PC + SUCCESS
                (1, true)
            },
            Opcode::GAS => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                // ctx.stack.push(U256::from(ctx.gas));
                ctx.stack.push(U256::max_value());
                // PC + SUCCESS
                (1, true)
            },
            Opcode::PUSH1 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let value = ctx.code[ctx.pc + 1];
                ctx.stack.push(value.into());
                // PC + SUCCESS
                (2, true)
            },
            Opcode::PUSH2 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 3];
                ctx.stack.push(value.into());
                // PC + SUCCESS
                (3, true)
            },
            Opcode::PUSH3 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 4];
                ctx.stack.push(value.into());
                // PC + SUCCESS
                (4, true)
            },
            Opcode::PUSH4 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 5];
                ctx.stack.push(value.into());
                // PC + SUCCESS
                (5, true)
            },
            Opcode::PUSH5 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 6];
                ctx.stack.push(value.into());
                // PC + SUCCESS
                (6, true)
            },
            Opcode::PUSH6 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 7];
                ctx.stack.push(value.into());
                // PC + SUCCESS
                (7, true)
            },
            Opcode::PUSH7 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 8];
                ctx.stack.push(value.into());
                // PC + SUCCESS
                (8, true)
            },
            Opcode::PUSH8 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 9];
                ctx.stack.push(value.into());
                // PC + SUCCESS
                (9, true)
            },
            Opcode::PUSH9 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 10];
                ctx.stack.push(value.into());
                // PC + SUCCESS
                (10, true)
            },
            Opcode::PUSH10 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 11];
                ctx.stack.push(value.into());
                // PC + SUCCESS
                (11, true)
            },
            Opcode::PUSH11 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 12];
                ctx.stack.push(value.into());
                // PC + SUCCESS
                (12, true)
            },
            Opcode::PUSH12 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 13];
                ctx.stack.push(value.into());
                // PC + SUCCESS
                (13, true)
            },
            Opcode::PUSH13 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 14];
                ctx.stack.push(value.into());
                // PC + SUCCESS
                (14, true)
            },
            Opcode::PUSH14 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 15];
                ctx.stack.push(value.into());
                // PC + SUCCESS
                (15, true)
            },
            Opcode::PUSH15 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 16];
                ctx.stack.push(value.into());
                // PC + SUCCESS
                (16, true)
            },
            Opcode::PUSH16 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 17];
                ctx.stack.push(value.into());
                // PC + SUCCESS
                (17, true)
            },
            Opcode::PUSH17 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 18];
                ctx.stack.push(value.into());
                // PC + SUCCESS
                (18, true)
            },
            Opcode::PUSH18 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 19];
                ctx.stack.push(value.into());
                // PC + SUCCESS
                (19, true)
            },
            Opcode::PUSH19 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 20];
                ctx.stack.push(value.into());
                // PC + SUCCESS
                (20, true)
            },
            Opcode::PUSH20 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 21];
                ctx.stack.push(value.into());
                // PC + SUCCESS
                (21, true)
            },
            Opcode::PUSH21 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 22];
                ctx.stack.push(value.into());
                // PC + SUCCESS
                (22, true)
            },
            Opcode::PUSH22 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 23];
                ctx.stack.push(value.into());
                // PC + SUCCESS
                (23, true)
            },
            Opcode::PUSH23 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 24];
                ctx.stack.push(value.into());
                // PC + SUCCESS
                (24, true)
            },
            Opcode::PUSH24 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 25];
                ctx.stack.push(value.into());
                // PC + SUCCESS
                (25, true)
            },
            Opcode::PUSH25 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 26];
                ctx.stack.push(value.into());
                // PC + SUCCESS
                (26, true)
            },
            Opcode::PUSH26 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 27];
                ctx.stack.push(value.into());
                // PC + SUCCESS
                (27, true)
            },
            Opcode::PUSH27 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 28];
                ctx.stack.push(value.into());
                // PC + SUCCESS
                (28, true)
            },
            Opcode::PUSH28 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 29];
                ctx.stack.push(value.into());
                // PC + SUCCESS
                (29, true)
            },
            Opcode::PUSH29 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 30];
                ctx.stack.push(value.into());
                // PC + SUCCESS
                (30, true)
            },
            Opcode::PUSH30 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 31];
                ctx.stack.push(value.into());
                // PC + SUCCESS
                (31, true)
            },
            Opcode::PUSH31 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 32];
                ctx.stack.push(value.into());
                // PC + SUCCESS
                (32, true)
            },
            Opcode::PUSH32 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 33];
                ctx.stack.push(value.into());
                // PC + SUCCESS
                (33, true)
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
                // PC + SUCCESS
                (1, true)
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
                // PC + SUCCESS
                (1, true)
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
                // PC + SUCCESS
                (1, true)
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
                // PC + SUCCESS
                (1, true)
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
                // PC + SUCCESS
                (1, true)
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
                // PC + SUCCESS
                (1, true)
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
                // PC + SUCCESS
                (1, true)
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
                // PC + SUCCESS
                (1, true)
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
                // PC + SUCCESS
                (1, true)
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
                // PC + SUCCESS
                (1, true)
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
                // PC + SUCCESS
                (1, true)
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
                // PC + SUCCESS
                (1, true)
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
                // PC + SUCCESS
                (1, true)
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
                // PC + SUCCESS
                (1, true)
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
                // PC + SUCCESS
                (1, true)
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
                // PC + SUCCESS
                (1, true)
            },
            Opcode::SWAP1 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                ctx.stack.swap(1);
                // PC + SUCCESS
                (1, true)
            },
            Opcode::SWAP2 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                ctx.stack.swap(2);
                // PC + SUCCESS
                (1, true)
            },
            Opcode::SWAP3 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                ctx.stack.swap(3);
                // PC + SUCCESS
                (1, true)
            },
            Opcode::SWAP4 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                ctx.stack.swap(4);
                // PC + SUCCESS
                (1, true)
            },
            Opcode::SWAP5 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                ctx.stack.swap(5);
                // PC + SUCCESS
                (1, true)
            },
            Opcode::SWAP6 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                ctx.stack.swap(6);
                // PC + SUCCESS
                (1, true)
            },
            Opcode::SWAP7 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                ctx.stack.swap(7);
                // PC + SUCCESS
                (1, true)
            },
            Opcode::SWAP8 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                ctx.stack.swap(8);
                // PC + SUCCESS
                (1, true)
            },
            Opcode::SWAP9 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                ctx.stack.swap(9);
                // PC + SUCCESS
                (1, true)
            },
            Opcode::SWAP10 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                ctx.stack.swap(10);
                // PC + SUCCESS
                (1, true)
            },
            Opcode::SWAP11 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                ctx.stack.swap(11);
                // PC + SUCCESS
                (1, true)
            },
            Opcode::SWAP12 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                ctx.stack.swap(12);
                // PC + SUCCESS
                (1, true)
            },
            Opcode::SWAP13 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                ctx.stack.swap(13);
                // PC + SUCCESS
                (1, true)
            },
            Opcode::SWAP14 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                ctx.stack.swap(14);
                // PC + SUCCESS
                (1, true)
            },
            Opcode::SWAP15 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                ctx.stack.swap(15);
                // PC + SUCCESS
                (1, true)
            },
            Opcode::SWAP16 => {
                // GAS
                ctx.gas += self.fix_gas();
                // OPERATION
                ctx.stack.swap(16);
                // PC + SUCCESS
                (1, true)
            },
            Opcode::INVALID => {
                // PC + SUCCESS
                (1, false)
            },
        }
    }
}


impl Opcode {
    pub fn fix_gas(&self) -> usize {
        match self {
            // Gas: Zero
            Opcode::STOP => 0,
            Opcode::INVALID => 0,
            // Gas: Base
            Opcode::POP => 2,
            Opcode::PC => 2,
            Opcode::GAS => 2,
            // Gas: Verylow
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
            // Gas: High
            // Gas: Copy
            // Gas: Call
            // Gas: Extaccount
            Opcode::EXP => 10,
        }
    }
}