use sahara::{
    ConstantPool, ExecutionContext, FunctionId, FunctionIndex, FunctionTable, Instruction,
    LocalSlots, ModuleRegistry, TypeTable, VirtualMachine,
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
    let mut function_table = FunctionTable::new();
    let mut type_table = TypeTable::new();
    let onepone = module_name.function_id("one_plus_one");
    let mut pool = ConstantPool::default();
    let func_idx = one_plus_one(&mut pool, &mut function_table, onepone);
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
    let main_idx = function_table.insert(main, instructions, locals);
    let mut vm = VirtualMachine::new(context, function_table, pool, type_table);
    vm.run(main_idx);
}
