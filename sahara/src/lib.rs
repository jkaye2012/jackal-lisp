mod constant_pool;
mod execution_context;
mod function;
mod instruction;
mod local;
mod module_registry;
mod util;
mod value;
mod vm;

pub use constant_pool::ConstantPool;
pub use execution_context::ExecutionContext;
pub use function::{Function, FunctionId, FunctionIndex, FunctionTable};
pub use instruction::Instruction;
pub use local::LocalSlots;
pub use module_registry::ModuleRegistry;
pub use value::ValueType;
pub use vm::VirtualMachine;
