use crate::{memory_mapping::MemoryMapping, registers::Registers};

pub(crate) struct CPU(Registers);

impl CPU {
    pub(crate) fn new() -> Self {
        CPU(Registers::new())
    }

    pub(crate) fn run_instruction(&mut self, memory: MemoryMapping) {
        let instruction = memory[self.0.pc];
        match instruction {
            0x0 => {},

            _ => { println!("not implemented: {:x}", instruction) }
        }

    }
}
