#![allow(dead_code)]

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

pub(crate) enum RegisterU8 {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

pub(crate) enum RegisterU16 {
    AF,
    BC,
    DE,
    HL,
}

pub(crate) enum Flags {
    /// Bit position of all flags
    /// Used only for flag mask
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

    pub fn get_u16(&self, regs: &RegisterU16) -> u16 {
        let (hi, lo) = match regs {
            RegisterU16::AF => (self.a, self.f),
            RegisterU16::BC => (self.b, self.c),
            RegisterU16::DE => (self.d, self.e),
            RegisterU16::HL => (self.h, self.l),
        };
        (hi as u16) << 8 | lo as u16
    }

    pub fn set_u16(&mut self, regs: &RegisterU16, val: u16) {
        let (hi, lo) = match regs {
            RegisterU16::AF => (&mut self.a, &mut self.f),
            RegisterU16::BC => (&mut self.b, &mut self.c),
            RegisterU16::DE => (&mut self.d, &mut self.e),
            RegisterU16::HL => (&mut self.h, &mut self.l),
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

pub struct ALU;

impl ALU {
    /// `flag_mask` specifies which flags should be affected
    pub(crate) fn add_u8(
        reg: &mut Registers,
        reg1: &RegisterU8,
        b: u8,
        carry: bool,
        flag_mask: u8,
    ) {
        let a = reg.get_u8(reg1);

        let mut flag: u8 = 0;
        let (res, cy) = a.carrying_add(b, carry);

        let hc = (((a & 0xF) + (b & 0xF)) & 0x10) == 0x10;
        if hc {
            flag |= Flags::H as u8;
        }
        if cy {
            flag |= Flags::CY as u8;
        }
        if res == 0 {
            flag |= Flags::Z as u8;
        }

        reg.set_u8(reg1, res);
        reg.f = (flag & flag_mask) | (reg.f & !flag_mask);
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

        let mut flag: u8 = 0;
        let (res, cy) = a.carrying_add(b, carry);

        let hc = (((a & 0xFFF) + (b & 0xFFF)) & 0x1000) == 0x1000;
        if hc {
            flag |= Flags::H as u8;
        }
        if cy {
            flag |= Flags::CY as u8;
        }
        if res == 0 {
            flag |= Flags::Z as u8;
        }

        reg.set_u16(reg1, res);
        reg.f = (flag & flag_mask) | (reg.f & !flag_mask);
    }

    pub(crate) fn sub(reg: &mut Registers, reg1: &RegisterU8, b: u8, borrow: bool, flag_mask: u8) {
        let a = reg.get_u8(reg1);
        let mut flag = Flags::N as u8;

        let (res, bo) = a.borrowing_sub(b, borrow);

        let (_, hc) = (a & 0xf).borrowing_sub(b & 0xf, borrow);
        if hc {
            flag |= Flags::H as u8;
        }
        if bo {
            flag |= Flags::CY as u8;
        }
        if res == 0 {
            flag |= Flags::Z as u8;
        }

        reg.set_u8(reg1, res);
        reg.f = (flag & flag_mask) | (reg.f & !flag_mask);
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
        let mut flag = Flags::H as u8;
        let res = a & b;

        if res == 0 {
            flag |= Flags::Z as u8;
        }

        reg.f = flag;
        reg.set_u8(&RegisterU8::A, res);
    }

    /// OR Operation, Stores value in register A
    pub(crate) fn or(reg: &mut Registers, b: u8) {
        let a = reg.get_u8(&RegisterU8::A);
        let mut flag = 0;
        let res = a | b;

        if res == 0 {
            flag |= Flags::Z as u8;
        }

        reg.f = flag;
        reg.set_u8(&RegisterU8::A, res);
    }

    /// XOR Operation, Stores value in register A
    pub(crate) fn xor(reg: &mut Registers, b: u8) {
        let a = reg.get_u8(&RegisterU8::A);
        let mut flag = 0;
        let res = a ^ b;

        if res == 0 {
            flag |= Flags::Z as u8;
        }

        reg.f = flag;
        reg.set_u8(&RegisterU8::A, res);
    }

    /// CMP Operation, Register A - `b`, does not affect Register A
    pub(crate) fn cmp(reg: &mut Registers, b: u8) {
        let a = reg.get_u8(&RegisterU8::A);
        ALU::sub(reg, &RegisterU8::A, b, false, Flags::All as u8);
        reg.set_u8(&RegisterU8::A, a);
    }
}

#[allow(non_snake_case)]
#[cfg(test)]
mod ALU_test {
    use crate::registers::{ALU, Flags, RegisterU8, RegisterU16, Registers};

    #[test]
    fn add() {
        let mut reg = Registers::default();
        ALU::add_u8(&mut reg, &RegisterU8::A, 3, false, Flags::All as u8);
        assert_eq!(reg.a, 3);
        assert_eq!(reg.f, 0);

        reg.a = 255;
        ALU::add_u8(&mut reg, &RegisterU8::A, 1, false, Flags::All as u8);
        assert_eq!(reg.a, 0);
        assert_eq!(reg.f, Flags::Z as u8 | Flags::CY as u8 | Flags::H as u8);

        reg.f = 0;
        reg.a = 255;
        ALU::add_u8(&mut reg, &RegisterU8::A, 1, false, Flags::All as u8 ^ Flags::CY as u8);
        assert!(reg.f & Flags::CY as u8 == 0);

        reg.set_u16(&RegisterU16::HL, 0xfff);
        ALU::add_u16(&mut reg, &RegisterU16::HL, 1, false, Flags::All as u8);
        assert_eq!(reg.get_u16(&RegisterU16::HL), 0x1000);
        assert_eq!(reg.f, Flags::H as u8);

        reg.set_u16(&RegisterU16::HL, 0xffff);
        ALU::add_u16(&mut reg, &RegisterU16::HL, 1, false, Flags::All as u8);
        assert_eq!(reg.get_u16(&RegisterU16::HL), 0);
        assert_eq!(reg.f, Flags::H as u8 | Flags::Z as u8 | Flags::CY as u8);
    }

    #[test]
    fn sub() {
        let mut reg = Registers::default();

        reg.a = 1;
        ALU::sub(&mut reg, &RegisterU8::A, 1, false, Flags::All as u8);
        assert_eq!(reg.a, 0);
        assert_eq!(reg.f, Flags::Z as u8 | Flags::N as u8);

        reg.a = 0;
        ALU::sub(&mut reg, &RegisterU8::A, 1, false, Flags::All as u8);
        assert_eq!(reg.a, 255);
        assert_eq!(reg.f, Flags::N as u8 | Flags::CY as u8 | Flags::H as u8);
    }

    #[test]
    fn cmp() {
        let mut reg = Registers::default();
        reg.a = 2;
        ALU::cmp(&mut reg, 2);
        assert_eq!(reg.a, 2);
        assert_eq!(reg.f, Flags::Z as u8 | Flags::N as u8);

        reg.a = 1;
        ALU::cmp(&mut reg, 2);
        assert_eq!(reg.a, 1);
        assert_eq!(reg.f, Flags::CY as u8 | Flags::N as u8 | Flags::H as u8);

        reg.a = 2;
        ALU::cmp(&mut reg, 1);
        assert_eq!(reg.a, 2);
        assert_eq!(reg.f, Flags::N as u8);
    }
}
