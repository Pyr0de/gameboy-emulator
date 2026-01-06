#![allow(dead_code)]

use crate::instructions::FlagCondition;

#[derive(Debug, Default)]
#[allow(unused)]
pub(super) struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub f: u8,
    pub sp: u16,
    pub pc: u16,
}

#[derive(Debug, Default, Clone)]
pub(crate) enum RegisterU8 {
    #[default]
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

#[derive(Debug, Default, Clone)]
pub(crate) enum RegisterU16 {
    AF,
    BC,
    DE,
    #[default]
    HL,
    SP,
}

#[derive(Debug, Default, Clone, Copy)]
pub(crate) enum Flags {
    /// Bit position of all flags
    /// Used only for flag mask
    #[default]
    All = 0xF0,
    /// Zero flag
    Z = 0x80,
    /// Subtraction flag
    N = 0x40,
    /// Half carry flag
    H = 0x20,
    /// Carry flag
    CY = 0x10,
}

impl Registers {
    pub fn new() -> Self {
        // TODO: use powerup sequence values
        Registers {
            pc: 0x100,
            sp: 0xFFFE,
            ..Default::default()
        }
    }

    pub fn get_flag(&self, flag: Flags) -> bool {
        self.f & (flag as u8) > 0
    }
    pub fn get_flag_condition(&self, flag_condition: FlagCondition) -> bool {
        match flag_condition {
            FlagCondition::Z => self.get_flag(Flags::Z),
            FlagCondition::NZ => !self.get_flag(Flags::Z),
            FlagCondition::C => self.get_flag(Flags::CY),
            FlagCondition::NC => !self.get_flag(Flags::CY),
        }
    }
    pub fn set_flag(&mut self, flag: Flags, set: bool, flag_mask: u8) {
        if self.get_flag(flag) != set && flag_mask & (flag as u8) != 0 {
            self.f ^= flag as u8;
        }
    }

    pub fn get_u16(&self, regs: &RegisterU16) -> u16 {
        let (hi, lo) = match regs {
            RegisterU16::AF => (self.a, self.f),
            RegisterU16::BC => (self.b, self.c),
            RegisterU16::DE => (self.d, self.e),
            RegisterU16::HL => (self.h, self.l),
            RegisterU16::SP => return self.sp,
        };
        (hi as u16) << 8 | lo as u16
    }

    pub fn get_split_u16(&self, regs: &RegisterU16) -> (u8, u8) {
        match regs {
            RegisterU16::AF => (self.a, self.f),
            RegisterU16::BC => (self.b, self.c),
            RegisterU16::DE => (self.d, self.e),
            RegisterU16::HL => (self.h, self.l),
            RegisterU16::SP => return ((self.sp >> 8) as u8, (self.sp & 0xff) as u8),
        }
    }

    pub fn set_split_u16(&mut self, regs: &RegisterU16, higher: u8, lower: u8) {
        let (hi, lo) = match regs {
            RegisterU16::AF => (&mut self.a, &mut self.f),
            RegisterU16::BC => (&mut self.b, &mut self.c),
            RegisterU16::DE => (&mut self.d, &mut self.e),
            RegisterU16::HL => (&mut self.h, &mut self.l),
            RegisterU16::SP => {
                self.sp = (higher as u16) << 8 | lower as u16;
                return;
            }
        };
        *hi = higher;
        *lo = lower;
    }

    pub fn set_u16(&mut self, regs: &RegisterU16, val: u16) {
        let (hi, lo) = match regs {
            RegisterU16::AF => (&mut self.a, &mut self.f),
            RegisterU16::BC => (&mut self.b, &mut self.c),
            RegisterU16::DE => (&mut self.d, &mut self.e),
            RegisterU16::HL => (&mut self.h, &mut self.l),
            RegisterU16::SP => {
                self.sp = val;
                return;
            }
        };
        *hi = (val >> 8) as u8;
        *lo = (val & 0xFF) as u8;
    }

