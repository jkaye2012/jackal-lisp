use crate::{constant_pool::ConstantIndex, function::FunctionIndex, util::index::InstructionIndex};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum Opcode {
    Halt,
    ImmU8,
    ConstU64,
    ImmI8,
    ConstI64,
    ImmTrue,
    ImmFalse,
    Add,
    Print,
    Call,
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
            1 => Self::ImmU8,
            2 => Self::ConstU64,
            3 => Self::ImmI8,
            4 => Self::ConstI64,
            5 => Self::ImmTrue,
            6 => Self::ImmFalse,
            7 => Self::Add,
            8 => Self::Print,
            9 => Self::Call,
            _ => panic!("encountered unknown opcode: {}", value),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Instruction {
    bytecode: u32,
}

// impl From<Instruction> for u32 {
//     fn from(value: Instruction) -> Self {
//         (value.opcode as u32) << 24
//             | u32::from(value.a) << 16
//             | u32::from(value.b) << 8
//             | u32::from(value.c)
//     }
// }

// impl From<u32> for Instruction {
//     fn from(value: u32) -> Self {
//         let opcode = Opcode::from((value >> 24) as u8);
//         let a = ((value >> 16) & 0xff) as u8;
//         let b = ((value >> 8) & 0xff) as u8;
//         let c = (value & 0xff) as u8;
//         Instruction { opcode, a, b, c }
//     }
// }

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

    fn unary(op: Opcode, a: u8) -> Instruction {
        Instruction {
            bytecode: (op as u32) << 24 | (a as u32) << 16,
        }
    }

    fn binary(op: Opcode, a: u8, b: u8) -> Instruction {
        Instruction {
            bytecode: (op as u32) << 24 | (a as u32) << 16 | (b as u32) << 8,
        }
    }

    fn trinary(op: Opcode, a: u8, b: u8, c: u8) -> Instruction {
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

    pub fn imm_u8(value: u8) -> Instruction {
        Self::unary(Opcode::ImmU8, value)
    }

    pub fn const_u64(idx: ConstantIndex) -> Instruction {
        Self::indexed(Opcode::ConstU64, idx.into())
    }

    pub fn add() -> Instruction {
        Self::nullary(Opcode::Add)
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instruction_to_u32() {
        let instruction = Instruction::trinary(Opcode::ImmU8, 1, 1, 1);
        assert_eq!(instruction.bytecode, 16843009);
    }

    #[test]
    fn test_u32_to_instruction() {
        let instruction = Instruction { bytecode: 33686018 };
        assert_eq!(instruction.op(), Opcode::ConstU64);
        assert_eq!(instruction.a(), 2);
        assert_eq!(instruction.b(), 2);
        assert_eq!(instruction.c(), 2);
    }
}
