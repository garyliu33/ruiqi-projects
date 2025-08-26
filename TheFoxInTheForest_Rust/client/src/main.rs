mod display_constants;
mod table_view;
mod center_view;
mod hand_view;
mod card_view;
mod card_back_view;
mod opponent_hand_view;
mod played_card_view;
mod player_stat_view;
mod empty_card_view;

use std::io;
use std::io::{BufRead, Write};
use std::net::TcpStream;
use common::client_game_state::ClientGameState;
use common::client_move::ClientMove;
use macroquad::prelude::*;
use display_constants::*;
use crate::card_back_view::load_card_back_texture;
use crate::table_view::TableView;

fn window_conf() -> Conf {
    Conf {
        window_title: "The Fox in the Forest".to_owned(),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    load_card_back_texture().await;

    print!("Enter Host IP: ");
    io::stdout().flush().unwrap();
    
    let mut ip = String::new();
    io::stdin().read_line(&mut ip).unwrap();
    
    let mut stream = TcpStream::connect(format!("{}:4000", ip.trim()))
        .expect("Failed to connect to host");
    stream.set_nonblocking(true).expect("Failed to set steam to non blocking");
    
    let mut reader = io::BufReader::new(stream.try_clone().unwrap());
    let mut table: Option<TableView> = None;

    loop {
        let mut buffer = String::new();
        match reader.read_line(&mut buffer) {
            Ok(0) => {
                println!("Server disconnected.");
                break;
            }
            Ok(_) => {
                if !buffer.trim().is_empty() {
                    let state: ClientGameState = serde_json::from_str(&buffer).unwrap();
                    table = Some(TableView::new(&state));
                }
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => (),
            Err(e) => {
                eprintln!("Error reading from server: {e}");
                break;
            }
        }

        clear_background(WHITE);

        if let Some(table_view) = &mut table {
            table_view.draw();
            if let Some(response) = table_view.handle_click() {
                let json = serde_json::to_string(&response).unwrap();
                writeln!(stream, "{}", json).expect("Failed to send message");
            }
        }

        next_frame().await;
    }
}