use serde::{Deserialize, Serialize};
use crate::card::Card;

#[derive(Serialize, Deserialize)]
pub struct ClientMove {
    pub card: Card
}

impl ClientMove {
    pub fn new(card: Card) -> Self {
        Self { card }
    }
}