use std::fmt::Display;

use crate::{
    constant_pool::ConstantIndex, function::FunctionIndex, local::LocalIndex,
    util::index::InstructionIndex,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum Opcode {
    Halt,
    Add,
    Sub,
    Mul,
    Div,
    Print,
    Call,
    Return,
    LocalStore,
    LocalRead,
    ImmI16 = 248,
    ImmI8 = 249,
    ImmU16 = 250,
    ImmU8 = 251,
    ImmChar = 252,
    ImmBool = 253,
    Const = 254,
}

impl From<Opcode> for u8 {
    fn from(value: Opcode) -> Self {
        value as u8
    }
}

impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Halt,
            1 => Self::Add,
            2 => Self::Sub,
            3 => Self::Mul,
            4 => Self::Div,
            5 => Self::Print,
            6 => Self::Call,
            7 => Self::Return,
            8 => Self::LocalStore,
            9 => Self::LocalRead,
            248 => Self::ImmI16,
            249 => Self::ImmI8,
            250 => Self::ImmU16,
            251 => Self::ImmU8,
            252 => Self::ImmChar,
            253 => Self::ImmBool,
            254 => Self::Const,
            _ => panic!("encountered unknown opcode: {}", value),
        }
    }
}

impl Display for Opcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Halt => write!(f, "halt"),
            Self::Add => write!(f, "add"),
            Self::Sub => write!(f, "sub"),
            Self::Mul => write!(f, "mul"),
            Self::Div => write!(f, "div"),
            Self::Print => write!(f, "print"),
            Self::Call => write!(f, "call"),
            Self::Return => write!(f, "return"),
            Self::LocalStore => write!(f, "local_store"),
            Self::LocalRead => write!(f, "local_read"),
            Self::ImmI16 => write!(f, "imm_i16"),
            Self::ImmI8 => write!(f, "imm_i8"),
            Self::ImmU16 => write!(f, "imm_u16"),
            Self::ImmU8 => write!(f, "imm_u8"),
            Self::ImmChar => write!(f, "imm_char"),
            Self::ImmBool => write!(f, "imm_bool"),
            Self::Const => write!(f, "const"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Instruction {
    bytecode: u32,
}

impl From<Instruction> for InstructionIndex {
    fn from(value: Instruction) -> Self {
        InstructionIndex::new((value.bytecode & 0xFFFFFF) as usize)
    }
}

impl Instruction {
    fn nullary(op: Opcode) -> Instruction {
        Instruction {
            bytecode: (op as u32) << 24,
        }
    }

    fn _unary(op: Opcode, a: u8) -> Instruction {
        Instruction {
            bytecode: (op as u32) << 24 | (a as u32) << 16,
        }
    }

    fn _binary(op: Opcode, a: u8, b: u8) -> Instruction {
        Instruction {
            bytecode: (op as u32) << 24 | (a as u32) << 16 | (b as u32) << 8,
        }
    }

    fn _trinary(op: Opcode, a: u8, b: u8, c: u8) -> Instruction {
        Instruction {
            bytecode: (op as u32) << 24 | (a as u32) << 16 | (b as u32) << 8 | (c as u32),
        }
    }

    fn indexed(op: Opcode, idx: InstructionIndex) -> Instruction {
        let us: usize = idx.into();
        Instruction {
            bytecode: (op as u32) << 24 | (us & 0xFFFFFF) as u32,
        }
    }

    pub fn op(&self) -> Opcode {
        ((self.bytecode >> 24) as u8).into()
    }

    pub fn a(&self) -> u8 {
        ((self.bytecode >> 16) & 0xFF) as u8
    }

    pub fn b(&self) -> u8 {
        ((self.bytecode >> 8) & 0xFF) as u8
    }

    pub fn c(&self) -> u8 {
        (self.bytecode & 0xFF) as u8
    }

    pub fn ab(&self) -> u16 {
        ((self.bytecode >> 8) & 0xFFFF) as u16
    }

    pub fn bc(&self) -> u16 {
        (self.bytecode & 0xFFFF) as u16
    }

    pub fn abc(&self) -> u32 {
        self.bytecode & 0xFFFFFF
    }

    pub fn function_index(&self) -> FunctionIndex {
        self.abc().into()
    }

    pub fn constant_index(&self) -> ConstantIndex {
        self.abc().into()
    }

    pub fn local_index(&self) -> LocalIndex {
        self.abc().into()
    }

    pub fn u8(&self) -> u8 {
        self.a()
    }

    pub fn u16(&self) -> u16 {
        self.ab()
    }

    pub fn i8(&self) -> i8 {
        self.a() as i8
    }

    pub fn i16(&self) -> i16 {
        self.ab() as i16
    }

    pub fn char(&self) -> char {
        self.a() as char
    }

    pub fn bool(&self) -> bool {
        self.a() != 0
    }

    pub fn constant(idx: ConstantIndex) -> Instruction {
        Self::indexed(Opcode::Const, idx.into())
    }

    pub fn add() -> Instruction {
        Self::nullary(Opcode::Add)
    }

    pub fn sub() -> Instruction {
        Self::nullary(Opcode::Sub)
    }

    pub fn mul() -> Instruction {
        Self::nullary(Opcode::Mul)
    }

    pub fn div() -> Instruction {
        Self::nullary(Opcode::Div)
    }

    pub fn print() -> Instruction {
        Self::nullary(Opcode::Print)
    }

    pub fn halt() -> Instruction {
        Self::nullary(Opcode::Halt)
    }

    pub fn call(idx: FunctionIndex) -> Instruction {
        Self::indexed(Opcode::Call, idx.into())
    }

    pub fn ret() -> Instruction {
        Self::nullary(Opcode::Return)
    }

    pub fn local_store() -> Instruction {
        Self::nullary(Opcode::LocalStore)
    }

    pub fn local_read(idx: LocalIndex) -> Instruction {
        Self::indexed(Opcode::LocalRead, idx.into())
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.op())?;
        match self.op() {
            Opcode::Call => write!(f, " {}", self.abc()),
            Opcode::LocalRead => write!(f, " {}", self.abc()),
            Opcode::ImmI16 => write!(f, " {}", self.i16()),
            Opcode::ImmI8 => write!(f, " {}", self.i8()),
            Opcode::ImmU16 => write!(f, " {}", self.u16()),
            Opcode::ImmU8 => write!(f, " {}", self.u8()),
            Opcode::ImmChar => write!(f, " {}", self.char()),
            Opcode::ImmBool => write!(f, " {}", self.bool()),
            Opcode::Const => write!(f, " {}", self.abc()),
            Opcode::Halt
            | Opcode::Return
            | Opcode::Add
            | Opcode::Sub
            | Opcode::Mul
            | Opcode::Div
            | Opcode::LocalStore
            | Opcode::Print => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instruction_to_u32() {
        let instruction = Instruction::_trinary(Opcode::Const, 1, 1, 1);
        assert_eq!(instruction.bytecode, 4261478657);
    }

    #[test]
    fn test_u32_to_instruction() {
        let instruction = Instruction { bytecode: 33686018 };
        assert_eq!(instruction.op(), Opcode::Print);
        assert_eq!(instruction.a(), 2);
        assert_eq!(instruction.b(), 2);
        assert_eq!(instruction.c(), 2);
    }
}
