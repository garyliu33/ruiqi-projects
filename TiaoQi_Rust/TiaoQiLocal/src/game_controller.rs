use crate::board::Board;

pub struct GameController {
    board: Board
}

impl GameController {
    pub fn new(num_players: usize) -> Self {
        let mut board = Board::new();
        board.setup(match num_players {
            2 => vec![0, 3],
            3 => vec![0, 2, 4],
            4 => vec![0, 1, 3, 4],
            6 => vec![0, 1, 2, 3, 4, 5],
            _ => unreachable!()
        });
        Self { board }
    }
}