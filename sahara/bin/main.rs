use sahara::{
    ConstantPool, ExecutionContext, Function, FunctionId, FunctionTable, Instruction,
    ModuleRegistry, VirtualMachine,
};

fn one_plus_one(pool: &mut ConstantPool) -> Function {
    let instructions = vec![
        Instruction::constant(pool.add_u64(100)),
        Instruction::local_store(),
        Instruction::local_read(0.into()),
        Instruction::constant(pool.add_u64(1)),
        Instruction::add(),
        Instruction::print(),
        Instruction::ret(),
    ];
    Function::from_instructions(instructions)
}

fn main() {
    let mut modules = ModuleRegistry::new();
    let module_name = modules.register("main".to_string());
    let context = ExecutionContext::new();
    let mut pool = ConstantPool::default();
    let func = one_plus_one(&mut pool);
    let mut table = FunctionTable::new();
    let func_idx = table.insert(module_name.function_id("one_plus_one"), func);
    let instructions = vec![
        Instruction::constant(pool.add_u64(200)),
        Instruction::local_store(),
        Instruction::constant(pool.add_u64(2)),
        Instruction::constant(pool.add_u64(2)),
        Instruction::add(),
        Instruction::constant(pool.add_u64(3)),
        Instruction::mul(),
        Instruction::print(),
        Instruction::call(func_idx),
        Instruction::halt(),
    ];
    let entrypoint = Function::from_instructions(instructions);
    table.insert(module_name.function_id("main"), entrypoint);
    let mut vm = VirtualMachine::new(context, table, pool);
    vm.run();
}
