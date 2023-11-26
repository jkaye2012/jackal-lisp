use crate::{
    constant_pool::ConstantPool, data_type::TypeTable, execution_context::ExecutionContext,
    function::FunctionTable, memory::ContextHeap, util::index::FunctionIndex,
};

pub struct VirtualMachine {
    context: ExecutionContext<ContextHeap>,
    function_table: FunctionTable,
    constants: ConstantPool,
    type_table: TypeTable,
}

impl VirtualMachine {
    pub fn new(
        context: ExecutionContext<ContextHeap>,
        function_table: FunctionTable,
        constants: ConstantPool,
        type_table: TypeTable,
    ) -> Self {
        VirtualMachine {
            context,
            function_table,
            constants,
            type_table,
        }
    }

    pub fn run(&mut self, entrypoint: FunctionIndex) {
        let global_context =
            GlobalContext::new(&self.constants, &self.function_table, &self.type_table);
        self.context.run(&global_context, entrypoint);
    }
}

pub struct GlobalContext<'a> {
    constant_pool: &'a ConstantPool,
    function_table: &'a FunctionTable,
    type_table: &'a TypeTable,
}

impl<'a> GlobalContext<'a> {
    pub fn new(
        constant_pool: &'a ConstantPool,
        function_table: &'a FunctionTable,
        type_table: &'a TypeTable,
    ) -> Self {
        GlobalContext {
            constant_pool,
            function_table,
            type_table,
        }
    }

    pub fn constant_pool(&self) -> &'a ConstantPool {
        self.constant_pool
    }

    pub fn function_table(&self) -> &'a FunctionTable {
        self.function_table
    }

    pub fn type_table(&self) -> &'a TypeTable {
        self.type_table
    }
}
