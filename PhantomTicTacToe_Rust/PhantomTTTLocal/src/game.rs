use std::cmp::PartialEq;
use std::io::Write;
use crate::player::Player;

#[derive(Clone, Copy, PartialEq)]
pub enum CellState {
    Empty, X, O
}

pub struct Game {
    pub board: [[CellState; 3]; 3],
    pub current_player: Player,
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: [[CellState::Empty; 3]; 3],
            current_player: Player::X,
        }
    }

    pub fn display(&self) {
        for r in 0..3 {
            for c in 0..3 {
                print!(" {} ", match self.board[r][c] {
                    CellState::Empty => " ",
                    CellState::X => {
                        if self.current_player == Player::X {
                            "X"
                        } else {
                            " "
                        }
                    },
                    CellState::O => {
                        if self.current_player == Player::O {
                            "O"
                        } else {
                            " "
                        }
                    },
                });
                if c < 2 {
                    print!("|");
                }
            }
            println!();
            if r < 2 {
                println!("---+---+---");
            }
        }
    }

    pub fn display_instructions(&self) {
        println!("Enter the number corresponding to the cell to make your move.");
        for r in 0..3 {
            for c in 0..3 {
                print!(" {} ", r * 3 + c + 1);
                if c < 2 {
                    print!("|");
                }
            }
            println!();
            if r < 2 {
                println!("---+---+---");
            }
        }
    }

    pub fn display_final(&self) {
        for r in 0..3 {
            for c in 0..3 {
                print!(" {} ", match self.board[r][c] {
                    CellState::Empty => " ",
                    CellState::X => "X",
                    CellState::O => "O",
                });
                if c < 2 {
                    print!("|");
                }
            }
            println!();
            if r < 2 {
                println!("---+---+---");
            }
        }
    }

    pub fn take_turn(&mut self) {
        loop {
            print!("Player {}, enter your move (1-9): ", match self.current_player {
                Player::X => "X",
                Player::O => "O",
            });
            std::io::stdout().flush().expect("Failed to flush stdout");

            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Failed to read line");

            let move_num = input.trim().parse::<usize>();
            match move_num {
                Ok(1) => if self.play_move(0, 0) { break; },
                Ok(2) => if self.play_move(0, 1) { break; },
                Ok(3) => if self.play_move(0, 2) { break; },
                Ok(4) => if self.play_move(1, 0) { break; },
                Ok(5) => if self.play_move(1, 1) { break; },
                Ok(6) => if self.play_move(1, 2) { break; },
                Ok(7) => if self.play_move(2, 0) { break; },
                Ok(8) => if self.play_move(2, 1) { break; },
                Ok(9) => if self.play_move(2, 2) { break; },
                _ => println!("Invalid input."),
            }
        }
    }

    fn play_move(&mut self, r: usize, c: usize) -> bool {
        if self.board[r][c] == CellState::Empty {
            self.board[r][c] = match self.current_player {
                Player::X => CellState::X,
                Player::O => CellState::O,
            };
            return true;
        }

        println!("Cell already taken.");
        false
    }

    pub fn switch_player(&mut self) {
        self.current_player = self.current_player.other();
    }

    pub fn is_game_over(&self) -> bool {
        for r in 0..3 {
            if self.board[r][0] != CellState::Empty &&
               self.board[r][0] == self.board[r][1] &&
               self.board[r][0] == self.board[r][2] {
                println!("Player {} wins!", match self.board[r][0] {
                    CellState::X => "X",
                    CellState::O => "O",
                    _ => unreachable!(),
                });
                return true;
            }
        }

        for c in 0..3 {
            if self.board[0][c] != CellState::Empty &&
                self.board[0][c] == self.board[1][c] &&
                self.board[0][c] == self.board[2][c] {
                println!("Player {} wins!", match self.board[0][c] {
                    CellState::X => "X",
                    CellState::O => "O",
                    _ => unreachable!(),
                });
                return true;
            }
        }

        if self.board[0][0] != CellState::Empty &&
            self.board[0][0] == self.board[1][1] &&
            self.board[0][0] == self.board[2][2] {
            println!("Player {} wins!", match self.board[0][0] {
                CellState::X => "X",
                CellState::O => "O",
                _ => unreachable!(),
            });
            return true;
        }

        if self.board[0][2] != CellState::Empty &&
            self.board[0][2] == self.board[1][1] &&
            self.board[0][2] == self.board[2][0] {
            println!("Player {} wins!", match self.board[0][2] {
                CellState::X => "X",
                CellState::O => "O",
                _ => unreachable!(),
            });
            return true;
        }

        for r in 0..3 {
            for c in 0..3 {
                if self.board[r][c] == CellState::Empty {
                    return false;
                }
            }
        }

        println!("It's a tie!");
        true
    }
}