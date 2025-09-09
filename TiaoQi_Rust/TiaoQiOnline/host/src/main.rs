use std::sync::mpsc;
use std::{io, thread};
use std::time::Duration;
use crate::game_controller::GameController;
use crate::network::Network;

mod game_controller;
mod network;
pub mod board;
pub mod cell;

fn main() {
    let mut network = Network::new().expect("Failed to start server");

    let (tx, rx) = mpsc::channel::<String>();

    // Spawn a thread to read console input without blocking the server
    thread::spawn(move || {
        loop {
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).expect("Failed to read from stdin");
            if tx.send(buffer.trim().to_lowercase()).is_err() {
                break; // Exit if main thread has closed the channel
            }
        }
    });

    let mut last_player_count = 0;
    println!("\nWaiting for players... 0 players connected.");
    println!("Type 'start' in this console and press Enter to begin the game when ready.");


    loop {
        network.accept_new_players();

        let current_player_count = network.get_connected_player_count();
        if current_player_count != last_player_count {
            println!("{} players connected.", current_player_count);
            last_player_count = current_player_count;
        }

        // Check for the "start" command without blocking.
        if let Ok(cmd) = rx.try_recv() {
            if cmd == "start" {
                let player_count = network.get_connected_player_count();
                match player_count {
                    2 | 3 | 4 | 6 => {
                        println!("'start' command received. Starting game with {} players...", player_count);
                        break; // Exit the lobby loop
                    }
                    _ => {
                        println!("Cannot start game. Invalid number of players: {}. Required: 2, 3, 4, or 6.", player_count);
                    }
                }
            }
        }

        // Sleep to prevent the loop from using 100% CPU.
        thread::sleep(Duration::from_millis(100));
    }

    network.start_game();

    // The GameController takes over to run the actual game logic.
    let mut game_controller = GameController::new(last_player_count, network);
    game_controller.start_game();
}
