use crate::{execution_context::ExecutionContext, function::FunctionTable, ConstantPool};

pub struct VirtualMachine {
    context: ExecutionContext,
    function_table: FunctionTable,
    constants: ConstantPool,
}

impl VirtualMachine {
    pub fn new(
        context: ExecutionContext,
        function_table: FunctionTable,
        constants: ConstantPool,
    ) -> Self {
        VirtualMachine {
            context,
            function_table,
            constants,
        }
    }

    // TODO: consider packaging these references up into a struct to simplify this signature?
    pub fn run(&mut self) {
        self.context
            .run(&self.constants, &self.function_table, "main::main");
    }
}
