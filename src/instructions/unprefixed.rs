use crate::{
    instructions::{FlagCondition, Instruction, Operand, OperandU8, OperandU16},
    registers::{RegisterU8, RegisterU16},
};

pub(crate) fn decode_byte(byte: u8) -> Instruction {
    match byte {
        // NOP
        0x00 => Instruction::NOP,
        // LD BC n16
        0x01 => Instruction::LD(
            Operand::U16(OperandU16::RegisterPair(RegisterU16::BC)),
            Operand::U16(OperandU16::Immediate),
        ),
        // LD (BC) A
        0x02 => Instruction::LD(
            Operand::U8(OperandU8::Memory(OperandU16::RegisterPair(RegisterU16::BC))),
            Operand::U8(OperandU8::Register(RegisterU8::A)),
        ),
        // INC BC
        0x03 => Instruction::INC(Operand::U16(OperandU16::RegisterPair(RegisterU16::BC))),
        // INC B
        0x04 => Instruction::INC(Operand::U8(OperandU8::Register(RegisterU8::B))),
        // DEC B
        0x05 => Instruction::DEC(Operand::U8(OperandU8::Register(RegisterU8::B))),
        // LD B n8
        0x06 => Instruction::LD(
            Operand::U8(OperandU8::Register(RegisterU8::B)),
            Operand::U8(OperandU8::Immediate),
        ),
        // RLCA
        0x07 => Instruction::RLC(OperandU8::Register(RegisterU8::A)),
        // LD (a16) SP
        0x08 => Instruction::LD(
            Operand::U8(OperandU8::Memory(OperandU16::Immediate)),
            Operand::U16(OperandU16::RegisterPair(RegisterU16::SP)),
        ),
        // ADD HL BC
        0x09 => Instruction::ADD(
            Operand::U16(OperandU16::RegisterPair(RegisterU16::HL)),
            Operand::U16(OperandU16::RegisterPair(RegisterU16::BC)),
        ),
        // LD A (BC)
        0x0A => Instruction::LD(
            Operand::U8(OperandU8::Register(RegisterU8::A)),
            Operand::U8(OperandU8::Memory(OperandU16::RegisterPair(RegisterU16::BC))),
        ),
        // DEC BC
        0x0B => Instruction::DEC(Operand::U16(OperandU16::RegisterPair(RegisterU16::BC))),
        // INC C
        0x0C => Instruction::INC(Operand::U8(OperandU8::Register(RegisterU8::C))),
        // DEC C
        0x0D => Instruction::DEC(Operand::U8(OperandU8::Register(RegisterU8::C))),
        // LD C n8
        0x0E => Instruction::LD(
            Operand::U8(OperandU8::Register(RegisterU8::C)),
            Operand::U8(OperandU8::Immediate),
        ),
        // RRCA
        0x0F => Instruction::RRC(OperandU8::Register(RegisterU8::A)),
        // STOP n8
        0x10 => Instruction::STOP(OperandU8::Immediate),
        // LD DE n16
        0x11 => Instruction::LD(
            Operand::U16(OperandU16::RegisterPair(RegisterU16::DE)),
            Operand::U16(OperandU16::Immediate),
        ),
        // LD (DE) A
        0x12 => Instruction::LD(
            Operand::U8(OperandU8::Memory(OperandU16::RegisterPair(RegisterU16::DE))),
            Operand::U8(OperandU8::Register(RegisterU8::A)),
        ),
        // INC DE
        0x13 => Instruction::INC(Operand::U16(OperandU16::RegisterPair(RegisterU16::DE))),
        // INC D
        0x14 => Instruction::INC(Operand::U8(OperandU8::Register(RegisterU8::D))),
        // DEC D
        0x15 => Instruction::DEC(Operand::U8(OperandU8::Register(RegisterU8::D))),
        // LD D n8
        0x16 => Instruction::LD(
            Operand::U8(OperandU8::Register(RegisterU8::D)),
            Operand::U8(OperandU8::Immediate),
        ),
        // RLA
        0x17 => Instruction::RL(OperandU8::Register(RegisterU8::A)),
        // JR e8
        0x18 => Instruction::JR(None, OperandU8::Immediate),
        // ADD HL DE
        0x19 => Instruction::ADD(
            Operand::U16(OperandU16::RegisterPair(RegisterU16::HL)),
            Operand::U16(OperandU16::RegisterPair(RegisterU16::DE)),
        ),
        // LD A (DE)
        0x1A => Instruction::LD(
            Operand::U8(OperandU8::Register(RegisterU8::A)),
            Operand::U8(OperandU8::Memory(OperandU16::RegisterPair(RegisterU16::DE))),
        ),
        // DEC DE
        0x1B => Instruction::DEC(Operand::U16(OperandU16::RegisterPair(RegisterU16::DE))),
        // INC E
        0x1C => Instruction::INC(Operand::U8(OperandU8::Register(RegisterU8::E))),
        // DEC E
        0x1D => Instruction::DEC(Operand::U8(OperandU8::Register(RegisterU8::E))),
        // LD E n8
        0x1E => Instruction::LD(
            Operand::U8(OperandU8::Register(RegisterU8::E)),
            Operand::U8(OperandU8::Immediate),
        ),
        // RRA
        0x1F => Instruction::RR(OperandU8::Register(RegisterU8::A)),
        // JR NZ e8
        0x20 => Instruction::JR(Some(FlagCondition::NZ), OperandU8::Immediate),
        // LD HL n16
        0x21 => Instruction::LD(
            Operand::U16(OperandU16::RegisterPair(RegisterU16::HL)),
            Operand::U16(OperandU16::Immediate),
        ),
        // LD (HL+) A
        0x22 => Instruction::LD22,
        // INC HL
        0x23 => Instruction::INC(Operand::U16(OperandU16::RegisterPair(RegisterU16::HL))),
        // INC H
        0x24 => Instruction::INC(Operand::U8(OperandU8::Register(RegisterU8::H))),
        // DEC H
        0x25 => Instruction::DEC(Operand::U8(OperandU8::Register(RegisterU8::H))),
        // LD H n8
        0x26 => Instruction::LD(
            Operand::U8(OperandU8::Register(RegisterU8::H)),
            Operand::U8(OperandU8::Immediate),
        ),
        // DAA
        0x27 => Instruction::DAA,
        // JR Z e8
        0x28 => Instruction::JR(Some(FlagCondition::Z), OperandU8::Immediate),
        // ADD HL HL
        0x29 => Instruction::ADD(
            Operand::U16(OperandU16::RegisterPair(RegisterU16::HL)),
            Operand::U16(OperandU16::RegisterPair(RegisterU16::HL)),
        ),
        // LD A (HL+)
        0x2A => Instruction::LD2A,
        // DEC HL
        0x2B => Instruction::DEC(Operand::U16(OperandU16::RegisterPair(RegisterU16::HL))),
        // INC L
        0x2C => Instruction::INC(Operand::U8(OperandU8::Register(RegisterU8::L))),
        // DEC L
        0x2D => Instruction::DEC(Operand::U8(OperandU8::Register(RegisterU8::L))),
        // LD L n8
        0x2E => Instruction::LD(
            Operand::U8(OperandU8::Register(RegisterU8::L)),
            Operand::U8(OperandU8::Immediate),
        ),
        // CPL
        0x2F => Instruction::CPL,
        // JR NC e8
        0x30 => Instruction::JR(Some(FlagCondition::NC), OperandU8::Immediate),
        // LD SP n16
        0x31 => Instruction::LD(
            Operand::U16(OperandU16::RegisterPair(RegisterU16::SP)),
            Operand::U16(OperandU16::Immediate),
        ),
        // LD (HL-) A
        0x32 => Instruction::LD32,
        // INC SP
        0x33 => Instruction::INC(Operand::U16(OperandU16::RegisterPair(RegisterU16::SP))),
        // INC (HL)
        0x34 => Instruction::INC(Operand::U8(OperandU8::Memory(OperandU16::RegisterPair(
            RegisterU16::HL,
        )))),
        // DEC (HL)
        0x35 => Instruction::DEC(Operand::U8(OperandU8::Memory(OperandU16::RegisterPair(
            RegisterU16::HL,
        )))),
        // LD (HL) n8
        0x36 => Instruction::LD(
            Operand::U8(OperandU8::Memory(OperandU16::RegisterPair(RegisterU16::HL))),
            Operand::U8(OperandU8::Immediate),
        ),
        // SCF
        0x37 => Instruction::SCF,
        // JR C e8
        0x38 => Instruction::JR(Some(FlagCondition::C), OperandU8::Immediate),
        // ADD HL SP
        0x39 => Instruction::ADD(
            Operand::U16(OperandU16::RegisterPair(RegisterU16::HL)),
            Operand::U16(OperandU16::RegisterPair(RegisterU16::SP)),
        ),
        // LD A, (HL-)
        0x3A => Instruction::LD3A,
        // DEC SP
        0x3B => Instruction::DEC(Operand::U16(OperandU16::RegisterPair(RegisterU16::SP))),
        // INC A
        0x3C => Instruction::INC(Operand::U8(OperandU8::Register(RegisterU8::A))),
        // DEC A
        0x3D => Instruction::DEC(Operand::U8(OperandU8::Register(RegisterU8::A))),
        // LD A n8
        0x3E => Instruction::LD(
            Operand::U8(OperandU8::Register(RegisterU8::A)),
            Operand::U8(OperandU8::Immediate),
        ),
        // CCF
        0x3F => Instruction::CCF,
        // LD B B
        0x40 => Instruction::LD(
            Operand::U8(OperandU8::Register(RegisterU8::B)),
            Operand::U8(OperandU8::Register(RegisterU8::B)),
        ),
        // LD B C
        0x41 => Instruction::LD(
            Operand::U8(OperandU8::Register(RegisterU8::B)),
            Operand::U8(OperandU8::Register(RegisterU8::C)),
        ),
        // LD B D
        0x42 => Instruction::LD(
            Operand::U8(OperandU8::Register(RegisterU8::B)),
            Operand::U8(OperandU8::Register(RegisterU8::D)),
        ),
        // LD B E
        0x43 => Instruction::LD(
            Operand::U8(OperandU8::Register(RegisterU8::B)),
            Operand::U8(OperandU8::Register(RegisterU8::E)),
        ),
        // LD B H
        0x44 => Instruction::LD(
            Operand::U8(OperandU8::Register(RegisterU8::B)),
            Operand::U8(OperandU8::Register(RegisterU8::H)),
        ),
        // LD B L
        0x45 => Instruction::LD(
            Operand::U8(OperandU8::Register(RegisterU8::B)),
            Operand::U8(OperandU8::Register(RegisterU8::L)),
        ),
        // LD B (HL)
        0x46 => Instruction::LD(
            Operand::U8(OperandU8::Register(RegisterU8::B)),
            Operand::U8(OperandU8::Memory(OperandU16::RegisterPair(RegisterU16::HL))),
        ),
        // LD B A
        0x47 => Instruction::LD(
            Operand::U8(OperandU8::Register(RegisterU8::B)),
            Operand::U8(OperandU8::Register(RegisterU8::A)),
        ),
        // LD C B
        0x48 => Instruction::LD(
            Operand::U8(OperandU8::Register(RegisterU8::C)),
            Operand::U8(OperandU8::Register(RegisterU8::B)),
        ),
        // LD C C
        0x49 => Instruction::LD(
            Operand::U8(OperandU8::Register(RegisterU8::C)),
            Operand::U8(OperandU8::Register(RegisterU8::C)),
        ),
        // LD C D
        0x4A => Instruction::LD(
            Operand::U8(OperandU8::Register(RegisterU8::C)),
            Operand::U8(OperandU8::Register(RegisterU8::D)),
        ),
        // LD C E
        0x4B => Instruction::LD(
            Operand::U8(OperandU8::Register(RegisterU8::C)),
            Operand::U8(OperandU8::Register(RegisterU8::E)),
        ),
        // LD C H
        0x4C => Instruction::LD(
            Operand::U8(OperandU8::Register(RegisterU8::C)),
            Operand::U8(OperandU8::Register(RegisterU8::H)),
        ),
        // LD C L
        0x4D => Instruction::LD(
            Operand::U8(OperandU8::Register(RegisterU8::C)),
            Operand::U8(OperandU8::Register(RegisterU8::L)),
        ),
        // LD C (HL)
        0x4E => Instruction::LD(
            Operand::U8(OperandU8::Register(RegisterU8::C)),
            Operand::U8(OperandU8::Memory(OperandU16::RegisterPair(RegisterU16::HL))),
        ),
        // LD C A
        0x4F => Instruction::LD(
            Operand::U8(OperandU8::Register(RegisterU8::C)),
            Operand::U8(OperandU8::Register(RegisterU8::A)),
        ),
        // LD D B
        0x50 => Instruction::LD(
            Operand::U8(OperandU8::Register(RegisterU8::D)),
            Operand::U8(OperandU8::Register(RegisterU8::B)),
        ),
        // LD D C
        0x51 => Instruction::LD(
            Operand::U8(OperandU8::Register(RegisterU8::D)),
            Operand::U8(OperandU8::Register(RegisterU8::C)),
        ),
        // LD D D
        0x52 => Instruction::LD(
            Operand::U8(OperandU8::Register(RegisterU8::D)),
            Operand::U8(OperandU8::Register(RegisterU8::D)),
        ),
        // LD D E
        0x53 => Instruction::LD(
            Operand::U8(OperandU8::Register(RegisterU8::D)),
            Operand::U8(OperandU8::Register(RegisterU8::E)),
        ),
        // LD D H
        0x54 => Instruction::LD(
            Operand::U8(OperandU8::Register(RegisterU8::D)),
            Operand::U8(OperandU8::Register(RegisterU8::H)),
        ),
        // LD D L
        0x55 => Instruction::LD(
            Operand::U8(OperandU8::Register(RegisterU8::D)),
            Operand::U8(OperandU8::Register(RegisterU8::L)),
        ),
        // LD D (HL)
        0x56 => Instruction::LD(
            Operand::U8(OperandU8::Register(RegisterU8::D)),
            Operand::U8(OperandU8::Memory(OperandU16::RegisterPair(RegisterU16::HL))),
        ),
        // LD D A
        0x57 => Instruction::LD(
            Operand::U8(OperandU8::Register(RegisterU8::D)),
            Operand::U8(OperandU8::Register(RegisterU8::A)),
        ),
        // LD E B
        0x58 => Instruction::LD(
            Operand::U8(OperandU8::Register(RegisterU8::E)),
            Operand::U8(OperandU8::Register(RegisterU8::B)),
        ),
        // LD E C
        0x59 => Instruction::LD(
            Operand::U8(OperandU8::Register(RegisterU8::E)),
            Operand::U8(OperandU8::Register(RegisterU8::C)),
        ),
        // LD E D
        0x5A => Instruction::LD(
            Operand::U8(OperandU8::Register(RegisterU8::E)),
            Operand::U8(OperandU8::Register(RegisterU8::D)),
        ),
        // LD E E
        0x5B => Instruction::LD(
            Operand::U8(OperandU8::Register(RegisterU8::E)),
            Operand::U8(OperandU8::Register(RegisterU8::E)),
        ),
        // LD E H
        0x5C => Instruction::LD(
            Operand::U8(OperandU8::Register(RegisterU8::E)),
            Operand::U8(OperandU8::Register(RegisterU8::H)),
        ),
        // LD E L
        0x5D => Instruction::LD(
            Operand::U8(OperandU8::Register(RegisterU8::E)),
            Operand::U8(OperandU8::Register(RegisterU8::L)),
        ),
        // LD E (HL)
        0x5E => Instruction::LD(
            Operand::U8(OperandU8::Register(RegisterU8::E)),
            Operand::U8(OperandU8::Memory(OperandU16::RegisterPair(RegisterU16::HL))),
        ),
        // LD E A
        0x5F => Instruction::LD(
            Operand::U8(OperandU8::Register(RegisterU8::E)),
            Operand::U8(OperandU8::Register(RegisterU8::A)),
        ),
        // LD H B
        0x60 => Instruction::LD(
            Operand::U8(OperandU8::Register(RegisterU8::H)),
            Operand::U8(OperandU8::Register(RegisterU8::B)),
        ),
        // LD H C
        0x61 => Instruction::LD(
            Operand::U8(OperandU8::Register(RegisterU8::H)),
            Operand::U8(OperandU8::Register(RegisterU8::C)),
        ),
        // LD H D
        0x62 => Instruction::LD(
            Operand::U8(OperandU8::Register(RegisterU8::H)),
            Operand::U8(OperandU8::Register(RegisterU8::D)),
        ),
        // LD H E
        0x63 => Instruction::LD(
            Operand::U8(OperandU8::Register(RegisterU8::H)),
            Operand::U8(OperandU8::Register(RegisterU8::E)),
        ),
        // LD H H
        0x64 => Instruction::LD(
            Operand::U8(OperandU8::Register(RegisterU8::H)),
            Operand::U8(OperandU8::Register(RegisterU8::H)),
        ),
        // LD H L
        0x65 => Instruction::LD(
            Operand::U8(OperandU8::Register(RegisterU8::H)),
            Operand::U8(OperandU8::Register(RegisterU8::L)),
        ),
        // LD H (HL)
        0x66 => Instruction::LD(
            Operand::U8(OperandU8::Register(RegisterU8::H)),
            Operand::U8(OperandU8::Memory(OperandU16::RegisterPair(RegisterU16::HL))),
        ),
        // LD H A
        0x67 => Instruction::LD(
            Operand::U8(OperandU8::Register(RegisterU8::H)),
            Operand::U8(OperandU8::Register(RegisterU8::A)),
        ),
        // LD L B
        0x68 => Instruction::LD(
            Operand::U8(OperandU8::Register(RegisterU8::L)),
            Operand::U8(OperandU8::Register(RegisterU8::B)),
        ),
        // LD L C
        0x69 => Instruction::LD(
            Operand::U8(OperandU8::Register(RegisterU8::L)),
            Operand::U8(OperandU8::Register(RegisterU8::C)),
        ),
        // LD L D
        0x6A => Instruction::LD(
            Operand::U8(OperandU8::Register(RegisterU8::L)),
            Operand::U8(OperandU8::Register(RegisterU8::D)),
        ),
        // LD L E
        0x6B => Instruction::LD(
            Operand::U8(OperandU8::Register(RegisterU8::L)),
            Operand::U8(OperandU8::Register(RegisterU8::E)),
        ),
        // LD L H
        0x6C => Instruction::LD(
            Operand::U8(OperandU8::Register(RegisterU8::L)),
            Operand::U8(OperandU8::Register(RegisterU8::H)),
        ),
        // LD L L
        0x6D => Instruction::LD(
            Operand::U8(OperandU8::Register(RegisterU8::L)),
            Operand::U8(OperandU8::Register(RegisterU8::L)),
        ),
        // LD L (HL)
        0x6E => Instruction::LD(
            Operand::U8(OperandU8::Register(RegisterU8::L)),
            Operand::U8(OperandU8::Memory(OperandU16::RegisterPair(RegisterU16::HL))),
        ),
        // LD L A
        0x6F => Instruction::LD(
            Operand::U8(OperandU8::Register(RegisterU8::L)),
            Operand::U8(OperandU8::Register(RegisterU8::A)),
        ),
        // LD (HL) B
        0x70 => Instruction::LD(
            Operand::U8(OperandU8::Memory(OperandU16::RegisterPair(RegisterU16::HL))),
            Operand::U8(OperandU8::Register(RegisterU8::B)),
        ),
        // LD (HL) C
        0x71 => Instruction::LD(
            Operand::U8(OperandU8::Memory(OperandU16::RegisterPair(RegisterU16::HL))),
            Operand::U8(OperandU8::Register(RegisterU8::C)),
        ),
        // LD (HL) D
        0x72 => Instruction::LD(
            Operand::U8(OperandU8::Memory(OperandU16::RegisterPair(RegisterU16::HL))),
            Operand::U8(OperandU8::Register(RegisterU8::D)),
        ),
        // LD (HL) E
        0x73 => Instruction::LD(
            Operand::U8(OperandU8::Memory(OperandU16::RegisterPair(RegisterU16::HL))),
            Operand::U8(OperandU8::Register(RegisterU8::E)),
        ),
        // LD (HL) H
        0x74 => Instruction::LD(
            Operand::U8(OperandU8::Memory(OperandU16::RegisterPair(RegisterU16::HL))),
            Operand::U8(OperandU8::Register(RegisterU8::H)),
        ),
        // LD (HL) L
        0x75 => Instruction::LD(
            Operand::U8(OperandU8::Memory(OperandU16::RegisterPair(RegisterU16::HL))),
            Operand::U8(OperandU8::Register(RegisterU8::L)),
        ),
        // HALT
        0x76 => Instruction::HALT,
        // LD (HL) A
        0x77 => Instruction::LD(
            Operand::U8(OperandU8::Memory(OperandU16::RegisterPair(RegisterU16::HL))),
            Operand::U8(OperandU8::Register(RegisterU8::A)),
        ),
        // LD A B
        0x78 => Instruction::LD(
            Operand::U8(OperandU8::Register(RegisterU8::A)),
            Operand::U8(OperandU8::Register(RegisterU8::B)),
        ),
        // LD A C
        0x79 => Instruction::LD(
            Operand::U8(OperandU8::Register(RegisterU8::A)),
            Operand::U8(OperandU8::Register(RegisterU8::C)),
        ),
        // LD A D
        0x7A => Instruction::LD(
            Operand::U8(OperandU8::Register(RegisterU8::A)),
            Operand::U8(OperandU8::Register(RegisterU8::D)),
        ),
        // LD A E
        0x7B => Instruction::LD(
            Operand::U8(OperandU8::Register(RegisterU8::A)),
            Operand::U8(OperandU8::Register(RegisterU8::E)),
        ),
        // LD A H
        0x7C => Instruction::LD(
            Operand::U8(OperandU8::Register(RegisterU8::A)),
            Operand::U8(OperandU8::Register(RegisterU8::H)),
        ),
        // LD A L
        0x7D => Instruction::LD(
            Operand::U8(OperandU8::Register(RegisterU8::A)),
            Operand::U8(OperandU8::Register(RegisterU8::L)),
        ),
        // LD A (HL)
        0x7E => Instruction::LD(
            Operand::U8(OperandU8::Register(RegisterU8::A)),
            Operand::U8(OperandU8::Memory(OperandU16::RegisterPair(RegisterU16::HL))),
        ),
        // LD A A
        0x7F => Instruction::LD(
            Operand::U8(OperandU8::Register(RegisterU8::A)),
            Operand::U8(OperandU8::Register(RegisterU8::A)),
        ),
        // ADD A B
        0x80 => Instruction::ADD(
            Operand::U8(OperandU8::Register(RegisterU8::A)),
            Operand::U8(OperandU8::Register(RegisterU8::B)),
        ),
        // ADD A C
        0x81 => Instruction::ADD(
            Operand::U8(OperandU8::Register(RegisterU8::A)),
            Operand::U8(OperandU8::Register(RegisterU8::C)),
        ),
        // ADD A D
        0x82 => Instruction::ADD(
            Operand::U8(OperandU8::Register(RegisterU8::A)),
            Operand::U8(OperandU8::Register(RegisterU8::D)),
        ),
        // ADD A E
        0x83 => Instruction::ADD(
            Operand::U8(OperandU8::Register(RegisterU8::A)),
            Operand::U8(OperandU8::Register(RegisterU8::E)),
        ),
        // ADD A H
        0x84 => Instruction::ADD(
            Operand::U8(OperandU8::Register(RegisterU8::A)),
            Operand::U8(OperandU8::Register(RegisterU8::H)),
        ),
        // ADD A L
        0x85 => Instruction::ADD(
            Operand::U8(OperandU8::Register(RegisterU8::A)),
            Operand::U8(OperandU8::Register(RegisterU8::L)),
        ),
        // ADD A (HL)
        0x86 => Instruction::ADD(
            Operand::U8(OperandU8::Register(RegisterU8::A)),
            Operand::U8(OperandU8::Memory(OperandU16::RegisterPair(RegisterU16::HL))),
        ),
        // ADD A A
        0x87 => Instruction::ADD(
            Operand::U8(OperandU8::Register(RegisterU8::A)),
            Operand::U8(OperandU8::Register(RegisterU8::A)),
        ),
        // ADC A B
        0x88 => Instruction::ADC(
            OperandU8::Register(RegisterU8::A),
            OperandU8::Register(RegisterU8::B),
        ),
        // ADC A C
        0x89 => Instruction::ADC(
            OperandU8::Register(RegisterU8::A),
            OperandU8::Register(RegisterU8::C),
        ),
        // ADC A D
        0x8A => Instruction::ADC(
            OperandU8::Register(RegisterU8::A),
            OperandU8::Register(RegisterU8::D),
        ),
        // ADC A E
        0x8B => Instruction::ADC(
            OperandU8::Register(RegisterU8::A),
            OperandU8::Register(RegisterU8::E),
        ),
        // ADC A H
        0x8C => Instruction::ADC(
            OperandU8::Register(RegisterU8::A),
            OperandU8::Register(RegisterU8::H),
        ),
        // ADC A L
        0x8D => Instruction::ADC(
            OperandU8::Register(RegisterU8::A),
            OperandU8::Register(RegisterU8::L),
        ),
        // ADC A (HL)
        0x8E => Instruction::ADC(
            OperandU8::Register(RegisterU8::A),
            OperandU8::Memory(OperandU16::RegisterPair(RegisterU16::HL)),
        ),
        // ADC A A
        0x8F => Instruction::ADC(
            OperandU8::Register(RegisterU8::A),
            OperandU8::Register(RegisterU8::A),
        ),
        // SUB A B
        0x90 => Instruction::SUB(
            OperandU8::Register(RegisterU8::A),
            OperandU8::Register(RegisterU8::B),
        ),
        // SUB A C
        0x91 => Instruction::SUB(
            OperandU8::Register(RegisterU8::A),
            OperandU8::Register(RegisterU8::C),
        ),
        // SUB A D
        0x92 => Instruction::SUB(
            OperandU8::Register(RegisterU8::A),
            OperandU8::Register(RegisterU8::D),
        ),
        // SUB A E
        0x93 => Instruction::SUB(
            OperandU8::Register(RegisterU8::A),
            OperandU8::Register(RegisterU8::E),
        ),
        // SUB A H
        0x94 => Instruction::SUB(
            OperandU8::Register(RegisterU8::A),
            OperandU8::Register(RegisterU8::H),
        ),
        // SUB A L
        0x95 => Instruction::SUB(
            OperandU8::Register(RegisterU8::A),
            OperandU8::Register(RegisterU8::L),
        ),
        // SUB A (HL)
        0x96 => Instruction::SUB(
            OperandU8::Register(RegisterU8::A),
            OperandU8::Memory(OperandU16::RegisterPair(RegisterU16::HL)),
        ),
        // SUB A A
        0x97 => Instruction::SUB(
            OperandU8::Register(RegisterU8::A),
            OperandU8::Register(RegisterU8::A),
        ),
        // SBC A B
        0x98 => Instruction::SBC(
            OperandU8::Register(RegisterU8::A),
            OperandU8::Register(RegisterU8::B),
        ),
        // SBC A C
        0x99 => Instruction::SBC(
            OperandU8::Register(RegisterU8::A),
            OperandU8::Register(RegisterU8::C),
        ),
        // SBC A D
        0x9A => Instruction::SBC(
            OperandU8::Register(RegisterU8::A),
            OperandU8::Register(RegisterU8::D),
        ),
        // SBC A E
        0x9B => Instruction::SBC(
            OperandU8::Register(RegisterU8::A),
            OperandU8::Register(RegisterU8::E),
        ),
        // SBC A H
        0x9C => Instruction::SBC(
            OperandU8::Register(RegisterU8::A),
            OperandU8::Register(RegisterU8::H),
        ),
        // SBC A L
        0x9D => Instruction::SBC(
            OperandU8::Register(RegisterU8::A),
            OperandU8::Register(RegisterU8::L),
        ),
        // SBC A (HL)
        0x9E => Instruction::SBC(
            OperandU8::Register(RegisterU8::A),
            OperandU8::Memory(OperandU16::RegisterPair(RegisterU16::HL)),
        ),
        // SBC A A
        0x9F => Instruction::SBC(
            OperandU8::Register(RegisterU8::A),
            OperandU8::Register(RegisterU8::A),
        ),
        // AND A B
        0xA0 => Instruction::AND(
            OperandU8::Register(RegisterU8::A),
            OperandU8::Register(RegisterU8::B),
        ),
        // AND A C
        0xA1 => Instruction::AND(
            OperandU8::Register(RegisterU8::A),
            OperandU8::Register(RegisterU8::C),
        ),
        // AND A D
        0xA2 => Instruction::AND(
            OperandU8::Register(RegisterU8::A),
            OperandU8::Register(RegisterU8::D),
        ),
        // AND A E
        0xA3 => Instruction::AND(
            OperandU8::Register(RegisterU8::A),
            OperandU8::Register(RegisterU8::E),
        ),
        // AND A H
        0xA4 => Instruction::AND(
            OperandU8::Register(RegisterU8::A),
            OperandU8::Register(RegisterU8::H),
        ),
        // AND A L
        0xA5 => Instruction::AND(
            OperandU8::Register(RegisterU8::A),
            OperandU8::Register(RegisterU8::L),
        ),
        // AND A (HL)
        0xA6 => Instruction::AND(
            OperandU8::Register(RegisterU8::A),
            OperandU8::Memory(OperandU16::RegisterPair(RegisterU16::HL)),
        ),
        // AND A A
        0xA7 => Instruction::AND(
            OperandU8::Register(RegisterU8::A),
            OperandU8::Register(RegisterU8::A),
        ),
        // XOR A B
        0xA8 => Instruction::XOR(
            OperandU8::Register(RegisterU8::A),
            OperandU8::Register(RegisterU8::B),
        ),
        // XOR A C
        0xA9 => Instruction::XOR(
            OperandU8::Register(RegisterU8::A),
            OperandU8::Register(RegisterU8::C),
        ),
        // XOR A D
        0xAA => Instruction::XOR(
            OperandU8::Register(RegisterU8::A),
            OperandU8::Register(RegisterU8::D),
        ),
        // XOR A E
        0xAB => Instruction::XOR(
            OperandU8::Register(RegisterU8::A),
            OperandU8::Register(RegisterU8::E),
        ),
        // XOR A H
        0xAC => Instruction::XOR(
            OperandU8::Register(RegisterU8::A),
            OperandU8::Register(RegisterU8::H),
        ),
        // XOR A L
        0xAD => Instruction::XOR(
            OperandU8::Register(RegisterU8::A),
            OperandU8::Register(RegisterU8::L),
        ),
        // XOR A (HL)
        0xAE => Instruction::XOR(
            OperandU8::Register(RegisterU8::A),
            OperandU8::Memory(OperandU16::RegisterPair(RegisterU16::HL)),
        ),
        // XOR A A
        0xAF => Instruction::XOR(
            OperandU8::Register(RegisterU8::A),
            OperandU8::Register(RegisterU8::A),
        ),
        // OR A B
        0xB0 => Instruction::OR(
            OperandU8::Register(RegisterU8::A),
            OperandU8::Register(RegisterU8::B),
        ),
        // OR A C
        0xB1 => Instruction::OR(
            OperandU8::Register(RegisterU8::A),
            OperandU8::Register(RegisterU8::C),
        ),
        // OR A D
        0xB2 => Instruction::OR(
            OperandU8::Register(RegisterU8::A),
            OperandU8::Register(RegisterU8::D),
        ),
        // OR A E
        0xB3 => Instruction::OR(
            OperandU8::Register(RegisterU8::A),
            OperandU8::Register(RegisterU8::E),
        ),
        // OR A H
        0xB4 => Instruction::OR(
            OperandU8::Register(RegisterU8::A),
            OperandU8::Register(RegisterU8::H),
        ),
        // OR A L
        0xB5 => Instruction::OR(
            OperandU8::Register(RegisterU8::A),
            OperandU8::Register(RegisterU8::L),
        ),
        // OR A (HL)
        0xB6 => Instruction::OR(
            OperandU8::Register(RegisterU8::A),
            OperandU8::Memory(OperandU16::RegisterPair(RegisterU16::HL)),
        ),
        // OR A A
        0xB7 => Instruction::OR(
            OperandU8::Register(RegisterU8::A),
            OperandU8::Register(RegisterU8::A),
        ),
        // CP A B
        0xB8 => Instruction::CP(
            OperandU8::Register(RegisterU8::A),
            OperandU8::Register(RegisterU8::B),
        ),
        // CP A C
        0xB9 => Instruction::CP(
            OperandU8::Register(RegisterU8::A),
            OperandU8::Register(RegisterU8::C),
        ),
        // CP A D
        0xBA => Instruction::CP(
            OperandU8::Register(RegisterU8::A),
            OperandU8::Register(RegisterU8::D),
        ),
        // CP A E
        0xBB => Instruction::CP(
            OperandU8::Register(RegisterU8::A),
            OperandU8::Register(RegisterU8::E),
        ),
        // CP A H
        0xBC => Instruction::CP(
            OperandU8::Register(RegisterU8::A),
            OperandU8::Register(RegisterU8::H),
        ),
        // CP A L
        0xBD => Instruction::CP(
            OperandU8::Register(RegisterU8::A),
            OperandU8::Register(RegisterU8::L),
        ),
        // CP A (HL)
        0xBE => Instruction::CP(
            OperandU8::Register(RegisterU8::A),
            OperandU8::Memory(OperandU16::RegisterPair(RegisterU16::HL)),
        ),
        // CP A A
        0xBF => Instruction::CP(
            OperandU8::Register(RegisterU8::A),
            OperandU8::Register(RegisterU8::A),
        ),
        // RET NZ
        0xC0 => Instruction::RET(Some(FlagCondition::NZ)),
        // POP BC
        0xC1 => Instruction::POP(RegisterU16::BC),
        // JP NZ a16
        0xC2 => Instruction::JP(Some(FlagCondition::NZ), OperandU16::Immediate),
        // JP a16
        0xC3 => Instruction::JP(None, OperandU16::Immediate),
        // CALL NZ a16
        0xC4 => Instruction::CALL(Some(FlagCondition::NZ), OperandU16::Immediate),
        // PUSH BC
        0xC5 => Instruction::PUSH(RegisterU16::BC),
        // ADD A n8
        0xC6 => Instruction::ADD(
            Operand::U8(OperandU8::Register(RegisterU8::A)),
            Operand::U8(OperandU8::Immediate),
        ),
        // RST $00
        0xC7 => Instruction::RST(0),
        // RET Z
        0xC8 => Instruction::RET(Some(FlagCondition::Z)),
        // RET
        0xC9 => Instruction::RET(None),
        // JP Z a16
        0xCA => Instruction::JP(Some(FlagCondition::Z), OperandU16::Immediate),
        // CALL Z a16
        0xCC => Instruction::CALL(Some(FlagCondition::Z), OperandU16::Immediate),
        // CALL a16
        0xCD => Instruction::CALL(None, OperandU16::Immediate),
        // ADC A n8
        0xCE => Instruction::ADC(OperandU8::Register(RegisterU8::A), OperandU8::Immediate),
        // RST $08
        0xCF => Instruction::RST(1),
        // RET NC
        0xD0 => Instruction::RET(Some(FlagCondition::NC)),
        // POP DE
        0xD1 => Instruction::POP(RegisterU16::DE),
        // JP NC a16
        0xD2 => Instruction::JP(Some(FlagCondition::NC), OperandU16::Immediate),
        // CALL NC a16
        0xD4 => Instruction::CALL(Some(FlagCondition::NC), OperandU16::Immediate),
        // PUSH DE
        0xD5 => Instruction::PUSH(RegisterU16::DE),
        // SUB A n8
        0xD6 => Instruction::SUB(OperandU8::Register(RegisterU8::A), OperandU8::Immediate),
        // RST $10
        0xD7 => Instruction::RST(2),
        // RET C
        0xD8 => Instruction::RET(Some(FlagCondition::C)),
        // RETI
        0xD9 => Instruction::RETI,
        // JP C a16
        0xDA => Instruction::JP(Some(FlagCondition::C), OperandU16::Immediate),
        // CALL C a16
        0xDC => Instruction::CALL(Some(FlagCondition::C), OperandU16::Immediate),
        // SBC A n8
        0xDE => Instruction::SBC(OperandU8::Register(RegisterU8::A), OperandU8::Immediate),
        // RST $18
        0xDF => Instruction::RST(3),
        // LDH (a8) A
        0xE0 => Instruction::LDH(
            OperandU8::MemoryU8(Box::new(OperandU8::Immediate)),
            OperandU8::Register(RegisterU8::A),
        ),
        // POP HL
        0xE1 => Instruction::POP(RegisterU16::HL),
        // LDH (C) A
        0xE2 => Instruction::LDH(
            OperandU8::MemoryU8(Box::new(OperandU8::Register(RegisterU8::C))),
            OperandU8::Register(RegisterU8::A),
        ),
        // PUSH HL
        0xE5 => Instruction::PUSH(RegisterU16::HL),
        // AND A n8
        0xE6 => Instruction::AND(OperandU8::Register(RegisterU8::A), OperandU8::Immediate),
        // RST $20
        0xE7 => Instruction::RST(4),
        // ADD SP e8
        0xE8 => Instruction::ADD(
            Operand::U16(OperandU16::RegisterPair(RegisterU16::SP)),
            Operand::U8(OperandU8::Immediate),
        ),
        // JP HL
        0xE9 => Instruction::JP(None, OperandU16::RegisterPair(RegisterU16::HL)),
        // LD (a16) A
        0xEA => Instruction::LD(
            Operand::U8(OperandU8::Memory(OperandU16::Immediate)),
            Operand::U8(OperandU8::Register(RegisterU8::A)),
        ),
        // XOR A n8
        0xEE => Instruction::XOR(OperandU8::Register(RegisterU8::A), OperandU8::Immediate),
        // RST $28
        0xEF => Instruction::RST(5),
        // LDH A (a8)
        0xF0 => Instruction::LDH(
            OperandU8::Register(RegisterU8::A),
            OperandU8::MemoryU8(Box::new(OperandU8::Immediate)),
        ),
        // POP AF
        0xF1 => Instruction::POP(RegisterU16::AF),
        // LDH A (C)
        0xF2 => Instruction::LDH(
            OperandU8::Register(RegisterU8::A),
            OperandU8::MemoryU8(Box::new(OperandU8::Register(RegisterU8::C))),
        ),
        // DI
        0xF3 => Instruction::DI,
        // PUSH AF
        0xF5 => Instruction::PUSH(RegisterU16::AF),
        // OR A n8
        0xF6 => Instruction::OR(OperandU8::Register(RegisterU8::A), OperandU8::Immediate),
        // RST $30
        0xF7 => Instruction::RST(6),
        // LD HL SP+e8
        0xF8 => Instruction::LDF8,
        // LD SP HL
        0xF9 => Instruction::LD(
            Operand::U16(OperandU16::RegisterPair(RegisterU16::SP)),
            Operand::U16(OperandU16::RegisterPair(RegisterU16::HL)),
        ),
        // LD A (a16)
        0xFA => Instruction::LD(
            Operand::U8(OperandU8::Register(RegisterU8::A)),
            Operand::U8(OperandU8::Memory(OperandU16::Immediate)),
        ),
        // EI
        0xFB => Instruction::EI,
        // CP A n8
        0xFE => Instruction::CP(OperandU8::Register(RegisterU8::A), OperandU8::Immediate),
        // RST $38
        0xFF => Instruction::RST(7),
        // PREFIX
        0xCB => unreachable!(),
        _ => Instruction::ILLEGAL(byte),
    }
}
