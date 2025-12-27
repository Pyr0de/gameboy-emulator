mod registers;
mod memory_mapping;
mod cpu;

use std::{fs::File, io::{Error, Read}};

use crate::{cpu::CPU, memory_mapping::{MemoryMapping, ROM}};

fn main() -> Result<(), Error> {
    let mut file = File::open("tests/roms/halt_bug.gb")?;
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)?;
    let memory = MemoryMapping{ ROM: ROM{rom: buffer} };

    let mut cpu = CPU::new();

    for _ in 0..10 {
        cpu.run_instruction(&memory);
    }

    Ok(())
}
