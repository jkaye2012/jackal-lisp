use std::fmt::Display;

// TODO: macros for index newtypes to reduce duplication?

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct InstructionIndex(u32);

impl From<usize> for InstructionIndex {
    fn from(value: usize) -> Self {
        InstructionIndex::new(value.try_into().expect("Index exceeded max size"))
    }
}

impl From<u32> for InstructionIndex {
    fn from(value: u32) -> Self {
        InstructionIndex::new(value)
    }
}

impl From<InstructionIndex> for usize {
    fn from(value: InstructionIndex) -> Self {
        value.0 as usize
    }
}

impl InstructionIndex {
    pub fn new(idx: u32) -> Self {
        InstructionIndex(idx & 0xFFFFFF)
    }
}

impl Display for InstructionIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#06x}", self.0)
    }
}

macro_rules! make_index {
    ($t:ident) => {
        #[derive(Debug, PartialEq, Eq, Clone, Copy)]
        pub struct $t(crate::util::index::InstructionIndex);

        impl $t {
            pub fn be_bytes(&self) -> [u8; 4] {
                self.0 .0.to_be_bytes()
            }
        }

        impl From<u32> for $t {
            fn from(value: u32) -> Self {
                $t(value.into())
            }
        }

        impl From<usize> for $t {
            fn from(value: usize) -> Self {
                $t(value.into())
            }
        }

        impl From<$t> for crate::util::index::InstructionIndex {
            fn from(value: $t) -> Self {
                value.0
            }
        }

        impl From<$t> for usize {
            fn from(value: $t) -> Self {
                value.0.into()
            }
        }

        impl std::fmt::Display for $t {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
}

make_index!(TypeIndex);
make_index!(LocalIndex);
make_index!(FunctionIndex);
make_index!(ConstantIndex);
