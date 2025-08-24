use std::cmp::PartialEq;
use std::io::{BufRead, BufReader};
use std::net::TcpStream;
use common::Message;
use crate::network::send_message;
use crate::player::{Player, PlayerSymbol};

#[derive(Clone, Copy, PartialEq)]
pub enum CellState {
    Empty, X, O
}

pub enum Winner {
    X, O, Draw, None
}

pub struct Game {
    board: [[CellState; 3]; 3],
    players: Vec<Player>,
    current_player_id: usize,
}

impl Game {
    pub fn new(stream1: TcpStream, stream2: TcpStream) -> Self {
        let p1 = Player::new(PlayerSymbol::X, stream1);
        let p2 = Player::new(PlayerSymbol::O, stream2);
        Self {
            board: [[CellState::Empty; 3]; 3],
            players: vec![p1, p2],
            current_player_id: 0,
        }
    }

    pub fn start(&mut self) {
        {
            let instructions = create_instructions();
            let (player1, player2) = self.players.split_at_mut(1);
            send_message(&mut player1[0], &Message::new(format!("{}\nYou are X.", instructions).as_str(), false)).expect("Failed to send message");
            send_message(&mut player2[0], &Message::new(format!("{}\nYou are O.", instructions).as_str(), false)).expect("Failed to send message");
        }

        loop {
            self.take_turn(self.current_player_id);
            match self.get_winner() {
                Winner::X => {
                    let final_gamestate = self.create_final_gamestate();
                    let (player1, player2) = self.players.split_at_mut(1);
                    send_message(&mut player1[0], &Message::new(format!("{}\nYou win!", final_gamestate).as_str(), false)).expect("Failed to send message");
                    send_message(&mut player2[0], &Message::new(format!("{}\nYou lose!", final_gamestate).as_str(), false)).expect("Failed to send message");
                    break;
                },
                Winner::O => {
                    let final_gamestate = self.create_final_gamestate();
                    let (player1, player2) = self.players.split_at_mut(1);
                    send_message(&mut player1[0], &Message::new(format!("{}\nYou lose.", final_gamestate).as_str(), false)).expect("Failed to send message");
                    send_message(&mut player2[0], &Message::new(format!("{}\nYou win!", final_gamestate).as_str(), false)).expect("Failed to send message");
                    break;
                }
                Winner::Draw => {
                    let final_gamestate = self.create_final_gamestate();
                    let (player1, player2) = self.players.split_at_mut(1);
                    send_message(&mut player1[0], &Message::new(format!("{}\nIt's a tie.", final_gamestate).as_str(), false)).expect("Failed to send message");
                    send_message(&mut player2[0], &Message::new(format!("{}\nIt's a tie.", final_gamestate).as_str(), false)).expect("Failed to send message");
                    break;
                }
                Winner::None => ()
            }
            self.current_player_id = 1 - self.current_player_id;
        }
    }

    fn create_client_gamestate(&self, player: &Player) -> String {
        let mut s = String::new();
        let symbol = &player.symbol;
        for r in 0..3 {
            for c in 0..3 {
                s.push_str(format!(" {} ", match self.board[r][c] {
                    CellState::Empty => " ",
                    CellState::X => {
                        if symbol == &PlayerSymbol::X {
                            "X"
                        } else {
                            " "
                        }
                    },
                    CellState::O => {
                        if symbol == &PlayerSymbol::O {
                            "O"
                        } else {
                            " "
                        }
                    },
                }).as_str());
                if c < 2 {
                    s.push('|');
                }
            }
            s.push('\n');
            if r < 2 {
                s.push_str("---+---+---\n");
            }
        }
        s
    }

