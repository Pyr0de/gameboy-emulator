use anyhow::Result;
use imgui::{Context, StyleColor, TableFlags, Ui};
use imgui_sdl3_renderer::Renderer;
use imgui_sdl3_support::SdlPlatform;
use sdl3::{
    render::{Canvas, Texture, TextureCreator},
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
    pub breakpoints: Vec<u16>,
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
            breakpoints: Vec::new(),
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

    pub fn render<'b>(
        &mut self,
        canvas: &mut Canvas<Window>,
        textures: &Vec<Texture<'b>>,
    ) -> Result<()> {
        self.renderer
            .render(self.imgui_context.render(), canvas, textures)?;
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
                    ui.text(format!("Next Instruction: {instruction}"));
                }
            });
        reset
    }

    pub fn display_breakpoint_debugger(ui: &mut Ui, breakpoints: &mut Vec<u16>, pc: u16) {
        ui.window("Breakpoints")
            .size([200., 200.], imgui::Condition::FirstUseEver)
            .position([850., 50.], imgui::Condition::FirstUseEver)
            .build(|| {
                if ui.button("Break at current PC") && !breakpoints.contains(&pc) {
                    breakpoints.push(pc);
                }
                ui.text("0x");
                ui.same_line();
                let mut str = String::from("0000");
                if ui
                    .input_text("###breakpoint_input", &mut str)
                    .enter_returns_true(true)
                    .build()
                    && let Ok(n) = u16::from_str_radix(&str, 16)
                    && !breakpoints.contains(&n)
                {
                    breakpoints.push(n);
                }

                if let Some(_table) =
                    ui.begin_table_with_flags("breakpoints", 2, TableFlags::SIZING_FIXED_FIT)
                {
                    ui.table_setup_column("");
                    ui.table_setup_column("Address");
                    ui.table_headers_row();

                    for (i, addr) in breakpoints.clone().iter().enumerate() {
                        ui.table_next_row();
                        ui.table_set_column_index(0);
                        if ui.button(format!("-###{addr}")) {
                            breakpoints.remove(i);
                        }
                        ui.table_set_column_index(1);

                        let color = if &pc == addr {
                            [0., 1., 0., 1.]
                        } else {
                            [1., 1., 1., 1.]
                        };
                        let _text_color = ui.push_style_color(StyleColor::Text, color);
                        ui.text(format!("0x{addr:04X}"));
                    }
                }
            });
    }
}
