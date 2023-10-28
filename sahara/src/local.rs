use crate::{util::index::InstructionIndex, value::Value};

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct LocalAddress(usize);

impl LocalAddress {
    pub fn new() -> Self {
        LocalAddress(0)
    }

    pub fn increment(&mut self) {
        self.0 += 1;
    }

    pub fn relative_to(&self, idx: LocalIndex) -> LocalAddress {
        let sz: usize = idx.into();
        LocalAddress(self.0 + sz)
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
    locals: Vec<Value>,
}

impl Locals {
    pub fn new() -> Self {
        Locals { locals: Vec::new() }
    }

    pub fn store_local(&mut self, addr: LocalAddress, value: Value) {
        self.locals.insert(addr.0, value);
    }

    pub fn read_local(&self, addr: LocalAddress) -> Value {
        self.locals[addr.0]
    }
}
