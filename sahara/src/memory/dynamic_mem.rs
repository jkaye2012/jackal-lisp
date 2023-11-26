use std::{collections::BTreeSet, mem::size_of};

use crate::{data_type::TypeTable, util::index::TypeIndex};

use super::{
    common::{DynamicMemory, GrowableContiguousMemory},
    Memory, Pointer,
};

#[derive(Debug, Clone, Copy)]
struct References(u32);

impl References {
    fn new() -> Self {
        References(0x00000001)
    }

    fn be_bytes(&self) -> [u8; 4] {
        self.0.to_be_bytes()
    }

    fn increment(&mut self) {
        assert!(
            self.reference_count() < 0x7FFFFFFF,
            "Attempted to increment reference would result in reference overflow!"
        );
        self.0 += 1
    }

    fn decrement(&mut self) {
        assert!(
            self.0 > 0 && self.0 != 0x00000000,
            "Attempted to decrement references without an existing reference!"
        );
        self.0 -= 1
    }

    fn reference_count(&self) -> u32 {
        self.0
    }

    fn is_live(&self) -> bool {
        self.reference_count() > 0
    }
}

// TODO: document how references are used, limitation of 2^30 heap references
#[derive(Debug, Clone, Copy)]
struct HeapAllocation {
    references: References,
    type_index: TypeIndex,
    num: u32,
    size: u32,
}

impl HeapAllocation {
    fn new(type_index: TypeIndex, num: u32, size: u32) -> Self {
        HeapAllocation {
            type_index,
            references: References::new(),
            num,
            size,
        }
    }

    fn size() -> u32 {
        size_of::<HeapAllocation>().try_into().unwrap()
    }

    fn from_memory(memory: &[u8]) -> Self {
        let references = References(u32::from_be_bytes(memory[..4].try_into().unwrap()));
        let type_index = u32::from_be_bytes(memory[4..8].try_into().unwrap()).into();
        let num = u32::from_be_bytes(memory[8..12].try_into().unwrap());
        let size = u32::from_be_bytes(memory[12..16].try_into().unwrap());
        HeapAllocation {
            type_index,
            references,
            num,
            size,
        }
    }

    fn has_live_references(&self) -> bool {
        self.references.is_live()
    }

    fn be_bytes(&self) -> [u8; 16] {
        let mut bytes = [0u8; 16];
        bytes[..4].copy_from_slice(&self.references.be_bytes());
        bytes[4..8].copy_from_slice(&self.type_index.be_bytes());
        bytes[8..12].copy_from_slice(&self.num.to_be_bytes());
        bytes[12..].copy_from_slice(&self.size.to_be_bytes());
        bytes
    }
}

#[derive(Clone, Copy)]
struct IndexedAllocation {
    ptr: Pointer,
    size: u32,
}

impl PartialEq for IndexedAllocation {
    fn eq(&self, other: &Self) -> bool {
        self.ptr == other.ptr
    }
}

impl PartialOrd for IndexedAllocation {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for IndexedAllocation {}

impl Ord for IndexedAllocation {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.size.cmp(&other.size)
    }
}

pub struct ContextHeap {
    memory: GrowableContiguousMemory,
    free_ptr: Pointer,
    free_list: BTreeSet<IndexedAllocation>,
}

impl ContextHeap {
    fn deallocate(&mut self, ptr: Pointer, alloc: HeapAllocation) {
        self.memory
            .slice_mut(ptr.offset_range(alloc.size as usize))
            .fill(0);
        self.free_list.insert(IndexedAllocation {
            ptr,
            size: alloc.size,
        });
        // TODO: compact memory when possible? would require indices to be virtual?
    }

    fn get_alloc(&self, ptr: Pointer) -> HeapAllocation {
        HeapAllocation::from_memory(
            self.memory
                .slice(ptr.offset_range(size_of::<HeapAllocation>())),
        )
    }

