use crate::{
    data_type::TypeTable,
    util::index::TypeIndex,
    value::{Value, ValueType},
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pointer(usize);

impl std::fmt::Display for Pointer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Pointer {
    pub fn new(ptr: usize) -> Self {
        Pointer(ptr)
    }

    pub fn offset(&self, off: u32) -> Self {
        Self(self.0 + off as usize)
    }

    pub fn offset_range(&self, off: usize) -> std::ops::Range<usize> {
        self.0..self.0 + off
    }

    pub fn incr(&mut self, by: u32) {
        self.0 += by as usize;
    }

    pub fn range(&self, end: Self) -> std::ops::Range<usize> {
        self.0..end.0
    }

    pub fn be_bytes(&self) -> [u8; 8] {
        self.0.to_be_bytes()
    }
}

pub trait Memory: Default {
    fn store_value(&mut self, ptr: Pointer, value: Value) -> Pointer;

    fn read_value(&self, type_table: &TypeTable, ptr: Pointer, value_type: &ValueType) -> Value;
}

pub trait DynamicMemory: Memory {
    fn new() -> Self;

    fn allocate_n(&mut self, type_table: &TypeTable, type_index: TypeIndex, n: u32) -> Pointer;

    fn allocate(&mut self, type_table: &TypeTable, type_index: TypeIndex) -> Pointer {
        self.allocate_n(type_table, type_index, 1)
    }

    fn add_reference(&mut self, idx: Pointer);

    fn remove_reference(&mut self, idx: Pointer);

    fn is_allocation_valid(&self, idx: Pointer) -> bool;
}

pub struct GrowableContiguousMemory {
    storage: Vec<u8>,
}

impl Default for GrowableContiguousMemory {
    fn default() -> Self {
        Self {
            storage: Vec::with_capacity(4000), // TODO: make configurable
        }
    }
}

impl GrowableContiguousMemory {
    pub fn ensure_capacity(&mut self, size: usize) {
        self.storage.reserve(size);
        if self.storage.len() < self.storage.capacity() {
            self.storage.resize(self.storage.capacity(), 0);
        }
    }

    pub fn slice(&self, range: std::ops::Range<usize>) -> &[u8] {
        &self.storage[range]
    }

    pub fn slice_mut(&mut self, range: std::ops::Range<usize>) -> &mut [u8] {
        &mut self.storage[range]
    }
}

impl Memory for GrowableContiguousMemory {
    fn store_value(&mut self, ptr: Pointer, value: Value) -> Pointer {
        let size = value.size();
        let end = ptr.offset(size);
        self.ensure_capacity(size as usize);
        let mem = &mut self.storage[ptr.range(end)];
        value.into_slice(mem);
        end
    }

    fn read_value(&self, type_table: &TypeTable, ptr: Pointer, value_type: &ValueType) -> Value {
        let size = value_type.size(type_table);
        value_type.create_value(&self.storage[ptr.offset_range(size as usize)])
    }
}
