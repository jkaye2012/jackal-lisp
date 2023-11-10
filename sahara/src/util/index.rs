use std::fmt::Display;

// TODO: macros for index newtypes to reduce duplication?

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct InstructionIndex(usize);

impl From<usize> for InstructionIndex {
    fn from(value: usize) -> Self {
        InstructionIndex::new(value)
    }
}

impl From<u32> for InstructionIndex {
    fn from(value: u32) -> Self {
        InstructionIndex::new(value as usize)
    }
}

impl From<InstructionIndex> for usize {
    fn from(value: InstructionIndex) -> Self {
        value.0
    }
}

impl InstructionIndex {
    pub fn new(idx: usize) -> Self {
        InstructionIndex(idx & 0xFFFFFF)
    }
}

impl Display for InstructionIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#06x}", self.0)
    }
}
