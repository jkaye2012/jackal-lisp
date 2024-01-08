use crate::{memory::Pointer, util::index::LocalIndex, value::ValueType, TypeTable};

pub struct LocalSlots {
    types: Vec<ValueType>,
    heap_offsets: Vec<u32>,
    offsets: Vec<u32>,
    end: u32,
}

impl LocalSlots {
    pub fn new() -> Self {
        LocalSlots {
            types: Vec::new(),
            heap_offsets: Vec::new(),
            offsets: Vec::new(),
            end: 0,
        }
    }

    pub fn add_slot(&mut self, type_table: &TypeTable, value_type: ValueType) {
        self.types.push(value_type);
        self.offsets.push(self.end);
        if let ValueType::HeapData = value_type {
            self.heap_offsets.push(self.end);
        }
        self.end += value_type.size(type_table);
    }

    pub fn total_size(&self, type_table: &TypeTable) -> u32 {
        self.types.iter().map(|v| v.size(type_table)).sum()
    }

    pub fn allocate(&self, type_table: &TypeTable, ptr: Pointer) -> Pointer {
        ptr.offset(self.total_size(type_table))
    }

    pub fn slot_info(&self, slot_index: LocalIndex, ptr: Pointer) -> (ValueType, Pointer) {
        let idx: usize = slot_index.into();
        let bytes = self.offsets[idx];
        (self.types[idx], ptr.offset(bytes))
    }

    pub fn heap_offsets(&self) -> &[u32] {
        &self.heap_offsets
    }
}

impl Default for LocalSlots {
    fn default() -> Self {
        Self::new()
    }
}
