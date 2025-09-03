extern crate core;

mod board;
mod game_controller;
mod board_view;
mod cell_view;
mod piece_color;
mod display_constants;

use std::io;
use std::io::Write;
use macroquad::prelude::*;
use crate::game_controller::GameController;

fn window_conf() -> Conf {
    Conf {
        window_title: "Chinese Checkers".to_owned(),
        window_width: 1000,
        window_height: 1000,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game_controller = GameController::new(get_num_players());
    loop {
        clear_background(BLACK);
        game_controller.display_board();
        
        if let Some(winner) = game_controller.get_winner() {
            game_controller.display_winner(winner);
        } else {
            if is_mouse_button_pressed(MouseButton::Left) {
                game_controller.handle_click();
            }
        }
        
        next_frame().await;
    }
}

static VALID_PLAYER_NUMS: [usize; 4] = [2, 3, 4, 6];

fn get_num_players() -> usize {
    loop {
        print!("Enter number of players: ");
        io::stdout().flush().expect("failed to flush output");
        let mut n = String::new();
        io::stdin().read_line(&mut n).expect("failed to read line");

        match n.trim().parse::<usize>() {
            Ok(num) => {
                if VALID_PLAYER_NUMS.contains(&num) {
                    return num;
                } else {
                    println!("Invalid number of players");
                }
            } Err(_) => {
                println!("Invalid input");
            }
        }
    }
}