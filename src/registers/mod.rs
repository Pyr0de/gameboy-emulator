#![allow(dead_code)]

mod alu;

use crate::{debugger::DisplayDebugger, instructions::FlagCondition};
pub use alu::{Alu, Direction};
use imgui::*;

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
            RegisterU16::SP => ((self.sp >> 8) as u8, (self.sp & 0xff) as u8),
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

impl DisplayDebugger for Registers {
    fn display_debugger(&self, ui: &Ui) {
        ui.window("Registers")
            .position([50., 200.], imgui::Condition::FirstUseEver)
            .always_auto_resize(true)
            .build(|| {
                
                if let Some(_t) = ui.begin_table("Register", 2) {
                    ui.table_setup_column("Register");
                    ui.table_setup_column("Value (0x)");
                    ui.table_headers_row();

                    fn add_column_u8(ui: &Ui, reg: &str, val: u8) {
                        ui.table_next_row();
                        ui.table_set_column_index(0);
                        ui.text(reg);
                        ui.table_set_column_index(1);
                        ui.text(format!("{:02x}", val));
                    }
                    fn add_column_u16(ui: &Ui, reg: &str, val: u16) {
                        ui.table_next_row();
                        ui.table_set_column_index(0);
                        ui.text(reg);
                        ui.table_set_column_index(1);
                        ui.text(format!("{:04x}", val));
                    }

                    add_column_u8(ui, "A", self.a);
                    add_column_u8(ui, "B", self.b);
                    add_column_u8(ui, "C", self.c);
                    add_column_u8(ui, "D", self.d);
                    add_column_u8(ui, "E", self.e);
                    add_column_u8(ui, "H", self.h);
                    add_column_u8(ui, "L", self.l);
                    add_column_u8(ui, "F", self.f);
                    add_column_u16(ui, "SP", self.sp);
                    add_column_u16(ui, "PC", self.pc);
                }
            });
    }
}
