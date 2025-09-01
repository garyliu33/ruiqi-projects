extern crate core;

mod board;
mod game_controller;
mod board_view;
mod cell_view;
mod piece_color;
mod display_constants;

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
    let mut game_controller = GameController::new(6);
    loop {
        if is_mouse_button_pressed(MouseButton::Left) {
            game_controller.handle_click();
        }

        clear_background(WHITE);
        game_controller.display_board();
        
        next_frame().await;
    }
}