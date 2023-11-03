use std::fmt::Display;

use crate::{
    util::index::InstructionIndex,
    value::{Value, ValueType},
    TypeTable,
};

pub struct LocalSlots {
    types: Vec<ValueType>,
    offsets: Vec<usize>,
    end: usize,
}

impl LocalSlots {
    pub fn new() -> Self {
        LocalSlots {
            types: Vec::new(),
            offsets: Vec::new(),
            end: 0,
        }
    }

    pub fn add_slot(&mut self, type_table: &TypeTable, value_type: ValueType) {
        self.types.push(value_type);
        self.offsets.push(self.end);
        self.end += value_type.size(type_table);
    }

    pub fn total_size(&self, type_table: &TypeTable) -> usize {
        self.types.iter().map(|v| v.size(type_table)).sum()
    }

    pub fn allocate(&self, type_table: &TypeTable, addr: LocalAddress) -> LocalAddress {
        LocalAddress(addr.0 + self.total_size(type_table))
    }

    pub fn slot_info(
        &self,
        slot_index: LocalIndex,
        relative_to: LocalAddress,
    ) -> (ValueType, LocalAddress) {
        let idx: usize = slot_index.into();
        let bytes = self.offsets[idx];
        (self.types[idx], relative_to.offset(bytes))
    }
}

impl Default for LocalSlots {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct LocalAddress(usize);

impl Display for LocalAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl LocalAddress {
    pub fn new() -> Self {
        LocalAddress(0)
    }

    pub fn offset(&self, bytes: usize) -> LocalAddress {
        LocalAddress(self.0 + bytes)
    }
}

pub struct LocalIndex(InstructionIndex);

impl From<LocalIndex> for InstructionIndex {
    fn from(value: LocalIndex) -> Self {
        value.0
    }
}

impl From<LocalIndex> for usize {
    fn from(value: LocalIndex) -> Self {
        value.0.into()
    }
}

impl From<u32> for LocalIndex {
    fn from(value: u32) -> Self {
        LocalIndex(InstructionIndex::new(value as usize))
    }
}

pub struct Locals {
    bytes: Vec<u8>,
}

impl Locals {
    pub fn new() -> Self {
        Locals {
            bytes: Vec::with_capacity(4000), // TODO: make configurable
        }
    }

    pub fn store_local(
        &mut self,
        type_table: &TypeTable,
        addr: LocalAddress,
        value: Value,
    ) -> LocalAddress {
        let sz = value.size(type_table);
        let end = addr.offset(sz);
        self.bytes.reserve(sz);
        if self.bytes.len() < self.bytes.capacity() {
            self.bytes.resize(self.bytes.capacity(), 0);
        }
        let mem = &mut self.bytes[addr.0..end.0];
        value.into_slice(mem);
        end
    }

    pub fn read_local(
        &self,
        type_table: &TypeTable,
        addr: LocalAddress,
        value_type: &ValueType,
    ) -> Value {
        value_type.create_local(&self.bytes[addr.0..addr.0 + value_type.size(type_table)])
    }
}
