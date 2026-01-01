use crate::instructions::Instruction;

pub(crate) fn decode_byte(byte: u8) -> Instruction {
    match byte {
        _ => unreachable!(),
    }
}
