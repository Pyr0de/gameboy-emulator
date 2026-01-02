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
    INC(Operand),
    DEC(Operand),
    RLCA,
    ADD(Operand, Operand),
    RRCA,
    STOP(Operand),
    RLA,
    JR(Option<FlagCondition>, Operand),
    RRA,
    DAA,
    CPL,
    SCF,
    CCF,
    HALT,
    ADC(Operand, Operand),
    SUB(Operand, Operand),
    SBC(Operand, Operand),
    AND(Operand, Operand),
    XOR(Operand, Operand),
    OR(Operand, Operand),
    CP(Operand, Operand),
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
    BIT(Operand, Operand),
    RES(Operand, Operand),
    SET(Operand, Operand),
    ILLEGAL(u8),
}
