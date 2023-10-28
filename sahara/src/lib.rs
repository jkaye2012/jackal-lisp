mod constant_pool;
mod execution_context;
mod function;
mod instruction;
mod util;
mod value;
mod vm;

pub use constant_pool::ConstantPool;
pub use execution_context::ExecutionContext;
pub use function::{Function, FunctionId, FunctionTable};
pub use instruction::Instruction;
pub use vm::VirtualMachine;
