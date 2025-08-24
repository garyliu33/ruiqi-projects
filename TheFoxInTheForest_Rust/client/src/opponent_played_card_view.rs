use macroquad::color::{LIGHTGRAY};
use macroquad::prelude::{draw_rectangle_lines, BLACK};
use common::card::Card;
use crate::card_view::CardView;
use crate::display_constants::*;

pub struct OpponentPlayedCardView {
    card: Option<Card>,
    x: f32,
    y: f32
}

impl OpponentPlayedCardView {
    pub fn new(card: Option<Card>, x: f32, y: f32) -> Self {
        Self { card, x, y }
    }
    
    pub fn draw(&self) {
        draw_rectangle_lines(
            self.x - card_width() / 2.0 - played_card_padding(),
            self.y - card_height() / 2.0 - played_card_padding(),
            card_width() + 2.0 * played_card_padding(),
            card_height() + 2.0 * played_card_padding(),
            3.0,
            BLACK
        );

        if let Some(card) = self.card {
            CardView::new(card, self.x, self.y, false).draw()
        }
    }
}