    pub fn get_u8(&self, regs: &RegisterU8) -> u8 {
        match regs {
            RegisterU8::A => self.a,
            RegisterU8::B => self.b,
            RegisterU8::C => self.c,
            RegisterU8::D => self.d,
            RegisterU8::E => self.e,
            RegisterU8::H => self.h,
            RegisterU8::L => self.l,
        }
    }
    pub fn set_u8(&mut self, regs: &RegisterU8, val: u8) {
        let r = match regs {
            RegisterU8::A => &mut self.a,
            RegisterU8::B => &mut self.b,
            RegisterU8::C => &mut self.c,
            RegisterU8::D => &mut self.d,
            RegisterU8::E => &mut self.e,
            RegisterU8::H => &mut self.h,
            RegisterU8::L => &mut self.l,
        };
        *r = val;
    }
}

pub struct Alu;

impl Alu {
    /// `flag_mask` specifies which flags should be affected
    pub(crate) fn add_u8(reg: &mut Registers, a: u8, b: u8, carry: bool, flag_mask: u8) -> u8 {
        let (res, cy) = a.carrying_add(b, carry);

        let hc = (((a & 0xF) + (b & 0xF)) & 0x10) == 0x10;

        reg.set_flag(Flags::Z, res == 0, flag_mask);
        reg.set_flag(Flags::N, false, flag_mask);
        reg.set_flag(Flags::H, hc, flag_mask);
        reg.set_flag(Flags::CY, cy, flag_mask);

        res
    }

    /// `flag_mask` specifies which flags should be affected
    pub(crate) fn add_u16(
        reg: &mut Registers,
        reg1: &RegisterU16,
        b: u16,
        carry: bool,
        flag_mask: u8,
    ) {
        let a = reg.get_u16(reg1);
        let (res, cy) = a.carrying_add(b, carry);

        let hc = (((a & 0xFFF) + (b & 0xFFF)) & 0x1000) == 0x1000;

        reg.set_flag(Flags::Z, res == 0, flag_mask);
        reg.set_flag(Flags::N, false, flag_mask);
        reg.set_flag(Flags::H, hc, flag_mask);
        reg.set_flag(Flags::CY, cy, flag_mask);

        reg.set_u16(reg1, res);
    }

    pub(crate) fn sub(reg: &mut Registers, a: u8, b: u8, borrow: bool, flag_mask: u8) -> u8 {
        let (res, bo) = a.borrowing_sub(b, borrow);

        let (_, hc) = (a & 0xf).borrowing_sub(b & 0xf, borrow);

        reg.set_flag(Flags::Z, res == 0, flag_mask);
        reg.set_flag(Flags::N, true, flag_mask);
        reg.set_flag(Flags::H, hc, flag_mask);
        reg.set_flag(Flags::CY, bo, flag_mask);

        res
    }

    /// Does not affect flag register
    /// Only `DEC` instruction requires 16 bit subtraction
    pub(crate) fn dec_u16(reg: &mut Registers, reg1: &RegisterU16) {
        let a = reg.get_u16(reg1);

        let res = a.wrapping_sub(1);

        reg.set_u16(reg1, res);
    }

    /// AND Operation, Stores value in register A
    pub(crate) fn and(reg: &mut Registers, b: u8) {
        let a = reg.get_u8(&RegisterU8::A);
        let res = a & b;

        let flag_mask = Flags::All as u8;
        reg.set_flag(Flags::Z, res == 0, flag_mask);
        reg.set_flag(Flags::N, false, flag_mask);
        reg.set_flag(Flags::H, true, flag_mask);
        reg.set_flag(Flags::CY, false, flag_mask);

        reg.set_u8(&RegisterU8::A, res);
    }

    /// OR Operation, Stores value in register A
    pub(crate) fn or(reg: &mut Registers, b: u8) {
        let a = reg.get_u8(&RegisterU8::A);
        let res = a | b;

        let flag_mask = Flags::All as u8;
        reg.set_flag(Flags::Z, res == 0, flag_mask);
        reg.set_flag(Flags::N, false, flag_mask);
        reg.set_flag(Flags::H, false, flag_mask);
        reg.set_flag(Flags::CY, false, flag_mask);

        reg.set_u8(&RegisterU8::A, res);
    }

    /// XOR Operation, Stores value in register A
    pub(crate) fn xor(reg: &mut Registers, b: u8) {
        let a = reg.get_u8(&RegisterU8::A);
        let res = a ^ b;

        let flag_mask = Flags::All as u8;
        reg.set_flag(Flags::Z, res == 0, flag_mask);
        reg.set_flag(Flags::N, false, flag_mask);
        reg.set_flag(Flags::H, false, flag_mask);
        reg.set_flag(Flags::CY, false, flag_mask);

        reg.set_u8(&RegisterU8::A, res);
    }

    /// CMP Operation, Register A - `b`, does not affect Register A
    pub(crate) fn cmp(reg: &mut Registers, b: u8) {
        let a = reg.get_u8(&RegisterU8::A);
        Alu::sub(reg, a, b, false, Flags::All as u8);
    }

