use std::{
    fmt::Debug,
    ops::{Index, IndexMut},
};

use anyhow::{Result, bail};
use imgui::{Image, TextureId, Ui};
use sdl3::{
    pixels::{Color, Palette, PixelFormat},
    rect::Rect,
    render::{Texture, TextureCreator},
    video::WindowContext,
};

use crate::{interrupt::Interrupt, utils::BitFlag};

static DEFAULT_COLORS: [Color; 4] = [
    Color::RGB(0xc4, 0xf0, 0xc2),
    Color::RGB(0x5a, 0xb9, 0xa8),
    Color::RGB(0x1e, 0x60, 0x6e),
    Color::RGB(0x2d, 0x1b, 0x00),
];

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
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

impl From<LcdControl> for u8 {
    fn from(value: LcdControl) -> Self {
        value as u8
    }
}

#[derive(Debug, Clone, Copy)]
pub enum LcdStatus {
    /// Indicates if PPU is enabled
    PPUMode = 0b11,
    /// Set when LY == LYC
    LYCEqLY = 0b100,
    /// If set, selects the Mode 0 condition for the STAT interrupt.
    Mode0Int = 0b1000,
    /// If set, selects the Mode 1 condition for the STAT interrupt.
    Mode1Int = 0b10000,
    /// If set, selects the Mode 2 condition for the STAT interrupt.
    Mode2Int = 0b100000,
    /// If set, selects the LYC == LY condition for the STAT interrupt.
    LYCInt = 0b1000000,
}

impl From<LcdStatus> for u8 {
    fn from(value: LcdStatus) -> Self {
        value as u8
    }
}

#[derive(Debug, Default)]
struct DebuggerContext {
    page: usize,
    palette_colors: [Color; 4],
}

pub(crate) struct Graphics<'a> {
    pub vram: [u8; 0x2000],
    pub lcd_control: BitFlag<u8, LcdControl>,
    pub scroll_x: u8,
    pub scroll_y: u8,
    pub y_coord: u8,
    x_coord: u16,
    pub y_comp: u8,
    pub lcd_status: BitFlag<u8, LcdStatus>,
    pub textures: Vec<Texture<'a>>,
    changed_textures: Vec<u16>,

    pub bg_id: Option<TextureId>,

    debug: DebuggerContext,
}

impl<'a> Graphics<'a> {
    pub(crate) fn new() -> Self {
        Graphics {
            vram: [0; 0x2000],
            lcd_control: BitFlag::default(),
            scroll_x: 0,
            scroll_y: 0,
            y_coord: 0,
            x_coord: 0,
            y_comp: 0,
            lcd_status: BitFlag::default(),
            textures: Vec::new(),
            changed_textures: Vec::new(),
            bg_id: None,
            debug: DebuggerContext::default(),
        }
    }

    pub fn create_textures(
        &mut self,
        texture_creator: &'a mut TextureCreator<WindowContext>,
    ) -> Result<()> {
        self.debug.palette_colors = DEFAULT_COLORS;
        let palette = Palette::with_colors(&DEFAULT_COLORS)?;

        // Test sprite
        //self.vram[0..16].copy_from_slice(&[
        //    0x3C, 0x7E, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x7E, 0x5E, 0x7E, 0x0A, 0x7C, 0x56,
        //    0x38, 0x7C,
        //]);

        let textures = (0..0x2000 / 16)
            .map(|_| {
                let mut texture = texture_creator
                    .create_texture_streaming(PixelFormat::INDEX8, 8, 8)
                    .expect("Error creating texture");
                unsafe {
                    sdl3_sys::render::SDL_SetTexturePalette(texture.raw(), palette.raw());
                }
                texture.set_scale_mode(sdl3::render::ScaleMode::Nearest);
                texture
            })
            .collect();

        self.textures = textures;

        self.textures.push({
            let mut texture = texture_creator
                .create_texture_streaming(PixelFormat::INDEX8, 160, 144)
                .expect("Error creating texture");
            unsafe {
                sdl3_sys::render::SDL_SetTexturePalette(texture.raw(), palette.raw());
            }
            texture.set_scale_mode(sdl3::render::ScaleMode::Nearest);
            texture
        });
        // Considering +1 for font texture
        self.bg_id = Some(TextureId::new(self.textures.len()));

        Ok(())
    }

