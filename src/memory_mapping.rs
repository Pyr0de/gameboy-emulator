use std::ops::{Index, IndexMut};

use anyhow::{Result, bail};

use crate::graphics::Graphics;

#[derive(Debug)]
pub(crate) struct MemoryMapping {
    pub rom: Rom,
    pub vram: Graphics,
    pub external_ram: [u8; 0x2000],
    pub wram: WRam,
    pub stack: [u8; 0x7F],
}

impl Default for MemoryMapping {
    fn default() -> Self {
        Self {
            rom: Rom::default(),
            vram: Graphics::new(),
            external_ram: [0; 0x2000],
            wram: WRam::default(),
            stack: [0; 0x7F],
        }
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
