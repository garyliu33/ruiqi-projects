use macroquad::prelude::*;
use common::card::Card;
use crate::display_constants::*;

pub struct CardView {
    pub card: Card,
    x: f32,
    y: f32,
    selected: bool,
    playable: bool
}

impl CardView {
    pub fn new(card: Card, x: f32, y: f32, playable: bool) -> Self {
        Self { card, x, y, selected: false, playable }
    }

    pub fn draw(&self) {
        let x = self.x - card_width() / 2.0;
        let mut y = self.y - card_height() / 2.0;
        if self.selected {
            y -= pop_offset();
        }
        draw_rectangle(x, y, card_width(), card_height(), WHITE);

        if self.playable {
            if self.is_hovered() {
                draw_rectangle_lines(x, y, card_width(), card_height(), 2.0, GREEN);
            } else {
                draw_rectangle_lines(x, y, card_width(), card_height(), 2.0, ORANGE);
            }
        } else {
            draw_rectangle_lines(x, y, card_width(), card_height(), 2.0, BLACK);
        }

        draw_text(&self.card.to_string(), x + 5.0, y + 20.0, 30.0, BLACK);
    }

    pub fn is_hovered(&self) -> bool {
        let (mx, my) = mouse_position();
        mx >= self.x - card_width() / 2.0
            && mx <= self.x + card_width() / 2.0
            && my >= self.y - card_height() / 2.0
            && my <= self.y + card_height() / 2.0
    }

    pub fn is_playable(&self) -> bool {
        self.playable
    }

    pub fn is_selected(&self) -> bool {
        self.selected
    }

    pub fn set_selected(&mut self, selected: bool) {
        if self.playable {
            self.selected = selected;
        } else if selected {
            panic!("Attempted to select unplayable card");
        } else {
            self.selected = false;
        }
    }
}