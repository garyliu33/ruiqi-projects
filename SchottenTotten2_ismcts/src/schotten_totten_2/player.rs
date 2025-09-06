use std::fmt;

use crate::schotten_totten_2::{card::Card, types::Role};

#[derive(Debug, Clone)]
pub struct Player {
    pub hand: Vec<Card>,
    pub role: Role,
    pub oil_cauldrons: u8,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}: {} cards in hand", self.role, self.hand.len())
    }
}
