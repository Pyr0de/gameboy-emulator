mod cli;
mod cpu;
mod debugger;
mod graphics;
mod instructions;
mod memory_mapping;
mod registers;
mod sdl;

use std::{fs::File, io::Read, process::exit};

use anyhow::Error;

use crate::{
    cli::Args, cpu::Cpu, debugger::DisplayDebugger, instructions::Instruction, memory_mapping::{MemoryMapping, Rom}, sdl::SdlInstance
};

fn gameboy_emulator(args: Args) -> Result<(), Error> {
    let mut file = File::open(&args.file)?;
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)?;
    let memory = MemoryMapping {
        rom: Rom { rom: buffer },
        ..Default::default()
    };

    let mut cpu = Cpu::new(memory);

    let window_name = format!("Emulator: {}", args.file.to_str().unwrap_or(""));

    let mut sdl = SdlInstance::new(&window_name, 1600, 900)?;
    let texture_creator = sdl.canvas.texture_creator();
    let mut renderer =
        imgui_sdl3_renderer::Renderer::new(&texture_creator, &mut sdl.debugger.imgui_context)?;

    let mut pause = true;
    let mut step = false;

    'main: loop {
        // Handle sdl events
        if sdl.handle_event() {
            break 'main;
        }

        // Run execute instruction
        let (instruction, inc) = cpu.get_instruction()?;

        if !pause || step {
            let pc = cpu.registers.pc;
            if let Err(e) = cpu.run_instruction(instruction.clone(), inc) {
                if args.debug {
                    sdl.debugger.errors.push((pc, format!("{e:?}")));
                    continue;
                }else {
                    return Err(e)
                }
            }

            if let Instruction::STOP(_) = instruction {
                break;
            }
        }

        // Update graphics
        sdl.update_graphics(&mut renderer, |ui| {
            ui.window("Execution")
                .size([400., 100.], imgui::Condition::FirstUseEver)
                .build(|| {
                    ui.checkbox("Pause", &mut pause);
                    step = ui.button("Step");
                    if pause {
                        ui.text(format!("Next Instruction: {instruction:?}"));
                    }
                });
            cpu.registers.display_debugger(ui);
        })?;
    }

    Ok(())
}

fn main() {
    if let Err(e) = gameboy_emulator(Args::new()) {
        eprintln!("{e:?}");
        exit(1);
    }
}
