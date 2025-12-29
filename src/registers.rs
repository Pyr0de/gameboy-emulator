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

    pub fn get_bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    pub fn set_bc(&mut self, bc: u16) {
        self.b = (bc >> 8) as u8;
        self.c = (bc & 0xFF) as u8;
    }

    pub fn get_de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }

    pub fn set_de(&mut self, de: u16) {
        self.d = (de >> 8) as u8;
        self.e = (de & 0xFF) as u8;
    }

    pub fn get_hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }

    pub fn set_hl(&mut self, hl: u16) {
        self.h = (hl >> 8) as u8;
        self.l = (hl & 0xFF) as u8;
    }
}

pub struct ALU;

impl ALU {
    pub(crate) fn add(a: &mut u8, b: u8, carry: bool, flag_reg: &mut u8) {
        let mut flag: u8 = 0;
        let (res, cy) = a.carrying_add(b, carry);

        let hc = (((*a & 0xF) + (b & 0xF)) & 0x10) == 0x10;
        println!("{hc}");

        if hc {
            flag |= Flags::H as u8;
        }
        if cy {
            flag |= Flags::CY as u8;
        }
        if res == 0 {
            flag |= Flags::Z as u8;
        }

        
        *a = res;
        *flag_reg = flag;
    }
    pub(crate) fn sub(a: &mut u8, b: u8, carry: bool) {
        
    }
}

#[allow(non_snake_case)]
mod ALU_test {
    use crate::registers::{ALU, Flags, Registers};

    #[test]
    fn add() {
        let mut reg = Registers::default();
        ALU::add(&mut reg.a, 3, false, &mut reg.f);
        assert_eq!(reg.f, 0);

        reg.a = 255;
        reg.f = ALU::add(&mut reg.a, 1, false);
        assert_eq!(reg.f, Flags::Z as u8 | Flags::CY as u8 | Flags::H as u8);

        reg.f = ALU::add(&mut reg.a, 255, carry)
    }
}
