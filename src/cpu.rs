use crate::{memory_mapping::MemoryMapping, registers::Registers};

pub(crate) struct CPU {
    registers: Registers
}

impl CPU {
    pub(crate) fn new() -> Self {
        CPU{ registers: Registers::new() }
    }

    pub(crate) fn run_instruction(&mut self, memory: &MemoryMapping) {
        let instruction = memory[self.registers.pc];
        println!("{:x} {:x}",self.registers.pc, instruction);
        self.registers.pc += 1;
        match instruction {
            0x00 => {},
            0x01 => {
                let data = self.get_u16(memory);
                self.registers.set_bc(data);
            }
            0x02 => self.registers.a = memory[self.registers.get_bc()],
            0x06 => self.registers.b = self.get_u8(memory),
            0x0e => {
                let data = self.get_u8(memory);
                self.registers.c = data;
            }
            0x21 => {
                let data = self.get_u16(memory);
                self.registers.set_hl(data);
            },
            0x3e => self.registers.a = self.get_u8(memory),
            0x47 => self.registers.b = self.registers.a,
            0xc3 => {
                let addr = self.get_u16(memory);
                self.registers.pc = addr;
            },
            0xcb => self.cb_prefixed_instructions(memory[self.registers.pc]),
            _ => println!("not implemented: {:x}", instruction)
        };
    }

    fn cb_prefixed_instructions(&mut self, byte: u8) {
        match byte {
            _ => println!("not implemented: 0xcb {:x}", byte)
        }
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
