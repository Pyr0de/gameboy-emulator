use anyhow::Result;
use imgui::{Context, Ui};
use imgui_sdl3_renderer::Renderer;
use imgui_sdl3_support::SdlPlatform;
use sdl3::{
    render::{Canvas, TextureCreator},
    video::{Window, WindowContext},
};

pub struct Debugger<'a> {
    pub imgui_context: Context,
    pub platform: SdlPlatform,
    pub renderer: Renderer<'a>,

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
            errors: Vec::new(),
        })
    }

    pub fn update_graphics<F: FnOnce(&Ui)>(
        &mut self,
        canvas: &mut Canvas<Window>,
        callback: F,
    ) -> Result<()> {
        let ui = self.imgui_context.new_frame();
        callback(ui);

        ui.window("Errors")
            .position([500., 50.], imgui::Condition::FirstUseEver)
            .size([300., 200.], imgui::Condition::FirstUseEver)
            .horizontal_scrollbar(true)
            .build(|| {
                for (pc, err) in &self.errors {
                    ui.text(format!("PC: 0x{pc:04x} -> {err}"));
                }
            });

        self.renderer.render(self.imgui_context.render(), canvas)?;

        Ok(())
    }
}
