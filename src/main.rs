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
mod utils;

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
    args: &Args,
    sdl: &mut SdlInstance,
    debugger: &mut Debugger,
) -> Result<bool, Error> {
    let mut texture_creator = sdl.canvas.texture_creator();

    let memory = MemoryMapping::new(Rom::new(&args.file)?);
    let mut cpu = Cpu::new(memory);

    cpu.memory.vram.create_textures(&mut texture_creator)?;

    let mut errors: Vec<(u16, String)> = Vec::new();

    'main: loop {
        // Handle sdl events
        if sdl.handle_event(debugger) {
            break 'main;
        }

        // Run execute instruction
        let (instruction, inc) = cpu.get_instruction()?;
        let sleep_duration = if debugger.should_execute() {
            let last = Instant::now();
            let pc = cpu.registers.pc;
            let cycles = match (cpu.run_instruction(instruction.clone(), inc), args.debug) {
                (Ok(c), _) => c,
                (Err(e), true) => {
                    errors.push((pc, format!("{e:?}")));
                    continue;
                }
                (Err(e), false) => {
                    eprintln!("{e:?}");
                    continue;
                }
            };

            cpu.memory
                .vram
                .do_cycles(cycles, &mut cpu.memory.interrupt)?;

            let time_taken = last.duration_since(Instant::now());

            if let Instruction::STOP(_) = instruction {
                break;
            }

            // Calculation: Clock speed = 4194304 Hz
            //              M-Cycles/sec = 4194304/4 = 1048576 M-cycles/sec
            //              1 M-cycles takes 1/1048576 sec = 0.000000954 sec
            //                                             = 954 ns

            if debugger.breakpoints.contains(&cpu.registers.pc) {
                debugger.execution_state = debugger::ExecutionState::Pause;
            }

            Duration::from_nanos(954 * cycles as u64).saturating_sub(time_taken)
        } else {
            sdl.to_sleep()
        };

        sleep(sleep_duration);

        cpu.memory.vram.update_textures()?;

        // Update graphics
        if let Some(mut token) = sdl.update_graphics(debugger) {
            let sdl = &mut token.0;

            cpu.memory.vram.display_screen(&mut sdl.canvas)?;

            if !args.debug {
                continue;
            }
            let ui = debugger.imgui_context.new_frame();

            let reset = Debugger::display_execution_debugger(
                ui,
                &mut debugger.execution_state,
                instruction,
            );
            Debugger::display_breakpoint_debugger(ui, &mut debugger.breakpoints, cpu.registers.pc);

            cpu.registers.display_debugger(ui);
            cpu.memory.display_debugger(ui, cpu.registers.pc);
            cpu.memory.vram.display_debugger(ui);

            ui.window("Errors")
                .position([500., 50.], imgui::Condition::FirstUseEver)
                .size([300., 200.], imgui::Condition::FirstUseEver)
                .horizontal_scrollbar(true)
                .build(|| {
                    for (pc, err) in &errors {
                        ui.text(format!("PC: 0x{pc:04x} -> {err}"));
                    }
                });

            debugger.render(&mut sdl.canvas, &cpu.memory.vram.textures)?;
            if reset {
                return Ok(true);
            }
        }
    }

    Ok(false)
}

fn main() {
    let args = Args::new();
    let debugger_str = if args.debug { " (Debug)" } else { "" };
    let window_name = format!(
        "Emulator{}: {}",
        debugger_str,
        args.file.to_str().unwrap_or("")
    );

    let mut sdl = SdlInstance::new(&window_name, 1600, 900).expect("Error Initializing SDL");
    let texture_creator = sdl.canvas.texture_creator();
    let mut debugger = Debugger::new(&texture_creator).expect("Error Initializing Imgui");
    if !args.debug {
        debugger.execution_state = debugger::ExecutionState::Execute;
    }

    loop {
        match gameboy_emulator(&args, &mut sdl, &mut debugger) {
            Err(e) => {
                eprintln!("{e:?}");
                exit(1);
            }
            Ok(false) => {
                break;
            }
            _ => {}
        }
    }
}
