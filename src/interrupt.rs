use crate::utils::BitFlag;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum InterruptPosition {
    VBlank = 0x01,
    Lcd = 0x02,
    Timer = 0x04,
    Serial = 0x08,
    Joypad = 0x10,
}

impl From<InterruptPosition> for u8 {
    fn from(value: InterruptPosition) -> Self {
        value as u8
    }
}

#[derive(Debug, Default)]
pub struct Interrupt {
    pub interrupt_enable: BitFlag<u8, InterruptPosition>,
    pub interrupt_flag: BitFlag<u8, InterruptPosition>,

    ime: bool,
    set_ime: bool,
}

impl Interrupt {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn handle_interrupts(&mut self) -> Option<u16> {
        let serviceable = self.interrupt_flag.value & self.interrupt_enable.value;
        if self.ime && serviceable != 0 {
            for i in 0..=4 {
                if self.interrupt_flag.get_into(1 << i) {
                    self.interrupt_flag.set_into(1 << i, false);
                    return Some(0x40 + i as u16 * 8);
                }
            }
        }

        // EI is delayed by 1 instruction
        if self.set_ime {
            self.set_ime = false;
            self.ime = true;
        }

        None
    }

    pub fn set_ime(&mut self) {
        // Enables interrupts and returns (same as ei immediately followed by ret)
        self.set_ime = true;
    }
    pub fn set_ime_forced(&mut self) {
        self.set_ime = false;
        self.ime = true;
    }

    pub fn reset_ime(&mut self) {
        self.ime = false;
        self.set_ime = false;
    }

    #[allow(dead_code)]
    pub fn request_int(&mut self, int: InterruptPosition) {
        self.interrupt_flag.set(int, true);
    }

    #[allow(dead_code)]
    pub fn enable_int(&mut self, int: InterruptPosition, val: bool) {
        if self.interrupt_enable.get(int) != val {
            self.interrupt_enable.set(int, true);
        }
    }
}
