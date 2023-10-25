use crate::value::Value;

pub struct ConstantIndex(u8, u8, u8);

impl From<usize> for ConstantIndex {
    fn from(value: usize) -> Self {
        let a = ((value >> 16) & 0xFF) as u8;
        let b = ((value >> 8) & 0xFF) as u8;
        let c = (value & 0xFF) as u8;
        ConstantIndex(a, b, c)
    }
}

impl From<ConstantIndex> for usize {
    fn from(value: ConstantIndex) -> Self {
        ((value.0 as usize) << 16) | ((value.1 as usize) << 8) | (value.2 as usize)
    }
}

impl ConstantIndex {
    pub fn new(a: u8, b: u8, c: u8) -> Self {
        ConstantIndex(a, b, c) // TODO: just treat the instruction as a 24-bit chunk
    }
    pub fn to_immediate(&self) -> (u8, u8, u8) {
        (self.0, self.1, self.2)
    }
}

#[derive(Default)]
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

    pub fn get(&self, index: ConstantIndex) -> Value {
        let i: usize = index.into();
        self.constants[i]
    }

    pub fn add_u64(&mut self, value: u64) -> ConstantIndex {
        self.find_or_insert(Value::U64(value))
    }
}
