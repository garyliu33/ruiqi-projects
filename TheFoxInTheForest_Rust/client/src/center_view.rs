use common::card::Card;
use crate::card_back_view::CardBackView;
use crate::card_view::CardView;
use crate::display_constants::*;

pub struct CenterView {
    center_card: Card,
    x: f32,
    y: f32,
}

impl CenterView {
    pub fn new(center_card: Card, x: f32, y: f32) -> Self {
        Self { center_card, x, y }
    }

    pub fn draw(&self) {
        CardBackView::new(self.x - center_view_gap() / 2.0 - card_width() / 2.0, self.y).draw();
        CardView::new(self.center_card, self.x + center_view_gap() / 2.0 + card_width() / 2.0, self.y, false).draw();
    }
}