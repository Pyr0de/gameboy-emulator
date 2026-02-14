use anyhow::Result;
use imgui::{Context, Ui};
use imgui_sdl3_renderer::Renderer;
use imgui_sdl3_support::SdlPlatform;
use sdl3::{render::Canvas, video::Window};

pub trait DisplayDebugger {
    fn display_debugger(&self, ui: &Ui);
}

pub struct Debugger {
    pub imgui_context: Context,
    pub platform: SdlPlatform,
}

impl Debugger {
    pub fn new() -> Result<Self> {
        let mut imgui_context = imgui::Context::create();
        imgui_context.set_ini_filename(None);

        imgui_context
            .fonts()
            .add_font(&[imgui::FontSource::DefaultFontData { config: Some(imgui::FontConfig {..Default::default() }) }]);

        let platform = SdlPlatform::new(&mut imgui_context);

        Ok(Debugger {
            imgui_context,
            platform,
        })
    }

    pub fn update_graphics<F: FnOnce(&Ui)>(
        &mut self,
        renderer: &mut Renderer,
        canvas: &mut Canvas<Window>,
        callback: F
    ) -> Result<()> {
        let ui = self.imgui_context.new_frame();
        callback(ui);

        renderer.render(self.imgui_context.render(), canvas)?;

        Ok(())
    }
}

