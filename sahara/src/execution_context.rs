use crate::constant_pool::ConstantPool;
use crate::function::{FunctionIndex, FunctionTable, InstructionPointer};
use crate::instruction::Opcode;
use crate::util::stack::Stack;
use crate::value::Value;

#[derive(Debug, Default, Clone, Copy)]
struct LocalAddress(usize);

struct Frame {
    ip: InstructionPointer,
    local_offset: LocalAddress,
    num_locals: u32,
    function: FunctionIndex,
}

impl Frame {
    pub fn new(locals: LocalAddress, function: FunctionIndex) -> Self {
        Frame {
            ip: InstructionPointer::new(),
            local_offset: locals,
            num_locals: 0,
            function,
        }
    }

    pub fn next_offset(&self) -> LocalAddress {
        LocalAddress(self.local_offset.0 + self.num_locals as usize)
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

    pub fn initialize(&mut self, entrypoint: FunctionIndex) -> &mut Frame {
        self.frames.push(Frame::new(LocalAddress(0), entrypoint));
        self.frames.peek_mut()
    }

    pub fn push(&mut self, func: FunctionIndex) -> &mut Frame {
        let current_frame = self.frames.peek();
        self.frames
            .push(Frame::new(current_frame.next_offset(), func));
        self.frames.peek_mut()
    }

    pub fn pop(&mut self) -> &mut Frame {
        self.frames.pop();
        self.frames.peek_mut()
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
        let entrypoint_index = function_table.address_of(entrypoint);
        let mut frame = self.callstack.initialize(entrypoint_index);
        let mut func = function_table.get(entrypoint_index);
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
                Opcode::Call => {
                    let idx = inst.function_index();
                    func = function_table.get(idx);
                    frame = self.callstack.push(idx);
                }
                Opcode::Return => {
                    frame = self.callstack.pop();
                    func = function_table.get(frame.function); // TODO: seems silly, find a better way
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
