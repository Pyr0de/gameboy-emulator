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

    pub(crate) fn run_instruction(&mut self, memory: &MemoryMapping) {
        let instruction = memory[self.registers.pc];
        println!("{:x} {:x}", self.registers.pc, instruction);
        self.registers.pc += 1;
        let _cycles = match instruction {
            // NOP
            0x00 => 1,
            // LD BC, d16
            0x01 => {
                let data = self.get_u16(memory);
                self.registers.set_u16(&RegisterU16::BC, data);
                3
            }
            // LD (BC), A
            0x02 => {
                self.registers.a = memory[self.registers.get_u16(&RegisterU16::BC)];
                2
            },
            // INC BC
            0x03 => {
                Alu::add_u16(&mut self.registers, &RegisterU16::BC, 1, false, 0);
                2
            },
            0x04 => {
                Alu::add_u8(
                    &mut self.registers,
                    &RegisterU8::B,
                    1,
                    false,
                    Flags::All as u8 ^ Flags::CY as u8,
                );
                1
            },
            0x05 => {
                Alu::sub(
                    &mut self.registers,
                    &RegisterU8::B,
                    1,
                    false,
                    Flags::All as u8 ^ Flags::CY as u8,
                );
                1
            },
            0x06 => {
                self.registers.b = self.get_u8(memory);
                2
            },
            0x0e => {
                let data = self.get_u8(memory);
                self.registers.c = data;
                2
            }
            0x21 => {
                let data = self.get_u16(memory);
                self.registers.set_u16(&RegisterU16::HL, data);
                3
            }
            0x3e => {
                self.registers.a = self.get_u8(memory);
                2
            },
            0x47 => {
                self.registers.b = self.registers.a;
                2
            },
            0xc3 => {
                let addr = self.get_u16(memory);
                self.registers.pc = addr;
                4
            }
            0xcb => self.cb_prefixed_instructions(memory[self.registers.pc]),
            _ => {
                println!("not implemented: {:x}", instruction);
                0
            },
        };
    }

    fn cb_prefixed_instructions(&mut self, byte: u8) -> usize {
        #[allow(clippy::match_single_binding)]
        match byte {
            _ => {
                println!("not implemented: 0xcb {:x}", byte);
                0
            },
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
