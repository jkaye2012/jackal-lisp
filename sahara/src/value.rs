use std::ops;

// TODO: more compact representation, prevent padding for small values
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Value {
    Bool(bool),
    Char(char),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
}

impl ops::Add<Value> for Value {
    type Output = Value;

    fn add(self, rhs: Value) -> Self::Output {
        // TODO: implement for other arithmetic types
        Self::U64(self.u64() + rhs.u64())
    }
}

impl Value {
    pub fn u64(&self) -> u64 {
        match self {
            Self::U64(value) => *value,
            _ => panic!("Attempted to extract u64 from incompatible Value"),
        }
    }
}
