use std::thread;
use std::time::Duration;
use crate::game_controller::GameController;
use crate::network::Network;

mod game_controller;
mod network;
pub mod board;
pub mod cell;

fn main() {
    let mut network = Network::new().expect("Failed to start server");
    println!("\nWaiting for players... 0 players connected.");

    while !network.game_started {
        if network.maintain_lobby() {
            let player_count = network.get_connected_player_count();
            match player_count {
                2 | 3 | 4 | 6 => {
                    println!("Starting game with {} players...", player_count);
                    network.game_started = true;
                    break;
                }
                _ => {
                    println!("Cannot start game. Invalid number of players: {}. Required: 2, 3, 4, or 6.", player_count);
                    network.send_info(0, true);
                }
            }
        }
        
        thread::sleep(Duration::from_millis(100));
    }
    
    let mut game_controller = GameController::new(network.get_connected_player_count(), network);
    game_controller.start_game();
}
