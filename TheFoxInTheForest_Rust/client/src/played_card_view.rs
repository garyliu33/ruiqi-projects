use macroquad::color::{BLACK, GREEN, LIGHTGRAY};
use macroquad::input::mouse_position;
use macroquad::prelude::draw_rectangle_lines;
use common::card::Card;
use common::client_move::ClientMove;
use crate::card_view::CardView;
use crate::display_constants::*;

pub struct PlayedCardView {
    card: Option<Card>,
    x: f32,
    y: f32,
    hoverable: bool
}

impl PlayedCardView {
    pub fn new(card: Option<Card>, x: f32, y: f32, hoverable: bool) -> Self {
        Self { card, x, y, hoverable }
    }

    pub fn draw(&self) {
        let color;
        if self.is_hovered() {
            color = GREEN;
        } else {
            color = BLACK;
        }
        draw_rectangle_lines(
            self.x - card_width() / 2.0 - empty_card_padding(),
            self.y - card_height() / 2.0 - empty_card_padding(),
            card_width() + 2.0 * empty_card_padding(),
            card_height() + 2.0 * empty_card_padding(),
            3.0,
            color
        );

        if let Some(card) = self.card {
            CardView::new(card, self.x, self.y, false, false).draw()
        }
    }

    pub fn is_hovered(&self) -> bool {
        let (mx, my) = mouse_position();
        self.hoverable && mx > self.x - card_width() / 2.0 - empty_card_padding() && mx < self.x + card_width() / 2.0 + empty_card_padding() && my > self.y - card_height() / 2.0 - empty_card_padding() && my < self.y + card_height() / 2.0 + empty_card_padding()
    }
}