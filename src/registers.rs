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
impl Registers {
    pub fn new() -> Self {
        // TODO: use powerup sequence values
        Registers {
            pc: 0x100,
            sp: 0xFFFE,
            ..Default::default()
        }
    }

    /// Reads zero flag - register F bit 7
    pub fn get_flag_z(&self) -> bool {
        self.f & 0x80 > 0
    }
    /// Reads subtraction flag - register F bit 6
    pub fn get_flag_n(&self) -> bool {
        self.f & 0x40 > 0
    }
    /// Reads half carry flag - register F bit 5
    pub fn get_flag_h(&self) -> bool {
        self.f & 0x20 > 0
    }
    /// Reads carry flag - register F bit 4
    pub fn get_flag_cy(&self) -> bool {
        self.f & 0x10 > 0
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
