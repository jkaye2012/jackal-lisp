use std::{borrow::Borrow, collections::HashMap, fmt::Display};

use crate::{
    local::LocalSlots, module_registry::ModuleName, util::index::InstructionIndex, Instruction,
};

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
    index: FunctionIndex,
    instructions: Vec<Instruction>,
    local_slots: LocalSlots,
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in &self.instructions {
            writeln!(f, "{}", i)?;
        }
        Ok(())
    }
}

impl Function {
    pub fn new(index: FunctionIndex, local_slots: LocalSlots) -> Self {
        Function {
            index,
            instructions: Vec::new(),
            local_slots,
        }
    }

    pub fn index(&self) -> FunctionIndex {
        self.index
    }

    pub fn from_instructions(
        index: FunctionIndex,
        local_slots: LocalSlots,
        instructions: Vec<Instruction>,
    ) -> Self {
        Function {
            index,
            instructions,
            local_slots,
        }
    }

    pub fn next_instruction(&self, ip: &mut InstructionPointer) -> Instruction {
        self.instructions[ip.increment()]
    }

    pub fn local_slots(&self) -> &LocalSlots {
        &self.local_slots
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct FunctionId {
    fq_name: String,
}

impl Display for FunctionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.fq_name)
    }
}

impl FunctionId {
    pub fn new(module_name: &ModuleName, function_name: &str) -> Self {
        let fq_name = format!("{}::{}", module_name.name(), function_name);
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

impl From<u32> for FunctionIndex {
    fn from(value: u32) -> Self {
        FunctionIndex(InstructionIndex::new(value as usize))
    }
}

impl From<usize> for FunctionIndex {
    fn from(value: usize) -> Self {
        FunctionIndex(InstructionIndex::new(value))
    }
}

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

    pub fn insert(
        &mut self,
        id: FunctionId,
        instructions: Vec<Instruction>,
        locals: LocalSlots,
    ) -> FunctionIndex {
        if self.indices.contains_key(&id) {
            panic!("Attempted registration of duplicate function: {}", id);
        }
        let idx = self.functions.len();
        let function_index: FunctionIndex = idx.into();
        let func = Function::from_instructions(function_index, locals, instructions);
        self.functions.push(func);
        self.indices.insert(id, idx);
        function_index
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
