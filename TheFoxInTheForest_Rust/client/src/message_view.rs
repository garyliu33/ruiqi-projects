use macroquad::prelude::*;

pub struct MessageView {
    str: String,
    x: f32,
    y: f32
}

impl MessageView {
    pub fn new(str: String, x: f32, y: f32) -> Self {
        Self {str, x, y}
    }

    pub fn draw(&self) {
        let font_size = 120.0;
        let text_dims = measure_text(&self.str, None, font_size as u16, 1.0);
        let text_x = self.x - text_dims.width / 2.0;
        let text_y = self.y + text_dims.height / 2.0;
        draw_text(&self.str, text_x, text_y, font_size, BLACK);
    }
}