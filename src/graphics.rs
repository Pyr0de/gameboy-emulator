use std::{
    fmt::Debug,
    ops::{Index, IndexMut},
};

use anyhow::{Result, bail};
use sdl3::{
    pixels::{Color, Palette, PixelFormat},
    render::{Canvas, FRect, Texture, TextureCreator},
    sys::pixels::SDL_PixelFormat,
    video::{Window, WindowContext},
};

#[allow(dead_code)]
pub(crate) enum LcdControl {
    /// LCD & PPU enable: 0 = Off; 1 = On
    Enable = 0x80,
    /// Window tile map area: 0 = 9800–9BFF; 1 = 9C00–9FFF
    WindowTileMap = 0x40,
    /// Window enable: 0 = Off; 1 = On
    WindowEnable = 0x20,
    /// BG & Window tile data area: 0 = 8800–97FF; 1 = 8000–8FFF
    BGWindowTileData = 0x10,
    /// BG tile map area: 0 = 9800–9BFF; 1 = 9C00–9FFF
    BGTileMap = 0x8,
    /// OBJ size: 0 = 8×8; 1 = 8×16
    OBJSize = 0x4,
    /// OBJ enable: 0 = Off; 1 = On
    OBJEnable = 0x2,
    /// BG & Window enable / priority [Different meaning in CGB Mode]: 0 = Off; 1 = On
    BGWindowEnable = 0x1,
}

pub(crate) struct Graphics<'a> {
    pub vram: [u8; 0x2000],
    pub lcd_control: u8,
    textures: Option<Vec<Texture<'a>>>,
    changed_textures: Vec<u16>,
}

impl<'a> Graphics<'a> {
    pub(crate) fn new() -> Self {
        Graphics {
            vram: [0; 0x2000],
            lcd_control: 0,
            textures: None,
            changed_textures: Vec::new(),
        }
    }

    pub fn create_textures(
        &mut self,
        texture_creator: &'a mut TextureCreator<WindowContext>,
    ) -> Result<()> {
        static PALETTE: [Color; 4] = [
            Color::RGB(0xc4, 0xf0, 0xc2),
            Color::RGB(0x5a, 0xb9, 0xa8),
            Color::RGB(0x1e, 0x60, 0x6e),
            Color::RGB(0x2d, 0x1b, 0x00),
        ];
        let palette = Palette::with_colors(&PALETTE)?;

        // Test sprite
        self.vram[0..16].copy_from_slice(&[
            0x3C, 0x7E, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x7E, 0x5E, 0x7E, 0x0A, 0x7C, 0x56,
            0x38, 0x7C,
        ]);

        let pixel_format = unsafe { PixelFormat::from_ll(SDL_PixelFormat::INDEX8) };

        let textures = (0..0x2000 / 16)
            .map(|_| {
                let mut texture = texture_creator
                    .create_texture_streaming(pixel_format, 8, 8)
                    .expect("Error creating texture");
                unsafe {
                    sdl3_sys::render::SDL_SetTexturePalette(texture.raw(), palette.raw());
                }
                texture.set_scale_mode(sdl3::render::ScaleMode::Nearest);
                texture
            })
            .collect();

        self.textures = Some(textures);

        Ok(())
    }

    pub fn update_textures(&mut self) -> Result<()> {
        let Some(textures) = &mut self.textures else {
            bail!("Textures not created");
        };

        for i in &self.changed_textures {
            let tex_id = *i as usize / 16;
            textures[tex_id as usize].with_lock(None, |data, _| {
                let (start, end) = (tex_id * 16, tex_id * 16 + 16);
                let m = to_8bit_indexed(&self.vram[start..end]);
                data.copy_from_slice(&m);
            })?;
        }
        self.changed_textures = Vec::new();
        Ok(())
    }

    pub fn render_textures(&mut self, canvas: &mut Canvas<Window>) -> Result<()> {
        //for i in &self.textures {
        //    canvas.copy(&i, None, Some(FRect::new(0., 0., 256., 256.)))?;
        //}
        Ok(())
    }
}

/// Convert indexed 2 bit msb to indexed 8 bit
/// joining byte 0 and byte 1 to maked 8 bit index for sdl
fn to_8bit_indexed(bytes: &[u8]) -> [u8; 64] {
    let mut ans = [0; 64];
    for i in 0..8 {
        let (b1, b2) = (bytes[2 * i], bytes[2 * i + 1]);

        for j in (0..8).rev() {
            let b = (((b2 >> j) & 1) << 1) | ((b1 >> j) & 1);
            ans[i * 8 + (7 - j)] = b;
        }
    }
    ans
}

impl<'a> Index<u16> for Graphics<'a> {
    type Output = u8;
    fn index(&self, index: u16) -> &Self::Output {
        &self.vram[index as usize]
    }
}
impl<'a> IndexMut<u16> for Graphics<'a> {
    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
        self.changed_textures.push(index);
        &mut self.vram[index as usize]
    }
}

impl<'a> Debug for Graphics<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Graphics")
            .field("vram", &self.vram)
            .field("lcd_control", &self.lcd_control)
            .finish()
    }
}
