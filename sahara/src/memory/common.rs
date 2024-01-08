use crate::{
    data_type::TypeTable,
    util::index::TypeIndex,
    value::{Value, ValueType},
    TypeDefinition,
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pointer(usize);

impl std::fmt::Display for Pointer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&mut [u8]> for Pointer {
    fn from(value: &mut [u8]) -> Self {
        Pointer::new(usize::from_be_bytes(
            value
                .try_into()
                .expect("Invalid memory size for pointer conversion"),
        ))
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

    pub fn is_valid_allocation(&self) -> bool {
        self.0 > 0
    }
}

pub struct StorageResult {
    end: Pointer,
    allocations: Option<(Pointer, Pointer)>,
}

impl StorageResult {
    pub fn end(&self) -> Pointer {
        self.end
    }

    pub fn allocations(&self) -> Option<(Pointer, Pointer)> {
        self.allocations
    }
}

pub trait Memory: Default {
    fn store_value(&mut self, ptr: Pointer, value: Value) -> StorageResult;

    fn read_value(&self, type_table: &TypeTable, ptr: Pointer, value_type: &ValueType) -> Value;

    fn zero(&mut self, from: Pointer, to: Pointer);
}

pub trait DynamicMemory: Memory {
    fn allocate_n(&mut self, type_table: &TypeTable, type_index: TypeIndex, n: u32) -> Pointer;

    fn allocate(&mut self, type_table: &TypeTable, type_index: TypeIndex) -> Pointer {
        self.allocate_n(type_table, type_index, 1)
    }

    fn type_of<'a>(&self, type_table: &'a TypeTable, ptr: Pointer) -> &'a TypeDefinition;

    fn add_reference(&mut self, idx: Pointer);

    fn remove_reference(&mut self, idx: Pointer);

    fn replace_reference(&mut self, prev: Pointer, new: Pointer);

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
    fn store_value(&mut self, ptr: Pointer, value: Value) -> StorageResult {
        let mut allocations = None;
        let size = value.size();
        let end = ptr.offset(size);
        self.ensure_capacity(size as usize);
        let mem = &mut self.storage[ptr.range(end)];
        if let Value::HeapData(new) = value {
            allocations = Some((mem.into(), new));
        }
        value.into_slice(mem);
        StorageResult { end, allocations }
    }

    fn read_value(&self, type_table: &TypeTable, ptr: Pointer, value_type: &ValueType) -> Value {
        let size = value_type.size(type_table);
        value_type.create_value(&self.storage[ptr.offset_range(size as usize)])
    }

    fn zero(&mut self, from: Pointer, to: Pointer) {
        self.storage[from.0..to.0].fill(0);
    }
}
