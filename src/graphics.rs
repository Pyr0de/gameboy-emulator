use std::ops::{Index, IndexMut};

#[allow(dead_code)]
pub(crate) enum LcdControl {
    /// LCD & PPU enable: 0 = Off; 1 = On
    Enable = 0x80,
    /// Window tile map area: 0 = 9800–9BFF; 1 = 9C00–9FFF
    WindowTileMap = 0x40,
    /// Window enable: 0 = Off; 1 = On
    WindowEnable = 0x20,
    /// BG & Window tile data area: 0 = 8800–97FF; 1 = 8000–8FFF
    BGWindowTileData = 0x10,
    /// BG tile map area: 0 = 9800–9BFF; 1 = 9C00–9FFF
    BGTileMap = 0x8,
    /// OBJ size: 0 = 8×8; 1 = 8×16
    OBJSize = 0x4,
    /// OBJ enable: 0 = Off; 1 = On
    OBJEnable = 0x2,
    /// BG & Window enable / priority [Different meaning in CGB Mode]: 0 = Off; 1 = On
    BGWindowEnable = 0x1,
}

#[derive(Debug)]
pub(crate) struct Graphics {
    pub vram: [u8; 0x2000],
    pub lcd_control: u8,
}

impl Graphics {
    pub(crate) fn new() -> Self {
        Graphics {
            vram: [0; 0x2000],
            lcd_control: 0,
        }
    }
}
impl Index<u16> for Graphics {
    type Output = u8;
    fn index(&self, index: u16) -> &Self::Output {
        &self.vram[index as usize]
    }
}
impl IndexMut<u16> for Graphics {
    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
        &mut self.vram[index as usize]
    }
}
