use macroquad::input::mouse_position;
use common::card::Card;
use crate::card_view::CardView;
use crate::display_constants::{card_height, card_width, empty_card_padding};
use crate::empty_card_view::EmptyCardView;

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
        EmptyCardView::new(self.x, self.y, self.hoverable).draw();
        if let Some(card) = self.card {
            CardView::new(card, self.x, self.y, false).draw();
        }
    }
    
    pub fn is_hovered(&self) -> bool {
        let (mx, my) = mouse_position();
        self.hoverable && mx > self.x - card_width() / 2.0 - empty_card_padding() && mx < self.x + card_width() / 2.0 + empty_card_padding() && my > self.y - card_height() / 2.0 - empty_card_padding() && my < self.y + card_height() / 2.0 + empty_card_padding()
    }
}