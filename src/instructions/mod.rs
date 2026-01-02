#![allow(dead_code)]

use crate::registers::{RegisterU8, RegisterU16};

pub mod cbprefixed;
pub mod unprefixed;

#[derive(Debug)]
pub(crate) enum Operand {
    U8(OperandU8),
    U16(OperandU16),
}

#[derive(Debug)]
pub(crate) enum OperandU8 {
    Register(RegisterU8),
    Immediate,
    Memory(OperandU16),
    /// Memory address to access will be 0xFF(U8)
    MemoryU8(Box<OperandU8>),
}

#[derive(Debug)]
pub(crate) enum OperandU16 {
    RegisterPair(RegisterU16),
    Immediate,
}

#[derive(Debug)]
pub(crate) enum FlagCondition {
    Z,
    NZ,
    C,
    NC,
}

#[derive(Debug)]
pub(crate) enum Instruction {
    NOP,
    LD(Operand, Operand),
    LDF8,
    INC(Operand),
    DEC(Operand),
    RLCA,
    ADD(Operand, Operand),
    RRCA,
    STOP(OperandU8),
    RLA,
    JR(Option<FlagCondition>, OperandU8),
    RRA,
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
    POP(Operand),
    JP(Option<FlagCondition>, Operand),
    PUSH(Operand),
    RST(u8),
    CALL(Option<FlagCondition>, Operand),
    RETI,
    LDH(Operand, Operand),
    DI,
    EI,
    RLC(Operand),
    RRC(Operand),
    RL(Operand),
    RR(Operand),
    SLA(Operand),
    SRA(Operand),
    SWAP(Operand),
    SRL(Operand),
    BIT(u8, Operand),
    RES(u8, Operand),
    SET(u8, Operand),
    ILLEGAL(u8),
}
