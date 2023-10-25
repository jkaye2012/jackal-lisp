use std::ops;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Value {
    U64(u64),
}

impl ops::Add<Value> for Value {
    type Output = Value;

    fn add(self, rhs: Value) -> Self::Output {
        match self {
            Self::U64(a) => match rhs {
                Value::U64(b) => {
                    return Value::U64(a + b);
                }
            },
        }
    }
}
