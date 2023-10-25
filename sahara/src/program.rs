use crate::instruction::Instruction;

#[derive(Debug, Default, Clone, Copy)]
pub struct InstructionAddress(usize);

impl InstructionAddress {
    pub fn increment(&mut self) -> usize {
        let current = self.0;
        self.0 += 1;
        current
    }
}

#[derive(Default)]
pub struct Program {
    instructions: Vec<Instruction>,
    ip: InstructionAddress,
}

pub type AddressedInstruction = (InstructionAddress, Instruction);

impl Program {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add(&mut self, inst: Instruction) {
        self.instructions.push(inst);
    }

    pub fn step(&mut self) -> AddressedInstruction {
        let addr = self.ip;
        (addr, self.instructions[self.ip.increment()]) // TODO: this panics with a bad message if the bytecode doesn't contain
                                                       // a HALT instruction
    }

    pub fn set_ip(&mut self, ip: InstructionAddress) {
        self.ip = ip;
    }
}
