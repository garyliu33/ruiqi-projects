use serde::{Deserialize, Serialize};
use crate::card::Card;

#[derive(Serialize, Deserialize, Clone)]
pub enum ServerMessage {
    GameState(ClientGameState),
    GameOver(ClientGameState, String)
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ClientGameState {
    pub your_card: Option<Card>,
    pub opponent_card: Option<Card>,
    pub center_card: Option<Card>,
    pub your_hand: Vec<Card>,
    pub your_playable_cards: Vec<Card>,
    pub opponent_hand_size: usize,
    pub your_tricks: usize,
    pub opponent_tricks: usize,
    pub your_points: usize,
    pub opponent_points: usize,
    pub is_your_turn: bool
}