    pub(crate) fn daa(reg: &mut Registers) {
        let mut offset = 0_u8;
        let mut should_carry = false;

        let a_value = reg.a;
        let half_carry = reg.get_flag(Flags::H);
        let carry = reg.get_flag(Flags::CY);
        let subtract = reg.get_flag(Flags::N);

        if (!subtract && a_value & 0xF > 0x09) || half_carry {
            offset |= 0x06;
        }

        if (!subtract && a_value > 0x99) || carry {
            offset |= 0x60;
            should_carry = true;
        }

        reg.a = if !subtract {
            a_value.wrapping_add(offset)
        } else {
            a_value.wrapping_sub(offset)
        };

        let flag_mask = Flags::All as u8 ^ Flags::N as u8;
        reg.set_flag(Flags::Z, reg.a == 0, flag_mask);
        reg.set_flag(Flags::H, false, flag_mask);
        reg.set_flag(Flags::CY, should_carry, flag_mask);
    }
}

#[allow(non_snake_case)]
#[cfg(test)]
mod Alu_test {
    use crate::registers::{Alu, Flags, RegisterU16, Registers};

    #[test]
    fn add() {
        let mut reg = Registers::default();
        reg.a = Alu::add_u8(&mut reg, 0, 3, false, Flags::All as u8);
        assert_eq!(reg.a, 3);
        assert_eq!(reg.f, 0);

        reg.a = Alu::add_u8(&mut reg, 255, 1, false, Flags::All as u8);
        assert_eq!(reg.a, 0);
        assert_eq!(reg.f, Flags::Z as u8 | Flags::CY as u8 | Flags::H as u8);

        reg.f = 0;
        reg.a = Alu::add_u8(&mut reg, 255, 1, false, Flags::All as u8 ^ Flags::CY as u8);
        assert!(reg.f & Flags::CY as u8 == 0);

        reg.set_u16(&RegisterU16::HL, 0xfff);
        Alu::add_u16(&mut reg, &RegisterU16::HL, 1, false, Flags::All as u8);
        assert_eq!(reg.get_u16(&RegisterU16::HL), 0x1000);
        assert_eq!(reg.f, Flags::H as u8);

        reg.set_u16(&RegisterU16::HL, 0xffff);
        Alu::add_u16(&mut reg, &RegisterU16::HL, 1, false, Flags::All as u8);
        assert_eq!(reg.get_u16(&RegisterU16::HL), 0);
        assert_eq!(reg.f, Flags::H as u8 | Flags::Z as u8 | Flags::CY as u8);
    }

    #[test]
    fn sub() {
        let mut reg = Registers::default();

        reg.a = Alu::sub(&mut reg, 1, 1, false, Flags::All as u8);
        assert_eq!(reg.a, 0);
        assert_eq!(reg.f, Flags::Z as u8 | Flags::N as u8);

        reg.a = Alu::sub(&mut reg, 0, 1, false, Flags::All as u8);
        assert_eq!(reg.a, 255);
        assert_eq!(reg.f, Flags::N as u8 | Flags::CY as u8 | Flags::H as u8);
    }

    #[test]
    fn cmp() {
        let mut reg = Registers::default();
        reg.a = 2;
        Alu::cmp(&mut reg, 2);
        assert_eq!(reg.a, 2);
        assert_eq!(reg.f, Flags::Z as u8 | Flags::N as u8);

        reg.a = 1;
        Alu::cmp(&mut reg, 2);
        assert_eq!(reg.a, 1);
        assert_eq!(reg.f, Flags::CY as u8 | Flags::N as u8 | Flags::H as u8);

        reg.a = 2;
        Alu::cmp(&mut reg, 1);
        assert_eq!(reg.a, 2);
        assert_eq!(reg.f, Flags::N as u8);
    }

    #[test]
    fn dda() {
        let mut reg = Registers::default();
        reg.f = 0;
        reg.a = 0x77;
        Alu::dda(&mut reg);
        assert_eq!(reg.a, 0x77);

        reg.f = 0;
        reg.a = 0x7C;
        Alu::dda(&mut reg);
        assert_eq!(reg.a, 0x82);

        reg.f = Flags::H as u8;
        reg.a = 0x9C;
        Alu::dda(&mut reg);
        assert_eq!(reg.a, 0x02);
        assert_eq!(reg.f, Flags::CY as u8);

        reg.f = Flags::H as u8 | Flags::N as u8;
        reg.a = 0x0D;
        Alu::dda(&mut reg);
        assert_eq!(reg.a, 0x07);
        assert_eq!(reg.f, Flags::N as u8);
    }
}
