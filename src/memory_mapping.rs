use std::ops::Index;

#[derive(Default, Debug)]
pub(crate) struct MemoryMapping {
    pub ROM: ROM,
    // RAM,
    // VRAM,
}

impl Index<u16> for MemoryMapping {
    type Output = u8;
    fn index(&self, index: u16) -> &Self::Output {
        match index {
            0x0..0x8000 => {
                &self.ROM[index]
            }
            _ => {
                unimplemented!()
            }
        }
    }
}

#[derive(Default, Debug)]
pub(crate) struct ROM {
    // Header
    pub rom: Vec<u8>
}

impl Index<u16> for ROM {
    type Output = u8;
    fn index(&self, index: u16) -> &Self::Output {
        // TODO: switchable banks
        &self.rom[index as usize]
    }
}

