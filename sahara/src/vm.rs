use crate::{execution_context::ExecutionContext, function::FunctionTable};

pub struct VirtualMachine {
    context: ExecutionContext,
    function_table: FunctionTable,
}

impl VirtualMachine {
    pub fn new(context: ExecutionContext, function_table: FunctionTable) -> Self {
        VirtualMachine {
            context,
            function_table,
        }
    }

    pub fn run(&mut self) {
        self.context.run(&self.function_table, "main");
    }
}
