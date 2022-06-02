use eframe::egui;

pub struct Renderer {
    rendered_image: egui::ColorImage,
    rendering_image: egui::ColorImage,
}

impl Renderer {
    pub fn image(&self) -> egui::ColorImage {
        self.rendered_image.clone()
    }

    pub fn default_image() -> egui::ColorImage {
        let size = [
            core::display_size().x as usize,
            core::display_size().y as usize,
        ];
        egui::ColorImage::new(size, egui::Color32::BLACK)
    }
}

impl Default for Renderer {
    fn default() -> Self {
        Self {
            rendered_image: Self::default_image(),
            rendering_image: Self::default_image(),
        }
    }
}

impl core::Renderer for Renderer {
    fn render(&mut self, position: core::Vec2, color: core::Color) {
        use core::Color::*;
        self.rendering_image.pixels
            [position.y as usize * core::display_size().x as usize + position.x as usize] =
            match color {
                White => egui::Color32::from_rgb(134, 163, 90),
                LightGray => egui::Color32::from_rgb(111, 137, 79),
                DarkGray => egui::Color32::from_rgb(88, 117, 79),
                Black => egui::Color32::from_rgb(50, 84, 79),
            };
        if position.x == core::display_size().x - 1 && position.y == core::display_size().y - 1 {
            std::mem::swap(&mut self.rendered_image, &mut self.rendering_image);
        }
    }
}
