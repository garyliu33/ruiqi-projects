use std::collections::HashSet;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use crate::piece_color::PieceColor;

#[derive(Serialize, Deserialize)]
pub enum ServerMessage {
    GameState(ClientGameState),
    GameOver(ClientGameState, String)
}

#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct ClientGameState {
    #[serde_as(as = "[_; 121]")]
    pub cells: [Option<PieceColor>; 121],
    pub clickable_cells: HashSet<usize>,
    pub selected_piece: Option<usize>,
    pub previous_move_path: Option<Vec<usize>>,
    pub is_your_turn: bool
}

impl ClientGameState {
    pub fn new(cells: [Option<PieceColor>; 121], clickable_cells: HashSet<usize>, selected_piece: Option<usize>, previous_move_path: Option<Vec<usize>>, is_your_turn: bool) -> Self {
        Self { cells, clickable_cells, selected_piece, previous_move_path, is_your_turn }
    }
}