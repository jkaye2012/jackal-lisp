use std::{fmt::Display, ops};

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

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bool(val) => write!(f, "Bool({})", val),
            Self::Char(val) => write!(f, "Char({})", val),
            Self::U8(val) => write!(f, "U8({})", val),
            Self::U16(val) => write!(f, "U16({})", val),
            Self::U32(val) => write!(f, "U32({})", val),
            Self::U64(val) => write!(f, "U64({})", val),
            Self::I8(val) => write!(f, "I8({})", val),
            Self::I16(val) => write!(f, "I16({})", val),
            Self::I32(val) => write!(f, "I32({})", val),
            Self::I64(val) => write!(f, "I64({})", val),
            Self::F32(val) => write!(f, "F32({})", val),
            Self::F64(val) => write!(f, "F64({})", val),
        }
    }
}

impl Value {
    fn u8(&self) -> u8 {
        if let Self::U8(val) = self {
            *val
        } else {
            panic!("Attempted to coerce invalid type to u8: {}", self);
        }
    }

    fn u16(&self) -> u16 {
        match self {
            Self::U8(val) => *val as u16,
            Self::U16(val) => *val,
            _ => panic!("Attempted to coerce invalid type to u16: {}", self),
        }
    }

    fn u32(&self) -> u32 {
        match self {
            Self::U8(val) => *val as u32,
            Self::U16(val) => *val as u32,
            Self::U32(val) => *val,
            _ => panic!("Attempted to coerce invalid type to u32: {}", self),
        }
    }

    fn u64(&self) -> u64 {
        match self {
            Self::U8(val) => *val as u64,
            Self::U16(val) => *val as u64,
            Self::U32(val) => *val as u64,
            Self::U64(val) => *val,
            _ => panic!("Attempted to coerce invalid type to u64: {}", self),
        }
    }

    fn i8(&self) -> i8 {
        if let Self::I8(val) = self {
            *val
        } else {
            panic!("Attempted to coerce invalid type to i8: {}", self);
        }
    }

    fn i16(&self) -> i16 {
        match self {
            Self::I8(val) => *val as i16,
            Self::I16(val) => *val,
            _ => panic!("Attempted to coerce invalid type to i16: {}", self),
        }
    }

    fn i32(&self) -> i32 {
        match self {
            Self::I8(val) => *val as i32,
            Self::I16(val) => *val as i32,
            Self::I32(val) => *val,
            _ => panic!("Attempted to coerce invalid type to i32: {}", self),
        }
    }

    fn i64(&self) -> i64 {
        match self {
            Self::I8(val) => *val as i64,
            Self::I16(val) => *val as i64,
            Self::I32(val) => *val as i64,
            Self::I64(val) => *val,
            _ => panic!("Attempted to coerce invalid type to i64: {}", self),
        }
    }

    fn f32(&self) -> f32 {
        match self {
            Self::I8(val) => *val as f32,
            Self::I16(val) => *val as f32,
            Self::I32(val) => *val as f32,
            Self::U8(val) => *val as f32,
            Self::U16(val) => *val as f32,
            Self::U32(val) => *val as f32,
            Self::F32(val) => *val,
            _ => panic!("Attempted to coerce invalid type to f32: {}", self),
        }
    }

    fn f64(&self) -> f64 {
        match self {
            Self::I8(val) => *val as f64,
            Self::I16(val) => *val as f64,
            Self::I32(val) => *val as f64,
            Self::I64(val) => *val as f64,
            Self::U8(val) => *val as f64,
            Self::U16(val) => *val as f64,
            Self::U32(val) => *val as f64,
            Self::U64(val) => *val as f64,
            Self::F32(val) => *val as f64,
            Self::F64(val) => *val,
            _ => panic!("Attempted to coerce invalid type to f64: {}", self),
        }
    }
}

impl ops::Add<Value> for Value {
    type Output = Value;

