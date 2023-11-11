use crate::{util::index::InstructionIndex, value::Value};

pub struct ConstantIndex(InstructionIndex);

impl From<ConstantIndex> for InstructionIndex {
    fn from(value: ConstantIndex) -> Self {
        value.0
    }
}

impl From<u32> for ConstantIndex {
    fn from(value: u32) -> Self {
        ConstantIndex(InstructionIndex::new(value as usize))
    }
}

#[derive(Default)]
pub struct ConstantPool {
    constants: Vec<Value>,
}

impl ConstantPool {
    pub fn add(&mut self, value: Value) -> ConstantIndex {
        if let Some(idx) = self.constants.iter().position(|v| v == &value) {
            ConstantIndex(idx.into())
        } else {
            let idx = self.constants.len();
            self.constants.push(value);
            ConstantIndex(idx.into())
        }
    }

    pub fn get(&self, index: InstructionIndex) -> Value {
        let i: usize = index.into();
        self.constants[i]
    }
}
