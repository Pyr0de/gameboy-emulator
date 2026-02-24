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
pub enum ExecutionState {
    #[default]
    Pause,
    Step,
    Execute,
}

pub struct Debugger<'a> {
    pub imgui_context: Context,
    pub platform: SdlPlatform,
    pub renderer: Renderer<'a>,

    pub execution_state: ExecutionState,
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

    pub fn render(&mut self, canvas: &mut Canvas<Window>) -> Result<()> {
        self.renderer.render(self.imgui_context.render(), canvas)?;
        Ok(())
    }

    pub fn display_execution_debugger(
        ui: &mut Ui,
        execution_state: &mut ExecutionState,
        instruction: Instruction,
    ) -> bool {
        let mut reset = false;

        ui.window("Execution")
            .size([400., 150.], imgui::Condition::FirstUseEver)
            .build(|| {
                ui.text(format!("{} fps", ui.io().framerate as usize));

                if ui.button("Reset") {
                    reset = true;
                }
                ui.same_line();

                let mut pause = !matches!(execution_state, ExecutionState::Execute);
                ui.checkbox("Pause", &mut pause);
                if ui.button("Step") && pause {
                    *execution_state = ExecutionState::Step
                } else {
                    *execution_state = match pause {
                        true => ExecutionState::Pause,
                        false => ExecutionState::Execute,
                    }
                }
                if pause {
                    ui.text(format!("Next Instruction: {instruction:?}"));
                }
            });
        reset
    }
}
