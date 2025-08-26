use macroquad::input::mouse_position;
use macroquad::prelude::*;
use crate::display_constants::*;

pub struct EmptyCardView {
    x: f32,
    y: f32,
    hoverable: bool
}

impl EmptyCardView {
    pub fn new(x: f32, y: f32, hoverable: bool) -> Self {
        Self { x, y, hoverable }
    }
    
    pub fn draw(&self) {
        draw_rectangle_lines(
            self.x - card_width() / 2.0 - empty_card_padding(),
            self.y - card_height() / 2.0 - empty_card_padding(),
            card_width() + 2.0 * empty_card_padding(),
            card_height() + 2.0 * empty_card_padding(),
            3.0,
            match self.hoverable && self.is_hovered() {
                true => GREEN,
                false => BLACK
            }
        )
    }
    
    pub fn is_hovered(&self) -> bool {
        let (mx, my) = mouse_position();
        mx > self.x - card_width() / 2.0 - empty_card_padding() && mx < self.x + card_width() / 2.0 + empty_card_padding() && my > self.y - card_height() / 2.0 - empty_card_padding() && my < self.y + card_height() / 2.0 + empty_card_padding()
    }
}