use crate::card::*;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub struct Deck {
    cards: Vec<Card>
}

impl Deck {
    pub fn new() -> Self {
        let mut v: Vec<Card> = Vec::new();
        for rank in Rank::all_ranks() {
            for suit in Suit::all_suits() {
                v.push(Card::new(rank, suit))
            }
        }
        Deck {
            cards: v
        }
    }

    pub fn pop(&mut self) -> Card {
        self.cards.pop().unwrap()
    }

    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    pub fn add_to_bottom(&mut self, card: Card) {
        self.cards.insert(0, card);
    }
}