    fn memset(&mut self, ptr: Pointer, alloc: HeapAllocation) {
        self.memory
            .slice_mut(ptr.offset_range(size_of::<HeapAllocation>()))
            .copy_from_slice(&alloc.be_bytes());
    }

    fn try_free_list(&mut self, req_size: u32) -> Option<IndexedAllocation> {
        self.free_list
            .iter()
            .find(|i| i.size <= req_size)
            .cloned()
            .and_then(|i| self.free_list.take(&i))
    }

    fn bump(&mut self, sz: u32) -> Pointer {
        let ptr = self.free_ptr;
        self.free_ptr.incr(sz);
        ptr
    }
}

impl Memory for ContextHeap {
    fn store_value(&mut self, ptr: Pointer, value: crate::Value) -> Pointer {
        self.memory.store_value(ptr, value)
    }

    fn read_value(
        &self,
        type_table: &TypeTable,
        ptr: Pointer,
        value_type: &crate::ValueType,
    ) -> crate::Value {
        self.memory.read_value(type_table, ptr, value_type)
    }
}

impl DynamicMemory for ContextHeap {
    fn new() -> Self {
        ContextHeap {
            memory: Default::default(),
            free_ptr: Pointer::new(0),
            free_list: BTreeSet::new(),
        }
    }

    fn allocate_n(&mut self, type_table: &TypeTable, type_index: TypeIndex, n: u32) -> Pointer {
        let sz = HeapAllocation::size() + type_table.get(type_index).total_size(type_table) * n;
        let alloc = HeapAllocation::new(type_index, n, sz);

        if let Some(free) = self.try_free_list(sz) {
            let residual = free.size - sz;
            if residual > 0 {
                let ptr = free.ptr.offset(sz);
                self.free_list.insert(IndexedAllocation {
                    ptr,
                    size: residual,
                });
                // TODO: this will eventually lead to fragmentation almost certainly, though it does mean
                // that heap indices are always guaranteed to be interpretable as HeapAllocations once
                // assigned
            }
            self.memset(free.ptr, alloc);
            free.ptr
        } else {
            self.memory.ensure_capacity(sz as usize);
            self.memset(self.free_ptr, alloc);
            self.bump(sz)
        }
    }

    fn add_reference(&mut self, ptr: Pointer) {
        let mut alloc = self.get_alloc(ptr);
        alloc.references.increment();
        self.memset(ptr, alloc);
    }

    fn remove_reference(&mut self, ptr: Pointer) {
        let mut alloc = self.get_alloc(ptr);
        alloc.references.decrement();
        if alloc.has_live_references() {
            self.memset(ptr, alloc);
        } else {
            self.deallocate(ptr, alloc);
        }
    }

    fn is_allocation_valid(&self, ptr: Pointer) -> bool {
        u32::from_be_bytes(self.memory.slice(ptr.offset_range(4)).try_into().unwrap()) > 0
    }
}

impl Default for ContextHeap {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{test_utils, Field, TypeDefinition, ValueType};

    #[test]
    fn test_references_new_should_create_single_reference() {
        let refs = References::new();
        assert_eq!(refs.reference_count(), 1);
    }

    #[test]
    #[should_panic(expected = "Attempted to decrement references without an existing reference!")]
    fn test_references_decrement_without_existing_heap_reference_panics() {
        let mut refs = References::new();
        refs.decrement();
        refs.decrement();
    }

