mod control;
mod interrupt_source;
mod palette;
mod renderer;
mod vec2;

pub mod oam;
pub mod vram;

pub use palette::{Color, Palette};
pub use renderer::{NoRenderer, Renderer};
pub use vec2::Vec2;

use control::Control;
use interrupt_source::InterruptSource;
use oam::{Oam, Object};
use vram::{TileDataArea, TileMapArea, Vram};

use crate::interrupt::{Interrupt, InterruptController};

const LINES_PER_FRAME: u8 = 154;

const CYCLES_PER_LINE: u64 = 456;

const OAM_SEARCH_CYCLES: u64 = 80;

const TRANSFER_CYCLES: u64 = 172;

const WINDOW_OFFSET: u8 = 7;

pub fn display_size() -> Vec2 {
    Vec2::new(160, 144)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Mode {
    HBlank = 0b00,
    VBlank = 0b01,
    OamSearch = 0b10,
    Transfer = 0b11,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Ppu {
    vram: Vram,
    oam: Oam,
    control: Control,
    interrupt_source: InterruptSource,
    current_position: Vec2,
    y_compare: u8,
    background_palette: Palette,
    object_palette0: Palette,
    object_palette1: Palette,
    scroll_position: Vec2,
    window_position: Vec2,
    cycles_in_line: u64,
}

impl Default for Ppu {
    fn default() -> Self {
        Self {
            vram: Default::default(),
            oam: Oam::default(),
            control: Default::default(),
            interrupt_source: Default::default(),
            current_position: Vec2::new(0, 0x91),
            y_compare: 0,
            background_palette: 0xFC.into(),
            object_palette0: Default::default(),
            object_palette1: Default::default(),
            scroll_position: Default::default(),
            window_position: Default::default(),
            cycles_in_line: 0x91 * CYCLES_PER_LINE,
        }
    }
}

impl Ppu {
    fn mode(&self) -> Mode {
        use Mode::*;
        if !self.control.is_enabled() {
            HBlank
        } else if self.current_position.y >= display_size().y {
            VBlank
        } else if self.cycles_in_line < OAM_SEARCH_CYCLES {
            OamSearch
        } else if self.cycles_in_line < OAM_SEARCH_CYCLES + TRANSFER_CYCLES {
            Transfer
        } else {
            HBlank
        }
    }

    fn object_pixel_color(&self, object: &Object) -> Option<Color> {
        let position_in_object =
            object.position_in_object(self.current_position, self.control.uses_large_object());
        let tile_id = if self.control.uses_large_object() {
            if position_in_object.y >= 8 {
                object.tile_id | 0b1
            } else {
                object.tile_id & !0b1
            }
        } else {
            object.tile_id
        };
        let color_id = self
            .vram
            .tile_data(TileDataArea::Origin, tile_id)
            .color_id(position_in_object);
        if color_id == 0b00 {
            None
        } else {
            let palette = if object.palette_number == 0 {
                &self.object_palette0
            } else {
                &self.object_palette1
            };
            Some(palette.apply(color_id))
        }
    }

    fn drawn_object(&self) -> Option<(Object, Color)> {
        self.oam
            .objects_at_position(self.current_position, self.control.uses_large_object())
            .filter_map(|object| {
                self.object_pixel_color(&object)
                    .map(|color| (object, color))
            })
            .min_by_key(|(object, _)| object.position.x)
    }

    fn pixel_color_from_tile_map(&self, tile_map: TileMapArea, position: Vec2) -> Color {
        self.background_palette.apply(
            self.vram
                .tile_map(tile_map, self.control.background_tile_data_area())
                .color_id(position),
        )
    }

    fn background_pixel_color(&self) -> Color {
        self.pixel_color_from_tile_map(
            self.control.background_tile_map_area(),
            self.current_position.wrapping_add(self.scroll_position),
        )
    }

    fn is_in_window(&self) -> bool {
        self.control.window_enabled()
            && self.current_position.y >= self.window_position.y
            && self.current_position.x >= self.window_position.x.wrapping_sub(WINDOW_OFFSET)
    }

    fn window_pixel_color(&self) -> Color {
        self.pixel_color_from_tile_map(
            self.control.window_tile_map_area(),
            self.current_position
                .wrapping_sub(self.window_position)
                .wrapping_add(Vec2::new(WINDOW_OFFSET, 0)),
        )
    }

    fn render_pixel(&self, renderer: &mut impl Renderer) {
        let background_pixel_color = if self.control.background_and_window_enabled() {
            if self.is_in_window() {
                self.window_pixel_color()
            } else {
                self.background_pixel_color()
            }
        } else {
            Color::White
        };
        let color = if self.control.object_enabled() {
            self.drawn_object()
                .map_or(background_pixel_color, |(object, object_pixel)| {
                    if object.is_under_background {
                        background_pixel_color
                    } else {
                        object_pixel
                    }
                })
        } else {
            background_pixel_color
        };
        renderer.render(self.current_position, color)
    }

    fn advance_cycle(&mut self) {
        self.cycles_in_line += 1;
        if self.cycles_in_line >= CYCLES_PER_LINE {
            self.current_position.y += 1;
            self.current_position.x = 0;
            self.cycles_in_line = 0;
            if self.current_position.y >= LINES_PER_FRAME {
                self.current_position.y = 0;
            }
        }
    }

    fn request_interrupt(
        &self,
        previous_mode: Mode,
        interrupt_controller: &mut InterruptController,
    ) {
        use Mode::*;
        if previous_mode != self.mode() && self.mode() == VBlank {
            interrupt_controller.request(Interrupt::VBlank);
        }
        if self.current_position.y == self.y_compare && self.interrupt_source.ly()
            || (previous_mode != self.mode()
                && match self.mode() {
                    HBlank => self.interrupt_source.hblank(),
                    VBlank => self.interrupt_source.vblank(),
                    OamSearch => self.interrupt_source.oam(),
                    _ => false,
                })
        {
            interrupt_controller.request(Interrupt::LcdStat);
        }
    }

    pub fn tick(
        &mut self,
        interrupt_controller: &mut InterruptController,
        renderer: &mut impl Renderer,
    ) {
        if !self.control.is_enabled() {
            return;
        }
        let current_mode = self.mode();
        if current_mode == Mode::Transfer && self.current_position.x < display_size().x {
            self.render_pixel(renderer);
            self.current_position.x += 1;
        }
        self.advance_cycle();
        self.request_interrupt(current_mode, interrupt_controller);
    }

    pub fn vram(&self) -> &Vram {
        &self.vram
    }

    pub fn vram_mut(&mut self) -> &mut Vram {
        &mut self.vram
    }

    pub fn oam(&self) -> &Oam {
        &self.oam
    }

    pub fn oam_mut(&mut self) -> &mut Oam {
        &mut self.oam
    }

    pub fn control_bits(&self) -> u8 {
        self.control.bits()
    }

    pub fn set_control_bits(&mut self, value: u8) {
        self.control.set_bits(value);
        if !self.control.is_enabled() {
            self.current_position.y = 0;
        }
    }

    pub fn status_bits(&self) -> u8 {
        0b1 << 7
            | self.interrupt_source.bits() << 3
            | ((self.current_position.y == self.y_compare) as u8) << 2
            | (self.mode() as u8)
    }

    pub fn set_status_bits(&mut self, value: u8) {
        self.interrupt_source.set_bits(value >> 3);
    }

    pub fn current_y(&self) -> u8 {
        self.current_position.y
    }

    pub fn y_compare(&self) -> u8 {
        self.y_compare
    }

    pub fn background_palette(&self) -> &Palette {
        &self.background_palette
    }

    pub fn background_palette_mut(&mut self) -> &mut Palette {
        &mut self.background_palette
    }

    pub fn object_palette0(&self) -> &Palette {
        &self.object_palette0
    }

    pub fn object_palette0_mut(&mut self) -> &mut Palette {
        &mut self.object_palette0
    }

    pub fn object_palette1(&self) -> &Palette {
        &self.object_palette1
    }

    pub fn object_palette1_mut(&mut self) -> &mut Palette {
        &mut self.object_palette1
    }

    pub fn scroll_position(&self) -> Vec2 {
        self.scroll_position
    }

    pub fn scroll_position_mut(&mut self) -> &mut Vec2 {
        &mut self.scroll_position
    }

    pub fn set_y_compare(&mut self, value: u8) {
        self.y_compare = value;
    }

    pub fn window_position(&self) -> Vec2 {
        self.window_position
    }

    pub fn window_position_mut(&mut self) -> &mut Vec2 {
        &mut self.window_position
    }
}
