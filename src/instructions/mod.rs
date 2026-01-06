#![allow(dead_code)]

use crate::registers::{RegisterU8, RegisterU16};

pub mod cbprefixed;
pub mod unprefixed;

#[derive(Debug, Clone)]
pub(crate) enum Operand {
    U8(OperandU8),
    U16(OperandU16),
}

#[derive(Debug, Clone)]
pub(crate) enum OperandU8 {
    Register(RegisterU8),
    Immediate,
    Memory(OperandU16),
    /// Memory address to access will be 0xFF(U8)
    MemoryU8(Box<OperandU8>),
}

#[derive(Debug, Clone)]
pub(crate) enum OperandU16 {
    RegisterPair(RegisterU16),
    Immediate,
}

#[derive(Debug, Clone)]
pub(crate) enum FlagCondition {
    Z,
    NZ,
    C,
    NC,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug)]
pub(crate) enum Instruction {
    NOP,
    LD(Operand, Operand),
    LD22,
    LD2A,
    LD32,
    LD3A,
    LDF8,
    INC(Operand),
    DEC(Operand),
    ADD(Operand, Operand),
    STOP(OperandU8),
    JR(Option<FlagCondition>, OperandU8),
    DAA,
    CPL,
    SCF,
    CCF,
    HALT,
    ADC(OperandU8, OperandU8),
    SUB(OperandU8, OperandU8),
    SBC(OperandU8, OperandU8),
    AND(OperandU8, OperandU8),
    XOR(OperandU8, OperandU8),
    OR(OperandU8, OperandU8),
    CP(OperandU8, OperandU8),
    RET(Option<FlagCondition>),
    POP(RegisterU16),
    JP(Option<FlagCondition>, OperandU16),
    PUSH(RegisterU16),
    RST(u8),
    CALL(Option<FlagCondition>, OperandU16),
    RETI,
    LDH(OperandU8, OperandU8),
    DI,
    EI,
    RLC(OperandU8),
    RRC(OperandU8),
    RL(OperandU8),
    RR(OperandU8),
    SLA(OperandU8),
    SRA(OperandU8),
    SWAP(OperandU8),
    SRL(OperandU8),
    BIT(u8, OperandU8),
    RES(u8, OperandU8),
    SET(u8, OperandU8),
    ILLEGAL(u8),
}
