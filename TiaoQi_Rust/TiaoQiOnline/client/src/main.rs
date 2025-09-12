use std::io;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::sync::RwLock;
use macroquad::prelude::*;
use common::server_message::ServerMessage;
use crate::board_view::BoardView;
use crate::display_assets::*;

mod display_assets;
mod cell_view;
mod board_view;

fn window_conf() -> Conf {
    Conf {
        window_title: "Chinese Checkers".to_owned(),
        window_width: 1000,
        window_height: 1000,
        window_resizable: true,
        fullscreen: false,
        ..Default::default()
    }
}

enum AppState {
    EnterIp {
        ip_string: String,
        error_msg: Option<String>,
    },
    Connecting {
        ip: String,
    },
    InGame {
        stream: TcpStream,
        reader: BufReader<TcpStream>,
        board: Option<BoardView>,
    },
    GameOver {
        board: BoardView,
        msg: String
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let initial_constants = DisplayConstants::new(screen_width(), screen_height());
    DISPLAY_CONSTANTS.set(RwLock::new(initial_constants)).unwrap();

    let mut state = AppState::EnterIp {
        ip_string: String::new(),
        error_msg: None,
    };

    loop {
        let mut update_constants = false;
        let constants_screen_width = DISPLAY_CONSTANTS.get().unwrap().read().unwrap().screen_width;
        let constants_screen_height = DISPLAY_CONSTANTS.get().unwrap().read().unwrap().screen_height;
        if (screen_width() - constants_screen_width).abs() > 0.01 || (screen_height() - constants_screen_height).abs() > 0.01 {
            let mut writer = DISPLAY_CONSTANTS.get().unwrap().write().unwrap();
            *writer = DisplayConstants::new(screen_width(), screen_height());
            update_constants = true;
        }

        clear_background(BLACK);

        match &mut state {
            AppState::EnterIp { ip_string, error_msg } => {
                while let Some(c) = get_char_pressed() {
                    if c.is_ascii_graphic() || c == '.' {
                        ip_string.push(c);
                    }
                }
                if is_key_pressed(KeyCode::Backspace) {
                    ip_string.pop();
                }

                let font_size = 30.0;
                let text_prompt = "Enter Host IP:";
                let prompt_dims = measure_text(text_prompt, None, font_size as u16, 1.0);
                draw_text(text_prompt, screen_width() / 2.0 - prompt_dims.width / 2.0, screen_height() / 2.0 - 50.0, font_size, WHITE);

                let box_width = 300.0;
                let box_height = 40.0;
                let box_x = screen_width() / 2.0 - box_width / 2.0;
                let box_y = screen_height() / 2.0 - box_height / 2.0;
                draw_rectangle_lines(box_x, box_y, box_width, box_height, 2.0, GRAY);
                draw_text(ip_string.as_str(), box_x + 10.0, box_y + box_height - 10.0, font_size, WHITE);

                let btn_width = 150.0;
                let btn_height = 50.0;
                let btn_x = screen_width() / 2.0 - btn_width / 2.0;
                let btn_y = screen_height() / 2.0 + 50.0;
                let btn_rect = Rect::new(btn_x, btn_y, btn_width, btn_height);

                let (mouse_x, mouse_y) = mouse_position();
                let mouse_over_button = btn_rect.contains(vec2(mouse_x, mouse_y));
                let btn_color = if mouse_over_button { GREEN } else { LIME };

                draw_rectangle(btn_x, btn_y, btn_width, btn_height, btn_color);
                draw_text("Connect", btn_x + 25.0, btn_y + 35.0, font_size, WHITE);

                if let Some(msg) = error_msg {
                    let err_dims = measure_text(msg, None, 20, 1.0);
                    draw_text(msg, screen_width() / 2.0 - err_dims.width / 2.0, btn_y + 80.0, 20.0, RED);
                }

                if is_key_pressed(KeyCode::Enter) || (mouse_over_button && is_mouse_button_pressed(MouseButton::Left)) {
                    let ip_to_connect = if ip_string.trim().is_empty() {
                        "localhost".to_string()
                    } else {
                        ip_string.clone()
                    };
                    state = AppState::Connecting { ip: ip_to_connect };
                }
            }
            AppState::Connecting { ip } => {
                draw_text(&format!("Connecting to {}...", ip.trim()), 20.0, 20.0, 30.0, WHITE);

                match TcpStream::connect(format!("{}:12345", ip.trim())) {
                    Ok(stream) => {
                        stream.set_nonblocking(true).expect("Failed to set stream to non-blocking");
                        let reader = BufReader::new(stream.try_clone().unwrap());
                        state = AppState::InGame { stream, reader, board: None };
                    }
                    Err(e) => {
                        state = AppState::EnterIp {
                            ip_string: ip.clone(),
                            error_msg: Some(format!("Failed to connect: {}", e)),
                        };
                    }
                }
            }
            AppState::InGame { stream, reader, board } => {
                let mut buffer = String::new();
                match reader.read_line(&mut buffer) {
                    Ok(0) => {
                        println!("Server disconnected.");
                        state = AppState::EnterIp {
                            ip_string: String::new(),
                            error_msg: Some("Server closed the connection.".to_string()),
                        };
                        continue;
                    }
                    Ok(_) => {
                        if !buffer.trim().is_empty() {
                            let server_message: ServerMessage = serde_json::from_str(&buffer).unwrap();
                            match server_message {
                                ServerMessage::GameState(game_state) => {
                                    let current_board = board.get_or_insert_with(|| BoardView::new(game_state.rotation, game_state.ids.clone()));
                                    current_board.update_board(&game_state)
                                }
                                ServerMessage::GameOver(game_state, msg) => {
                                    let current_board = board.get_or_insert_with(|| BoardView::new(game_state.rotation, game_state.ids.clone()));
                                    current_board.update_board(&game_state);

                                    if let Some(final_board) = board.take() {
                                        state = AppState::GameOver { board: final_board, msg };
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => (),
                    Err(e) => {
                        eprintln!("Error reading from server: {e}");
                        state = AppState::EnterIp {
                            ip_string: String::new(),
                            error_msg: Some(format!("Connection error: {}", e)),
                        };
                        continue;
                    }
                }

                if let Some(board_view) = board {
                    if update_constants {
                        board_view.update_positions(&DISPLAY_CONSTANTS.get().unwrap().read().unwrap());
                    }

                    board_view.draw();
                    if is_mouse_button_pressed(MouseButton::Left) {
                        if let Some(response) = board_view.handle_click() {
                            let json = serde_json::to_string(&response).unwrap();
                            if writeln!(stream, "{}", json).is_err() {
                                state = AppState::EnterIp {
                                    ip_string: String::new(),
                                    error_msg: Some("Failed to send message. Disconnected.".to_string()),
                                };
                                continue;
                            }
                        }
                    }
                }
            }
            AppState::GameOver { board, msg } => {
                if update_constants {
                    board.update_positions(&DISPLAY_CONSTANTS.get().unwrap().read().unwrap());
                }
                board.draw_with_message(msg.to_string());
            }
        }

        next_frame().await;
    }
}
