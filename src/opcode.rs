use std::convert::TryFrom;
use ethereum_types::{U256, U512};
use std::ops::{Add, BitAnd, BitOr, BitXor, Not, Shl, Shr};

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
            // ... other opcodes
            _ => Err(format!("Invalid opcode: {}", value)),
        }
    }
}

impl Opcode {
    pub fn execute(&self, ctx: &mut ExecutionContext) -> usize {
        match self {
            Opcode::STOP => {
                ctx.stopped = true;
                1
            },
            Opcode::ADD => {
                let a = ctx.stack.pop();
                let b = ctx.stack.pop();
                // rely on U256 overflowing_add to handle overflow
                let (result, _) = a.overflowing_add(b);
                ctx.stack.push(result);
                1
            },
            Opcode::MUL => {
                let a = ctx.stack.pop();
                let b = ctx.stack.pop();
                // rely on U256 overflowing_mul to handle overflow
                let (result, _) = a.overflowing_mul(b);
                ctx.stack.push(result);
                1
            },
            Opcode::SUB => {
                let a = ctx.stack.pop();
                let b = ctx.stack.pop();
                // rely on U256 overflowing_sub to handle underflow
                let (result, _) = a.overflowing_sub(b);
                ctx.stack.push(result);
                1
            },
            Opcode::DIV => {
                let a = ctx.stack.pop();
                let b = ctx.stack.pop();
                let result = if b == U256::zero() {
                    U256::zero()
                } else {
                    a / b
                };
                ctx.stack.push(result);
                1
            },
            Opcode::SDIV => {
                let a = ctx.stack.pop();
                let b = ctx.stack.pop();
                let result = if b == U256::zero() {
                    U256::zero()
                } else {
                    let ( a_neg, a_twos ) = (a.bit(255), a.not().overflowing_add(U256::one()).0);
                    let ( b_neg, b_twos ) = (b.bit(255), b.not().overflowing_add(U256::one()).0);
                    let div = if a_neg { a_twos } else { a } / if b_neg { b_twos } else { b };
                    if a_neg ^ b_neg { div.not().overflowing_add(U256::one()).0 } else { div }
                };
                ctx.stack.push(result);
                1
            },
            Opcode::MOD => {
                let a = ctx.stack.pop();
                let b = ctx.stack.pop();
                let result = if b == U256::zero() {
                    U256::zero()
                } else {
                    a % b
                };
                ctx.stack.push(result);
                1
            },
            Opcode::SMOD => {
                let a = ctx.stack.pop();
                let b = ctx.stack.pop();
                let result = if b == U256::zero() {
                    U256::zero()
                } else {
                    let ( a_neg, a_twos ) = (a.bit(255), a.not().overflowing_add(U256::one()).0);
                    let ( b_neg, b_twos ) = (b.bit(255), b.not().overflowing_add(U256::one()).0);
                    let div = if a_neg { a_twos } else { a } % if b_neg { b_twos } else { b };
                    if a_neg | b_neg { div.not().overflowing_add(U256::one()).0 } else { div }
                };
                ctx.stack.push(result);
                1
            },
            Opcode::ADDMOD => {
                let a: U512 = ctx.stack.pop().try_into().unwrap();
                let b: U512 = ctx.stack.pop().try_into().unwrap();
                let c: U256 = ctx.stack.pop();
                let result = if c == U256::zero() {
                    U256::zero()
                } else {
                    ((a + b) % c).try_into().unwrap()
                };
                ctx.stack.push(result);
                1
            },
            Opcode::MULMOD => {
                let a: U512 = ctx.stack.pop().try_into().unwrap();
                let b: U512 = ctx.stack.pop().try_into().unwrap();
                let c: U256 = ctx.stack.pop();
                let result = if c == U256::zero() {
                    U256::zero()
                } else {
                    ((a * b) % c).try_into().unwrap()
                };
                println!("MULMOD: (a: {:#X} * b: {:#X}) % (c: {:#X}) = {:#X}", a, b, c, result);
                ctx.stack.push(result);
                1
            },
            Opcode::EXP => {
                let a = ctx.stack.pop();
                let b = ctx.stack.pop();
                let (result, _) = a.overflowing_pow(b);
                ctx.stack.push(result);
                1
            },
            Opcode::SIGNEXTEND => {
                let a = ctx.stack.pop();
                let b = ctx.stack.pop();
                let id_b = (a.as_usize() + 1) * 8;
                let result = if b.bit(id_b - 1) {
                    U256::MAX.shl(id_b).bitor(b)
                } else { b };
                ctx.stack.push(result);
                1
            },
            Opcode::LT => {
                let a = ctx.stack.pop();
                let b = ctx.stack.pop();
                let result = if a < b { U256::one() } else { U256::zero() };
                ctx.stack.push(result);
                1
            },
            Opcode::GT => {
                let a = ctx.stack.pop();
                let b = ctx.stack.pop();
                let result = if a > b { U256::one() } else { U256::zero() };
                ctx.stack.push(result);
                1
            },
            Opcode::SLT => {
                let a = ctx.stack.pop();
                let b = ctx.stack.pop();
                let (a_twos, _) = a.not().overflowing_add(U256::one());
                let (b_twos, _) = b.not().overflowing_add(U256::one());
                let result = if a_twos > b_twos { U256::one() } else { U256::zero() };
                ctx.stack.push(result);
                1
            },
            Opcode::SGT => {
                let a = ctx.stack.pop();
                let b = ctx.stack.pop();
                let (a_twos, _) = a.not().overflowing_add(U256::one());
                let (b_twos, _) = b.not().overflowing_add(U256::one());
                let result = if a_twos < b_twos { U256::one() } else { U256::zero() };
                ctx.stack.push(result);
                1
            },
            Opcode::EQ => {
                let a = ctx.stack.pop();
                let b = ctx.stack.pop();
                let result = if a == b { U256::one() } else { U256::zero() };
                ctx.stack.push(result);
                1
            },
            Opcode::ISZERO => {
                let a = ctx.stack.pop();
                let result = if a == U256::zero() { U256::one() } else { U256::zero() };
                ctx.stack.push(result);
                1
            },
            Opcode::AND => {
                let a = ctx.stack.pop();
                let b = ctx.stack.pop();
                let result = a.bitand(b);
                ctx.stack.push(result);
                1
            },
            Opcode::OR => {
                let a = ctx.stack.pop();
                let b = ctx.stack.pop();
                let result = a.bitor(b);
                ctx.stack.push(result);
                1
            },
            Opcode::XOR => {
                let a = ctx.stack.pop();
                let b = ctx.stack.pop();
                let result = a.bitxor(b);
                ctx.stack.push(result);
                1
            },
            Opcode::NOT => {
                let a = ctx.stack.pop();
                let result = a.not();
                ctx.stack.push(result);
                1
            },
            Opcode::BYTE => {
                let a = ctx.stack.pop();
                let b = ctx.stack.pop();
                let a = a.as_usize();
                let result = U256::from(if a < 32 {b.byte(31 - a)} else { 0 });
                ctx.stack.push(result);
                1
            },
            Opcode::SHL => {
                let a = ctx.stack.pop();
                let b = ctx.stack.pop();
                let result = b.shl(a.as_usize());
                ctx.stack.push(result);
                1
            },
            Opcode::SHR => {
                let a = ctx.stack.pop();
                let b = ctx.stack.pop();
                let result = b.shr(a.as_usize());
                ctx.stack.push(result);
                1
            },
            Opcode::SAR => {
                let a = ctx.stack.pop();
                let b = ctx.stack.pop();
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
                1
            },
            // TODO
            Opcode::POP => {
                ctx.stack.pop();
                1
            },
            Opcode::PUSH1 => {
                let value = ctx.code[ctx.pc + 1];
                ctx.stack.push(value.into());
                2
            },
            Opcode::PUSH2 => {
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 3];
                ctx.stack.push(value.into());
                3
            },
            Opcode::PUSH3 => {
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 4];
                ctx.stack.push(value.into());
                4
            },
            Opcode::PUSH4 => {
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 5];
                ctx.stack.push(value.into());
                5
            },
            Opcode::PUSH5 => {
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 6];
                ctx.stack.push(value.into());
                6
            },
            Opcode::PUSH6 => {
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 7];
                ctx.stack.push(value.into());
                7
            },
            Opcode::PUSH7 => {
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 8];
                ctx.stack.push(value.into());
                8
            },
            Opcode::PUSH8 => {
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 9];
                ctx.stack.push(value.into());
                9
            },
            Opcode::PUSH9 => {
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 10];
                ctx.stack.push(value.into());
                10
            },
            Opcode::PUSH10 => {
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 11];
                ctx.stack.push(value.into());
                11
            },
            Opcode::PUSH11 => {
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 12];
                ctx.stack.push(value.into());
                12
            },
            Opcode::PUSH12 => {
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 13];
                ctx.stack.push(value.into());
                13
            },
            Opcode::PUSH13 => {
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 14];
                ctx.stack.push(value.into());
                14
            },
            Opcode::PUSH14 => {
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 15];
                ctx.stack.push(value.into());
                15
            },
            Opcode::PUSH15 => {
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 16];
                ctx.stack.push(value.into());
                16
            },
            Opcode::PUSH16 => {
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 17];
                ctx.stack.push(value.into());
                17
            },
            Opcode::PUSH17 => {
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 18];
                ctx.stack.push(value.into());
                18
            },
            Opcode::PUSH18 => {
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 19];
                ctx.stack.push(value.into());
                19
            },
            Opcode::PUSH19 => {
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 20];
                ctx.stack.push(value.into());
                20
            },
            Opcode::PUSH20 => {
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 21];
                ctx.stack.push(value.into());
                21
            },
            Opcode::PUSH21 => {
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 22];
                ctx.stack.push(value.into());
                22
            },
            Opcode::PUSH22 => {
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 23];
                ctx.stack.push(value.into());
                23
            },
            Opcode::PUSH23 => {
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 24];
                ctx.stack.push(value.into());
                24
            },
            Opcode::PUSH24 => {
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 25];
                ctx.stack.push(value.into());
                25
            },
            Opcode::PUSH25 => {
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 26];
                ctx.stack.push(value.into());
                26
            },
            Opcode::PUSH26 => {
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 27];
                ctx.stack.push(value.into());
                27
            },
            Opcode::PUSH27 => {
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 28];
                ctx.stack.push(value.into());
                28
            },
            Opcode::PUSH28 => {
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 29];
                ctx.stack.push(value.into());
                29
            },
            Opcode::PUSH29 => {
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 30];
                ctx.stack.push(value.into());
                30
            },
            Opcode::PUSH30 => {
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 31];
                ctx.stack.push(value.into());
                31
            },
            Opcode::PUSH31 => {
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 32];
                ctx.stack.push(value.into());
                32
            },
            Opcode::PUSH32 => {
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 33];
                ctx.stack.push(value.into());
                33
            },
        }
    }
}