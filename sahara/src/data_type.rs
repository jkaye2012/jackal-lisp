use std::{borrow::Borrow, collections::HashMap};

use crate::{
    memory::Pointer,
    module_registry::ModuleName,
    util::index::{InstructionIndex, TypeIndex},
    ValueType,
};

#[derive(Debug, Clone)]
pub struct Field {
    name: String,
    value_type: ValueType,
}

impl Field {
    pub fn new(name: String, value_type: ValueType) -> Self {
        Field { name, value_type }
    }

    pub fn size(&self, type_table: &TypeTable) -> u32 {
        self.value_type.size(type_table)
    }
}

type FieldOffset = (Field, u32);

pub struct TypeDefinition {
    name: TypeId,
    fields: Vec<FieldOffset>,
    flattened_fields: Vec<FieldOffset>,
    path_lookup: HashMap<String, u32>,
}

enum FieldCategory {
    TopLevel,
    SubField,
}

type FieldPointer = (ValueType, Pointer);

impl TypeDefinition {
    pub fn new(name: TypeId) -> Self {
        TypeDefinition {
            name,
            fields: Vec::new(),
            flattened_fields: Vec::new(),
            path_lookup: HashMap::new(),
        }
    }

    pub fn add_field(&mut self, type_table: &TypeTable, field: Field) {
        let path = field.name.clone(); // TODO: clone not really necessary
        self.add_flattened_fields(type_table, field, FieldCategory::TopLevel, &path)
    }

    fn add_flattened_fields(
        &mut self,
        type_table: &TypeTable,
        field: Field,
        category: FieldCategory,
        path: &str,
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
                        &format!("{}.{}", path, subfield.name),
                    );
                }
            }
            _ => {
                self.path_lookup.insert(path.to_string(), self.num_fields());
                self.flattened_fields.push((field.clone(), offset));
            }
        }

        if let FieldCategory::TopLevel = category {
            self.fields.push((field, offset));
        }
    }

    pub fn num_fields(&self) -> u32 {
        self.flattened_fields
            .len()
            .try_into()
            .expect("data types may contain at most 2^31 fields")
    }

    pub fn total_size(&self, type_table: &TypeTable) -> u32 {
        self.flattened_fields
            .iter()
            .map(|(f, _)| f.size(type_table))
            .sum()
    }

    pub fn field_pointer(&self, ptr: Pointer, field_idx: InstructionIndex) -> FieldPointer {
        let idx: usize = field_idx.into();
        let (field, offset) = &self.flattened_fields[idx];
        (field.value_type, ptr.offset(*offset))
    }

    pub fn query(&self, path: &[&str]) -> Option<u32> {
        let pathname = path.join(".");
        self.path_lookup.get(&pathname).copied()
    }

    pub fn get(&self, field_idx: u32) -> &FieldOffset {
        &self.flattened_fields[field_idx as usize]
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

    pub fn insert(&mut self, definition: TypeDefinition) -> TypeIndex {
        let idx: TypeIndex = self.types.len().into();
        self.indices.insert(definition.name.clone(), idx);
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
        let i: usize = idx.into();
        &self.types[i]
    }

    pub fn size(&self, idx: TypeIndex) -> u32 {
        self.get(idx).total_size(self)
    }
}

impl Default for TypeTable {
    fn default() -> Self {
        Self::new()
    }
}

// unit tests
#[cfg(test)]
mod tests {
    use crate::ModuleRegistry;

    use super::*;

    fn create_type_definition(name: &str) -> TypeDefinition {
        let mut module_registry = ModuleRegistry::new();
        let test_module = module_registry.register("test".to_string());
        TypeDefinition::new(TypeId::new(&test_module, name))
    }

    #[test]
    fn type_definition_add_primitive_field_adds_single_field() {
        let type_table = TypeTable::new();
        let mut type_defn = create_type_definition("TestType");
        type_defn.add_field(&type_table, Field::new("red".to_string(), ValueType::U8));
        assert_eq!(type_defn.num_fields(), 1);
        let field_idx = type_defn.query(&["red"]);
        let (field, _) = type_defn.get(field_idx.unwrap());
        assert_eq!(field.name, "red");
        assert_eq!(field.value_type, ValueType::U8);
    }

    #[test]
    fn type_definition_add_nested_field_adds_multiple_fields() {
        let mut type_table = TypeTable::new();

        let mut rgb = create_type_definition("Rgb");
        rgb.add_field(&type_table, Field::new("red".to_string(), ValueType::U8));
        rgb.add_field(&type_table, Field::new("green".to_string(), ValueType::U8));
        rgb.add_field(&type_table, Field::new("blue".to_string(), ValueType::U8));
        let rgb_index = type_table.insert(rgb);

        let mut type_defn = create_type_definition("TestType");
        type_defn.add_field(
            &type_table,
            Field::new("color".to_string(), ValueType::LocalData(rgb_index)),
        );
        assert_eq!(type_defn.num_fields(), 3);
        let field_idx = type_defn.query(&["color", "red"]);
        let (field, _) = type_defn.get(field_idx.unwrap());
        assert_eq!(field.name, "red");
        assert_eq!(field.value_type, ValueType::U8);
        let (field, _) = &type_defn.flattened_fields[1];
        assert_eq!(field.name, "green");
        assert_eq!(field.value_type, ValueType::U8);
        let (field, _) = &type_defn.flattened_fields[2];
        assert_eq!(field.name, "blue");
        assert_eq!(field.value_type, ValueType::U8);
    }
}
