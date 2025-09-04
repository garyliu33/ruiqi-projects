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

enum AppState {
    ChooseNumPlayers,
    InGame {
        game_controller: GameController
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut state = AppState::ChooseNumPlayers;
    loop {
        clear_background(BLACK);

        match &mut state {
            AppState::ChooseNumPlayers => {
                let prompt_text = "Select Number of Players";
                let font_size = 60.0;
                let text_dims = measure_text(prompt_text, None, font_size as u16, 1.0);
                draw_text(
                    prompt_text,
                    screen_width() / 2.0 - text_dims.width / 2.0,
                    screen_height() / 2.0 - 150.0,
                    60.0,
                    WHITE,
                );

                let button_width = 120.0;
                let button_height = 120.0;
                let spacing = 30.0;
                let num_buttons = VALID_PLAYER_NUMS.len() as f32;
                let total_width = num_buttons * button_width + (num_buttons - 1.0) * spacing;
                let start_x = (screen_width() - total_width) / 2.0;
                let start_y = screen_height() / 2.0;

                let pos = mouse_position();
                let mouse_pos = vec2(pos.0, pos.1);

                for (i, &num) in VALID_PLAYER_NUMS.iter().enumerate() {
                    let btn_rect = Rect::new(
                        start_x + i as f32 * (button_width + spacing),
                        start_y,
                        button_width,
                        button_height,
                    );

                    let btn_color = if btn_rect.contains(mouse_pos) {
                        GRAY
                    } else {
                        DARKGRAY
                    };
                    draw_rectangle(btn_rect.x, btn_rect.y, btn_rect.w, btn_rect.h, btn_color);

                    let num_text = num.to_string();
                    let num_text_dims = measure_text(&num_text, None, font_size as u16, 1.0);
                    draw_text(
                        &num_text,
                        btn_rect.x + (btn_rect.w - num_text_dims.width) / 2.0,
                        btn_rect.y + (btn_rect.h + num_text_dims.height) / 2.0 - 10.0,
                        font_size,
                        WHITE,
                    );

                    if is_mouse_button_pressed(MouseButton::Left) {
                        if btn_rect.contains(mouse_pos) {
                            state = AppState::InGame {
                                game_controller: GameController::new(num),
                            };
                            break;
                        }
                    }
                }
            }
            AppState::InGame { game_controller } => {
                game_controller.display_board();

                if let Some(winner) = game_controller.get_winner() {
                    game_controller.display_winner(winner);
                } else {
                    if is_mouse_button_pressed(MouseButton::Left) {
                        game_controller.handle_click();
                    }
                }
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