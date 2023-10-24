use crate::value::Value;

pub struct ConstantIndex(u8, u8, u8);

impl From<usize> for ConstantIndex {
    fn from(value: usize) -> Self {
        ConstantIndex(0, 0, 0)
    }
}

impl ConstantIndex {
    pub fn to_immediate(&self) -> (u8, u8, u8) {
        (self.0, self.1, self.2)
    }
}

pub struct ConstantPool {
    constants: Vec<Value>,
}

impl ConstantPool {
    fn find_or_insert(&mut self, value: Value) -> ConstantIndex {
        if let Some(idx) = self.constants.iter().position(|v| v == &value) {
            idx.into()
        } else {
            let idx = self.constants.len();
            self.constants.push(value);
            idx.into()
        }
    }

    pub fn add_u64(&mut self, value: u64) -> ConstantIndex {
        self.find_or_insert(Value::U64(value))
    }
}
