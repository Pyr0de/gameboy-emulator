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
            // LD BC n16
            0x01 => {
                let data = self.get_u16(memory);
                self.registers.set_u16(&RegisterU16::BC, data);
                3
            }
            // LD (BC) A
            0x02 => {
                self.registers.a = memory[self.registers.get_u16(&RegisterU16::BC)];
                2
            }
            // INC BC
            0x03 => {
                Alu::add_u16(&mut self.registers, &RegisterU16::BC, 1, false, 0);
                2
            }
            // INC B
            0x04 => {
                Alu::add_u8(
                    &mut self.registers,
                    &RegisterU8::B,
                    1,
                    false,
                    Flags::All as u8 ^ Flags::CY as u8,
                );
                1
            }
            // DEC B
            0x05 => {
                Alu::sub(
                    &mut self.registers,
                    &RegisterU8::B,
                    1,
                    false,
                    Flags::All as u8 ^ Flags::CY as u8,
                );
                1
            }
            // LD B n8
            0x06 => {
                self.registers.b = self.get_u8(memory);
                2
            }
            // RLCA
            0x07 => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // LD (a16) SP
            0x08 => {
                println!("unimplemented instruction: {instruction:x}");
                5
            }
            // ADD HL BC
            0x09 => {
                println!("unimplemented instruction: {instruction:x}");
                2
            }
            // LD A (BC)
            0x0A => {
                println!("unimplemented instruction: {instruction:x}");
                2
            }
            // DEC BC
            0x0B => {
                println!("unimplemented instruction: {instruction:x}");
                2
            }
            // INC C
            0x0C => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // DEC C
            0x0D => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // LD C n8
            0x0E => {
                let data = self.get_u8(memory);
                self.registers.c = data;
                2
            }
            // RRCA
            0x0F => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // STOP n8
            0x10 => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // LD DE n16
            0x11 => {
                println!("unimplemented instruction: {instruction:x}");
                3
            }
            // LD (DE) A
            0x12 => {
                println!("unimplemented instruction: {instruction:x}");
                2
            }
            // INC DE
            0x13 => {
                println!("unimplemented instruction: {instruction:x}");
                2
            }
            // INC D
            0x14 => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // DEC D
            0x15 => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // LD D n8
            0x16 => {
                println!("unimplemented instruction: {instruction:x}");
                2
            }
            // RLA
            0x17 => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // JR e8
            0x18 => {
                println!("unimplemented instruction: {instruction:x}");
                3
            }
            // ADD HL DE
            0x19 => {
                println!("unimplemented instruction: {instruction:x}");
                2
            }
            // LD A (DE)
            0x1A => {
                println!("unimplemented instruction: {instruction:x}");
                2
            }
            // DEC DE
            0x1B => {
                println!("unimplemented instruction: {instruction:x}");
                2
            }
            // INC E
            0x1C => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // DEC E
            0x1D => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // LD E n8
            0x1E => {
                println!("unimplemented instruction: {instruction:x}");
                2
            }
            // RRA
            0x1F => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // JR NZ e8
            0x20 => {
                println!("unimplemented instruction: {instruction:x}");
                3
            }
            // LD HL n16
            0x21 => {
                let data = self.get_u16(memory);
                self.registers.set_u16(&RegisterU16::HL, data);
                3
            }
            // LD (HL) A
            0x22 => {
                println!("unimplemented instruction: {instruction:x}");
                2
            }
            // INC HL
            0x23 => {
                println!("unimplemented instruction: {instruction:x}");
                2
            }
            // INC H
            0x24 => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // DEC H
            0x25 => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // LD H n8
            0x26 => {
                println!("unimplemented instruction: {instruction:x}");
                2
            }
            // DAA
            0x27 => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // JR Z e8
            0x28 => {
                println!("unimplemented instruction: {instruction:x}");
                3
            }
            // ADD HL HL
            0x29 => {
                println!("unimplemented instruction: {instruction:x}");
                2
            }
            // LD A (HL)
            0x2A => {
                println!("unimplemented instruction: {instruction:x}");
                2
            }
            // DEC HL
            0x2B => {
                println!("unimplemented instruction: {instruction:x}");
                2
            }
            // INC L
            0x2C => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // DEC L
            0x2D => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // LD L n8
            0x2E => {
                println!("unimplemented instruction: {instruction:x}");
                2
            }
            // CPL
            0x2F => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // JR NC e8
            0x30 => {
                println!("unimplemented instruction: {instruction:x}");
                3
            }
            // LD SP n16
            0x31 => {
                println!("unimplemented instruction: {instruction:x}");
                3
            }
            // LD (HL) A
            0x32 => {
                println!("unimplemented instruction: {instruction:x}");
                2
            }
            // INC SP
            0x33 => {
                println!("unimplemented instruction: {instruction:x}");
                2
            }
            // INC (HL)
            0x34 => {
                println!("unimplemented instruction: {instruction:x}");
                3
            }
            // DEC (HL)
            0x35 => {
                println!("unimplemented instruction: {instruction:x}");
                3
            }
            // LD (HL) n8
            0x36 => {
                println!("unimplemented instruction: {instruction:x}");
                3
            }
            // SCF
            0x37 => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // JR C e8
            0x38 => {
                println!("unimplemented instruction: {instruction:x}");
                3
            }
            // ADD HL SP
            0x39 => {
                println!("unimplemented instruction: {instruction:x}");
                2
            }
            // LD A (HL)
            0x3A => {
                println!("unimplemented instruction: {instruction:x}");
                2
            }
            // DEC SP
            0x3B => {
                println!("unimplemented instruction: {instruction:x}");
                2
            }
            // INC A
            0x3C => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // DEC A
            0x3D => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // LD A n8
            0x3E => {
                self.registers.a = self.get_u8(memory);
                2
            }
            // CCF
            0x3F => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // LD B B
            0x40 => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // LD B C
            0x41 => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // LD B D
            0x42 => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // LD B E
            0x43 => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // LD B H
            0x44 => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // LD B L
            0x45 => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // LD B (HL)
            0x46 => {
                println!("unimplemented instruction: {instruction:x}");
                2
            }
            // LD B A
            0x47 => {
                self.registers.b = self.registers.a;
                1
            }
            // LD C B
            0x48 => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // LD C C
            0x49 => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // LD C D
            0x4A => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // LD C E
            0x4B => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // LD C H
            0x4C => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // LD C L
            0x4D => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // LD C (HL)
            0x4E => {
                println!("unimplemented instruction: {instruction:x}");
                2
            }
            // LD C A
            0x4F => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // LD D B
            0x50 => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // LD D C
            0x51 => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // LD D D
            0x52 => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // LD D E
            0x53 => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // LD D H
            0x54 => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // LD D L
            0x55 => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // LD D (HL)
            0x56 => {
                println!("unimplemented instruction: {instruction:x}");
                2
            }
            // LD D A
            0x57 => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // LD E B
            0x58 => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // LD E C
            0x59 => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // LD E D
            0x5A => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // LD E E
            0x5B => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // LD E H
            0x5C => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // LD E L
            0x5D => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // LD E (HL)
            0x5E => {
                println!("unimplemented instruction: {instruction:x}");
                2
            }
            // LD E A
            0x5F => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // LD H B
            0x60 => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // LD H C
            0x61 => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // LD H D
            0x62 => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // LD H E
            0x63 => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // LD H H
            0x64 => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // LD H L
            0x65 => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // LD H (HL)
            0x66 => {
                println!("unimplemented instruction: {instruction:x}");
                2
            }
            // LD H A
            0x67 => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // LD L B
            0x68 => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // LD L C
            0x69 => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // LD L D
            0x6A => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // LD L E
            0x6B => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // LD L H
            0x6C => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // LD L L
            0x6D => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // LD L (HL)
            0x6E => {
                println!("unimplemented instruction: {instruction:x}");
                2
            }
            // LD L A
            0x6F => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // LD (HL) B
            0x70 => {
                println!("unimplemented instruction: {instruction:x}");
                2
            }
            // LD (HL) C
            0x71 => {
                println!("unimplemented instruction: {instruction:x}");
                2
            }
            // LD (HL) D
            0x72 => {
                println!("unimplemented instruction: {instruction:x}");
                2
            }
            // LD (HL) E
            0x73 => {
                println!("unimplemented instruction: {instruction:x}");
                2
            }
            // LD (HL) H
            0x74 => {
                println!("unimplemented instruction: {instruction:x}");
                2
            }
            // LD (HL) L
            0x75 => {
                println!("unimplemented instruction: {instruction:x}");
                2
            }
            // HALT
            0x76 => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // LD (HL) A
            0x77 => {
                println!("unimplemented instruction: {instruction:x}");
                2
            }
            // LD A B
            0x78 => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // LD A C
            0x79 => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // LD A D
            0x7A => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // LD A E
            0x7B => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // LD A H
            0x7C => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // LD A L
            0x7D => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // LD A (HL)
            0x7E => {
                println!("unimplemented instruction: {instruction:x}");
                2
            }
            // LD A A
            0x7F => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // ADD A B
            0x80 => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // ADD A C
            0x81 => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // ADD A D
            0x82 => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // ADD A E
            0x83 => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // ADD A H
            0x84 => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // ADD A L
            0x85 => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // ADD A (HL)
            0x86 => {
                println!("unimplemented instruction: {instruction:x}");
                2
            }
            // ADD A A
            0x87 => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // ADC A B
            0x88 => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // ADC A C
            0x89 => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // ADC A D
            0x8A => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // ADC A E
            0x8B => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // ADC A H
            0x8C => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // ADC A L
            0x8D => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // ADC A (HL)
            0x8E => {
                println!("unimplemented instruction: {instruction:x}");
                2
            }
            // ADC A A
            0x8F => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // SUB A B
            0x90 => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // SUB A C
            0x91 => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // SUB A D
            0x92 => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // SUB A E
            0x93 => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // SUB A H
            0x94 => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // SUB A L
            0x95 => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // SUB A (HL)
            0x96 => {
                println!("unimplemented instruction: {instruction:x}");
                2
            }
            // SUB A A
            0x97 => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // SBC A B
            0x98 => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // SBC A C
            0x99 => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // SBC A D
            0x9A => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // SBC A E
            0x9B => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // SBC A H
            0x9C => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // SBC A L
            0x9D => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // SBC A (HL)
            0x9E => {
                println!("unimplemented instruction: {instruction:x}");
                2
            }
            // SBC A A
            0x9F => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // AND A B
            0xA0 => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // AND A C
            0xA1 => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // AND A D
            0xA2 => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // AND A E
            0xA3 => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // AND A H
            0xA4 => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // AND A L
            0xA5 => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // AND A (HL)
            0xA6 => {
                println!("unimplemented instruction: {instruction:x}");
                2
            }
            // AND A A
            0xA7 => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // XOR A B
            0xA8 => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // XOR A C
            0xA9 => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // XOR A D
            0xAA => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // XOR A E
            0xAB => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // XOR A H
            0xAC => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // XOR A L
            0xAD => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // XOR A (HL)
            0xAE => {
                println!("unimplemented instruction: {instruction:x}");
                2
            }
            // XOR A A
            0xAF => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // OR A B
            0xB0 => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // OR A C
            0xB1 => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // OR A D
            0xB2 => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // OR A E
            0xB3 => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // OR A H
            0xB4 => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // OR A L
            0xB5 => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // OR A (HL)
            0xB6 => {
                println!("unimplemented instruction: {instruction:x}");
                2
            }
            // OR A A
            0xB7 => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // CP A B
            0xB8 => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // CP A C
            0xB9 => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // CP A D
            0xBA => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // CP A E
            0xBB => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // CP A H
            0xBC => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // CP A L
            0xBD => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // CP A (HL)
            0xBE => {
                println!("unimplemented instruction: {instruction:x}");
                2
            }
            // CP A A
            0xBF => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // RET NZ
            0xC0 => {
                println!("unimplemented instruction: {instruction:x}");
                5
            }
            // POP BC
            0xC1 => {
                println!("unimplemented instruction: {instruction:x}");
                3
            }
            // JP NZ a16
            0xC2 => {
                println!("unimplemented instruction: {instruction:x}");
                4
            }
            // JP a16
            0xC3 => {
                let addr = self.get_u16(memory);
                self.registers.pc = addr;
                4
            }
            // CALL NZ a16
            0xC4 => {
                println!("unimplemented instruction: {instruction:x}");
                6
            }
            // PUSH BC
            0xC5 => {
                println!("unimplemented instruction: {instruction:x}");
                4
            }
            // ADD A n8
            0xC6 => {
                println!("unimplemented instruction: {instruction:x}");
                2
            }
            // RST $00
            0xC7 => {
                println!("unimplemented instruction: {instruction:x}");
                4
            }
            // RET Z
            0xC8 => {
                println!("unimplemented instruction: {instruction:x}");
                5
            }
            // RET
            0xC9 => {
                println!("unimplemented instruction: {instruction:x}");
                4
            }
            // JP Z a16
            0xCA => {
                println!("unimplemented instruction: {instruction:x}");
                4
            }
            // PREFIX
            0xCB => {
                let byte = self.get_u8(memory);
                self.cb_prefixed_instructions(byte)
            }
            // CALL Z a16
            0xCC => {
                println!("unimplemented instruction: {instruction:x}");
                6
            }
            // CALL a16
            0xCD => {
                println!("unimplemented instruction: {instruction:x}");
                6
            }
            // ADC A n8
            0xCE => {
                println!("unimplemented instruction: {instruction:x}");
                2
            }
            // RST $08
            0xCF => {
                println!("unimplemented instruction: {instruction:x}");
                4
            }
            // RET NC
            0xD0 => {
                println!("unimplemented instruction: {instruction:x}");
                5
            }
            // POP DE
            0xD1 => {
                println!("unimplemented instruction: {instruction:x}");
                3
            }
            // JP NC a16
            0xD2 => {
                println!("unimplemented instruction: {instruction:x}");
                4
            }
            // CALL NC a16
            0xD4 => {
                println!("unimplemented instruction: {instruction:x}");
                6
            }
            // PUSH DE
            0xD5 => {
                println!("unimplemented instruction: {instruction:x}");
                4
            }
            // SUB A n8
            0xD6 => {
                println!("unimplemented instruction: {instruction:x}");
                2
            }
            // RST $10
            0xD7 => {
                println!("unimplemented instruction: {instruction:x}");
                4
            }
            // RET C
            0xD8 => {
                println!("unimplemented instruction: {instruction:x}");
                5
            }
            // RETI
            0xD9 => {
                println!("unimplemented instruction: {instruction:x}");
                4
            }
            // JP C a16
            0xDA => {
                println!("unimplemented instruction: {instruction:x}");
                4
            }
            // CALL C a16
            0xDC => {
                println!("unimplemented instruction: {instruction:x}");
                6
            }
            // SBC A n8
            0xDE => {
                println!("unimplemented instruction: {instruction:x}");
                2
            }
            // RST $18
            0xDF => {
                println!("unimplemented instruction: {instruction:x}");
                4
            }
            // LDH (a8) A
            0xE0 => {
                println!("unimplemented instruction: {instruction:x}");
                3
            }
            // POP HL
            0xE1 => {
                println!("unimplemented instruction: {instruction:x}");
                3
            }
            // LDH (C) A
            0xE2 => {
                println!("unimplemented instruction: {instruction:x}");
                2
            }
            // PUSH HL
            0xE5 => {
                println!("unimplemented instruction: {instruction:x}");
                4
            }
            // AND A n8
            0xE6 => {
                println!("unimplemented instruction: {instruction:x}");
                2
            }
            // RST $20
            0xE7 => {
                println!("unimplemented instruction: {instruction:x}");
                4
            }
            // ADD SP e8
            0xE8 => {
                println!("unimplemented instruction: {instruction:x}");
                4
            }
            // JP HL
            0xE9 => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // LD (a16) A
            0xEA => {
                println!("unimplemented instruction: {instruction:x}");
                4
            }
            // ILLEGAL_EB
            0xEB => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // ILLEGAL_EC
            0xEC => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // ILLEGAL_ED
            0xED => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // XOR A n8
            0xEE => {
                println!("unimplemented instruction: {instruction:x}");
                2
            }
            // RST $28
            0xEF => {
                println!("unimplemented instruction: {instruction:x}");
                4
            }
            // LDH A (a8)
            0xF0 => {
                println!("unimplemented instruction: {instruction:x}");
                3
            }
            // POP AF
            0xF1 => {
                println!("unimplemented instruction: {instruction:x}");
                3
            }
            // LDH A (C)
            0xF2 => {
                println!("unimplemented instruction: {instruction:x}");
                2
            }
            // DI
            0xF3 => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // ILLEGAL_F4
            0xF4 => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // PUSH AF
            0xF5 => {
                println!("unimplemented instruction: {instruction:x}");
                4
            }
            // OR A n8
            0xF6 => {
                println!("unimplemented instruction: {instruction:x}");
                2
            }
            // RST $30
            0xF7 => {
                println!("unimplemented instruction: {instruction:x}");
                4
            }
            // LD HL SP e8
            0xF8 => {
                println!("unimplemented instruction: {instruction:x}");
                3
            }
            // LD SP HL
            0xF9 => {
                println!("unimplemented instruction: {instruction:x}");
                2
            }
            // LD A (a16)
            0xFA => {
                println!("unimplemented instruction: {instruction:x}");
                4
            }
            // EI
            0xFB => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // ILLEGAL_FC
            0xFC => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // ILLEGAL_FD
            0xFD => {
                println!("unimplemented instruction: {instruction:x}");
                1
            }
            // CP A n8
            0xFE => {
                println!("unimplemented instruction: {instruction:x}");
                2
            }
            // RST $38
            0xFF => {
                println!("unimplemented instruction: {instruction:x}");
                4
            }
            _ => {
                println!("Illegal instruction: {instruction:x}");
                1
            }
        };
    }

    fn cb_prefixed_instructions(&mut self, byte: u8) -> usize {
        #[allow(clippy::match_single_binding)]
        match byte {
            _ => {
                println!("not implemented: 0xcb {:x}", byte);
                0
            }
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