    #[test]
    #[should_panic(
        expected = "Attempted to increment reference would result in reference overflow!"
    )]
    fn test_references_increment_with_max_references_panics() {
        let mut refs = References::new();
        refs.0 = u32::MAX;
        refs.increment();
    }

    #[test]
    fn test_references_increment_increases_reference_count() {
        let mut refs = References::new();
        refs.increment();
        assert_eq!(refs.reference_count(), 2);
    }

    #[test]
    fn test_references_decrement_decreases_reference_count() {
        let mut refs = References::new();
        refs.increment();
        refs.increment();
        assert_eq!(refs.reference_count(), 3);
        refs.decrement();
        assert_eq!(refs.reference_count(), 2);
        refs.increment();
        assert_eq!(refs.reference_count(), 3);
        refs.decrement();
        refs.decrement();
        refs.decrement();
        assert!(!refs.is_live());
    }

    fn setup() -> (ContextHeap, TypeTable, TypeDefinition) {
        let type_table = TypeTable::new();
        let ctx_heap = ContextHeap::new();
        let defn = test_utils::create_type_definition("HeapType");
        (ctx_heap, type_table, defn)
    }

    #[test]
    fn test_context_heap_can_allocate() {
        let (mut ctx_heap, mut type_table, type_defn) = setup();
        let type_idx = type_table.insert(type_defn);
        let idx = ctx_heap.allocate(&type_table, type_idx);
        assert_eq!(idx, Pointer::new(0));

        let alloc = ctx_heap.get_alloc(idx);
        assert_eq!(alloc.size, 16);
    }

    #[test]
    fn test_context_heap_is_allocation_valid_with_live_allocation_should_return_true() {
        let (mut ctx_heap, mut type_table, type_defn) = setup();
        let type_idx = type_table.insert(type_defn);
        let idx = ctx_heap.allocate(&type_table, type_idx);
        assert!(ctx_heap.is_allocation_valid(idx));
    }

    #[test]
    fn test_context_heap_is_allocation_valid_without_live_allocation_should_return_false() {
        let (mut ctx_heap, mut type_table, type_defn) = setup();
        let type_idx = type_table.insert(type_defn);
        let idx = ctx_heap.allocate(&type_table, type_idx);
        ctx_heap.remove_reference(idx);
        assert!(!ctx_heap.is_allocation_valid(idx));
    }

    #[test]
    fn test_context_heap_can_allocate_multiple() {
        let (mut ctx_heap, mut type_table, type_defn) = setup();
        let type_idx = type_table.insert(type_defn);
        let idx = ctx_heap.allocate_n(&type_table, type_idx, 5);
        assert_eq!(idx, Pointer::new(0));

        let alloc = ctx_heap.get_alloc(idx);
        assert_eq!(alloc.size, 16);
    }

    #[test]
    fn test_context_heap_allocate_with_single_field_allocates_correct_size() {
        let (mut ctx_heap, mut type_table, mut type_defn) = setup();
        type_defn.add_field(&type_table, Field::new("field".to_string(), ValueType::U64));
        let type_idx = type_table.insert(type_defn);
        let idx = ctx_heap.allocate_n(&type_table, type_idx, 3);

        let alloc = ctx_heap.get_alloc(idx);
        assert_eq!(alloc.size, 40);
    }

    #[test]
    fn test_context_heap_allocate_with_multiple_fields_allocates_correct_size() {
        let (mut ctx_heap, mut type_table, mut type_defn) = setup();
        type_defn.add_field(&type_table, Field::new("field".to_string(), ValueType::U64));
        type_defn.add_field(
            &type_table,
            Field::new("field2".to_string(), ValueType::U64),
        );
        let type_idx = type_table.insert(type_defn);
        let idx = ctx_heap.allocate_n(&type_table, type_idx, 3);

        let alloc = ctx_heap.get_alloc(idx);
        assert_eq!(alloc.size, 64);
    }

    #[test]
    fn test_context_heap_should_add_allocation_to_free_list_after_all_references_die() {
        let (mut ctx_heap, mut type_table, type_defn) = setup();
        let type_idx = type_table.insert(type_defn);
        let idx = ctx_heap.allocate(&type_table, type_idx);
        // TODO: for cases with no fields in the type (just testing allocation), setup can be factored to a single call
        ctx_heap.remove_reference(idx);
        assert_eq!(ctx_heap.free_list.len(), 1);
    }
}
