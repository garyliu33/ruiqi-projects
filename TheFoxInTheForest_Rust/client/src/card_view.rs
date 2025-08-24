use macroquad::prelude::*;
use common::card::Card;
use crate::display_constants::*;

pub struct CardView {
    pub(crate) card: Card,
    x: f32,
    y: f32,
    pub selected: bool,
    hoverable: bool,
    playable: bool
}

impl CardView {
    pub fn new(card: Card, x: f32, y: f32, hoverable: bool) -> Self {
        Self { card, x, y, selected: false, hoverable }
    }

    pub fn draw(&self) {
        let x = self.x - card_width() / 2.0;
        
        let mut y = self.y - card_height() / 2.0;
        if self.selected {
            y -= pop_offset();
        }
        draw_rectangle(x, y, card_width(), card_height(), WHITE);

        if self.hoverable && self.is_hovered() {
            draw_rectangle_lines(x, y, card_width(), card_height(), 2.0, GREEN);
        } else {
            draw_rectangle_lines(x, y, card_width(), card_height(), 2.0, BLACK);
        }
        draw_text(&self.card.to_string(), x + 5.0, y + 20.0, 20.0, BLACK);
    }

    pub fn is_hovered(&self) -> bool {
        let (mx, my) = mouse_position();
        mx >= self.x - card_width() / 2.0
            && mx <= self.x + card_width() / 2.0
            && my >= self.y - card_height() / 2.0
            && my <= self.y + card_height() / 2.0
    }
}