    pub fn update_textures(&mut self) -> Result<()> {
        for i in &self.changed_textures {
            let tex_id = *i as usize / 16;
            self.textures[tex_id].with_lock(None, |data, _| {
                let (start, end) = (tex_id * 16, tex_id * 16 + 16);
                let m = to_8bit_indexed(&self.vram[start..end]);
                data.copy_from_slice(&m);
            })?;
        }
        self.changed_textures = Vec::new();
        Ok(())
    }

    /// Updating each line at once after 172 dots in Mode 3 (ignoring penalties)
    /// TODO: Penalties and update each dot instead of whole line
    pub fn do_cycles(&mut self, cycles: u8, interrupt: &mut Interrupt) -> Result<()> {
        let (old_x, old_y) = (self.x_coord, self.y_coord);

        self.x_coord += cycles as u16;
        if self.x_coord > 456 {
            self.x_coord %= 456;
            self.y_coord += 1;
        }
        if self.y_coord > 153 {
            self.y_coord %= 153
        }

        let mode0_int =
            old_x <= 252 && self.x_coord > 252 && self.lcd_status.get(LcdStatus::Mode0Int);
        let mode1_int =
            old_y <= 143 && self.y_coord > 143 && self.lcd_status.get(LcdStatus::Mode1Int);
        let mode2_int =
            old_x > 80 && self.x_coord <= 80 && self.lcd_status.get(LcdStatus::Mode2Int);
        let lyc_int = old_y != self.y_coord
            && self.y_coord == self.y_comp
            && self.lcd_status.get(LcdStatus::LYCInt);

        if mode0_int || mode1_int || mode2_int || lyc_int {
            interrupt.request_int(crate::interrupt::InterruptPosition::Lcd);
        }

        if old_x <= 252 && self.x_coord > 252 && self.y_coord < 144 {
            self.update_background_texture(self.y_coord)?;
        }

        Ok(())
    }

    fn update_background_texture(&mut self, y_coord: u8) -> Result<()> {
        let Some(bg_id) = self.bg_id else {
            bail!("Background texture not created");
        };

        let Some(bg) = self.textures.get_mut(bg_id.id() - 1) else {
            bail!("Invalid background texture id");
        };

        let y_line = Rect::new(0, y_coord.into(), 160, 1);

        bg.with_lock(y_line, |data, _| {
            let tile_map_start_addr = if self.lcd_control.get(LcdControl::BGTileMap) {
                0x1C00
            } else {
                0x1800
            };
            //let starting_tile_map = 0x1800;
            let offset_y_pixels = y_coord.wrapping_add(self.scroll_y) as usize;
            let tile_map_y_offset = 256 / 8 * (offset_y_pixels / 8);
            let tile_map_y_start = tile_map_start_addr + tile_map_y_offset;

            let tile_data_bit = self.lcd_control.get(LcdControl::BGWindowTileData);

            let mut i = 0;
            while i < 160 {
                let (start, end) = match i {
                    0 => (self.scroll_x as usize % 8, 8),
                    // Ending 7 pixels remaining
                    153..160 => (0, self.scroll_x as usize % 8),
                    160.. => unreachable!(),
                    _ => (0, 8),
                };

                let offset_x_pixels = (i + self.scroll_x as usize) % 256;
                let tile_map = tile_map_y_start + offset_x_pixels/8;
                let tile_data_idx = self.vram[tile_map];

                let tile_data_addr = match (tile_data_idx, tile_data_bit) {
                    (0..128, false) => 0x1000 + tile_data_idx as usize * 16,
                    (_, _) => tile_data_idx as usize * 16,
                };

                let tile_data_offset_y = offset_y_pixels % 8;
                let b1 = self.vram[tile_data_addr + tile_data_offset_y * 2];
                let b2 = self.vram[tile_data_addr + tile_data_offset_y * 2 + 1];

                data[i+start..i+end].copy_from_slice(&to_8bit_indexed_2byte(b1, b2)[start..end]);
                i += end-start;
            }
        })?;
        Ok(())
    }