    fn add(self, rhs: Value) -> Self::Output {
        match self {
            Self::U8(lhs) => Self::U8(lhs + rhs.u8()),
            Self::U16(lhs) => Self::U16(lhs + rhs.u16()),
            Self::U32(lhs) => Self::U32(lhs + rhs.u32()),
            Self::U64(lhs) => Self::U64(lhs + rhs.u64()),
            Self::I8(lhs) => Self::I8(lhs + rhs.i8()),
            Self::I16(lhs) => Self::I16(lhs + rhs.i16()),
            Self::I32(lhs) => Self::I32(lhs + rhs.i32()),
            Self::I64(lhs) => Self::I64(lhs + rhs.i64()),
            Self::F32(lhs) => Self::F32(lhs + rhs.f32()),
            Self::F64(lhs) => Self::F64(lhs + rhs.f64()),
            _ => panic!("Attempted to add invalid type: {}", self),
        }
    }
}

impl ops::Sub<Value> for Value {
    type Output = Value;

    fn sub(self, rhs: Value) -> Self::Output {
        match self {
            Self::U8(lhs) => Self::U8(lhs - rhs.u8()),
            Self::U16(lhs) => Self::U16(lhs - rhs.u16()),
            Self::U32(lhs) => Self::U32(lhs - rhs.u32()),
            Self::U64(lhs) => Self::U64(lhs - rhs.u64()),
            Self::I8(lhs) => Self::I8(lhs - rhs.i8()),
            Self::I16(lhs) => Self::I16(lhs - rhs.i16()),
            Self::I32(lhs) => Self::I32(lhs - rhs.i32()),
            Self::I64(lhs) => Self::I64(lhs - rhs.i64()),
            Self::F32(lhs) => Self::F32(lhs - rhs.f32()),
            Self::F64(lhs) => Self::F64(lhs - rhs.f64()),
            _ => panic!("Attempted to subtract invalid type: {}", self),
        }
    }
}

impl ops::Mul<Value> for Value {
    type Output = Value;

    fn mul(self, rhs: Value) -> Self::Output {
        match self {
            Self::U8(lhs) => Self::U8(lhs * rhs.u8()),
            Self::U16(lhs) => Self::U16(lhs * rhs.u16()),
            Self::U32(lhs) => Self::U32(lhs * rhs.u32()),
            Self::U64(lhs) => Self::U64(lhs * rhs.u64()),
            Self::I8(lhs) => Self::I8(lhs * rhs.i8()),
            Self::I16(lhs) => Self::I16(lhs * rhs.i16()),
            Self::I32(lhs) => Self::I32(lhs * rhs.i32()),
            Self::I64(lhs) => Self::I64(lhs * rhs.i64()),
            Self::F32(lhs) => Self::F32(lhs * rhs.f32()),
            Self::F64(lhs) => Self::F64(lhs * rhs.f64()),
            _ => panic!("Attempted to multiply invalid type: {}", self),
        }
    }
}

impl ops::Div<Value> for Value {
    type Output = Value;

    fn div(self, rhs: Value) -> Self::Output {
        match self {
            Self::U8(lhs) => Self::U8(lhs / rhs.u8()),
            Self::U16(lhs) => Self::U16(lhs / rhs.u16()),
            Self::U32(lhs) => Self::U32(lhs / rhs.u32()),
            Self::U64(lhs) => Self::U64(lhs / rhs.u64()),
            Self::I8(lhs) => Self::I8(lhs / rhs.i8()),
            Self::I16(lhs) => Self::I16(lhs / rhs.i16()),
            Self::I32(lhs) => Self::I32(lhs / rhs.i32()),
            Self::I64(lhs) => Self::I64(lhs / rhs.i64()),
            Self::F32(lhs) => Self::F32(lhs / rhs.f32()),
            Self::F64(lhs) => Self::F64(lhs / rhs.f64()),
            _ => panic!("Attempted to divide invalid type: {}", self),
        }
    }
}
