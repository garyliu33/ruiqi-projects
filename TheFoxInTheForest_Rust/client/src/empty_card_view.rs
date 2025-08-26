use macroquad::color::{BLACK, GREEN};
use macroquad::input::mouse_position;
use macroquad::prelude::draw_rectangle_lines;
use crate::display_constants::{card_height, card_width, center_view_gap, empty_card_padding};

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
            match self.is_hovered() {
                true => GREEN,
                false => BLACK
            }
        )
    }
    
    pub fn is_hovered(&self) -> bool {
        let (mx, my) = mouse_position();
        self.hoverable && mx > self.x - card_width() / 2.0 - empty_card_padding() && mx < self.x + card_width() / 2.0 + empty_card_padding() && my > self.y - card_height() / 2.0 - empty_card_padding() && my < self.y + card_height() / 2.0 + empty_card_padding()
    }
}