use crate::{
    instructions::{self, Instruction, Operand, OperandU8, OperandU16},
    memory_mapping::MemoryMapping,
    registers::{Alu, Flags, RegisterU16, Registers},
};

#[derive(Debug)]
pub(crate) struct Cpu {
    pub registers: Registers,
    pub memory: MemoryMapping,
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
            0xCB => instructions::cbprefixed::decode_byte(self.memory[self.registers.pc]),
            _ => instructions::unprefixed::decode_byte(byte),
        };

        let _cycles = match instruction {
            Instruction::NOP => 1,
            Instruction::LD(Operand::U8(a), Operand::U8(b)) => {
                let (value, a_cycles) = self.get_u8(b);
                let b_cycles = self.set_u8(a, value);
                u8::max(a_cycles, b_cycles)
            }
            Instruction::LD(Operand::U16(a), Operand::U16(b)) => {
                let (value, a_cycles) = self.get_u16(b);
                let b_cycles = self.set_u16(a, value);
                u8::max(a_cycles, b_cycles)
            }
            Instruction::LDF8 => {
                let (signed, _cycles) = self.get_u8(OperandU8::Immediate);
                self.registers.set_u16(
                    &RegisterU16::HL,
                    self.registers.sp.wrapping_add_signed(u8_to_i16(signed)),
                );
                3
            }
            Instruction::INC(Operand::U8(a)) => {
                let (val, _) = self.get_u8(a.clone());
                let res = Alu::add_u8(
                    &mut self.registers,
                    val,
                    1,
                    false,
                    Flags::All as u8 ^ Flags::CY as u8,
                );
                self.set_u8(a.clone(), res);
                match a {
                    OperandU8::Memory(_) => 3,
                    OperandU8::Register(_) => 1,
                    _ => unreachable!(),
                }
            }
            Instruction::INC(Operand::U16(OperandU16::RegisterPair(r))) => {
                Alu::add_u16(&mut self.registers, &r, 1, false, 0);
                2
            }
            Instruction::DEC(Operand::U8(a)) => {
                let (val, _) = self.get_u8(a.clone());
                let res = Alu::sub(
                    &mut self.registers,
                    val,
                    1,
                    false,
                    Flags::All as u8 ^ Flags::CY as u8,
                );
                self.set_u8(a.clone(), res);
                match a {
                    OperandU8::Memory(_) => 3,
                    OperandU8::Register(_) => 1,
                    _ => unreachable!(),
                }
            }
            Instruction::DEC(Operand::U16(OperandU16::RegisterPair(r))) => {
                Alu::dec_u16(&mut self.registers, &r);
                2
            }
            Instruction::ADD(Operand::U8(a), Operand::U8(b)) => {
                let (a, _) = self.get_u8(a);
                let (b, b_cycles) = self.get_u8(b);
                self.registers.a = Alu::add_u8(&mut self.registers, a, b, false, Flags::All as u8);
                b_cycles
            }
            Instruction::ADD(Operand::U16(a), Operand::U16(b)) => {
                let r = match a {
                    OperandU16::RegisterPair(r) => r,
                    _ => unreachable!(),
                };
                let (b, _) = self.get_u16(b);
                Alu::add_u16(&mut self.registers, &r, b, false, Flags::All as u8);
                2
            }
            Instruction::ADC(a, b) => {
                let cy = self.registers.get_flag(Flags::CY);
                let (a, _) = self.get_u8(a);
                let (b, b_cycles) = self.get_u8(b);
                self.registers.a = Alu::add_u8(&mut self.registers, a, b, cy, Flags::All as u8);
                b_cycles
            }
            Instruction::SUB(a, b) => {
                let (a, _) = self.get_u8(a);
                let (b, b_cycles) = self.get_u8(b);
                self.registers.a = Alu::sub(&mut self.registers, a, b, false, Flags::All as u8);
                b_cycles
            }
            Instruction::SBC(a, b) => {
                let cy = self.registers.get_flag(Flags::CY);
                let (a, _) = self.get_u8(a);
                let (b, b_cycles) = self.get_u8(b);
                self.registers.a = Alu::sub(&mut self.registers, a, b, cy, Flags::All as u8);
                b_cycles
            }
            Instruction::AND(_a, b) => {
                let (b, cycles) = self.get_u8(b);
                Alu::and(&mut self.registers, b);
                cycles
            }
            Instruction::OR(_a, b) => {
                let (b, cycles) = self.get_u8(b);
                Alu::or(&mut self.registers, b);
                cycles
            }
            Instruction::XOR(_a, b) => {
                let (b, cycles) = self.get_u8(b);
                Alu::xor(&mut self.registers, b);
                cycles
            }
            Instruction::CP(_a, b) => {
                let (b, cycles) = self.get_u8(b);
                Alu::cmp(&mut self.registers, b);
                cycles
            }
            Instruction::JP(condition, op) => {
                let (addr, _cycles) = self.get_u16(op);
                if condition.is_none_or(|cond| self.registers.get_flag_condition(cond)) {
                    self.registers.pc = addr;
                    4
                } else {
                    3
                }
            }
            Instruction::JR(condition, op) => {
                let (offset_u8, _cycles) = self.get_u8(op);
                let offset = u8_to_i16(offset_u8);
                if condition.is_none_or(|cond| self.registers.get_flag_condition(cond)) {
                    self.registers.pc = self.registers.pc.wrapping_add_signed(offset);
                    3
                } else {
                    2
                }
            }
            Instruction::CALL(condition, op) => {
                let (addr, _cycles) = self.get_u16(op);
                if condition.is_none_or(|cond| self.registers.get_flag_condition(cond)) {
                    self.memory[self.registers.sp - 1] = (self.registers.pc & 0xff) as u8;
                    self.memory[self.registers.sp - 2] = (self.registers.pc >> 8) as u8;
                    self.registers.sp -= 2;
                    self.registers.pc = addr;
                    6
                } else {
                    3
                }
            }
            Instruction::RET(condition) => {
                if condition.clone().is_none_or(|cond| self.registers.get_flag_condition(cond)) {
                    let addr = (self.memory[self.registers.sp + 1] as u16) << 8
                        | self.memory[self.registers.sp] as u16;
                    self.registers.sp += 2;
                    self.registers.pc = addr;
                    match condition {
                        None => 4,
                        Some(_) => 5,
                    }
                } else {
                    2
                }
            }
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

#[inline(always)]
fn u8_to_i16(a: u8) -> i16 {
    (a as i8) as i16
}
