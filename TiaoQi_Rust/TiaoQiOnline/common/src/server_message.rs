use std::collections::HashSet;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use crate::piece_color::PieceColor;

#[derive(Serialize, Deserialize)]
pub enum ServerMessage {
    Info(Info),
    GameState(ClientGameState),
    GameOver(ClientGameState, String)
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Info {
    pub your_id: usize,
    pub num_players: usize,
    pub msg: Option<String>
}

impl Info {
    pub fn new(your_id: usize, num_players: usize, msg: Option<String>) -> Self {
        Self { your_id, num_players, msg }
    }
}

#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct ClientGameState {
    #[serde_as(as = "[_; 121]")]
    pub cells: [Option<PieceColor>; 121],
    pub clickable_cells: HashSet<usize>,
    pub selected_piece: Option<usize>,
    pub previous_move_path: Option<Vec<usize>>,
    pub is_your_turn: bool,
    pub ids: Vec<usize>,
    pub rotation: f32
}

impl ClientGameState {
    pub fn new(cells: [Option<PieceColor>; 121], clickable_cells: HashSet<usize>, selected_piece: Option<usize>, previous_move_path: Option<Vec<usize>>, is_your_turn: bool, ids: Vec<usize>, rotation: f32) -> Self {
        Self { cells, clickable_cells, selected_piece, previous_move_path, is_your_turn, ids, rotation }
    }
}