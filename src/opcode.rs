use std::convert::TryFrom;
use ethereum_types::U256;

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
    POP,
    PUSH1,
    PUSH2,
    PUSH4,
    PUSH6,
    PUSH10,
    PUSH11,
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
            0x50 => Ok(Opcode::POP),
            0x60 => Ok(Opcode::PUSH1),
            0x61 => Ok(Opcode::PUSH2),
            0x63 => Ok(Opcode::PUSH4),
            0x65 => Ok(Opcode::PUSH6),
            0x69 => Ok(Opcode::PUSH10),
            0x6A => Ok(Opcode::PUSH11),
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
                todo!("SDIV");
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
                todo!("SMOD");
            },
            Opcode::ADDMOD => {
                let a = ctx.stack.pop();
                let b = ctx.stack.pop();
                let c = ctx.stack.pop();
                let result = if c == U256::zero() {
                    U256::zero()
                } else {
                    // rely on U256 overflowing_add to handle overflow
                    let (add, _) = a.overflowing_add(b);
                    add % c
                };
                ctx.stack.push(result);
                1
            },
            Opcode::MULMOD => {
                let a = ctx.stack.pop();
                let b = ctx.stack.pop();
                let c = ctx.stack.pop();
                let result = if c == U256::zero() {
                    U256::zero()
                } else {
                    // rely on U256 overflowing_mul to handle overflow
                    let (mul, _) = a.overflowing_mul(b);
                    mul % c
                };
                println!("MULMOD: (a: {:#X} * b: {:#X}) % (c: {:#X}) = {:#X}", a, b, c, result);
                ctx.stack.push(result);
                1
            },
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
            Opcode::PUSH4 => {
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 5];
                ctx.stack.push(value.into());
                5
            },
            Opcode::PUSH6 => {
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 7];
                ctx.stack.push(value.into());
                7
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
            Opcode::PUSH32 => {
                let value = &ctx.code[ctx.pc + 1..ctx.pc + 33];
                ctx.stack.push(value.into());
                33
            },
        }
    }
}