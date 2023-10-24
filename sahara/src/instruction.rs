use crate::constant_pool::ConstantPool;

#[repr(u8)]
pub enum Opcode {
    ConstU8,
    ConstU64,
    ConstI8,
    ConstI64,
    True,
    False,
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
            1 => Self::ConstU64,
            2 => Self::ConstI8,
            3 => Self::ConstI64,
            4 => Self::True,
            5 => Self::False,
            _ => panic!("encountered unknown opcode: {}", value),
        }
    }
}

pub struct Instruction {
    opcode: Opcode,
    a: u8,
    b: u8,
    c: u8,
}

impl Instruction {
    pub fn const_u8(value: u8) -> Instruction {
        Instruction {
            opcode: Opcode::ConstU8,
            a: value,
            b: 0,
            c: 0,
        }
    }

    pub fn const_u64(pool: &mut ConstantPool, value: u64) -> Instruction {
        let (a, b, c) = pool.add_u64(value).to_immediate();
        Instruction {
            opcode: Opcode::ConstU64,
            a,
            b,
            c,
        }
    }
}
