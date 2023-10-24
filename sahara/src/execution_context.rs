use generational_arena::Arena;

use crate::instruction::Instruction;
use crate::util::stack::Stack;
use crate::value::Value;

struct InstructionAddress(usize);

struct Program {
    instructions: Vec<Instruction>,
}

struct Callstack {}

struct MetaInformation {}

struct DebugInformation {}

struct Heap {
    arena: Arena<Value>,
}

pub struct ExecutionContext {
    program: Program,
    data: Stack<Value>,
    callstack: Callstack,
    meta: MetaInformation,
    heap: Heap,
    debug: Option<DebugInformation>,
}
