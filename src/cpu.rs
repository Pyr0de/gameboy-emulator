use crate::{
    memory_mapping::MemoryMapping,
    registers::{Alu, Flags, RegisterU8, RegisterU16, Registers},
};

pub(crate) struct Cpu {
    registers: Registers,
}

impl Cpu {
    pub(crate) fn new() -> Self {
        Cpu {
            registers: Registers::new(),
        }
    }

    pub(crate) fn run_instruction(&mut self, memory: &mut MemoryMapping) {
        let instruction = memory[self.registers.pc];
        println!("{:x} {:x}", self.registers.pc, instruction);
        self.registers.pc += 1;
        let _cycles = match instruction  {
            _ => unimplemented!(),
        };
    }

    fn get_u16(&mut self, memory: &MemoryMapping) -> u16 {
        self.registers.pc += 2;
        memory[self.registers.pc - 2] as u16 | ((memory[self.registers.pc - 1] as u16) << 8)
    }
    fn get_u8(&mut self, memory: &MemoryMapping) -> u8 {
        self.registers.pc += 1;
        memory[self.registers.pc - 1]
    }
}
