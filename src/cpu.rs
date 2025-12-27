use crate::{memory_mapping::MemoryMapping, registers::Registers};

pub(crate) struct CPU(Registers);

impl CPU {
    pub(crate) fn new() -> Self {
        CPU(Registers::new())
    }

    pub(crate) fn run_instruction(&mut self, memory: &MemoryMapping) {
        let instruction = memory[self.0.pc];
        println!("{:x} {:x}",self.0.pc, instruction);
        self.0.pc += 1;
        match instruction {
            0x0 => {},
            0x0e => {
                let data = self.get_u8(memory);
                self.0.c = data;
            }
            0x21 => {
                let data = self.get_u16(memory);
                self.0.set_hl(data);
            },
            0x47 => self.0.b = self.0.a,
            0xc3 => {
                let addr = self.get_u16(memory);
                self.0.pc = addr;
            }
            _ => println!("not implemented: {:x}", instruction)
        };
    }

    fn get_u16(&mut self, memory: &MemoryMapping) -> u16 {
        self.0.pc += 2;
        memory[self.0.pc - 2] as u16 | ((memory[self.0.pc - 1] as u16) << 8)
    }
    fn get_u8(&mut self, memory: &MemoryMapping) -> u8 {
        self.0.pc += 1;
        memory[self.0.pc - 1]
    }
}
