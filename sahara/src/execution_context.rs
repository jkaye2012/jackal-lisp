use crate::constant_pool::ConstantPool;
use crate::function::{FunctionIndex, FunctionTable, InstructionPointer};
use crate::instruction::Opcode;
use crate::local::{LocalAddress, LocalIndex, Locals};
use crate::util::stack::Stack;
use crate::value::Value;

struct Frame {
    ip: InstructionPointer,
    begin: LocalAddress,
    end: LocalAddress,
    function: FunctionIndex,
}

impl Frame {
    pub fn new(locals: LocalAddress, function: FunctionIndex) -> Self {
        Frame {
            ip: InstructionPointer::new(),
            begin: locals,
            end: locals,
            function,
        }
    }

    pub fn next_local(&mut self) -> LocalAddress {
        let curr = self.end;
        self.end.increment();
        curr
    }

    pub fn local_address(&self, idx: LocalIndex) -> LocalAddress {
        self.begin.relative_to(idx)
    }
}

struct Callstack {
    frames: Stack<Frame>,
}

impl Callstack {
    pub fn new() -> Self {
        Callstack {
            frames: Stack::new(),
        }
    }

    pub fn initialize(&mut self, entrypoint: FunctionIndex) -> &mut Frame {
        self.frames
            .push(Frame::new(LocalAddress::new(), entrypoint));
        self.frames.peek_mut()
    }

    pub fn push(&mut self, func: FunctionIndex) -> &mut Frame {
        let current_frame = self.frames.peek();
        self.frames.push(Frame::new(current_frame.end, func));
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
    locals: Locals,
    _meta: MetaInformation,
    _heap: Heap,
    _debug: Option<DebugInformation>,
}

impl ExecutionContext {
    pub fn new() -> Self {
        Self {
            data: Stack::new(),
            callstack: Callstack::new(),
            locals: Locals::new(),
            _meta: MetaInformation {},
            _heap: Heap {},
            _debug: None,
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
                    func = function_table.get(frame.function);
                }
                Opcode::Print => {
                    let val = self.data.pop();
                    dbg!(val);
                }
                Opcode::LocalStore => {
                    let addr = frame.next_local();
                    let value = self.data.pop();
                    self.locals.store_local(addr, value);
                }
                Opcode::LocalRead => {
                    let idx = inst.local_index();
                    let value = self.locals.read_local(frame.local_address(idx));
                    self.data.push(value);
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
