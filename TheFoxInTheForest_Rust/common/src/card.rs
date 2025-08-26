use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize, Deserialize)]
pub enum Suit {
    Bells, Moons, Keys
}

impl Suit {
    pub fn all_suits() -> Vec<Suit> {
        vec![Suit::Bells, Suit::Moons, Suit::Keys]
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize, Deserialize)]
pub enum Rank {
    One, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Eleven
}

impl Rank {
    pub fn all_ranks() -> Vec<Rank> {
        vec![Rank::One, Rank::Two, Rank::Three, Rank::Four, Rank::Five, Rank::Six, Rank::Seven, Rank::Eight, Rank::Nine, Rank::Ten, Rank::Eleven]
    }

    pub fn value(&self) -> usize {
        match self {
            Rank::One => 1,
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten => 10,
            Rank::Eleven => 11
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Card {
    rank: Rank,
    suit: Suit,
}

impl Card {
    pub fn new(rank: Rank, suit: Suit) -> Self { Card { rank, suit } }

    pub fn rank(&self) -> Rank {
        self.rank
    }

    pub fn suit(&self) -> Suit {
        self.suit
    }

    pub fn to_string(&self) -> String {
        format!("{}{}", match self.rank {
            Rank::One => "1",
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "T",
            Rank::Eleven => "E"
        }, match self.suit {
            Suit::Bells => "B",
            Suit::Moons => "M",
            Suit::Keys => "K"
        })
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.suit.cmp(&other.suit)
            .then(self.rank.cmp(&other.rank))
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}