    pub fn display_debugger(&mut self, ui: &Ui) {
        ui.window("Graphics")
            .size([400., 500.], imgui::Condition::FirstUseEver)
            .position([850., 250.], imgui::Condition::FirstUseEver)
            .build(|| {
                if let Some(_t) = ui.tab_bar("graphics") {
                    if let Some(_r) = ui.tab_item("Tile Data") {

                        let offset = self.debug.page * 64;
                        for i in 0..64 {
                            let texture_id = TextureId::new(offset + i + 1);
                            Image::new(texture_id, [32., 32.]).build(ui);
                            if i % 8 != 7 {
                                ui.same_line();
                            }
                        }

                        if ui.button("< Prev") {
                            self.debug.page = self.debug.page.saturating_sub(1);
                        }
                        ui.same_line();
                        ui.text(format!(" Page {} ", self.debug.page + 1));
                        ui.same_line();
                        if ui.button("Next >") {
                            self.debug.page += 1;
                            // 64 * 6 pages exist, 2 pages for each tile data block
                            if self.debug.page > 5 {
                                self.debug.page = 5;
                            }
                        }

                        if ui.button("Tile block 0") {
                            self.debug.page = 0;
                        }
                        ui.same_line();
                        if ui.button("Tile block 1") {
                            self.debug.page = 2;
                        }
                        ui.same_line();
                        if ui.button("Tile block 2") {
                            self.debug.page = 4;
                        }

                        let mut colors = self.debug.palette_colors.map(|color| {
                            [
                                color.r as f32 / 255.,
                                color.g as f32 / 255.,
                                color.b as f32 / 255.,
                            ]
                        });

                        ui.new_line();
                        for (i, c) in colors.iter_mut().enumerate() {
                            ui.color_edit3(format!("Palette color {}", i + 1), c);
                        }

                        self.debug.palette_colors = colors.map(|color| {
                            sdl3::pixels::Color::RGBA(
                                (color[0] * 255.) as u8,
                                (color[1] * 255.) as u8,
                                (color[2] * 255.) as u8,
                                255,
                            )
                        });

                        if ui.button("Reset") {
                            self.debug.palette_colors = DEFAULT_COLORS;
                        }
                        ui.same_line();
                        if ui.button("Set Palette") {
                            let palette = Palette::with_colors(&self.debug.palette_colors).unwrap();
                            for t in &self.textures {
                                unsafe {
                                    sdl3_sys::render::SDL_SetTexturePalette(t.raw(), palette.raw());
                                }
                            }
                        }
                    }
                    if let Some(_r) = ui.tab_item("Background") {
                        Image::new(self.bg_id.unwrap(), [160., 144.]).build(ui);

                    }
                }
            });
    }
}

/// Convert indexed 2 bit msb to indexed 8 bit
/// joining byte 0 and byte 1 to maked 8 bit index for sdl
fn to_8bit_indexed(bytes: &[u8]) -> [u8; 64] {
    let mut ans = [0; 64];
    for i in 0..8 {
        let (b1, b2) = (bytes[2 * i], bytes[2 * i + 1]);
        ans[i * 8..i * 8 + 8].copy_from_slice(&to_8bit_indexed_2byte(b1, b2));
    }
    ans
}

fn to_8bit_indexed_2byte(b1: u8, b2: u8) -> [u8; 8] {
    let mut ans = [0; 8];
    for j in (0..8).rev() {
        let b = (((b2 >> j) & 1) << 1) | ((b1 >> j) & 1);
        ans[7 - j] = b;
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
