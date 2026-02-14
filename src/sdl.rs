use anyhow::Result;
use imgui::Ui;
use imgui_sdl3_renderer::Renderer;
use sdl3::{EventPump, Sdl, event::Event, render::Canvas, video::{SwapInterval, Window}};

use crate::debugger::Debugger;

pub struct SdlInstance {
    pub sdl_context: Sdl,
    pub canvas: Canvas<Window>,
    pub event_pump: EventPump,

    pub debugger: Debugger,
}

impl SdlInstance {
    pub fn new(window_name: &str, width: u32, height: u32) -> Result<Self> {
        let sdl_context = sdl3::init()?;
        let video_subsystem = sdl_context.video()?;

        let window = video_subsystem
            .window(window_name, width, height)
            .opengl()
            .position_centered()
            .resizable()
            .high_pixel_density()
            .build()?;

        let canvas = window.into_canvas();
        let event_pump = sdl_context.event_pump()?;

        video_subsystem.gl_set_swap_interval(SwapInterval::VSync)?;

        Ok(Self {
            sdl_context,
            canvas,
            event_pump,
            debugger: Debugger::new()?,
        })
    }

    pub fn handle_event(&mut self) -> bool {
        for event in self.event_pump.poll_iter() {
            self.debugger
                .platform
                .handle_event(&mut self.debugger.imgui_context, &event);

            if let Event::Quit { .. } = event {
                return true;
            }
        }

        false
    }

    pub fn update_graphics<F: FnOnce(&Ui)>(&mut self, renderer: &mut Renderer, callback: F) -> Result<()> {
        self.debugger.platform.prepare_frame(
            &mut self.sdl_context,
            &mut self.debugger.imgui_context,
            self.canvas.window(),
            &self.event_pump,
        );

        self.canvas.clear();

        // Emulator graphics
        self.debugger.update_graphics(renderer, &mut self.canvas, callback)?;

        self.canvas.present();

        Ok(())
    }
}
