use crate::{
    instructions::{Instruction, Operand, OperandU8, OperandU16},
    registers::{RegisterU8, RegisterU16},
};

pub(crate) fn decode_byte(byte: u8) -> Instruction {
    match byte {
        //RLC B
        0x00 => Instruction::RLC(Operand::U8(OperandU8::Register(RegisterU8::B))),
        //RLC C
        0x01 => Instruction::RLC(Operand::U8(OperandU8::Register(RegisterU8::C))),
        //RLC D
        0x02 => Instruction::RLC(Operand::U8(OperandU8::Register(RegisterU8::D))),
        //RLC E
        0x03 => Instruction::RLC(Operand::U8(OperandU8::Register(RegisterU8::E))),
        //RLC H
        0x04 => Instruction::RLC(Operand::U8(OperandU8::Register(RegisterU8::H))),
        //RLC L
        0x05 => Instruction::RLC(Operand::U8(OperandU8::Register(RegisterU8::L))),
        //RLC (HL)
        0x06 => Instruction::RLC(Operand::U8(OperandU8::Memory(OperandU16::RegisterPair(
            RegisterU16::HL,
        )))),
        //RLC A
        0x07 => Instruction::RLC(Operand::U8(OperandU8::Register(RegisterU8::A))),
        //RRC B
        0x08 => Instruction::RRC(Operand::U8(OperandU8::Register(RegisterU8::B))),
        //RRC C
        0x09 => Instruction::RRC(Operand::U8(OperandU8::Register(RegisterU8::C))),
        //RRC D
        0x0A => Instruction::RRC(Operand::U8(OperandU8::Register(RegisterU8::D))),
        //RRC E
        0x0B => Instruction::RRC(Operand::U8(OperandU8::Register(RegisterU8::E))),
        //RRC H
        0x0C => Instruction::RRC(Operand::U8(OperandU8::Register(RegisterU8::H))),
        //RRC L
        0x0D => Instruction::RRC(Operand::U8(OperandU8::Register(RegisterU8::L))),
        //RRC (HL)
        0x0E => Instruction::RRC(Operand::U8(OperandU8::Memory(OperandU16::RegisterPair(
            RegisterU16::HL,
        )))),
        //RRC A
        0x0F => Instruction::RRC(Operand::U8(OperandU8::Register(RegisterU8::A))),
        //RL B
        0x10 => Instruction::RL(Operand::U8(OperandU8::Register(RegisterU8::B))),
        //RL C
        0x11 => Instruction::RL(Operand::U8(OperandU8::Register(RegisterU8::C))),
        //RL D
        0x12 => Instruction::RL(Operand::U8(OperandU8::Register(RegisterU8::D))),
        //RL E
        0x13 => Instruction::RL(Operand::U8(OperandU8::Register(RegisterU8::E))),
        //RL H
        0x14 => Instruction::RL(Operand::U8(OperandU8::Register(RegisterU8::H))),
        //RL L
        0x15 => Instruction::RL(Operand::U8(OperandU8::Register(RegisterU8::L))),
        //RL (HL)
        0x16 => Instruction::RL(Operand::U8(OperandU8::Memory(OperandU16::RegisterPair(
            RegisterU16::HL,
        )))),
        //RL A
        0x17 => Instruction::RL(Operand::U8(OperandU8::Register(RegisterU8::A))),
        //RR B
        0x18 => Instruction::RR(Operand::U8(OperandU8::Register(RegisterU8::B))),
        //RR C
        0x19 => Instruction::RR(Operand::U8(OperandU8::Register(RegisterU8::C))),
        //RR D
        0x1A => Instruction::RR(Operand::U8(OperandU8::Register(RegisterU8::D))),
        //RR E
        0x1B => Instruction::RR(Operand::U8(OperandU8::Register(RegisterU8::E))),
        //RR H
        0x1C => Instruction::RR(Operand::U8(OperandU8::Register(RegisterU8::H))),
        //RR L
        0x1D => Instruction::RR(Operand::U8(OperandU8::Register(RegisterU8::L))),
        //RR (HL)
        0x1E => Instruction::RR(Operand::U8(OperandU8::Memory(OperandU16::RegisterPair(
            RegisterU16::HL,
        )))),
        //RR A
        0x1F => Instruction::RR(Operand::U8(OperandU8::Register(RegisterU8::A))),
        //SLA B
        0x20 => Instruction::SLA(Operand::U8(OperandU8::Register(RegisterU8::B))),
        //SLA C
        0x21 => Instruction::SLA(Operand::U8(OperandU8::Register(RegisterU8::C))),
        //SLA D
        0x22 => Instruction::SLA(Operand::U8(OperandU8::Register(RegisterU8::D))),
        //SLA E
        0x23 => Instruction::SLA(Operand::U8(OperandU8::Register(RegisterU8::E))),
        //SLA H
        0x24 => Instruction::SLA(Operand::U8(OperandU8::Register(RegisterU8::H))),
        //SLA L
        0x25 => Instruction::SLA(Operand::U8(OperandU8::Register(RegisterU8::L))),
        //SLA (HL)
        0x26 => Instruction::SLA(Operand::U8(OperandU8::Memory(OperandU16::RegisterPair(
            RegisterU16::HL,
        )))),
        //SLA A
        0x27 => Instruction::SLA(Operand::U8(OperandU8::Register(RegisterU8::A))),
        //SRA B
        0x28 => Instruction::SRA(Operand::U8(OperandU8::Register(RegisterU8::B))),
        //SRA C
        0x29 => Instruction::SRA(Operand::U8(OperandU8::Register(RegisterU8::C))),
        //SRA D
        0x2A => Instruction::SRA(Operand::U8(OperandU8::Register(RegisterU8::D))),
        //SRA E
        0x2B => Instruction::SRA(Operand::U8(OperandU8::Register(RegisterU8::E))),
        //SRA H
        0x2C => Instruction::SRA(Operand::U8(OperandU8::Register(RegisterU8::H))),
        //SRA L
        0x2D => Instruction::SRA(Operand::U8(OperandU8::Register(RegisterU8::L))),
        //SRA (HL)
        0x2E => Instruction::SRA(Operand::U8(OperandU8::Memory(OperandU16::RegisterPair(
            RegisterU16::HL,
        )))),
        //SRA A
        0x2F => Instruction::SRA(Operand::U8(OperandU8::Register(RegisterU8::A))),
        //SWAP B
        0x30 => Instruction::SWAP(Operand::U8(OperandU8::Register(RegisterU8::B))),
        //SWAP C
        0x31 => Instruction::SWAP(Operand::U8(OperandU8::Register(RegisterU8::C))),
        //SWAP D
        0x32 => Instruction::SWAP(Operand::U8(OperandU8::Register(RegisterU8::D))),
        //SWAP E
        0x33 => Instruction::SWAP(Operand::U8(OperandU8::Register(RegisterU8::E))),
        //SWAP H
        0x34 => Instruction::SWAP(Operand::U8(OperandU8::Register(RegisterU8::H))),
        //SWAP L
        0x35 => Instruction::SWAP(Operand::U8(OperandU8::Register(RegisterU8::L))),
        //SWAP (HL)
        0x36 => Instruction::SWAP(Operand::U8(OperandU8::Memory(OperandU16::RegisterPair(
            RegisterU16::HL,
        )))),
        //SWAP A
        0x37 => Instruction::SWAP(Operand::U8(OperandU8::Register(RegisterU8::A))),
        //SRL B
        0x38 => Instruction::SRL(Operand::U8(OperandU8::Register(RegisterU8::B))),
        //SRL C
        0x39 => Instruction::SRL(Operand::U8(OperandU8::Register(RegisterU8::C))),
        //SRL D
        0x3A => Instruction::SRL(Operand::U8(OperandU8::Register(RegisterU8::D))),
        //SRL E
        0x3B => Instruction::SRL(Operand::U8(OperandU8::Register(RegisterU8::E))),
        //SRL H
        0x3C => Instruction::SRL(Operand::U8(OperandU8::Register(RegisterU8::H))),
        //SRL L
        0x3D => Instruction::SRL(Operand::U8(OperandU8::Register(RegisterU8::L))),
        //SRL (HL)
        0x3E => Instruction::SRL(Operand::U8(OperandU8::Memory(OperandU16::RegisterPair(
            RegisterU16::HL,
        )))),
        //SRL A
        0x3F => Instruction::SRL(Operand::U8(OperandU8::Register(RegisterU8::A))),
        //BIT 0 B
        0x40 => Instruction::BIT(0, Operand::U8(OperandU8::Register(RegisterU8::B))),
        //BIT 0 C
        0x41 => Instruction::BIT(0, Operand::U8(OperandU8::Register(RegisterU8::C))),
        //BIT 0 D
        0x42 => Instruction::BIT(0, Operand::U8(OperandU8::Register(RegisterU8::D))),
        //BIT 0 E
        0x43 => Instruction::BIT(0, Operand::U8(OperandU8::Register(RegisterU8::E))),
        //BIT 0 H
        0x44 => Instruction::BIT(0, Operand::U8(OperandU8::Register(RegisterU8::H))),
        //BIT 0 L
        0x45 => Instruction::BIT(0, Operand::U8(OperandU8::Register(RegisterU8::L))),
        //BIT 0 (HL)
        0x46 => Instruction::BIT(
            0,
            Operand::U8(OperandU8::Memory(OperandU16::RegisterPair(RegisterU16::HL))),
        ),
        //BIT 0 A
        0x47 => Instruction::BIT(0, Operand::U8(OperandU8::Register(RegisterU8::A))),
        //BIT 1 B
        0x48 => Instruction::BIT(1, Operand::U8(OperandU8::Register(RegisterU8::B))),
        //BIT 1 C
        0x49 => Instruction::BIT(1, Operand::U8(OperandU8::Register(RegisterU8::C))),
        //BIT 1 D
        0x4A => Instruction::BIT(1, Operand::U8(OperandU8::Register(RegisterU8::D))),
        //BIT 1 E
        0x4B => Instruction::BIT(1, Operand::U8(OperandU8::Register(RegisterU8::E))),
        //BIT 1 H
        0x4C => Instruction::BIT(1, Operand::U8(OperandU8::Register(RegisterU8::H))),
        //BIT 1 L
        0x4D => Instruction::BIT(1, Operand::U8(OperandU8::Register(RegisterU8::L))),
        //BIT 1 (HL)
        0x4E => Instruction::BIT(
            1,
            Operand::U8(OperandU8::Memory(OperandU16::RegisterPair(RegisterU16::HL))),
        ),
        //BIT 1 A
        0x4F => Instruction::BIT(1, Operand::U8(OperandU8::Register(RegisterU8::A))),
        //BIT 2 B
        0x50 => Instruction::BIT(2, Operand::U8(OperandU8::Register(RegisterU8::B))),
        //BIT 2 C
        0x51 => Instruction::BIT(2, Operand::U8(OperandU8::Register(RegisterU8::C))),
        //BIT 2 D
        0x52 => Instruction::BIT(2, Operand::U8(OperandU8::Register(RegisterU8::D))),
        //BIT 2 E
        0x53 => Instruction::BIT(2, Operand::U8(OperandU8::Register(RegisterU8::E))),
        //BIT 2 H
        0x54 => Instruction::BIT(2, Operand::U8(OperandU8::Register(RegisterU8::H))),
        //BIT 2 L
        0x55 => Instruction::BIT(2, Operand::U8(OperandU8::Register(RegisterU8::L))),
        //BIT 2 (HL)
        0x56 => Instruction::BIT(
            2,
            Operand::U8(OperandU8::Memory(OperandU16::RegisterPair(RegisterU16::HL))),
        ),
        //BIT 2 A
        0x57 => Instruction::BIT(2, Operand::U8(OperandU8::Register(RegisterU8::A))),
        //BIT 3 B
        0x58 => Instruction::BIT(3, Operand::U8(OperandU8::Register(RegisterU8::B))),
        //BIT 3 C
        0x59 => Instruction::BIT(3, Operand::U8(OperandU8::Register(RegisterU8::C))),
        //BIT 3 D
        0x5A => Instruction::BIT(3, Operand::U8(OperandU8::Register(RegisterU8::D))),
        //BIT 3 E
        0x5B => Instruction::BIT(3, Operand::U8(OperandU8::Register(RegisterU8::E))),
        //BIT 3 H
        0x5C => Instruction::BIT(3, Operand::U8(OperandU8::Register(RegisterU8::H))),
        //BIT 3 L
        0x5D => Instruction::BIT(3, Operand::U8(OperandU8::Register(RegisterU8::L))),
        //BIT 3 (HL)
        0x5E => Instruction::BIT(
            3,
            Operand::U8(OperandU8::Memory(OperandU16::RegisterPair(RegisterU16::HL))),
        ),
        //BIT 3 A
        0x5F => Instruction::BIT(3, Operand::U8(OperandU8::Register(RegisterU8::A))),
        //BIT 4 B
        0x60 => Instruction::BIT(4, Operand::U8(OperandU8::Register(RegisterU8::B))),
        //BIT 4 C
        0x61 => Instruction::BIT(4, Operand::U8(OperandU8::Register(RegisterU8::C))),
        //BIT 4 D
        0x62 => Instruction::BIT(4, Operand::U8(OperandU8::Register(RegisterU8::D))),
        //BIT 4 E
        0x63 => Instruction::BIT(4, Operand::U8(OperandU8::Register(RegisterU8::E))),
        //BIT 4 H
        0x64 => Instruction::BIT(4, Operand::U8(OperandU8::Register(RegisterU8::H))),
        //BIT 4 L
        0x65 => Instruction::BIT(4, Operand::U8(OperandU8::Register(RegisterU8::L))),
        //BIT 4 (HL)
        0x66 => Instruction::BIT(
            4,
            Operand::U8(OperandU8::Memory(OperandU16::RegisterPair(RegisterU16::HL))),
        ),
        //BIT 4 A
        0x67 => Instruction::BIT(4, Operand::U8(OperandU8::Register(RegisterU8::A))),
        //BIT 5 B
        0x68 => Instruction::BIT(5, Operand::U8(OperandU8::Register(RegisterU8::B))),
        //BIT 5 C
        0x69 => Instruction::BIT(5, Operand::U8(OperandU8::Register(RegisterU8::C))),
        //BIT 5 D
        0x6A => Instruction::BIT(5, Operand::U8(OperandU8::Register(RegisterU8::D))),
        //BIT 5 E
        0x6B => Instruction::BIT(5, Operand::U8(OperandU8::Register(RegisterU8::E))),
        //BIT 5 H
        0x6C => Instruction::BIT(5, Operand::U8(OperandU8::Register(RegisterU8::H))),
        //BIT 5 L
        0x6D => Instruction::BIT(5, Operand::U8(OperandU8::Register(RegisterU8::L))),
        //BIT 5 (HL)
        0x6E => Instruction::BIT(
            5,
            Operand::U8(OperandU8::Memory(OperandU16::RegisterPair(RegisterU16::HL))),
        ),
        //BIT 5 A
        0x6F => Instruction::BIT(5, Operand::U8(OperandU8::Register(RegisterU8::A))),
        //BIT 6 B
        0x70 => Instruction::BIT(6, Operand::U8(OperandU8::Register(RegisterU8::B))),
        //BIT 6 C
        0x71 => Instruction::BIT(6, Operand::U8(OperandU8::Register(RegisterU8::C))),
        //BIT 6 D
        0x72 => Instruction::BIT(6, Operand::U8(OperandU8::Register(RegisterU8::D))),
        //BIT 6 E
        0x73 => Instruction::BIT(6, Operand::U8(OperandU8::Register(RegisterU8::E))),
        //BIT 6 H
        0x74 => Instruction::BIT(6, Operand::U8(OperandU8::Register(RegisterU8::H))),
        //BIT 6 L
        0x75 => Instruction::BIT(6, Operand::U8(OperandU8::Register(RegisterU8::L))),
        //BIT 6 (HL)
        0x76 => Instruction::BIT(
            6,
            Operand::U8(OperandU8::Memory(OperandU16::RegisterPair(RegisterU16::HL))),
        ),
        //BIT 6 A
        0x77 => Instruction::BIT(6, Operand::U8(OperandU8::Register(RegisterU8::A))),
        //BIT 7 B
        0x78 => Instruction::BIT(7, Operand::U8(OperandU8::Register(RegisterU8::B))),
        //BIT 7 C
        0x79 => Instruction::BIT(7, Operand::U8(OperandU8::Register(RegisterU8::C))),
        //BIT 7 D
        0x7A => Instruction::BIT(7, Operand::U8(OperandU8::Register(RegisterU8::D))),
        //BIT 7 E
        0x7B => Instruction::BIT(7, Operand::U8(OperandU8::Register(RegisterU8::E))),
        //BIT 7 H
        0x7C => Instruction::BIT(7, Operand::U8(OperandU8::Register(RegisterU8::H))),
        //BIT 7 L
        0x7D => Instruction::BIT(7, Operand::U8(OperandU8::Register(RegisterU8::L))),
        //BIT 7 (HL)
        0x7E => Instruction::BIT(
            7,
            Operand::U8(OperandU8::Memory(OperandU16::RegisterPair(RegisterU16::HL))),
        ),
        //BIT 7 A
        0x7F => Instruction::BIT(7, Operand::U8(OperandU8::Register(RegisterU8::A))),
        //RES 0 B
        0x80 => Instruction::RES(0, Operand::U8(OperandU8::Register(RegisterU8::B))),
        //RES 0 C
        0x81 => Instruction::RES(0, Operand::U8(OperandU8::Register(RegisterU8::C))),
        //RES 0 D
        0x82 => Instruction::RES(0, Operand::U8(OperandU8::Register(RegisterU8::D))),
        //RES 0 E
        0x83 => Instruction::RES(0, Operand::U8(OperandU8::Register(RegisterU8::E))),
        //RES 0 H
        0x84 => Instruction::RES(0, Operand::U8(OperandU8::Register(RegisterU8::H))),
        //RES 0 L
        0x85 => Instruction::RES(0, Operand::U8(OperandU8::Register(RegisterU8::L))),
        //RES 0 (HL)
        0x86 => Instruction::RES(
            0,
            Operand::U8(OperandU8::Memory(OperandU16::RegisterPair(RegisterU16::HL))),
        ),
        //RES 0 A
        0x87 => Instruction::RES(0, Operand::U8(OperandU8::Register(RegisterU8::A))),
        //RES 1 B
        0x88 => Instruction::RES(1, Operand::U8(OperandU8::Register(RegisterU8::B))),
        //RES 1 C
        0x89 => Instruction::RES(1, Operand::U8(OperandU8::Register(RegisterU8::C))),
        //RES 1 D
        0x8A => Instruction::RES(1, Operand::U8(OperandU8::Register(RegisterU8::D))),
        //RES 1 E
        0x8B => Instruction::RES(1, Operand::U8(OperandU8::Register(RegisterU8::E))),
        //RES 1 H
        0x8C => Instruction::RES(1, Operand::U8(OperandU8::Register(RegisterU8::H))),
        //RES 1 L
        0x8D => Instruction::RES(1, Operand::U8(OperandU8::Register(RegisterU8::L))),
        //RES 1 (HL)
        0x8E => Instruction::RES(
            1,
            Operand::U8(OperandU8::Memory(OperandU16::RegisterPair(RegisterU16::HL))),
        ),
        //RES 1 A
        0x8F => Instruction::RES(1, Operand::U8(OperandU8::Register(RegisterU8::A))),
        //RES 2 B
        0x90 => Instruction::RES(2, Operand::U8(OperandU8::Register(RegisterU8::B))),
        //RES 2 C
        0x91 => Instruction::RES(2, Operand::U8(OperandU8::Register(RegisterU8::C))),
        //RES 2 D
        0x92 => Instruction::RES(2, Operand::U8(OperandU8::Register(RegisterU8::D))),
        //RES 2 E
        0x93 => Instruction::RES(2, Operand::U8(OperandU8::Register(RegisterU8::E))),
        //RES 2 H
        0x94 => Instruction::RES(2, Operand::U8(OperandU8::Register(RegisterU8::H))),
        //RES 2 L
        0x95 => Instruction::RES(2, Operand::U8(OperandU8::Register(RegisterU8::L))),
        //RES 2 (HL)
        0x96 => Instruction::RES(
            2,
            Operand::U8(OperandU8::Memory(OperandU16::RegisterPair(RegisterU16::HL))),
        ),
        //RES 2 A
        0x97 => Instruction::RES(2, Operand::U8(OperandU8::Register(RegisterU8::A))),
        //RES 3 B
        0x98 => Instruction::RES(3, Operand::U8(OperandU8::Register(RegisterU8::B))),
        //RES 3 C
        0x99 => Instruction::RES(3, Operand::U8(OperandU8::Register(RegisterU8::C))),
        //RES 3 D
        0x9A => Instruction::RES(3, Operand::U8(OperandU8::Register(RegisterU8::D))),
        //RES 3 E
        0x9B => Instruction::RES(3, Operand::U8(OperandU8::Register(RegisterU8::E))),
        //RES 3 H
        0x9C => Instruction::RES(3, Operand::U8(OperandU8::Register(RegisterU8::H))),
        //RES 3 L
        0x9D => Instruction::RES(3, Operand::U8(OperandU8::Register(RegisterU8::L))),
        //RES 3 (HL)
        0x9E => Instruction::RES(
            3,
            Operand::U8(OperandU8::Memory(OperandU16::RegisterPair(RegisterU16::HL))),
        ),
        //RES 3 A
        0x9F => Instruction::RES(3, Operand::U8(OperandU8::Register(RegisterU8::A))),
        //RES 4 B
        0xA0 => Instruction::RES(4, Operand::U8(OperandU8::Register(RegisterU8::B))),
        //RES 4 C
        0xA1 => Instruction::RES(4, Operand::U8(OperandU8::Register(RegisterU8::C))),
        //RES 4 D
        0xA2 => Instruction::RES(4, Operand::U8(OperandU8::Register(RegisterU8::D))),
        //RES 4 E
        0xA3 => Instruction::RES(4, Operand::U8(OperandU8::Register(RegisterU8::E))),
        //RES 4 H
        0xA4 => Instruction::RES(4, Operand::U8(OperandU8::Register(RegisterU8::H))),
        //RES 4 L
        0xA5 => Instruction::RES(4, Operand::U8(OperandU8::Register(RegisterU8::L))),
        //RES 4 (HL)
        0xA6 => Instruction::RES(
            4,
            Operand::U8(OperandU8::Memory(OperandU16::RegisterPair(RegisterU16::HL))),
        ),
        //RES 4 A
        0xA7 => Instruction::RES(4, Operand::U8(OperandU8::Register(RegisterU8::A))),
        //RES 5 B
        0xA8 => Instruction::RES(5, Operand::U8(OperandU8::Register(RegisterU8::B))),
        //RES 5 C
        0xA9 => Instruction::RES(5, Operand::U8(OperandU8::Register(RegisterU8::C))),
        //RES 5 D
        0xAA => Instruction::RES(5, Operand::U8(OperandU8::Register(RegisterU8::D))),
        //RES 5 E
        0xAB => Instruction::RES(5, Operand::U8(OperandU8::Register(RegisterU8::E))),
        //RES 5 H
        0xAC => Instruction::RES(5, Operand::U8(OperandU8::Register(RegisterU8::H))),
        //RES 5 L
        0xAD => Instruction::RES(5, Operand::U8(OperandU8::Register(RegisterU8::L))),
        //RES 5 (HL)
        0xAE => Instruction::RES(
            5,
            Operand::U8(OperandU8::Memory(OperandU16::RegisterPair(RegisterU16::HL))),
        ),
        //RES 5 A
        0xAF => Instruction::RES(5, Operand::U8(OperandU8::Register(RegisterU8::A))),
        //RES 6 B
        0xB0 => Instruction::RES(6, Operand::U8(OperandU8::Register(RegisterU8::B))),
        //RES 6 C
        0xB1 => Instruction::RES(6, Operand::U8(OperandU8::Register(RegisterU8::C))),
        //RES 6 D
        0xB2 => Instruction::RES(6, Operand::U8(OperandU8::Register(RegisterU8::D))),
        //RES 6 E
        0xB3 => Instruction::RES(6, Operand::U8(OperandU8::Register(RegisterU8::E))),
        //RES 6 H
        0xB4 => Instruction::RES(6, Operand::U8(OperandU8::Register(RegisterU8::H))),
        //RES 6 L
        0xB5 => Instruction::RES(6, Operand::U8(OperandU8::Register(RegisterU8::L))),
        //RES 6 (HL)
        0xB6 => Instruction::RES(
            6,
            Operand::U8(OperandU8::Memory(OperandU16::RegisterPair(RegisterU16::HL))),
        ),
        //RES 6 A
        0xB7 => Instruction::RES(6, Operand::U8(OperandU8::Register(RegisterU8::A))),
        //RES 7 B
        0xB8 => Instruction::RES(7, Operand::U8(OperandU8::Register(RegisterU8::B))),
        //RES 7 C
        0xB9 => Instruction::RES(7, Operand::U8(OperandU8::Register(RegisterU8::C))),
        //RES 7 D
        0xBA => Instruction::RES(7, Operand::U8(OperandU8::Register(RegisterU8::D))),
        //RES 7 E
        0xBB => Instruction::RES(7, Operand::U8(OperandU8::Register(RegisterU8::E))),
        //RES 7 H
        0xBC => Instruction::RES(7, Operand::U8(OperandU8::Register(RegisterU8::H))),
        //RES 7 L
        0xBD => Instruction::RES(7, Operand::U8(OperandU8::Register(RegisterU8::L))),
        //RES 7 (HL)
        0xBE => Instruction::RES(
            7,
            Operand::U8(OperandU8::Memory(OperandU16::RegisterPair(RegisterU16::HL))),
        ),
        //RES 7 A
        0xBF => Instruction::RES(7, Operand::U8(OperandU8::Register(RegisterU8::A))),
        //SET 0 B
        0xC0 => Instruction::SET(0, Operand::U8(OperandU8::Register(RegisterU8::B))),
        //SET 0 C
        0xC1 => Instruction::SET(0, Operand::U8(OperandU8::Register(RegisterU8::C))),
        //SET 0 D
        0xC2 => Instruction::SET(0, Operand::U8(OperandU8::Register(RegisterU8::D))),
        //SET 0 E
        0xC3 => Instruction::SET(0, Operand::U8(OperandU8::Register(RegisterU8::E))),
        //SET 0 H
        0xC4 => Instruction::SET(0, Operand::U8(OperandU8::Register(RegisterU8::H))),
        //SET 0 L
        0xC5 => Instruction::SET(0, Operand::U8(OperandU8::Register(RegisterU8::L))),
        //SET 0 (HL)
        0xC6 => Instruction::SET(
            0,
            Operand::U8(OperandU8::Memory(OperandU16::RegisterPair(RegisterU16::HL))),
        ),
        //SET 0 A
        0xC7 => Instruction::SET(0, Operand::U8(OperandU8::Register(RegisterU8::A))),
        //SET 1 B
        0xC8 => Instruction::SET(1, Operand::U8(OperandU8::Register(RegisterU8::B))),
        //SET 1 C
        0xC9 => Instruction::SET(1, Operand::U8(OperandU8::Register(RegisterU8::C))),
        //SET 1 D
        0xCA => Instruction::SET(1, Operand::U8(OperandU8::Register(RegisterU8::D))),
        //SET 1 E
        0xCB => Instruction::SET(1, Operand::U8(OperandU8::Register(RegisterU8::E))),
        //SET 1 H
        0xCC => Instruction::SET(1, Operand::U8(OperandU8::Register(RegisterU8::H))),
        //SET 1 L
        0xCD => Instruction::SET(1, Operand::U8(OperandU8::Register(RegisterU8::L))),
        //SET 1 (HL)
        0xCE => Instruction::SET(
            1,
            Operand::U8(OperandU8::Memory(OperandU16::RegisterPair(RegisterU16::HL))),
        ),
        //SET 1 A
        0xCF => Instruction::SET(1, Operand::U8(OperandU8::Register(RegisterU8::A))),
        //SET 2 B
        0xD0 => Instruction::SET(2, Operand::U8(OperandU8::Register(RegisterU8::B))),
        //SET 2 C
        0xD1 => Instruction::SET(2, Operand::U8(OperandU8::Register(RegisterU8::C))),
        //SET 2 D
        0xD2 => Instruction::SET(2, Operand::U8(OperandU8::Register(RegisterU8::D))),
        //SET 2 E
        0xD3 => Instruction::SET(2, Operand::U8(OperandU8::Register(RegisterU8::E))),
        //SET 2 H
        0xD4 => Instruction::SET(2, Operand::U8(OperandU8::Register(RegisterU8::H))),
        //SET 2 L
        0xD5 => Instruction::SET(2, Operand::U8(OperandU8::Register(RegisterU8::L))),
        //SET 2 (HL)
        0xD6 => Instruction::SET(
            2,
            Operand::U8(OperandU8::Memory(OperandU16::RegisterPair(RegisterU16::HL))),
        ),
        //SET 2 A
        0xD7 => Instruction::SET(2, Operand::U8(OperandU8::Register(RegisterU8::A))),
        //SET 3 B
        0xD8 => Instruction::SET(3, Operand::U8(OperandU8::Register(RegisterU8::B))),
        //SET 3 C
        0xD9 => Instruction::SET(3, Operand::U8(OperandU8::Register(RegisterU8::C))),
        //SET 3 D
        0xDA => Instruction::SET(3, Operand::U8(OperandU8::Register(RegisterU8::D))),
        //SET 3 E
        0xDB => Instruction::SET(3, Operand::U8(OperandU8::Register(RegisterU8::E))),
        //SET 3 H
        0xDC => Instruction::SET(3, Operand::U8(OperandU8::Register(RegisterU8::H))),
        //SET 3 L
        0xDD => Instruction::SET(3, Operand::U8(OperandU8::Register(RegisterU8::L))),
        //SET 3 (HL)
        0xDE => Instruction::SET(
            3,
            Operand::U8(OperandU8::Memory(OperandU16::RegisterPair(RegisterU16::HL))),
        ),
        //SET 3 A
        0xDF => Instruction::SET(3, Operand::U8(OperandU8::Register(RegisterU8::A))),
        //SET 4 B
        0xE0 => Instruction::SET(4, Operand::U8(OperandU8::Register(RegisterU8::B))),
        //SET 4 C
        0xE1 => Instruction::SET(4, Operand::U8(OperandU8::Register(RegisterU8::C))),
        //SET 4 D
        0xE2 => Instruction::SET(4, Operand::U8(OperandU8::Register(RegisterU8::D))),
        //SET 4 E
        0xE3 => Instruction::SET(4, Operand::U8(OperandU8::Register(RegisterU8::E))),
        //SET 4 H
        0xE4 => Instruction::SET(4, Operand::U8(OperandU8::Register(RegisterU8::H))),
        //SET 4 L
        0xE5 => Instruction::SET(4, Operand::U8(OperandU8::Register(RegisterU8::L))),
        //SET 4 (HL)
        0xE6 => Instruction::SET(
            4,
            Operand::U8(OperandU8::Memory(OperandU16::RegisterPair(RegisterU16::HL))),
        ),
        //SET 4 A
        0xE7 => Instruction::SET(4, Operand::U8(OperandU8::Register(RegisterU8::A))),
        //SET 5 B
        0xE8 => Instruction::SET(5, Operand::U8(OperandU8::Register(RegisterU8::B))),
        //SET 5 C
        0xE9 => Instruction::SET(5, Operand::U8(OperandU8::Register(RegisterU8::C))),
        //SET 5 D
        0xEA => Instruction::SET(5, Operand::U8(OperandU8::Register(RegisterU8::D))),
        //SET 5 E
        0xEB => Instruction::SET(5, Operand::U8(OperandU8::Register(RegisterU8::E))),
        //SET 5 H
        0xEC => Instruction::SET(5, Operand::U8(OperandU8::Register(RegisterU8::H))),
        //SET 5 L
        0xED => Instruction::SET(5, Operand::U8(OperandU8::Register(RegisterU8::L))),
        //SET 5 (HL)
        0xEE => Instruction::SET(
            5,
            Operand::U8(OperandU8::Memory(OperandU16::RegisterPair(RegisterU16::HL))),
        ),
        //SET 5 A
        0xEF => Instruction::SET(5, Operand::U8(OperandU8::Register(RegisterU8::A))),
        //SET 6 B
        0xF0 => Instruction::SET(6, Operand::U8(OperandU8::Register(RegisterU8::B))),
        //SET 6 C
        0xF1 => Instruction::SET(6, Operand::U8(OperandU8::Register(RegisterU8::C))),
        //SET 6 D
        0xF2 => Instruction::SET(6, Operand::U8(OperandU8::Register(RegisterU8::D))),
        //SET 6 E
        0xF3 => Instruction::SET(6, Operand::U8(OperandU8::Register(RegisterU8::E))),
        //SET 6 H
        0xF4 => Instruction::SET(6, Operand::U8(OperandU8::Register(RegisterU8::H))),
        //SET 6 L
        0xF5 => Instruction::SET(6, Operand::U8(OperandU8::Register(RegisterU8::L))),
        //SET 6 (HL)
        0xF6 => Instruction::SET(
            6,
            Operand::U8(OperandU8::Memory(OperandU16::RegisterPair(RegisterU16::HL))),
        ),
        //SET 6 A
        0xF7 => Instruction::SET(6, Operand::U8(OperandU8::Register(RegisterU8::A))),
        //SET 7 B
        0xF8 => Instruction::SET(7, Operand::U8(OperandU8::Register(RegisterU8::B))),
        //SET 7 C
        0xF9 => Instruction::SET(7, Operand::U8(OperandU8::Register(RegisterU8::C))),
        //SET 7 D
        0xFA => Instruction::SET(7, Operand::U8(OperandU8::Register(RegisterU8::D))),
        //SET 7 E
        0xFB => Instruction::SET(7, Operand::U8(OperandU8::Register(RegisterU8::E))),
        //SET 7 H
        0xFC => Instruction::SET(7, Operand::U8(OperandU8::Register(RegisterU8::H))),
        //SET 7 L
        0xFD => Instruction::SET(7, Operand::U8(OperandU8::Register(RegisterU8::L))),
        //SET 7 (HL)
        0xFE => Instruction::SET(
            7,
            Operand::U8(OperandU8::Memory(OperandU16::RegisterPair(RegisterU16::HL))),
        ),
        //SET 7 A
        0xFF => Instruction::SET(7, Operand::U8(OperandU8::Register(RegisterU8::A))),
        _ => unreachable!(),
    }
}
