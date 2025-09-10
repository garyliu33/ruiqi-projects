use std::collections::HashSet;
use crate::board::Board;
use crate::board_view::BoardView;
use crate::display_assets::DISPLAY_CONSTANTS;
use crate::piece_color::PieceColor;

pub struct GameController {
    board: Board,
    board_view: BoardView,
    ids: Vec<usize>,
    current_turn: usize,
    selected_piece: Option<usize>,
    previous_move: Option<[usize; 2]>
}

impl GameController {
    pub fn new(num_players: usize) -> Self {
        let mut board = Board::new();

        let ids = match num_players {
            2 => vec![0, 3],
            3 => vec![0, 2, 4],
            4 => vec![0, 1, 3, 4],
            6 => vec![0, 1, 2, 3, 4, 5],
            _ => unreachable!()
        };
        board.setup(ids.clone());
        let board_view = BoardView::new(&board, ids.clone());
        Self { board, board_view, ids, current_turn: 0, selected_piece: None, previous_move: None }
    }

    pub fn handle_click(&mut self) {
        if let Some(clicked) = self.board_view.get_hovered_cell() {
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
    }

    pub fn display_board(&mut self) {
        let previous_move_path = match self.previous_move {
            Some(previous_move) => self.board.find_path(previous_move),
            None => Vec::new()
        };

        self.board_view.update_board(&self.board, self.get_clickable_cells(), self.selected_piece, previous_move_path);
        self.board_view.draw();
    }
    
    pub fn update_cell_positions(&mut self) {
        self.board_view.update_positions(&DISPLAY_CONSTANTS.get().unwrap().read().unwrap());
    }

    pub fn get_winner(&self) -> Option<PieceColor> {
        self.board.get_winner()
    }

    pub fn display_winner(&self, winner: PieceColor) {
        self.board_view.display_winner(winner);
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
}