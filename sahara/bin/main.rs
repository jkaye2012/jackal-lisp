use sahara::{
    ConstantPool, ExecutionContext, Field, FunctionId, FunctionIndex, FunctionTable, Instruction,
    LocalSlots, ModuleName, ModuleRegistry, TypeDefinition, TypeId, TypeTable, ValueType,
    VirtualMachine,
};

fn one_plus_one(
    pool: &mut ConstantPool,
    function_table: &mut FunctionTable,
    type_table: &TypeTable,
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
    locals.add_slot(type_table, sahara::ValueType::U64);
    function_table.insert(id, instructions, locals)
}

fn mk_type(type_table: &mut TypeTable, module_name: &ModuleName) {
    let type_id = TypeId::new(module_name, "TestType");
    let mut type_defn = TypeDefinition::new(type_id.clone()); // TODO: don't force clone
    type_defn.add_field(type_table, Field::new("red".to_string(), ValueType::U8));
    type_defn.add_field(type_table, Field::new("green".to_string(), ValueType::U8));
    type_defn.add_field(type_table, Field::new("blue".to_string(), ValueType::U8));
    type_table.insert(type_id, type_defn);
}

fn main() {
    let mut modules = ModuleRegistry::new();
    let module_name = modules.register("main".to_string());
    let context = ExecutionContext::new();
    let mut function_table = FunctionTable::new();
    let onepone = module_name.function_id("one_plus_one");
    let mut pool = ConstantPool::default();
    let mut type_table = TypeTable::new();
    mk_type(&mut type_table, &module_name);
    let func_idx = one_plus_one(&mut pool, &mut function_table, &type_table, onepone);
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
        Instruction::constant(pool.add_u8(1)),
        Instruction::constant(pool.add_u8(2)),
        Instruction::constant(pool.add_u8(3)),
        Instruction::data_type_create(1.into()),
        Instruction::constant(pool.add_u8(4)),
        Instruction::extend(1_u32.into()),
        Instruction::data_type_set_field(1.into()),
        Instruction::extend(0_u32.into()),
        Instruction::data_type_read_field(1.into()),
        Instruction::print(),
        Instruction::halt(),
    ];
    let mut locals = LocalSlots::new();
    locals.add_slot(&type_table, sahara::ValueType::U64);
    locals.add_slot(&type_table, sahara::ValueType::LocalData(0_usize.into()));
    let main = module_name.function_id("main");
    let main_idx = function_table.insert(main, instructions, locals);
    let mut vm = VirtualMachine::new(context, function_table, pool, type_table);
    vm.run(main_idx);
}
