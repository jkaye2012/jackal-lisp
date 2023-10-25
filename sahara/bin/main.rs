use sahara::{ExecutionContext, Instruction, Program, VirtualMachine};

fn main() {
    let mut context = ExecutionContext::new(Program::new());
    let mut instructions = Vec::new();
    {
        let pool = context.constant_pool();
        instructions.push(Instruction::const_u64(pool, 2));
        instructions.push(Instruction::const_u64(pool, 2));
        instructions.push(Instruction::add());
        instructions.push(Instruction::print());
        instructions.push(Instruction::halt());
    }
    {
        let program = context.program();
        for inst in instructions.into_iter() {
            program.add(inst);
        }
    }
    let mut vm = VirtualMachine::new(context);
    vm.run();
}
