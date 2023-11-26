mod constant_pool;
mod data_type;
mod execution_context;
mod function;
mod instruction;
mod local;
mod memory;
mod module_registry;
mod util;
mod value;
mod vm;

// TODO: restructure exports so that everything isn't exposed at the top level
pub use constant_pool::ConstantPool;
pub use data_type::{Field, TypeDefinition, TypeId, TypeTable};
pub use execution_context::ExecutionContext;
pub use function::{Function, FunctionId, FunctionTable};
pub use instruction::Instruction;
pub use local::LocalSlots;
pub use module_registry::{ModuleName, ModuleRegistry};
pub use util::index::FunctionIndex;
pub use value::{Value, ValueType};
pub use vm::VirtualMachine;

#[cfg(test)]
mod test_utils;
