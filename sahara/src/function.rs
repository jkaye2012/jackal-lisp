use std::{borrow::Borrow, collections::HashMap, fmt::Display};

use crate::{
    local::LocalSlots, memory::Pointer, module_registry::ModuleName, util::index::FunctionIndex,
    Instruction,
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

pub struct HeapReferences<'a> {
    ptr: Pointer,
    offsets: &'a [u32],
    idx: usize,
}

impl<'a> HeapReferences<'a> {
    fn new(ptr: Pointer, offsets: &'a [u32]) -> Self {
        HeapReferences {
            ptr,
            offsets,
            idx: 0,
        }
    }
}

impl<'a> Iterator for HeapReferences<'a> {
    type Item = Pointer;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < self.offsets.len() {
            let offset = self.offsets[self.idx];
            self.idx += 1;
            Some(self.ptr.offset(offset))
        } else {
            None
        }
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

    pub fn heap_references(&self, ptr: Pointer) -> HeapReferences<'_> {
        HeapReferences::new(ptr, self.local_slots.heap_offsets())
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
            (*idx).into()
        } else {
            panic!("Requested unknown function {}", fq_name);
        }
    }

    pub fn get(&self, index: FunctionIndex) -> &Function {
        let idx: usize = index.into();
        &self.functions[idx]
    }
}

impl Default for FunctionTable {
    fn default() -> Self {
        Self::new()
    }
}
