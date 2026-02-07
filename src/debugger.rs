use anyhow::Result;
use imgui::Context;
use imgui_sdl3_renderer::Renderer;
use imgui_sdl3_support::SdlPlatform;
use sdl3::{render::Canvas, video::Window};

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
            .add_font(&[imgui::FontSource::DefaultFontData { config: None }]);

        let platform = SdlPlatform::new(&mut imgui_context);

        Ok(Debugger {
            imgui_context,
            platform,
        })
    }

    pub fn update_graphics(
        &mut self,
        renderer: &mut Renderer,
        canvas: &mut Canvas<Window>,
    ) -> Result<()> {
        let ui = self.imgui_context.new_frame();
        ui.show_demo_window(&mut true);

        renderer.render(self.imgui_context.render(), canvas)?;

        Ok(())
    }
}
