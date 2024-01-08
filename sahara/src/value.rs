use std::{fmt::Display, ops};

use crate::{memory::Pointer, util::index::TypeIndex, TypeTable};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueType {
    Bool,
    Char,
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    LocalData(TypeIndex),
    HeapData,
}

impl Display for ValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bool => write!(f, "Bool"),
            Self::Char => write!(f, "Char"),
            Self::U8 => write!(f, "U8"),
            Self::U16 => write!(f, "U16"),
            Self::U32 => write!(f, "U32"),
            Self::U64 => write!(f, "U64"),
            Self::I8 => write!(f, "I8"),
            Self::I16 => write!(f, "I16"),
            Self::I32 => write!(f, "I32"),
            Self::I64 => write!(f, "I64"),
            Self::F32 => write!(f, "F32"),
            Self::F64 => write!(f, "F64"),
            Self::LocalData(_) => write!(f, "LocalData"),
            Self::HeapData => write!(f, "Heap"),
        }
    }
}

impl ValueType {
    pub fn size(&self, type_table: &TypeTable) -> u32 {
        match self {
            Self::Bool => 1,
            Self::Char => 1,
            Self::U8 => 1,
            Self::U16 => 2,
            Self::U32 => 4,
            Self::U64 => 8,
            Self::I8 => 1,
            Self::I16 => 2,
            Self::I32 => 4,
            Self::I64 => 8,
            Self::F32 => 4,
            Self::F64 => 8,
            Self::LocalData(type_index) => type_table.get(*type_index).total_size(type_table),
            Self::HeapData => 8,
        }
    }

    pub fn is_primitive(&self) -> bool {
        match self {
            Self::Bool
            | Self::Char
            | Self::U8
            | Self::U16
            | Self::U32
            | Self::U64
            | Self::I8
            | Self::I16
            | Self::I32
            | Self::I64
            | Self::F32
            | Self::F64 => true,
            Self::LocalData(_) => false,
            Self::HeapData => false,
        }
    }

    pub fn create_value(&self, bytes: &[u8]) -> Value {
        match self {
            Self::Bool => {
                let mem: [u8; 1] = bytes.try_into().expect("Invalid memory");
                Value::Bool(mem[0] != 0)
            }
            Self::Char => {
                let mem: [u8; 1] = bytes.try_into().expect("Invalid memory");
                Value::Char(mem[0] as char)
            }
            Self::U8 => {
                let mem: [u8; 1] = bytes.try_into().expect("Invalid memory");
                Value::U8(mem[0])
            }
            Self::U16 => {
                let mem: [u8; 2] = bytes.try_into().expect("Invalid memory");
                Value::U16(u16::from_be_bytes(mem))
            }
            Self::U32 => {
                let mem: [u8; 4] = bytes.try_into().expect("Invalid memory");
                Value::U32(u32::from_be_bytes(mem))
            }
            Self::U64 => {
                let mem: [u8; 8] = bytes.try_into().expect("Invalid memory");
                Value::U64(u64::from_be_bytes(mem))
            }
            Self::I8 => {
                let mem: [u8; 1] = bytes.try_into().expect("Invalid memory");
                Value::I8(mem[0] as i8)
            }
            Self::I16 => {
                let mem: [u8; 2] = bytes.try_into().expect("Invalid memory");
                Value::I16(i16::from_be_bytes(mem))
            }
            Self::I32 => {
                let mem: [u8; 4] = bytes.try_into().expect("Invalid memory");
                Value::I32(i32::from_be_bytes(mem))
            }
            Self::I64 => {
                let mem: [u8; 8] = bytes.try_into().expect("Invalid memory");
                Value::I64(i64::from_be_bytes(mem))
            }
            Self::F32 => {
                let mem: [u8; 4] = bytes.try_into().expect("Invalid memory");
                Value::F32(f32::from_be_bytes(mem))
            }
            Self::F64 => {
                let mem: [u8; 8] = bytes.try_into().expect("Invalid memory");
                Value::F64(f64::from_be_bytes(mem))
            }
            _ => panic!(
                "Attempted to create_local with non-primitive ValueType: {}",
                self
            ),
        }
    }

    pub fn type_index(&self) -> TypeIndex {
        match self {
            Self::LocalData(idx) => *idx,
            _ => panic!(
                "Attempted to extract type index from unsupported ValueType: {}",
                self
            ),
        }
    }
}

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
    HeapData(Pointer),
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
            Self::HeapData(idx) => write!(f, "HeapData({})", idx),
        }
    }
}

impl Value {
    pub fn into_slice(self, mem: &mut [u8]) {
        match self {
            Self::Bool(val) => {
                if val {
                    mem.copy_from_slice(&[1]);
                } else {
                    mem.copy_from_slice(&[0]);
                }
            }
            Self::Char(val) => {
                val.encode_utf8(mem);
            }
            Self::U8(val) => mem.copy_from_slice(&val.to_be_bytes()),
            Self::U16(val) => mem.copy_from_slice(&val.to_be_bytes()),
            Self::U32(val) => mem.copy_from_slice(&val.to_be_bytes()),
            Self::U64(val) => mem.copy_from_slice(&val.to_be_bytes()),
            Self::I8(val) => mem.copy_from_slice(&val.to_be_bytes()),
            Self::I16(val) => mem.copy_from_slice(&val.to_be_bytes()),
            Self::I32(val) => mem.copy_from_slice(&val.to_be_bytes()),
            Self::I64(val) => mem.copy_from_slice(&val.to_be_bytes()),
            Self::F32(val) => mem.copy_from_slice(&val.to_be_bytes()),
            Self::F64(val) => mem.copy_from_slice(&val.to_be_bytes()),
            Self::HeapData(idx) => mem.copy_from_slice(&idx.be_bytes()),
        }
    }

    pub fn size(&self) -> u32 {
        match self {
            Self::Bool(_) => 1,
            Self::Char(_) => 1,
            Self::U8(_) => 1,
            Self::U16(_) => 2,
            Self::U32(_) => 4,
            Self::U64(_) => 8,
            Self::I8(_) => 1,
            Self::I16(_) => 2,
            Self::I32(_) => 4,
            Self::I64(_) => 8,
            Self::F32(_) => 4,
            Self::F64(_) => 8,
            Self::HeapData(_) => 8,
        }
    }

    pub fn pointer(&self) -> Pointer {
        match self {
            Self::HeapData(idx) => *idx,
            _ => panic!("Attempted to extract pointer from non-heap value: {}", self),
        }
    }

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
