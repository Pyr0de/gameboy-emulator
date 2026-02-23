use anyhow::Result;
use imgui::{Context, Ui};
use imgui_sdl3_renderer::Renderer;
use imgui_sdl3_support::SdlPlatform;
use sdl3::{
    render::{Canvas, TextureCreator},
    video::{Window, WindowContext},
};

use crate::instructions::Instruction;

#[derive(Debug, Default)]
enum ExecutionState {
    #[default]
    Pause,
    Step,
    Execute,
}

pub struct Debugger<'a> {
    pub imgui_context: Context,
    pub platform: SdlPlatform,
    pub renderer: Renderer<'a>,

    execution_state: ExecutionState,

    pub errors: Vec<(u16, String)>,
}

impl<'a> Debugger<'a> {
    pub fn new(texture_creator: &'a TextureCreator<WindowContext>) -> Result<Self> {
        let mut imgui_context = imgui::Context::create();
        imgui_context.io_mut().config_flags |= imgui::ConfigFlags::NAV_ENABLE_KEYBOARD;
        imgui_context.set_ini_filename(None);

        imgui_context
            .fonts()
            .add_font(&[imgui::FontSource::DefaultFontData {
                config: Some(imgui::FontConfig {
                    ..Default::default()
                }),
            }]);

        let platform = SdlPlatform::new(&mut imgui_context);
        let renderer = imgui_sdl3_renderer::Renderer::new(texture_creator, &mut imgui_context)?;

        Ok(Debugger {
            imgui_context,
            platform,
            renderer,
            execution_state: ExecutionState::default(),
            errors: Vec::new(),
        })
    }

    pub fn should_execute(&mut self) -> bool {
        match self.execution_state {
            ExecutionState::Pause => false,
            ExecutionState::Execute => true,
            ExecutionState::Step => {
                self.execution_state = ExecutionState::Pause;
                true
            }
        }
    }

    pub fn update_graphics<F: FnOnce(&Ui)>(
        &mut self,
        canvas: &mut Canvas<Window>,
        instruction: Instruction,
        callback: F,
    ) -> Result<()> {
        let ui = self.imgui_context.new_frame();

        ui.window("Execution")
            .size([400., 150.], imgui::Condition::FirstUseEver)
            .build(|| {
                ui.text(format!("{} fps", ui.io().framerate as usize));

                let mut pause = !matches!(self.execution_state, ExecutionState::Execute);
                ui.checkbox("Pause", &mut pause);
                if ui.button("Step") && pause {
                    self.execution_state = ExecutionState::Step
                } else {
                    self.execution_state = match pause {
                        true => ExecutionState::Pause,
                        false => ExecutionState::Execute,
                    }
                }
                if pause {
                    ui.text(format!("Next Instruction: {instruction:?}"));
                }
            });

        ui.window("Errors")
            .position([500., 50.], imgui::Condition::FirstUseEver)
            .size([300., 200.], imgui::Condition::FirstUseEver)
            .horizontal_scrollbar(true)
            .build(|| {
                for (pc, err) in &self.errors {
                    ui.text(format!("PC: 0x{pc:04x} -> {err}"));
                }
            });

        callback(ui);

        self.renderer.render(self.imgui_context.render(), canvas)?;

        Ok(())
    }
}
