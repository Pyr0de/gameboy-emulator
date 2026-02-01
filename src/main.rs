mod cpu;
mod instructions;
mod memory_mapping;
mod registers;

use std::{
    fs::File,
    io::Read, process::exit,
};

use anyhow::Error;

use crate::{
    cpu::Cpu,
    memory_mapping::{MemoryMapping, Rom},
};

fn gameboy_emulator(file: &str) -> Result<(), Error> {
    let mut file = File::open(file)?;
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)?;
    let memory = MemoryMapping {
        rom: Rom { rom: buffer },
        ..Default::default()
    };

    let mut cpu = Cpu::new(memory);

    for _ in 0..10 {
        cpu.run_instruction()?;
    }

    Ok(())
}

fn main() {
    let file = "tests/roms/halt_bug.gb";
    if let Err(e) = gameboy_emulator(file) {
        eprintln!("{e:?}");
        exit(1);
    }
}
