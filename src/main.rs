mod cli;
mod cpu;
mod graphics;
mod instructions;
mod memory_mapping;
mod registers;

use std::{
    fs::File,
    io::{Read, stdin},
    process::exit,
};

use anyhow::Error;

use crate::{
    cli::Args,
    cpu::Cpu,
    instructions::Instruction,
    memory_mapping::{MemoryMapping, Rom},
};

fn gameboy_emulator(args: Args) -> Result<(), Error> {
    let mut file = File::open(args.file)?;
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)?;
    let memory = MemoryMapping {
        rom: Rom { rom: buffer },
        ..Default::default()
    };

    let mut cpu = Cpu::new(memory);

    loop {
        let instruction = match cpu.run_instruction() {
            Ok(i) => i,
            Err(err) => {
                if args.debug {
                    eprintln!("{err:?}");
                    continue;
                } else {
                    return Err(err);
                }
            }
        };

        if args.debug {
            let mut _in = String::new();
            eprintln!("{:?}\n{:x?}", instruction, cpu.registers);
            stdin().read_line(&mut _in).expect("Failed to read line");
        }

        if let Instruction::STOP(_) = instruction {
            break;
        }
    }

    Ok(())
}

fn main() {
    if let Err(e) = gameboy_emulator(Args::new()) {
        eprintln!("{e:?}");
        exit(1);
    }
}
