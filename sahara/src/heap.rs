use std::fmt::Display;

use crate::util::index::TypeIndex;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct HeapIndex(usize);

impl Display for HeapIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl HeapIndex {
    pub fn be_bytes(&self) -> [u8; 8] {
        self.0.to_be_bytes()
    }
}

#[derive(Debug)]
struct HeapAllocation {
    type_index: TypeIndex,
    references: u32,
    num: usize,
}

impl HeapAllocation {
    fn be_bytes(&self) -> [u8; 16] {
        let mut bytes = [0u8; 16];
        bytes[..4].copy_from_slice(&self.type_index.be_bytes());
        bytes[4..8].copy_from_slice(&self.references.to_be_bytes());
        bytes[8..].copy_from_slice(&self.num.to_be_bytes());
        bytes
    }
}

// TODO: is there a way to make it type-safe that a heap can only be used by a single context?
pub struct ContextHeap {
    memory: Vec<u8>,
}
