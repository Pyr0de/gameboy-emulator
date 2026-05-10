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
    cell::RefCell,
    path::PathBuf,
    process::exit,
    rc::Rc,
    thread::sleep,
    time::{Duration, Instant},
};

use anyhow::{Error, bail};
use sdl3::dialog::{DialogCallback, show_open_file_dialog};

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

            let time_taken = Instant::now().duration_since(last);

            if let Instruction::STOP(_) = instruction {
                break;
            }

            if debugger.breakpoints.contains(&cpu.registers.pc) {
                debugger.execution_state = debugger::ExecutionState::Pause;
            }

            // Calculation: Clock speed = 4194304 Hz
            //              M-Cycles/sec = 4194304/4 = 1048576 M-cycles/sec
            //              1 M-cycles takes 1/1048576 sec = 0.000000954 sec
            //                                             = 954 ns
            Duration::from_nanos(954 * cycles as u64).saturating_sub(time_taken)
        } else {
            sdl.to_sleep()
        };

        sleep(sleep_duration);

        cpu.memory.vram.update_textures()?;

        // Update graphics
        let Some(mut token) = sdl.update_graphics(debugger) else {
            continue;
        };
        let sdl = &mut token.0;

        cpu.memory.vram.display_screen(&mut sdl.canvas)?;

        if args.debug {
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
    let mut sdl = SdlInstance::new("Emulator", 1600, 900).expect("Error Initializing SDL");
    let texture_creator = sdl.canvas.texture_creator();
    let mut debugger = Debugger::new(&texture_creator).expect("Error Initializing Imgui");

    let args = match Args::new() {
        Some(args) => args,
        None => show_arguments_gui(&mut sdl, &mut debugger).unwrap(),
    };

    let debugger_str = if args.debug { " (Debug)" } else { "" };
    let window_name = format!(
        "Emulator{}: {}",
        debugger_str,
        args.file.to_str().unwrap_or("")
    );
    sdl.canvas.window_mut().set_title(&window_name).unwrap();

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

fn show_arguments_gui(sdl: &mut SdlInstance, debugger: &mut Debugger) -> anyhow::Result<Args> {
    let file = Rc::new(RefCell::new(None::<PathBuf>));
    let mut debug = false;

    let mut running = true;

    while running {
        if sdl.handle_event(debugger) {
            exit(0);
        }
        let Some(mut token) = sdl.update_graphics(debugger) else {
            continue;
        };
        let sdl = &mut token.0;
        let ui = debugger.imgui_context.new_frame();

        ui.window("Gameboy Emulator")
            .size([300., 300.], imgui::Condition::FirstUseEver)
            .build(|| {
                if ui.button("Select ROM File") {
                    let file_ref = Rc::clone(&file);
                    let file_dialog_callback: DialogCallback = Box::new(move |files, _| {
                        let files = match files {
                            Ok(f) => f,
                            Err(e) => {
                                eprintln!("{e:?}");
                                return;
                            }
                        };
                        let Some(path) = files.first().filter(|f| f.exists()) else {
                            eprintln!("No valid path found");
                            return;
                        };

                        *file_ref.borrow_mut() = Some(PathBuf::from(path));
                    });
                    show_open_file_dialog(&[], None::<String>, false, None, file_dialog_callback)
                        .unwrap();
                }

                if let Some(rom_path) = file.borrow().as_ref() {
                    ui.same_line();
                    ui.text(format!("{}", rom_path.to_string_lossy()));
                }

                ui.checkbox("debug", &mut debug);

                if ui.button("Load ROM") {
                    running = false;
                }
            });

        debugger.render(&mut sdl.canvas, &Vec::new())?;
    }
    match file.borrow().as_ref() {
        Some(file) => Ok(Args {
            file: file.clone(),
            debug,
        }),
        None => bail!("No file found"),
    }
}
