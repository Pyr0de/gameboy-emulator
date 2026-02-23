mod cli;
mod cpu;
mod debugger;
mod graphics;
mod instructions;
mod interrupt;
mod memory_mapping;
mod registers;
mod sdl;
mod timer;

use std::{
    process::exit,
    thread::sleep,
    time::{Duration, Instant},
};

use anyhow::Error;

use crate::{
    cli::Args,
    cpu::Cpu,
    debugger::Debugger,
    instructions::Instruction,
    memory_mapping::{MemoryMapping, Rom},
    sdl::SdlInstance,
};

fn gameboy_emulator(
    args: Args,
    sdl: &mut SdlInstance,
    debugger: &mut Debugger,
) -> Result<(), Error> {
    let memory = MemoryMapping::new(Rom::new(&args.file)?);

    let mut cpu = Cpu::new(memory);

    'main: loop {
        // Handle sdl events
        if sdl.handle_event(debugger) {
            break 'main;
        }

        // Run execute instruction
        let (instruction, inc) = cpu.get_instruction()?;
        let mut graphics_sleep = false;

        if debugger.should_execute() {
            let last = Instant::now();
            let pc = cpu.registers.pc;
            let cycles = match (cpu.run_instruction(instruction.clone(), inc), args.debug) {
                (Ok(c), _) => c,
                (Err(e), true) => {
                    debugger.errors.push((pc, format!("{e:?}")));
                    continue;
                }
                (Err(e), false) => return Err(e),
            };

            let time_taken = last.duration_since(Instant::now());

            // Calculation: Clock speed = 4194304 Hz
            //              M-Cycles/sec = 4194304/4 = 1048576 M-cycles/sec
            //              1 M-cycles takes 1/1048576 sec = 0.000000954 sec
            //                                             = 954 ns
            sleep(Duration::from_nanos(954 * cycles as u64).saturating_sub(time_taken));

            if let Instruction::STOP(_) = instruction {
                break;
            }
        } else {
            graphics_sleep = true;
        }

        // Update graphics
        sdl.update_graphics(debugger, instruction, graphics_sleep, |ui| {
            cpu.registers.display_debugger(ui);
            cpu.memory.display_debugger(ui, cpu.registers.pc);
        })?;
    }

    Ok(())
}

fn main() {
    let args = Args::new();
    let window_name = format!("Emulator: {}", args.file.to_str().unwrap_or(""));

    let mut sdl = SdlInstance::new(&window_name, 1600, 900).expect("Error Initializing SDL");
    let texture_creator = sdl.canvas.texture_creator();
    let mut debugger = Debugger::new(&texture_creator).expect("Error Initializing Imgui");

    if let Err(e) = gameboy_emulator(args, &mut sdl, &mut debugger) {
        eprintln!("{e:?}");
        exit(1);
    }
}
