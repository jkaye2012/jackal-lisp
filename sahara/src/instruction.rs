use crate::constant_pool::{ConstantIndex, ConstantPool};

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
            _ => panic!("encountered unknown opcode: {}", value),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Instruction {
    opcode: Opcode,
    a: u8,
    b: u8,
    c: u8,
    // TODO: hold on to the original usize, provide access functions on top of it rather than
    // destructuring like this
}

impl From<Instruction> for u32 {
    fn from(value: Instruction) -> Self {
        (value.opcode as u32) << 24
            | u32::from(value.a) << 16
            | u32::from(value.b) << 8
            | u32::from(value.c)
    }
}

impl From<u32> for Instruction {
    fn from(value: u32) -> Self {
        let opcode = Opcode::from((value >> 24) as u8);
        let a = ((value >> 16) & 0xff) as u8;
        let b = ((value >> 8) & 0xff) as u8;
        let c = (value & 0xff) as u8;
        Instruction { opcode, a, b, c }
    }
}

impl From<Instruction> for ConstantIndex {
    fn from(value: Instruction) -> Self {
        ConstantIndex::new(value.a, value.b, value.c)
    }
}

impl Instruction {
    fn nullary(op: Opcode) -> Instruction {
        Instruction {
            opcode: op,
            a: 0,
            b: 0,
            c: 0,
        }
    }

    fn unary(op: Opcode, a: u8) -> Instruction {
        Instruction {
            opcode: op,
            a,
            b: 0,
            c: 0,
        }
    }

    fn binary(op: Opcode, a: u8, b: u8) -> Instruction {
        Instruction {
            opcode: op,
            a,
            b,
            c: 0,
        }
    }

    fn trinary(op: Opcode, a: u8, b: u8, c: u8) -> Instruction {
        Instruction {
            opcode: op,
            a,
            b,
            c,
        }
    }

    pub fn op(&self) -> Opcode {
        self.opcode
    }

    pub fn imm_u8(value: u8) -> Instruction {
        Self::unary(Opcode::ImmU8, value)
    }

    pub fn const_u64(pool: &mut ConstantPool, value: u64) -> Instruction {
        let (a, b, c) = pool.add_u64(value).to_immediate();
        Self::trinary(Opcode::ConstU64, a, b, c)
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instruction_to_u32() {
        let instruction = Instruction {
            opcode: Opcode::ImmU8,
            a: 1,
            b: 1,
            c: 1,
        };
        let encoded: u32 = instruction.into();
        assert_eq!(encoded, 16843009);
    }

    #[test]
    fn test_u32_to_instruction() {
        let encoded: u32 = 33686018;
        let instruction: Instruction = encoded.into();
        assert_eq!(
            instruction,
            Instruction {
                opcode: Opcode::ConstU64,
                a: 2,
                b: 2,
                c: 2,
            }
        );
    }
}
