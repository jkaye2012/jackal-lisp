use generational_arena::Arena;

use crate::constant_pool::ConstantPool;
use crate::function::{Function, FunctionIndex};
use crate::instruction::Opcode;
use crate::program::{InstructionAddress, Program};
use crate::util::stack::Stack;
use crate::value::Value;

#[derive(Debug, Default, Clone, Copy)]
struct LocalAddress(usize);

struct Frame {
    ip: InstructionAddress,
    locals: LocalAddress,
    function: FunctionIndex,
}

struct Callstack {
    frames: Stack<Frame>,
    locals: Vec<Value>,
}

impl Callstack {
    pub fn new() -> Self {
        Callstack {
            frames: Stack::new(),
            locals: Vec::new(),
        }
    }
}

struct MetaInformation {}

struct DebugInformation {}

struct Heap {}

pub struct ExecutionContext {
    program: Program,
    constants: ConstantPool,
    data: Stack<Value>,
    callstack: Callstack,
    meta: MetaInformation,
    heap: Heap,
    debug: Option<DebugInformation>,
}

impl ExecutionContext {
    pub fn new(program: Program) -> Self {
        Self {
            program,
            constants: Default::default(),
            data: Stack::new(),
            callstack: Callstack::new(),
            meta: MetaInformation {},
            heap: Heap {},
            debug: None,
        }
    }

    pub fn program(&mut self) -> &mut Program {
        &mut self.program
    }

    pub fn constant_pool(&mut self) -> &mut ConstantPool {
        &mut self.constants
    }

    pub fn run(&mut self) {
        while {
            let (addr, inst) = self.program.step();
            match inst.op() {
                Opcode::ConstU64 => {
                    let value = self.constant_pool().get(inst.into());
                    self.data.push(value);
                }
                Opcode::Add => {
                    let a = self.data.pop();
                    let b = self.data.pop();
                    self.data.push(a + b);
                }
                Opcode::Print => {
                    let val = self.data.pop();
                    dbg!(val);
                }
                Opcode::Halt => {}
                _ => panic!("opcode not yet implemented"),
            };
            inst.op() != Opcode::Halt
        } {}
    }
}
