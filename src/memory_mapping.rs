use std::{
    fs::File,
    io::Read,
    ops::{Index, IndexMut},
    path::Path,
};

use anyhow::{Result, bail};
use imgui::{StyleColor, TableFlags};

use crate::{graphics::Graphics, interrupt::Interrupt, timer::Timer};



#[derive(Debug)]
pub(crate) struct MemoryMapping<'a> {
    pub rom: Rom,
    pub vram: Graphics<'a>,
    pub external_ram: [u8; 0x2000],
    pub wram: WRam,
    pub stack: [u8; 0x7F],
    pub interrupt: Interrupt,
    pub timer: Timer,

    debugger_offset: i16,
    debugger_selected: u16,
}

impl<'a> Default for MemoryMapping<'a> {
    fn default() -> Self {
        Self {
            rom: Rom::default(),
            vram: Graphics::new(),
            external_ram: [0; 0x2000],
            wram: WRam::default(),
            stack: [0; 0x7F],
            interrupt: Interrupt::new(),
            timer: Timer::new(),
            debugger_offset: 0,
            debugger_selected: 0,
        }
    }
}

impl<'a> MemoryMapping<'a> {
    pub fn new(rom: Rom) -> Self {
        Self {
            rom,
            ..Default::default()
        }
    }

    pub fn display_debugger(&mut self, ui: &imgui::Ui, pc: u16) {
        self.vram.display_debugger(ui);

        ui.window("Memory")
            .size([600., 600.], imgui::Condition::FirstUseEver)
            .position([250., 250.], imgui::Condition::FirstUseEver)
            .build(|| {
                ui.text("0x");
                ui.same_line();
                let mut str = format!("{:04X}", self.debugger_offset as u16 * 256);
                if ui
                    .input_text("###search", &mut str)
                        .enter_returns_true(true)
                        .build()
                        && let Ok(n) = u16::from_str_radix(&str, 16)
                {
                    self.debugger_selected = n;
                    self.debugger_offset = (n/256) as i16;
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
                        let addr = self.debugger_offset as i32 * 256 + i;

                        if i % 16 == 0 {
                            ui.table_next_row();
                            ui.table_set_column_index(0);
                            ui.text(format!("0x{:04X} ", addr));
                        }
                        ui.table_set_column_index((i % 16 + 1) as usize);
                        let val = if let Ok(val) = &self.get(addr as u16) {
                            format!("{val:02X}###{addr}")
                        } else {
                            format!("--###{addr}")
                        };
                        let color = if pc == addr as u16 {
                            [0., 1., 0., 1.]
                        } else {
                            [1., 1., 1., 1.]
                        };

                        let _text_color = ui.push_style_color(StyleColor::Text, color);
                        let _button_color = if self.debugger_selected == addr as u16 {
                            Some(
                                ui.push_style_color(StyleColor::Button, [0., 0.6, 0.8, 1.]),
                            )
                        } else {
                            None
                        };

                        if ui.button(val) {
                            self.debugger_selected = addr as u16;
                        }
                    }
                }
                if ui.button("< Prev") {
                    self.debugger_offset -= 1;
                }
                ui.same_line();
                if ui.button("Next >") {
                    self.debugger_offset += 1;
                }
                ui.new_line();
                self.debugger_offset = self.debugger_offset.clamp(0, 255);
                if let Ok(val) = self.get(self.debugger_selected) {
                    ui.text(format!(
                        "0x{:04X}: 0x{val:02X} 0b{val:08b}",
                        self.debugger_selected
                    ));

                    if let Ok(val_mut) = self.get_mut(self.debugger_selected) {
                        let mut str = format!("{val_mut:02X}");
                        if ui
                            .input_text("replace", &mut str)
                            .enter_returns_true(true)
                            .build()
                            && let Ok(n) = u8::from_str_radix(&str, 16)
                        {
                            *val_mut = n;
                        }
                    }
                } else {
                    ui.text(format!("0x{:04X}: 0x-- 0b--------", self.debugger_selected));
                }
            });
    }

    pub fn get(&self, index: u16) -> Result<&u8> {
        Ok(match index {
            0x0..=0x7FFF => &self.rom[index],
            0x8000..=0x9FFF => &self.vram[index - 0x8000],
            0xA000..=0xBFFF => &self.external_ram[index as usize - 0xA000],
            0xC000..=0xDFFF => &self.wram[index - 0xC000],
            0xFF04 => &self.timer.divider_register,
            0xFF05 => &self.timer.timer_counter,
            0xFF06 => &self.timer.timer_modulo,
            0xFF07 => &self.timer.timer_controller,
            0xFF0F => &self.interrupt.interrupt_flag.value,
            0xFF40 => &self.vram.lcd_control.value,
            0xFF41 => &self.vram.lcd_status.value,
            0xFF42 => &self.vram.scroll_x,
            0xFF43 => &self.vram.scroll_y,
            0xFF44 => &self.vram.y_coord,
            0xFF45 => &self.vram.y_comp,
            0xFF70 => &self.wram.bank_select,
            0xFF80..=0xFFFE => &self.stack[index as usize - 0xFF80],
            0xFFFF => &self.interrupt.interrupt_enable.value,
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
            0xFF04 => &mut self.timer.divider_register,
            0xFF05 => &mut self.timer.timer_counter,
            0xFF06 => &mut self.timer.timer_modulo,
            0xFF07 => &mut self.timer.timer_controller,
            0xFF0F => &mut self.interrupt.interrupt_flag.value,
            0xFF40 => &mut self.vram.lcd_control.value,
            0xFF41 => &mut self.vram.lcd_status.value,
            0xFF42 => &mut self.vram.scroll_x,
            0xFF43 => &mut self.vram.scroll_y,
            0xFF44 => bail!("cannot write to: {:x}", index),
            0xFF45 => &mut self.vram.y_comp,
            0xFF70 => &mut self.wram.bank_select,
            0xFF80..=0xFFFE => &mut self.stack[index as usize - 0xFF80],
            0xFFFF => &mut self.interrupt.interrupt_enable.value,
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

impl Rom {
    pub fn new<P: AsRef<Path>>(file: P) -> Result<Self> {
        let mut file = File::open(file)?;
        let mut buffer = Vec::new();

        file.read_to_end(&mut buffer)?;
        Ok(Self { rom: buffer })
    }
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
