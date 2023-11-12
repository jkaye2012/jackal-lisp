use crate::{
    util::index::{ConstantIndex, InstructionIndex},
    value::Value,
};

#[derive(Default)]
pub struct ConstantPool {
    constants: Vec<Value>,
}

impl ConstantPool {
    pub fn add(&mut self, value: Value) -> ConstantIndex {
        if let Some(idx) = self.constants.iter().position(|v| v == &value) {
            idx.into()
        } else {
            let idx = self.constants.len();
            self.constants.push(value);
            idx.into()
        }
    }

    pub fn get(&self, index: InstructionIndex) -> Value {
        let i: usize = index.into();
        self.constants[i]
    }
}
