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
    fs::File,
    io::Read,
    process::exit,
    thread::sleep,
    time::{Duration, Instant},
};

use anyhow::Error;

use crate::{
    cli::Args,
    cpu::Cpu,
    instructions::Instruction,
    memory_mapping::{MemoryMapping, Rom},
    sdl::SdlInstance,
};

fn gameboy_emulator(args: Args) -> Result<(), Error> {
    let mut file = File::open(&args.file)?;
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)?;
    let memory = MemoryMapping::new(Rom { rom: buffer });

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
        let mut graphics_sleep = false;

        if !pause || step {
            let last = Instant::now();
            let pc = cpu.registers.pc;
            let cycles = match (cpu.run_instruction(instruction.clone(), inc), args.debug) {
                (Ok(c), _) => c,
                (Err(e), true) => {
                    sdl.debugger.errors.push((pc, format!("{e:?}")));
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
        sdl.update_graphics(&mut renderer, graphics_sleep, |ui| {
            ui.window("Execution")
                .size([400., 150.], imgui::Condition::FirstUseEver)
                .build(|| {
                    ui.text(format!("{} fps", ui.io().framerate as usize));
                    ui.checkbox("Pause", &mut pause);
                    step = ui.button("Step");
                    if pause {
                        ui.text(format!("Next Instruction: {instruction:?}"));
                    }
                });
            cpu.registers.display_debugger(ui);
            cpu.memory.display_debugger(ui, cpu.registers.pc);
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
