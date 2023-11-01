use sahara::{
    ConstantPool, ExecutionContext, Function, FunctionId, FunctionIndex, FunctionTable,
    Instruction, LocalSlots, ModuleRegistry, VirtualMachine,
};

fn one_plus_one(
    pool: &mut ConstantPool,
    table: &mut FunctionTable,
    id: FunctionId,
) -> FunctionIndex {
    let instructions = vec![
        Instruction::constant(pool.add_u64(100)),
        Instruction::local_store(),
        Instruction::local_read(0.into()),
        Instruction::constant(pool.add_u64(1)),
        Instruction::add(),
        Instruction::print(),
        Instruction::ret(),
    ];
    let mut locals = LocalSlots::new();
    locals.add_slot(sahara::ValueType::U64);
    table.insert(id, instructions, locals)
}

fn main() {
    let mut modules = ModuleRegistry::new();
    let module_name = modules.register("main".to_string());
    let context = ExecutionContext::new();
    let mut table = FunctionTable::new();
    let onepone = module_name.function_id("one_plus_one");
    let mut pool = ConstantPool::default();
    let func_idx = one_plus_one(&mut pool, &mut table, onepone);
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
    let mut locals = LocalSlots::new();
    locals.add_slot(sahara::ValueType::U64);
    let main = module_name.function_id("main");
    let main_idx = table.insert(main, instructions, locals);
    let mut vm = VirtualMachine::new(context, table, pool);
    vm.run(main_idx);
}
