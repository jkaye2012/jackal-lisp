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
    data: Stack<Value>,
    callstack: Callstack,
    meta: MetaInformation,
    heap: Heap,
    debug: Option<DebugInformation>,
}

impl ExecutionContext {
    pub fn new() -> Self {
        Self {
            data: Stack::new(),
            callstack: Callstack::new(),
            meta: MetaInformation {},
            heap: Heap {},
            debug: None,
        }
    }

    // TODO: does this need to be moved into the VM somehow? How should execution contexts
    // be interacted with?
    pub fn run(
        &mut self,
        constants: &ConstantPool,
        function_table: &FunctionTable,
        entrypoint: &str,
    ) {
        self.callstack.push(Frame::new(
            LocalAddress(0),
            function_table.address_of(entrypoint),
        ));
        let mut frame = self.callstack.peek();
        let mut func = function_table.get(frame.function);
        while {
            let inst = func.next_instruction(&mut frame.ip);
            match inst.op() {
                Opcode::ConstU64 => {
                    let value = constants.get(inst.into());
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

impl Default for ExecutionContext {
    fn default() -> Self {
        Self::new()
    }
}
