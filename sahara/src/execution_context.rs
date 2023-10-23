use generational_arena::Arena;

use crate::instruction::Instruction;
use crate::util::stack::Stack;
use crate::value::Value;

struct Location(u64);

struct Program {
    instructions: Vec<Instruction>,
}

struct MetaInformation {}

struct DebugInformation {}

struct Heap {
    arena: Arena<Value>,
}

pub struct ExecutionContext {
    program: Program,
    data: Stack<Value>,
    callstack: Stack<Location>,
    meta: MetaInformation,
    heap: Heap,
    debug: Option<DebugInformation>,
}
