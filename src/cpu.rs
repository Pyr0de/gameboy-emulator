use crate::{
    instructions::{self, Instruction, Operand, OperandU8, OperandU16},
    memory_mapping::MemoryMapping,
    registers::{Alu, Flags, RegisterU8, RegisterU16, Registers},
};

pub(crate) struct Cpu {
    registers: Registers,
    memory: MemoryMapping,
}

impl Cpu {
    pub(crate) fn new(memory: MemoryMapping) -> Self {
        Cpu {
            registers: Registers::new(),
            memory,
        }
    }

    pub(crate) fn run_instruction(&mut self) {
        let byte = self.memory[self.registers.pc];
        self.registers.pc += 1;
        let instruction = match byte {
            0x22 => unimplemented!(),
            0x2A => unimplemented!(),
            0x32 => unimplemented!(),
            0x3A => unimplemented!(),
            0xCB => instructions::cbprefixed::decode_byte(self.memory[self.registers.pc]),
            _ => instructions::unprefixed::decode_byte(byte),
        };
        let _cycles = match instruction {
            Instruction::NOP => 1,
            _ => unimplemented!("not implemented {byte:x}: {instruction:?}"),
        };
    }

    fn get_u16(&mut self, op: OperandU16) -> (u16, u8) {
        match op {
            OperandU16::RegisterPair(r) => (self.registers.get_u16(&r), 2),
            OperandU16::Immediate => {
                self.registers.pc += 2;
                (
                    self.memory[self.registers.pc - 2] as u16
                        | ((self.memory[self.registers.pc - 1] as u16) << 8),
                    3,
                )
            }
        }
    }
    fn set_u16(&mut self, op: OperandU16, value: u16) -> u8 {
        match op {
            OperandU16::RegisterPair(r) => {
                self.registers.set_u16(&r, value);
                2
            }
            OperandU16::Immediate => unreachable!("cannot write to immediate"),
        }
    }

    fn get_u8(&mut self, op: OperandU8) -> (u8, u8) {
        match op {
            OperandU8::Register(r) => (self.registers.get_u8(&r), 1),
            OperandU8::Immediate => {
                self.registers.pc += 1;
                (self.memory[self.registers.pc - 1], 2)
            }
            OperandU8::Memory(addr) => {
                let (a, cycles) = self.get_u16(addr);
                (self.memory[a], cycles)
            }
            OperandU8::MemoryU8(offset) => {
                let (a, cycles) = self.get_u8(*offset);
                (self.memory[0xff00 | a as u16], cycles)
            }
        }
    }
    fn set_u8(&mut self, op: OperandU8, value: u8) -> u8 {
        match op {
            OperandU8::Register(r) => {
                self.registers.set_u8(&r, value);
                1
            }
            OperandU8::Immediate => unreachable!("cannot write to immediate"),
            OperandU8::Memory(addr) => {
                let (a, cycles) = self.get_u16(addr);
                self.memory[a] = value;
                cycles
            }
            OperandU8::MemoryU8(offset) => {
                let (a, cycles) = self.get_u8(*offset);
                self.memory[0xff00 | a as u16] = value;
                cycles
            }
        }
    }
}
