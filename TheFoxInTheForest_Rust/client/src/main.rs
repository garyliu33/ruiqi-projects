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
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use common::client_game_state::ClientGameState;
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
        table: Option<TableView>,
    },
}

#[macroquad::main(window_conf)]
async fn main() {
    load_card_back_texture().await;

    let mut state = AppState::EnterIp {
        ip_string: String::new(),
        error_msg: None,
    };

    loop {
        clear_background(WHITE);

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
                draw_text(text_prompt, screen_width() / 2.0 - prompt_dims.width / 2.0, screen_height() / 2.0 - 50.0, font_size, BLACK);

                let box_width = 300.0;
                let box_height = 40.0;
                let box_x = screen_width() / 2.0 - box_width / 2.0;
                let box_y = screen_height() / 2.0 - box_height / 2.0;
                draw_rectangle_lines(box_x, box_y, box_width, box_height, 2.0, GRAY);
                draw_text(ip_string.as_str(), box_x + 10.0, box_y + box_height - 10.0, font_size, BLACK);

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
                draw_text(&format!("Connecting to {}...", ip.trim()), 20.0, 20.0, 30.0, BLACK);

                match TcpStream::connect(format!("{}:4000", ip.trim())) {
                    Ok(stream) => {
                        stream.set_nonblocking(true).expect("Failed to set stream to non-blocking");
                        let reader = io::BufReader::new(stream.try_clone().unwrap());
                        state = AppState::InGame { stream, reader, table: None };
                    }
                    Err(e) => {
                        state = AppState::EnterIp {
                            ip_string: ip.clone(),
                            error_msg: Some(format!("Failed to connect: {}", e)),
                        };
                    }
                }
            }
            AppState::InGame { stream, reader, table } => {
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
                            let game_state: ClientGameState = serde_json::from_str(&buffer).unwrap();
                            *table = Some(TableView::new(&game_state));
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

                if let Some(table_view) = table {
                    table_view.draw();
                    if let Some(response) = table_view.handle_click() {
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

        next_frame().await;
    }
}