    fn create_final_gamestate(&self) -> String {
        let mut s = String::new();
        for r in 0..3 {
            for c in 0..3 {
                s.push_str(format!(" {} ", match self.board[r][c] {
                    CellState::Empty => " ",
                    CellState::X => "X",
                    CellState::O => "O",
                }).as_str());
                if c < 2 {
                    s.push('|');
                }
            }
            s.push('\n');
            if r < 2 {
                s.push_str("---+---+---\n");
            }
        }
        s
    }

    fn take_turn(&mut self, player_id: usize) {
        let gamestate = self.create_client_gamestate(&self.players[player_id]);
        send_message(&mut self.players[player_id], &Message::new(&gamestate, false))
            .expect("Failed to send message");
        send_message(&mut self.players[1 - player_id], &Message::new("Opponent is thinking...", false))
            .expect("Failed to send message");
        loop {
            send_message(&mut self.players[player_id], &Message::new("Enter your move (1-9): ", true))
                .expect("Failed to send message");

            let mut reader = BufReader::new(&self.players[player_id].stream);
            let mut input_json = String::new();

            match reader.read_line(&mut input_json) {
                Ok(0) => {
                    println!("Client disconnected.");
                    break;
                },
                Ok(_) => {
                    let input: Message = serde_json::from_str(&input_json).expect("Failed to parse input JSON");
                    let move_num = input.content.trim().parse::<usize>();
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
                Err(e) => {
                    println!("Error reading from player stream: {}", e);
                    return;
                }
            }
        }
    }

    fn play_move(&mut self, r: usize, c: usize) -> bool {
        if self.board[r][c] == CellState::Empty {
            self.board[r][c] = match self.current_player_id {
                0 => CellState::X,
                1 => CellState::O,
                _ => unreachable!(),
            };
            let gamestate = self.create_client_gamestate(&self.players[self.current_player_id]);
            send_message(&mut self.players[self.current_player_id], &Message::new(&gamestate, false)).expect("Failed to send message");
            return true;
        }

        send_message(&mut self.players[self.current_player_id], &Message::new("Cell already taken.", false)).expect("Failed to send message");
        false
    }

    fn get_winner(&self) -> Winner {
        for r in 0..3 {
            if self.board[r][0] != CellState::Empty &&
                self.board[r][0] == self.board[r][1] &&
                self.board[r][0] == self.board[r][2] {
                match self.board[r][0] {
                    CellState::X => return Winner::X,
                    CellState::O => return Winner::O,
                    _ => unreachable!(),
                }
            }
        }

        for c in 0..3 {
            if self.board[0][c] != CellState::Empty &&
                self.board[0][c] == self.board[1][c] &&
                self.board[0][c] == self.board[2][c] {
                match self.board[0][c] {
                    CellState::X => return Winner::X,
                    CellState::O => return Winner::O,
                    _ => unreachable!(),
                }
            }
        }

        if self.board[0][0] != CellState::Empty &&
            self.board[0][0] == self.board[1][1] &&
            self.board[0][0] == self.board[2][2] {
            match self.board[0][0] {
                CellState::X => return Winner::X,
                CellState::O => return Winner::O,
                _ => unreachable!(),
            }
        }

        if self.board[0][2] != CellState::Empty &&
            self.board[0][2] == self.board[1][1] &&
            self.board[0][2] == self.board[2][0] {
            match self.board[0][2] {
                CellState::X => return Winner::X,
                CellState::O => return Winner::O,
                _ => unreachable!(),
            }
        }

        for r in 0..3 {
            for c in 0..3 {
                if self.board[r][c] == CellState::Empty {
                    return Winner::None;
                }
            }
        }

        Winner::Draw
    }
}

fn create_instructions() -> String {
    let mut s = String::from("Enter the number corresponding to the cell to make your move.\n");
    for r in 0..3 {
        for c in 0..3 {
            s.push_str(format!(" {} ", r * 3 + c + 1).as_str());
            if c < 2 {
                s.push('|');
            }
        }
        s.push('\n');
        if r < 2 {
            s.push_str("---+---+---\n");
        }
    }
    s
}