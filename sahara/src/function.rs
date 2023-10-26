use std::{borrow::Borrow, collections::HashMap};

use crate::{util::index::InstructionIndex, Instruction};

#[derive(Debug, Default, Clone, Copy)]
pub struct InstructionPointer(usize);

impl InstructionPointer {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn increment(&mut self) -> usize {
        let current = self.0;
        self.0 += 1;
        current
    }
}

pub struct Function {
    instructions: Vec<Instruction>,
}

impl Function {
    pub fn new() -> Self {
        Function {
            instructions: Vec::new(),
        }
    }

    pub fn from_instructions(instructions: Vec<Instruction>) -> Self {
        Function { instructions }
    }

    pub fn add(&mut self, inst: Instruction) {
        self.instructions.push(inst);
    }

    pub fn next_instruction(&self, ip: &mut InstructionPointer) -> Instruction {
        self.instructions[ip.increment()]
    }
}

impl Default for Function {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct FunctionId {
    fq_name: String,
}

impl FunctionId {
    pub fn new(module_name: &str, function_name: &str) -> Self {
        let fq_name = format!("{}::{}", module_name, function_name);
        FunctionId { fq_name }
    }

    pub fn from_fq_name(fq_name: String) -> Self {
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct FunctionIndex(InstructionIndex);

impl From<FunctionIndex> for InstructionIndex {
    fn from(value: FunctionIndex) -> Self {
        value.0
    }
}

impl FunctionTable {
    pub fn new() -> Self {
        FunctionTable {
            functions: Vec::new(),
            indices: HashMap::new(),
        }
    }

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

impl Default for FunctionTable {
    fn default() -> Self {
        Self::new()
    }
}
