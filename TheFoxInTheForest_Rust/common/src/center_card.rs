use crate::card::Card;

pub struct CenterCard {
    card: Option<Card>
}

impl CenterCard {
    pub fn new() -> Self {
        CenterCard{ card: None }
    }

    pub fn remove_card(&mut self) -> Card {
        let card = self.card.unwrap();
        self.card = None;
        card
    }
    
    pub fn get_card(&self) -> Option<Card> {
        self.card
    }

    pub fn set_card(&mut self, card: Card) {
        self.card = Some(card);
    }
}