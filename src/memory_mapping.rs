use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub(crate) struct MemoryMapping {
    pub rom: Rom,
    // RAM,
    // VRAM,
    pub stack: [u8; 0x7F],
}

impl Index<u16> for MemoryMapping {
    type Output = u8;
    fn index(&self, index: u16) -> &Self::Output {
        match index {
            0x0..=0x7FFF => &self.rom[index],
            0xFF80..=0xFFFE => &self.stack[index as usize - 0xFF80],
            _ => {
                unimplemented!()
            }
        }
    }
}
impl IndexMut<u16> for MemoryMapping {
    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
        match index {
            0x0..=0x7FFF => {
                panic!("cannot write to rom: {:x}", index);
            }
            0xFF80..=0xFFFE => &mut self.stack[index as usize - 0xFF80],
            _ => {
                unimplemented!()
            }
        }
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
