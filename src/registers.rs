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
#[allow(dead_code)]
pub(crate) enum RegisterU8 {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

#[allow(dead_code)]
pub(crate) enum RegisterU16 {
    AF,
    BC,
    DE,
    HL,
}

#[allow(dead_code)]
pub(crate) enum Flags {
    /// Zero flag
    Z = 0x80,
    /// Subtraction flag
    N = 0x40,
    /// Half carry flag
    H = 0x20,
    /// Carry flag
    CY = 0x10,
}

#[allow(dead_code)]
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

    pub fn set_u16(&mut self, regs: &RegisterU16, bc: u16) {
        let (hi, lo) = match regs {
            RegisterU16::AF => (&mut self.a, &mut self.f),
            RegisterU16::BC => (&mut self.b, &mut self.c),
            RegisterU16::DE => (&mut self.d, &mut self.e),
            RegisterU16::HL => (&mut self.h, &mut self.l),
        };
        *hi = (bc >> 8) as u8;
        *lo = (bc & 0xFF) as u8;
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
    pub(crate) fn add(reg: &mut Registers, reg1: &RegisterU8, b: u8, carry: bool) {
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
        reg.f = flag;
    }
    pub(crate) fn sub(a: &mut u8, b: u8, carry: bool) {
        
    }
}

#[allow(non_snake_case)]
mod ALU_test {
    use crate::registers::{ALU, Flags, RegisterU8, Registers};

    #[test]
    fn add() {
        let mut reg = Registers::default();
        ALU::add(&mut reg, &RegisterU8::A, 3, false);
        assert_eq!(reg.a, 3);
        assert_eq!(reg.f, 0);

        reg.a = 255;
        ALU::add(&mut reg, &RegisterU8::A, 1, false);
        assert_eq!(reg.a, 0);
        assert_eq!(reg.f, Flags::Z as u8 | Flags::CY as u8 | Flags::H as u8);

    }
}
