use std::{borrow::Borrow, collections::HashMap, fmt::Display};

use crate::{
    local::LocalAddress, module_registry::ModuleName, util::index::InstructionIndex, ValueType,
};

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

impl Display for TypeIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct Field {
    _name: String,
    value_type: ValueType,
}

impl Field {
    pub fn new(name: String, value_type: ValueType) -> Self {
        Field {
            _name: name,
            value_type,
        }
    }

    pub fn size(&self, type_table: &TypeTable) -> usize {
        self.value_type.size(type_table)
    }
}

type FieldOffset = (Field, usize);

pub struct TypeDefinition {
    _name: TypeId,
    fields: Vec<FieldOffset>,
    flattened_fields: Vec<FieldOffset>,
}

enum FieldCategory {
    TopLevel,
    SubField,
}

type ReadField = (ValueType, LocalAddress);

impl TypeDefinition {
    pub fn new(name: TypeId) -> Self {
        TypeDefinition {
            _name: name,
            fields: Vec::new(),
            flattened_fields: Vec::new(),
        }
    }

    pub fn add_field(&mut self, type_table: &TypeTable, field: Field) {
        self.add_flattened_fields(type_table, field, FieldCategory::TopLevel);
    }

    fn add_flattened_fields(
        &mut self,
        type_table: &TypeTable,
        field: Field,
        category: FieldCategory,
    ) {
        let offset = if let Some((prev_field, prev_offset)) = self.flattened_fields.last() {
            prev_offset + prev_field.size(type_table)
        } else {
            0
        };

        match field.value_type {
            ValueType::LocalData(type_idx) => {
                let subtype = type_table.get(type_idx);
                for (subfield, _) in &subtype.fields {
                    self.add_flattened_fields(
                        type_table,
                        subfield.clone(),
                        FieldCategory::SubField,
                    );
                }
            }
            _ => self.flattened_fields.push((field.clone(), offset)),
        }

        if let FieldCategory::TopLevel = category {
            self.fields.push((field, offset));
        }
    }

    pub fn num_fields(&self) -> usize {
        self.flattened_fields.len()
    }

    pub fn total_size(&self, type_table: &TypeTable) -> usize {
        self.flattened_fields
            .iter()
            .map(|(f, _)| f.size(type_table))
            .sum()
    }

    pub fn read_field(&self, addr: LocalAddress, field_idx: u32) -> ReadField {
        let (field, offset) = &self.flattened_fields[field_idx as usize];
        (field.value_type, addr.offset(*offset))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

impl Default for TypeTable {
    fn default() -> Self {
        Self::new()
    }
}
