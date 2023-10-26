#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct InstructionIndex(usize);

impl From<usize> for InstructionIndex {
    fn from(value: usize) -> Self {
        InstructionIndex::new(value)
    }
}

impl From<InstructionIndex> for usize {
    fn from(value: InstructionIndex) -> Self {
        value.0
    }
}

impl InstructionIndex {
    pub fn new(idx: usize) -> Self {
        InstructionIndex(idx & 0x00FFFFFF)
    }
}
