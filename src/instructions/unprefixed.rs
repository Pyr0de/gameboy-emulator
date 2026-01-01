use crate::{
    instructions::{Instruction, Operand, OperandU8, OperandU16},
    registers::{RegisterU8, RegisterU16},
};

pub(crate) fn decode_byte(byte: u8) -> Instruction {
    match byte {
        0x00 => Instruction::NOP,
        0x01 => Instruction::LD(
            Operand::U16(OperandU16::RegisterPair(RegisterU16::BC)),
            Operand::U16(OperandU16::Immediate),
        ),
        _ => unreachable!(),
    }
}
