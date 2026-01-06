use crate::{registers::{Flags, RegisterU8, RegisterU16, Registers}};

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Left = 0x80,
    Right = 0x01,
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

    pub(crate) fn rotate(reg: &mut Registers, dir: Direction, op: u8, carry: bool) -> u8 {
        let cy = reg.get_flag(Flags::CY);
        let new_cy = (op & dir as u8) > 0;

        let end_bit = (carry && cy) || (!carry && new_cy);

        let res = match dir {
            Direction::Left => {
                let end = if end_bit {
                    Direction::Right as u8
                } else {
                    0
                };
                (op << 1) | end
            }
            Direction::Right => {
                let end = if end_bit {
                    Direction::Left as u8
                } else {
                    0
                };
                (op >> 1) | end
            }
        };

        let flag_mask = Flags::All as u8;
        reg.set_flag(Flags::Z, res == 0, flag_mask);
        reg.set_flag(Flags::N, false, flag_mask);
        reg.set_flag(Flags::H, false, flag_mask);
        reg.set_flag(Flags::CY, new_cy, flag_mask);

        res
    }
}

#[allow(non_snake_case)]
#[cfg(test)]
mod Alu_test {
    use crate::registers::{Alu, Flags, RegisterU16, Registers, alu::Direction};

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
        Alu::daa(&mut reg);
        assert_eq!(reg.a, 0x77);

        reg.f = 0;
        reg.a = 0x7C;
        Alu::daa(&mut reg);
        assert_eq!(reg.a, 0x82);

        reg.f = Flags::H as u8;
        reg.a = 0x9C;
        Alu::daa(&mut reg);
        assert_eq!(reg.a, 0x02);
        assert_eq!(reg.f, Flags::CY as u8);

        reg.f = Flags::H as u8 | Flags::N as u8;
        reg.a = 0x0D;
        Alu::daa(&mut reg);
        assert_eq!(reg.a, 0x07);
        assert_eq!(reg.f, Flags::N as u8);
    }

    #[test]
    fn rotate() {
        let mut reg = Registers::default();
        reg.f = 0;
        assert_eq!(Alu::rotate(&mut reg, Direction::Left, 0b0010, false), 0b0100);
        assert_eq!(reg.f, 0);

        reg.f = 0;
        assert_eq!(Alu::rotate(&mut reg, Direction::Left, 0x80, false), 1);
        assert_eq!(reg.f, Flags::CY as u8);

        reg.f = 0;
        assert_eq!(Alu::rotate(&mut reg, Direction::Left, 0x80, true), 0);
        assert_eq!(reg.f, Flags::CY as u8 | Flags::Z as u8);

        reg.f = Flags::CY as u8;
        assert_eq!(Alu::rotate(&mut reg, Direction::Left, 0x80, true), 1);
        assert_eq!(reg.f, Flags::CY as u8);
        
        reg.f = 0;
        assert_eq!(Alu::rotate(&mut reg, Direction::Right, 0b0010, false), 1);
        assert_eq!(reg.f, 0);

        reg.f = 0;
        assert_eq!(Alu::rotate(&mut reg, Direction::Right, 0x01, false), 0x80);
        assert_eq!(reg.f, Flags::CY as u8);

        reg.f = 0;
        assert_eq!(Alu::rotate(&mut reg, Direction::Right, 0x01, true), 0);
        assert_eq!(reg.f, Flags::CY as u8 | Flags::Z as u8);

        reg.f = Flags::CY as u8;
        assert_eq!(Alu::rotate(&mut reg, Direction::Right, 0x01, true), 0x80);
        assert_eq!(reg.f, Flags::CY as u8);
    }
}
