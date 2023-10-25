use crate::execution_context::ExecutionContext;

pub struct VirtualMachine {
    context: ExecutionContext,
}

impl VirtualMachine {
    pub fn new(context: ExecutionContext) -> Self {
        VirtualMachine { context }
    }

    pub fn run(&mut self) {
        self.context.run();
    }
}
