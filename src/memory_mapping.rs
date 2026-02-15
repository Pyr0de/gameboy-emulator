use std::ops::{Index, IndexMut};

use anyhow::{Result, bail};
use imgui::TableFlags;

use crate::graphics::Graphics;

#[derive(Debug)]
pub(crate) struct MemoryMapping {
    pub rom: Rom,
    pub vram: Graphics,
    pub external_ram: [u8; 0x2000],
    pub wram: WRam,
    pub stack: [u8; 0x7F],

    debugger_offset: i16,
    debugger_starting_address: u16,
}

impl Default for MemoryMapping {
    fn default() -> Self {
        Self {
            rom: Rom::default(),
            vram: Graphics::new(),
            external_ram: [0; 0x2000],
            wram: WRam::default(),
            stack: [0; 0x7F],
            debugger_offset: 0,
            debugger_starting_address: 0,
        }
    }
}

impl MemoryMapping {
    pub fn new(rom: Rom) -> Self {
        Self {
            rom,
            ..Default::default()
        }
    }

    pub fn display_debugger(&mut self, ui: &imgui::Ui, pc: u16) {
        ui.window("Memory")
            .size([500., 500.], imgui::Condition::FirstUseEver)
            .position([500., 250.], imgui::Condition::FirstUseEver)
            .build(|| {
                if let Some(_m) = ui.tab_bar("mem") {
                    let mut starting_address: u16 = 0;

                    if let Some(_r) = ui.tab_item("ROM") {
                        starting_address = 0;
                    }
                    if let Some(_r) = ui.tab_item("VRAM") {
                        starting_address = 0x8000;
                    }
                    if let Some(_r) = ui.tab_item("External RAM") {
                        starting_address = 0xA000;
                    }
                    if let Some(_r) = ui.tab_item("WRAM") {
                        starting_address = 0xC000;
                    }
                    if let Some(_r) = ui.tab_item("Object Attribute Memory(OAM)") {
                        starting_address = 0xFE00;
                    }
                    if let Some(_r) = ui.tab_item("IO Registers") {
                        starting_address = 0xFF00;
                    }
                    if let Some(_r) = ui.tab_item("High RAM(HRAM)") {
                        starting_address = 0xFF80;
                    }

                    if self.debugger_starting_address != starting_address {
                        self.debugger_offset = 0;
                        self.debugger_starting_address = starting_address;
                    }

                    if let Some(_table) =
                        ui.begin_table_with_flags("", 17, TableFlags::SIZING_FIXED_FIT)
                    {
                        ui.table_setup_column("");
                        for i in 0..16 {
                            ui.table_setup_column(format!("{i:X}"));
                        }
                        ui.table_headers_row();

                        for i in 0..256 {
                            let mut addr =
                                starting_address as i32 + self.debugger_offset as i32 + i;
                            if addr < u16::MIN.into() {
                                addr = u16::MIN.into();
                                self.debugger_offset = 0;
                            }
                            if addr > u16::MAX.into() {
                                addr = u16::MAX.into();
                                self.debugger_offset = 0;
                            }

                            if i % 16 == 0 {
                                ui.table_next_row();
                                ui.table_set_column_index(0);
                                ui.text(format!("0x{:04X} ", addr));
                            }
                            ui.table_set_column_index((i % 16 + 1) as usize);
                            let val = if let Ok(val) = &self.get(addr as u16) {
                                format!("{val:02X}")
                            } else {
                                "--".to_string()
                            };
                            let color = if pc == addr as u16 {
                                [0., 1., 0., 1.]
                            } else {
                                [1., 1., 1., 1.]
                            };
                            ui.text_colored(color, val);
                            //ui.push_style_var(style_var)
                        }
                    }
                    if ui.button("< Prev") {
                        self.debugger_offset -= 256;
                    }
                    ui.same_line();
                    if ui.button("Next >") {
                        self.debugger_offset += 256;
                    }
                }
            });
    }
}

impl MemoryMapping {
    pub fn get(&self, index: u16) -> Result<&u8> {
        Ok(match index {
            0x0..=0x7FFF => &self.rom[index],
            0x8000..=0x9FFF => &self.vram[index - 0x8000],
            0xA000..=0xBFFF => &self.external_ram[index as usize - 0xA000],
            0xC000..=0xDFFF => &self.wram[index - 0xC000],
            0xFF40 => &self.vram.lcd_control,
            0xFF70 => &self.wram.bank_select,
            0xFF80..=0xFFFE => &self.stack[index as usize - 0xFF80],
            _ => {
                bail!("unimplemented memory 0x{:x}", index)
            }
        })
    }

    pub fn get_mut(&mut self, index: u16) -> Result<&mut u8> {
        Ok(match index {
            0x0..=0x7FFF => {
                bail!("cannot write to rom: {:x}", index);
            }
            0x8000..=0x9FFF => &mut self.vram[index - 0x8000],
            0xA000..=0xBFFF => &mut self.external_ram[index as usize - 0xA000],
            0xC000..=0xDFFF => &mut self.wram[index - 0xC000],
            0xFF40 => &mut self.vram.lcd_control,
            0xFF70 => &mut self.wram.bank_select,
            0xFF80..=0xFFFE => &mut self.stack[index as usize - 0xFF80],
            _ => {
                bail!("unimplemented memory 0x{:x}", index)
            }
        })
    }
}

#[derive(Default, Debug)]
pub(crate) struct Rom {
    // Header
    pub rom: Vec<u8>,
}

impl Index<u16> for Rom {
    type Output = u8;
    fn index(&self, index: u16) -> &Self::Output {
        // TODO: switchable banks
        &self.rom[index as usize]
    }
}

#[derive(Debug)]
pub(crate) struct WRam {
    /// 8 banks of 0x1000 (4KB)
    pub wram: [u8; 0x1000 * 8],
    pub bank_select: u8,
}

impl Default for WRam {
    fn default() -> Self {
        Self {
            wram: [0; 0x1000 * 8],
            bank_select: 1,
        }
    }
}

impl Index<u16> for WRam {
    type Output = u8;
    fn index(&self, index: u16) -> &Self::Output {
        let idx = match index {
            0..0x1000 => index,
            0x1000..0x2000 => {
                let bank_offset = (self.bank_select as u16 & 0b111).saturating_sub(1);
                index + 0x1000 * bank_offset
            }
            _ => unreachable!(),
        };
        &self.wram[idx as usize]
    }
}

impl IndexMut<u16> for WRam {
    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
        let idx = match index {
            0..0x1000 => index,
            0x1000..0x2000 => {
                let bank_offset = (self.bank_select as u16 & 0b111).saturating_sub(1);
                index + 0x1000 * bank_offset
            }
            _ => unreachable!(),
        };
        &mut self.wram[idx as usize]
    }
}
