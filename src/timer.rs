use crate::interrupt::{Interrupt, InterruptPosition};

enum TimerController {
    ClockSelect = 0b11,
    Enable = 0b100,
}

#[derive(Debug, Default)]
pub struct Timer {
    internal_counter: u8,

    // DIV should reset when value it changed and during STOP
    pub divider_register: u8,
    pub timer_counter: u8,
    pub timer_modulo: u8,
    pub timer_controller: u8,
}

impl Timer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn do_cycles(&mut self, interrupt: &mut Interrupt, cycles: u8) {
        for _ in 0..cycles {
            self.do_cycle(interrupt);
        }
    }

    pub fn do_cycle(&mut self, interrupt: &mut Interrupt) {
        self.internal_counter = self.internal_counter.wrapping_add(1);

        if self.internal_counter.is_multiple_of(64) {
            self.divider_register = self.divider_register.wrapping_add(1);
        }

        let enabled = self.timer_controller & TimerController::Enable as u8 != 0;
        if enabled {
            let increment_every = match self.timer_controller & TimerController::ClockSelect as u8 {
                0b00 => 256,
                0b01 => 4,
                0b10 => 16,
                0b11 => 64,
                _ => unreachable!(),
            };

            if (self.internal_counter as u16).is_multiple_of(increment_every) {
                self.timer_counter = match self.timer_counter.checked_add(1) {
                    Some(v) => v,
                    None => {
                        interrupt.request_int(InterruptPosition::Timer);
                        self.timer_modulo
                    }
                };
            }
        }
    }
}
