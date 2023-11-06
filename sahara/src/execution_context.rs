use crate::function::{FunctionIndex, InstructionPointer};
use crate::instruction::Opcode;
use crate::local::{LocalAddress, LocalIndex, Locals};
use crate::util::stack::Stack;
use crate::value::Value;
use crate::vm::GlobalContext;
use crate::{Function, Instruction, TypeTable, ValueType};

struct Frame {
    ip: InstructionPointer,
    locals_begin: LocalAddress,
    locals_end: LocalAddress,
    function: FunctionIndex,
}

impl Frame {
    pub fn new(type_table: &TypeTable, locals: LocalAddress, function: &Function) -> Self {
        Frame {
            ip: InstructionPointer::new(),
            locals_begin: locals,
            locals_end: function.local_slots().allocate(type_table, locals),
            function: function.index(),
        }
    }

    pub fn local_info(&self, function: &Function, idx: LocalIndex) -> (ValueType, LocalAddress) {
        function.local_slots().slot_info(idx, self.locals_begin)
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

    pub fn initialize(&mut self, type_table: &TypeTable, entrypoint: &Function) -> &mut Frame {
        self.frames
            .push(Frame::new(type_table, LocalAddress::new(), entrypoint));
        self.frames.peek_mut()
    }

    pub fn push(&mut self, type_table: &TypeTable, func: &Function) -> &mut Frame {
        let current_frame = self.frames.peek();
        self.frames
            .push(Frame::new(type_table, current_frame.locals_end, func));
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
    extensions: Stack<Instruction>,
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
            extensions: Stack::new(),
            locals: Locals::new(),
            _meta: MetaInformation {},
            _heap: Heap {},
            _debug: None,
        }
    }

    pub fn run(&mut self, global_context: &GlobalContext, entrypoint_index: FunctionIndex) {
        let entrypoint = global_context.function_table().get(entrypoint_index);
        let mut frame = self
            .callstack
            .initialize(global_context.type_table(), entrypoint);
        let mut func = entrypoint;
        while {
            let inst = func.next_instruction(&mut frame.ip);
            match inst.op() {
                Opcode::Halt => {}
                Opcode::Add => {
                    let a = self.data.pop();
                    let b = self.data.pop();
                    self.data.push(a + b);
                }
                Opcode::Sub => {
                    let a = self.data.pop();
                    let b = self.data.pop();
                    self.data.push(a - b);
                }
                Opcode::Mul => {
                    let a = self.data.pop();
                    let b = self.data.pop();
                    self.data.push(a * b);
                }
                Opcode::Div => {
                    let a = self.data.pop();
                    let b = self.data.pop();
                    self.data.push(a / b);
                }
                Opcode::Call => {
                    let idx = inst.function_index();
                    func = global_context.function_table().get(idx);
                    frame = self.callstack.push(global_context.type_table(), func);
                }
                Opcode::Return => {
                    frame = self.callstack.pop();
                    func = global_context.function_table().get(frame.function);
                }
                Opcode::Print => {
                    let val = self.data.pop();
                    dbg!(val);
                }
                Opcode::LocalStore => {
                    let idx = inst.local_index();
                    let (_, addr) = frame.local_info(func, idx);
                    let value = self.data.pop();
                    self.locals
                        .store_local(global_context.type_table(), addr, value);
                }
                Opcode::LocalRead => {
                    let idx = inst.local_index();
                    let (value_type, addr) = frame.local_info(func, idx);
                    let value =
                        self.locals
                            .read_local(global_context.type_table(), addr, &value_type);
                    self.data.push(value);
                }
                Opcode::DataTypeCreate => {
                    let local_idx = inst.local_index();
                    let (value_type, mut addr) = frame.local_info(func, local_idx);
                    let type_definition = global_context.type_table().get(value_type.type_index());
                    for _ in 0..type_definition.num_fields() {
                        let value = self.data.pop();
                        addr = self
                            .locals
                            .store_local(global_context.type_table(), addr, value);
                    }
                }
                Opcode::DataTypeReadField => {
                    let local_idx = inst.local_index();
                    let (dt_value_type, dt_addr) = frame.local_info(func, local_idx);
                    let type_definition =
                        global_context.type_table().get(dt_value_type.type_index());
                    let field_idx = self.extensions.pop().abc();
                    let (field_type, field_addr) = type_definition.read_field(dt_addr, field_idx);
                    let value = self.locals.read_local(
                        global_context.type_table(),
                        field_addr,
                        &field_type,
                    );
                    self.data.push(value);
                }
                Opcode::Extend => {
                    self.extensions.push(inst);
                }
                Opcode::ImmI16 => {
                    self.data.push(Value::I16(inst.i16()));
                }
                Opcode::ImmI8 => {
                    self.data.push(Value::I8(inst.i8()));
                }
                Opcode::ImmU16 => {
                    self.data.push(Value::U16(inst.u16()));
                }
                Opcode::ImmU8 => {
                    self.data.push(Value::U8(inst.u8()));
                }
                Opcode::ImmChar => {
                    self.data.push(Value::Char(inst.char()));
                }
                Opcode::ImmBool => {
                    self.data.push(Value::Bool(inst.bool()));
                }
                Opcode::Const => {
                    let value = global_context.constant_pool().get(inst.into());
                    self.data.push(value);
                }
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
