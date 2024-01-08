use crate::function::InstructionPointer;
use crate::instruction::Opcode;
use crate::memory::DynamicMemory;
use crate::memory::{Memory, Pointer, StaticMemory};
use crate::util::index::{FunctionIndex, LocalIndex};
use crate::util::stack::Stack;
use crate::value::Value;
use crate::vm::GlobalContext;
use crate::{Function, Instruction, TypeTable, ValueType};

struct Frame {
    ip: InstructionPointer,
    locals_begin: Pointer,
    locals_end: Pointer,
    function: FunctionIndex,
}

impl Frame {
    pub fn new(type_table: &TypeTable, locals: Pointer, function: &Function) -> Self {
        Frame {
            ip: InstructionPointer::new(),
            locals_begin: locals,
            locals_end: function.local_slots().allocate(type_table, locals),
            function: function.index(),
        }
    }

    // TODO: consider law of demeter, should function hide local slots?

    pub fn local_info(&self, function: &Function, idx: LocalIndex) -> (ValueType, Pointer) {
        function.local_slots().slot_info(idx, self.locals_begin)
    }

    pub fn deallocate<Heap>(&mut self, function: &Function, heap: &mut Heap)
    where
        Heap: DynamicMemory,
    {
        for ptr in function.heap_references(self.locals_begin) {
            heap.remove_reference(ptr);
        }
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
            .push(Frame::new(type_table, Pointer::default(), entrypoint));
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

pub struct ExecutionContext<Heap: DynamicMemory> {
    data: Stack<Value>,
    callstack: Callstack,
    extensions: Stack<Instruction>,
    locals: StaticMemory,
    _meta: MetaInformation,
    heap: Heap,
    _debug: Option<DebugInformation>,
}

macro_rules! store_value {
    ($locals:expr, $heap: expr, $ptr:ident, $value:ident) => {{
        let result = $locals.store_value($ptr, $value);
        if let Some((prev, new)) = result.allocations() {
            $heap.replace_reference(prev, new);
        }
        result.end()
    }};
    ($heap: expr, $ptr:ident, $value:ident) => {{
        let result = $heap.store_value($ptr, $value);
        if let Some((prev, new)) = result.allocations() {
            $heap.replace_reference(prev, new);
        }
        result.end()
    }};
}

impl<Heap: DynamicMemory> ExecutionContext<Heap> {
    pub fn new() -> Self {
        Self {
            data: Stack::new(),
            callstack: Callstack::new(),
            extensions: Stack::new(),
            locals: Default::default(),
            _meta: MetaInformation {},
            heap: Heap::default(),
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
                    self.locals.zero(frame.locals_begin, frame.locals_end);
                    frame.deallocate(func, &mut self.heap);
                    frame = self.callstack.pop();
                    func = global_context.function_table().get(frame.function);
                }
                Opcode::Print => {
                    let val = self.data.pop();
                    dbg!(val);
                }
                Opcode::LocalStore => {
                    let idx = inst.local_index();
                    let (_, ptr) = frame.local_info(func, idx);
                    let value = self.data.pop();
                    store_value!(self.locals, self.heap, ptr, value);
                }
                Opcode::LocalRead => {
                    let idx = inst.local_index();
                    let (value_type, ptr) = frame.local_info(func, idx);
                    let value =
                        self.locals
                            .read_value(global_context.type_table(), ptr, &value_type);
                    self.data.push(value);
                }
                Opcode::DataTypeCreate => {
                    let local_idx = inst.local_index();
                    let (value_type, mut ptr) = frame.local_info(func, local_idx);
                    let type_definition = global_context.type_table().get(value_type.type_index());
                    for _ in 0..type_definition.num_fields() {
                        let value = self.data.pop();
                        ptr = store_value!(self.locals, self.heap, ptr, value);
                    }
                }
                Opcode::DataTypeReadField => {
                    let local_idx = inst.local_index();
                    let (dt_value_type, dt_addr) = frame.local_info(func, local_idx);
                    let type_definition =
                        global_context.type_table().get(dt_value_type.type_index());
                    let field_idx = self.extensions.pop().instruction_index();
                    let (field_type, field_ptr) = type_definition.field_pointer(dt_addr, field_idx);
                    let value =
                        self.locals
                            .read_value(global_context.type_table(), field_ptr, &field_type);
                    self.data.push(value);
                }
                Opcode::DataTypeSetField => {
                    let local_idx = inst.local_index();
                    let (dt_value_type, dt_ptr) = frame.local_info(func, local_idx);
                    let type_definition =
                        global_context.type_table().get(dt_value_type.type_index());
                    let field_idx = self.extensions.pop().instruction_index();
                    let value = self.data.pop();
                    let (_, field_ptr) = type_definition.field_pointer(dt_ptr, field_idx);
                    store_value!(self.locals, self.heap, field_ptr, value);
                }
                Opcode::HeapAlloc => {
                    // TODO: separate stack from heap pointers for type safety? Almost bit me
                    let local_idx = inst.local_index();
                    let (value_type, stack_ptr) = frame.local_info(func, local_idx);
                    let mut ptr = self
                        .heap
                        .allocate(global_context.type_table(), value_type.type_index());
                    let res = Value::HeapData(ptr);
                    store_value!(self.locals, self.heap, stack_ptr, res);
                    let type_definition = global_context.type_table().get(value_type.type_index());
                    for _ in 0..type_definition.num_fields() {
                        let value = self.data.pop();
                        ptr = store_value!(self.heap, ptr, value);
                    }
                    self.data.push(res);
                }
                Opcode::HeapStore => {
                    let field_idx = inst.instruction_index();
                    let value = self.data.pop();
                    let ptr = self.data.pop().pointer();
                    let type_definition = self.heap.type_of(global_context.type_table(), ptr);
                    let (_, field_ptr) = type_definition.field_pointer(ptr, field_idx);
                    store_value!(self.heap, field_ptr, value);
                    self.data.push(value);
                }
                Opcode::HeapRead => {
                    let field_idx = inst.instruction_index();
                    let ptr = self.data.pop().pointer();
                    let type_definition = self.heap.type_of(global_context.type_table(), ptr);
                    let (value_type, field_ptr) = type_definition.field_pointer(ptr, field_idx);
                    let value =
                        self.heap
                            .read_value(global_context.type_table(), field_ptr, &value_type);
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

impl<Heap: DynamicMemory> Default for ExecutionContext<Heap> {
    fn default() -> Self {
        Self::new()
    }
}
