use generational_arena::Arena;

use crate::constant_pool::ConstantPool;
use crate::function::{Function, FunctionIndex, FunctionTable, InstructionPointer};
use crate::instruction::Opcode;
use crate::util::stack::Stack;
use crate::value::Value;

#[derive(Debug, Default, Clone, Copy)]
struct LocalAddress(usize);

struct Frame {
    ip: InstructionPointer,
    locals: LocalAddress,
    function: FunctionIndex,
}

impl Frame {
    pub fn new(locals: LocalAddress, function: FunctionIndex) -> Self {
        Frame {
            ip: InstructionPointer::new(),
            locals,
            function,
        }
    }
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

    pub fn push(&mut self, frame: Frame) {
        self.frames.push(frame);
    }

    pub fn peek(&mut self) -> &mut Frame {
        self.frames.peek_mut()
    }

    pub fn pop(&mut self) {
        self.frames.pop();
    }
}

struct MetaInformation {}

struct DebugInformation {}

struct Heap {}

pub struct ExecutionContext {
    constants: ConstantPool,
    data: Stack<Value>,
    callstack: Callstack,
    meta: MetaInformation,
    heap: Heap,
    debug: Option<DebugInformation>,
}

impl ExecutionContext {
    pub fn new() -> Self {
        Self {
            constants: Default::default(),
            data: Stack::new(),
            callstack: Callstack::new(),
            meta: MetaInformation {},
            heap: Heap {},
            debug: None,
        }
    }

    pub fn constant_pool(&mut self) -> &mut ConstantPool {
        &mut self.constants
    }

    pub fn run(&mut self, function_table: &FunctionTable, entrypoint: &str) {
        self.callstack.push(Frame::new(
            LocalAddress(0),
            function_table.address_of(entrypoint),
        ));
        while {
            // TODO: need a more efficient way to keep track of the functions, can they
            // somehow be embedded directly within the frames?
            let frame = self.callstack.peek();
            let func = function_table.get(frame.function);
            let inst = func.next_instruction(&mut frame.ip);
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
