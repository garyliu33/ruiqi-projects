use macroquad::color::BLACK;
use macroquad::prelude::draw_rectangle_lines;
use common::card::Card;
use crate::card_back_view::CardBackView;
use crate::card_view::CardView;
use crate::display_constants::*;

pub struct CenterView {
    center_card: Option<Card>,
    x: f32,
    y: f32,
}

impl CenterView {
    pub fn new(center_card: Option<Card>, x: f32, y: f32) -> Self {
        Self { center_card, x, y }
    }

    pub fn draw(&self) {
        CardBackView::new(self.x - center_view_gap() / 2.0 - card_width() / 2.0, self.y).draw();
        match self.center_card {
            Some(card) => CardView::new(card, self.x + center_view_gap() / 2.0 + card_width() / 2.0, self.y, false, false).draw(),
            None => draw_rectangle_lines(
                self.x - card_width() / 2.0 - empty_card_padding(),
                self.y - card_height() / 2.0 - empty_card_padding(),
                card_width() + 2.0 * empty_card_padding(),
                card_height() + 2.0 * empty_card_padding(),
                3.0,
                BLACK // TODO make empty card view
            )
        }
        
    }
}