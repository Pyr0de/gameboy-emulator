#![allow(dead_code)]

use std::fmt::Display;

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
#[derive(Debug, Clone)]
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

impl Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operand::U8(a) => write!(f, "{a}"),
            Operand::U16(a) => write!(f, "{a}"),
        }
    }
}

impl Display for OperandU8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OperandU8::Immediate => write!(f, "<8 bits immediate>"),
            OperandU8::Memory(m) => write!(f, "({m})"),
            OperandU8::Register(r) => write!(f, "{r}"),
            OperandU8::MemoryU8(m) => write!(f, "({m})"),
        }
    }
}

impl Display for OperandU16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OperandU16::Immediate => write!(f, "<16 bits immediate>"),
            OperandU16::RegisterPair(a) => write!(f, "{a}"),
        }
    }
}

impl Display for FlagCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::NOP => write!(f, "NOP"),
            Instruction::LD(a, b) => write!(f, "LD {a}, {b}"),
            Instruction::LD22 => write!(f, "LD (HL+), A"),
            Instruction::LD2A => write!(f, "LD A, (HL+)"),
            Instruction::LD32 => write!(f, "LD (HL-), A"),
            Instruction::LD3A => write!(f, "LD A, (HL-)"),
            Instruction::LDF8 => write!(f, "LD HL, SP+s8"),
            Instruction::INC(i) => write!(f, "INC {i}"),
            Instruction::DEC(i) => write!(f, "DEC {i}"),
            Instruction::ADD(i, j) => write!(f, "ADD {i}, {j}"),
            Instruction::STOP(i) => write!(f, "STOP {i}"),
            Instruction::JR(None, j) => write!(f, "JR {j}"),
            Instruction::JR(Some(i), j) => write!(f, "JR {i}, {j}"),
            Instruction::DAA => write!(f, "DAA"),
            Instruction::CPL => write!(f, "CPL"),
            Instruction::SCF => write!(f, "SCF"),
            Instruction::CCF => write!(f, "CCF"),
            Instruction::HALT => write!(f, "HALT"),
            Instruction::ADC(i, j) => write!(f, "ADC {i}, {j}"),
            Instruction::SUB(i, j) => write!(f, "SUB {i}, {j}"),
            Instruction::SBC(i, j) => write!(f, "SBC {i}, {j}"),
            Instruction::AND(i, j) => write!(f, "AND {i}, {j}"),
            Instruction::XOR(i, j) => write!(f, "XOR {i}, {j}"),
            Instruction::OR(i, j) => write!(f, "OR {i}, {j}"),
            Instruction::CP(i, j) => write!(f, "CP {i}, {j}"),
            Instruction::RET(Some(i)) => write!(f, "RET {i}"),
            Instruction::RET(None) => write!(f, "RET"),
            Instruction::POP(i) => write!(f, "POP {i}"),
            Instruction::JP(Some(i), j) => write!(f, "JP {i}, {j}"),
            Instruction::JP(None, i) => write!(f, "JP {i}"),
            Instruction::PUSH(i) => write!(f, "PUSH {i}"),
            Instruction::RST(i) => write!(f, "RST {i}"),
            Instruction::CALL(Some(i), j) => write!(f, "CALL {i}, {j}"),
            Instruction::CALL(None, i) => write!(f, "CALL {i}"),
            Instruction::RETI => write!(f, "RETI"),
            Instruction::LDH(i, j) => write!(f, "LDH {i}, {j}"),
            Instruction::DI => write!(f, "DI"),
            Instruction::EI => write!(f, "EI"),
            Instruction::RLC(i) => write!(f, "RLC {i}"),
            Instruction::RRC(i) => write!(f, "RRC {i}"),
            Instruction::RL(i) => write!(f, "RL {i}"),
            Instruction::RR(i) => write!(f, "RR {i}"),
            Instruction::SLA(i) => write!(f, "SLA {i}"),
            Instruction::SRA(i) => write!(f, "SRA {i}"),
            Instruction::SWAP(i) => write!(f, "SWAP {i}"),
            Instruction::SRL(i) => write!(f, "SRL {i}"),
            Instruction::BIT(i, j) => write!(f, "BIT {i}, {j}"),
            Instruction::RES(i, j) => write!(f, "RES {i}, {j}"),
            Instruction::SET(i, j) => write!(f, "SET {i}, {j}"),
            Instruction::ILLEGAL(i) => write!(f, "ILLEGAL {i}"),
        }
    }
}
