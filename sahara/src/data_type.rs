use std::{borrow::Borrow, collections::HashMap};

use crate::{module_registry::ModuleName, util::index::InstructionIndex, ValueType};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TypeIndex(InstructionIndex);

impl From<u32> for TypeIndex {
    fn from(value: u32) -> Self {
        TypeIndex(InstructionIndex::new(value as usize))
    }
}

impl From<usize> for TypeIndex {
    fn from(value: usize) -> Self {
        TypeIndex(InstructionIndex::new(value))
    }
}

impl From<TypeIndex> for InstructionIndex {
    fn from(value: TypeIndex) -> Self {
        value.0
    }
}

pub struct Field {
    name: String,
    value_type: ValueType,
}

impl Field {
    pub fn new(name: String, value_type: ValueType) -> Self {
        Field { name, value_type }
    }

    pub fn size(&self) -> usize {
        self.value_type.size()
    }
}

type FieldOffset = (Field, usize);

pub struct TypeDefinition {
    name: String,
    fields: Vec<FieldOffset>,
}

impl TypeDefinition {
    pub fn new(name: String) -> Self {
        TypeDefinition {
            name,
            fields: Vec::new(),
        }
    }

    pub fn add_field(&mut self, field: Field) {
        let offset = if let Some((prev_field, prev_offset)) = self.fields.last() {
            prev_offset + prev_field.size()
        } else {
            0
        };
        self.fields.push((field, offset));
    }

    pub fn total_size(&self) -> usize {
        self.fields.iter().map(|(f, _)| f.size()).sum()
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct TypeId {
    fq_name: String,
}

impl TypeId {
    pub fn new(module_name: &ModuleName, type_name: &str) -> Self {
        let fq_name = format!("{}::{}", module_name.name(), type_name);
        TypeId { fq_name }
    }
}

impl Borrow<str> for TypeId {
    fn borrow(&self) -> &str {
        &self.fq_name
    }
}

pub struct TypeTable {
    indices: HashMap<TypeId, TypeIndex>,
    types: Vec<TypeDefinition>,
}

impl TypeTable {
    pub fn new() -> Self {
        TypeTable {
            indices: HashMap::new(),
            types: Vec::new(),
        }
    }

    pub fn insert(&mut self, id: TypeId, definition: TypeDefinition) -> TypeIndex {
        let idx: TypeIndex = self.types.len().into();
        self.indices.insert(id, idx);
        self.types.push(definition);
        idx
    }

    pub fn index_of(&self, fq_name: &str) -> TypeIndex {
        if let Some(idx) = self.indices.get(fq_name) {
            *idx
        } else {
            panic!("Requested index of unknown type {}", fq_name);
        }
    }

    pub fn get(&self, idx: TypeIndex) -> &TypeDefinition {
        let i: usize = idx.0.into();
        &self.types[i]
    }
}
