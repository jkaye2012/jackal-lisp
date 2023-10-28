use std::ops;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Value {
    U64(u64),
    I64(i64),
}

impl ops::Add<Value> for Value {
    type Output = Value;

    fn add(self, rhs: Value) -> Self::Output {
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
