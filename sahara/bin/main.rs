use sahara::{ExecutionContext, Function, FunctionId, FunctionTable, Instruction, VirtualMachine};

fn main() {
    let mut context = ExecutionContext::new();
    let mut instructions = Vec::new();
    {
        let pool = context.constant_pool();
        instructions.push(Instruction::const_u64(pool.add_u64(2)));
        instructions.push(Instruction::const_u64(pool.add_u64(2)));
        instructions.push(Instruction::add());
        instructions.push(Instruction::print());
        instructions.push(Instruction::halt());
    }
    let entrypoint = Function::from_instructions(instructions);
    let mut table = FunctionTable::new();
    table.insert(FunctionId::from_fq_name("main".to_string()), entrypoint);
    let mut vm = VirtualMachine::new(context, table);
    vm.run();
}
