use std::io;
use std::io::Write;
use crate::card::*;

pub struct Player {
    hand: Vec<Card>,
    tricks: usize,
    score: usize
}

impl Player {
    pub fn new() -> Self {
        Player {
            hand: Vec::new(),
            tricks: 0,
            score: 0
        }
    }

    pub fn draw_card(&mut self, card: Card) {
        self.hand.push(card);
        self.sort_hand();
    }

    fn sort_hand(&mut self) {
        self.hand.sort();
    }

    pub fn display_hand(&self) {
        for card in &self.hand {
            print!("{} ", card.to_string());
        }
        io::stdout().flush().unwrap();
    }

    pub fn remove_card(&mut self, card: &Card) {
        if let Some(pos) = self.hand.iter().position(|x| x == card) {
            self.hand.remove(pos);
        }
    }

    pub fn get_score(&self) -> usize {
        self.score
    }

    pub fn add_points(&mut self, points: usize) {
        self.score += points;
    }

    pub fn num_tricks_won(&self) -> usize {
        self.tricks
    }
    
    pub fn win_trick(&mut self) {
        self.tricks += 1;
    }
    
    pub fn highest_card(&self, suit: Suit) -> Option<&Card> {
        self.hand.iter().rev().find(|card| card.suit() == suit)
    }
    
    pub fn get_hand(&self) -> Vec<Card> {
        self.hand.clone()
    }
    
    pub fn reset_tricks(&mut self) {
        self.tricks = 0;
    }
}