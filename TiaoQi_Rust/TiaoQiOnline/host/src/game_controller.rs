use std::collections::HashSet;
use std::thread;
use std::time::Duration;
use common::piece_color::PieceColor;
use common::server_message::ClientGameState;
use crate::board::Board;
use crate::network::Network;

pub struct GameController {
    board: Board,
    ids: Vec<usize>,
    current_turn: usize,
    selected_piece: Option<usize>,
    previous_move: Option<[usize; 2]>,
    network: Network
}

impl GameController {
    pub fn new(num_players: usize, network: Network) -> Self {
        let mut board = Board::new();

        let ids = match num_players {
            2 => vec![0, 3],
            3 => vec![0, 2, 4],
            4 => vec![0, 1, 3, 4],
            6 => vec![0, 1, 2, 3, 4, 5],
            _ => unreachable!()
        };
        board.setup(ids.clone());
        Self { board, ids, current_turn: 0, selected_piece: None, previous_move: None, network }
    }

    pub fn start_game(&mut self) {
        let mut sent_disconnect_message = false;

        loop {
            self.network.accept_new_players();
            self.send_game_state();

            let id = self.current_turn;
            if self.network.player_streams[id].is_some() {
                sent_disconnect_message = false;
                if let Some(client_move) = self.network.wait_for_move(self.current_turn) {
                    self.handle_click(client_move.cell);
                }
            } else {
                if !sent_disconnect_message {
                    println!("Player {} is disconnected. Waiting for reconnect...", id);
                    sent_disconnect_message = true;
                }
            }

            thread::sleep(Duration::from_millis(50));
        }
    }

    fn handle_click(&mut self, clicked: usize) {
        match self.selected_piece {
            Some(selected) => {
                if clicked == selected {
                    self.selected_piece = None;
                    return;
                }

                if self.board.get_possible_moves(selected).contains(&clicked) {
                    self.board.move_piece(selected, clicked);
                    self.previous_move = Some([clicked, selected]);
                    self.current_turn = (self.current_turn + 1) % self.ids.len();
                    self.selected_piece = None;
                } else if self.board.cells[clicked].color == Some(self.get_current_color()) {
                    self.selected_piece = Some(clicked);
                } else {
                    self.selected_piece = None;
                }
            }
            None => {
                if self.board.cells[clicked].color == Some(self.get_current_color()) {
                    self.selected_piece = Some(clicked);
                }
            }
        }
    }

    fn get_clickable_cells(&self) -> HashSet<usize> {
        let mut result = HashSet::new();

        for i in 0..121 {
            if self.board.cells[i].color == Some(self.get_current_color()) {
                result.insert(i);
            }
        }

        if let Some(i) = self.selected_piece {
            for j in self.board.get_possible_moves(i) {
                result.insert(j);
            }
        }

        result
    }

    fn get_current_color(&self) -> PieceColor {
        PieceColor::get_color(self.ids[self.current_turn])
    }

    fn send_game_state(&mut self) {
        if let Some(winner) = self.board.get_winner() {
            let w = self.ids.iter().position(|&num| num == winner).unwrap();
            for i in 0..self.ids.len() {
                self.network.send_win_message(self.create_client_game_state(i), w, i);
            }
        } else {
            for i in 0..self.ids.len() {
                self.network.send_game_state(self.create_client_game_state(i), i);
            }
        }
    }
    
    fn create_client_game_state(&self, id: usize) -> ClientGameState {
        let mut cells = [None; 121];
        for (i, cell) in self.board.cells.iter().enumerate() {
            cells[i] = cell.color;
        }

        let previous_move_path = match self.previous_move {
            Some(m) => Some(self.board.find_path(m)),
            None => None
        };
        
        ClientGameState::new(cells, self.get_clickable_cells(), self.selected_piece, previous_move_path, self.current_turn == id, self.ids[id] as f32 * 60.0)
    }
}