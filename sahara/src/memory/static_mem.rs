use crate::data_type::TypeTable;
use crate::value::{Value, ValueType};

use super::common::{GrowableContiguousMemory, StorageResult};
use super::{Memory, Pointer};

#[derive(Default)]
pub struct StaticMemory {
    memory: GrowableContiguousMemory, // TODO: should be able to configured static local memory if desired
}

impl Memory for StaticMemory {
    fn store_value(&mut self, ptr: Pointer, value: Value) -> StorageResult {
        self.memory.store_value(ptr, value)
    }

    fn read_value(&self, type_table: &TypeTable, ptr: Pointer, value_type: &ValueType) -> Value {
        self.memory.read_value(type_table, ptr, value_type)
    }

    fn zero(&mut self, from: Pointer, to: Pointer) {
        self.memory.zero(from, to);
    }
}
