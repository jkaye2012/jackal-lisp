use sahara::{
    ConstantPool, ExecutionContext, Function, FunctionId, FunctionTable, Instruction,
    VirtualMachine,
};

fn one_plus_one(pool: &mut ConstantPool) -> Function {
    let instructions = vec![
        Instruction::const_u64(pool.add_u64(1)),
        Instruction::const_u64(pool.add_u64(1)),
        Instruction::add(),
        Instruction::print(),
        Instruction::ret(),
    ];
    Function::from_instructions(instructions)
}

fn main() {
    let context = ExecutionContext::new();
    let mut pool = ConstantPool::default();
    let func = one_plus_one(&mut pool);
    let mut table = FunctionTable::new();
    let func_idx = table.insert(FunctionId::from_fq_name("one_plus_one".to_string()), func);
    let instructions = vec![
        Instruction::const_u64(pool.add_u64(2)),
        Instruction::const_u64(pool.add_u64(2)),
        Instruction::add(),
        Instruction::print(),
        Instruction::call(func_idx),
        Instruction::halt(),
    ];
    let entrypoint = Function::from_instructions(instructions);
    table.insert(FunctionId::from_fq_name("main".to_string()), entrypoint);
    let mut vm = VirtualMachine::new(context, table, pool);
    vm.run();
}
