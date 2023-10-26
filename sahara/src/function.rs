use std::{borrow::Borrow, collections::HashMap};

use crate::{util::index::InstructionIndex, Instruction};

pub struct Function {
    instructions: Vec<Instruction>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct FunctionId {
    fq_name: String,
}

impl FunctionId {
    pub fn new(module_name: &str, function_name: &str) -> Self {
        let fq_name = format!("{}::{}", module_name, function_name);
        FunctionId { fq_name }
    }
}

impl Borrow<str> for FunctionId {
    fn borrow(&self) -> &str {
        &self.fq_name
    }
}

pub struct FunctionTable {
    functions: Vec<Function>,
    indices: HashMap<FunctionId, usize>,
}

pub struct FunctionIndex(InstructionIndex);

impl From<FunctionIndex> for InstructionIndex {
    fn from(value: FunctionIndex) -> Self {
        value.0
    }
}

impl FunctionTable {
    pub fn insert(&mut self, id: FunctionId, func: Function) {
        let idx = self.functions.len();
        self.functions.push(func);
        self.indices.insert(id, idx);
    }

    pub fn address_of(&self, fq_name: &str) -> FunctionIndex {
        if let Some(idx) = self.indices.get(fq_name) {
            FunctionIndex((*idx).into())
        } else {
            panic!("Requested unknown function {}", fq_name);
        }
    }

    pub fn get(&self, index: FunctionIndex) -> &Function {
        let idx: usize = index.0.into();
        &self.functions[idx]
    }
}
