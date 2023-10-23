#[repr(u8)]
pub enum Opcode {
    ConstU8,
    ConstU16,
    ConstU32,
    ConstU64,
    ConstI8,
    ConstI16,
    ConstI32,
    ConstI64,
}

impl From<Opcode> for u8 {
    fn from(value: Opcode) -> Self {
        value as u8
    }
}

impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::ConstU8,
            1 => Self::ConstU16,
            2 => Self::ConstU32,
            3 => Self::ConstU64,
            4 => Self::ConstI8,
            5 => Self::ConstI16,
            6 => Self::ConstI32,
            7 => Self::ConstI64,
            _ => panic!("encountered unknown opcode: {}", value),
        }
    }
}

pub struct Instruction {}
