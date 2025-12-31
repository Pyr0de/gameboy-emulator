mod cpu;
mod memory_mapping;
mod registers;

use std::{
    fs::File,
    io::{Error, Read},
};

use crate::{
    cpu::Cpu,
    memory_mapping::{MemoryMapping, Rom},
};

fn main() -> Result<(), Error> {
    let mut file = File::open("tests/roms/halt_bug.gb")?;
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)?;
    let memory = MemoryMapping {
        rom: Rom { rom: buffer },
        stack: [0; 0x7F],
    };

    let mut cpu = Cpu::new();

    for _ in 0..10 {
        cpu.run_instruction(&memory);
    }

    Ok(())
}
