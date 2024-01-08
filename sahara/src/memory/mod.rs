mod common;
pub mod dynamic_mem;
pub mod static_mem;

pub use common::{DynamicMemory, Memory, Pointer, StorageResult};
pub use dynamic_mem::ContextHeap;
pub use static_mem::StaticMemory;
