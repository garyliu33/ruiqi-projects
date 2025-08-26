use macroquad::prelude::*;
use common::card::Card;
use crate::card_view::CardView;
use crate::display_constants::*;

pub struct HandView {
    pub cards: Vec<CardView>,
    pub x: f32,
    pub y: f32,
    pub is_turn: bool
}

impl HandView {
    pub fn new(cards: &Vec<Card>, playable_cards: &Vec<Card>, x: f32, y: f32, is_turn: bool) -> Self {
        let mut views = Vec::new();
        let num_cards = cards.len();
        for (i, card) in cards.into_iter().enumerate() {
            views.push(CardView::new(
                *card,
                x + (2 * i as i32 - num_cards as i32 + 1) as f32 * card_width() / 2.0,
                y,
                is_turn && playable_cards.contains(card)
            ));
        }
        Self { cards: views, x, y, is_turn }
    }

    pub fn update(&mut self) {
        for i in 0..self.cards.len() {
            if self.cards[i].is_hovered() && self.cards[i].is_playable() {
                for j in 0..self.cards.len() {
                    if j == i {
                        let selected = self.cards[j].is_selected();
                        self.cards[j].set_selected(!selected);
                    } else {
                        self.cards[j].set_selected(false);
                    }
                }
            }
        }
    }

    pub fn draw(&self) {
        if self.is_turn {
            self.draw_turn_indicator();
        }

        for card in &self.cards {
            card.draw();
        }
    }

    fn draw_turn_indicator(&self) {
        if !self.cards.is_empty() {
            let total_width = self.cards.len() as f32 * card_width();
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

    pub fn get_selected_card(&self) -> Option<Card> {
        self.cards
            .iter()
            .find(|c| c.is_selected())
            .map(|c| c.card.clone())
    }
}
