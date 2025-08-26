use macroquad::prelude::*;
use crate::card_back_view::CardBackView;
use crate::display_constants::*;

pub struct OpponentHandView {
    cardbacks: Vec<CardBackView>,
    x: f32,
    y: f32,
    is_turn: bool
}

impl OpponentHandView {
    pub fn new(num_cards: usize, x: f32, y: f32, is_turn: bool) -> Self {
        let mut views = Vec::new();
        for i in 0..num_cards {
            views.push(CardBackView::new(x + (2 * i as i32 - num_cards as i32 + 1) as f32 * card_width() / 2.0, y));
        }
        Self { cardbacks: views, x, y, is_turn }
    }

    pub fn draw(&self) {
        if self.is_turn {
            self.draw_turn_indicator();
        }

        for cardback in &self.cardbacks {
            cardback.draw();
        }
    }

    fn draw_turn_indicator(&self) {
        if !self.cardbacks.is_empty() {
            let total_width = self.cardbacks.len() as f32 * card_width();
            let total_height = card_height();

            draw_rectangle_lines(
                self.x - pop_offset() - total_width / 2.0,
                self.y - pop_offset() - total_height / 2.0,
                total_width + pop_offset() * 2.0,
                total_height + pop_offset() * 2.0,
                pop_offset() * 2.0,
                TURN_INDICATOR_COLOR
            )
        }
    }
}