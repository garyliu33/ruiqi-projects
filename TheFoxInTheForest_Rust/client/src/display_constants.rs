use macroquad::prelude::*;

pub const WINDOW_WIDTH: i32 = 1280;
pub const WINDOW_HEIGHT: i32 = 720;

const BASE_CARD_WIDTH: f32 = WINDOW_WIDTH as f32 / 24.0;
const BASE_CARD_HEIGHT: f32 = WINDOW_HEIGHT as f32 / 10.0;
const BASE_CENTER_VIEW_GAP: f32 = BASE_CARD_WIDTH / 2.5;
const BASE_POP_OFFSET: f32 = BASE_CARD_HEIGHT / 5.0;
const BASE_EMPTY_CARD_PADDING: f32 = 10.0;

pub const TURN_INDICATOR_COLOR: Color = Color::from_rgba(240, 221, 81, 120);

pub fn scale() -> f32 {
    let scale_x = screen_width() / WINDOW_WIDTH as f32;
    let scale_y = screen_height() / WINDOW_HEIGHT as f32;
    scale_x.min(scale_y)
}

pub fn card_width() -> f32 { BASE_CARD_WIDTH * scale() }
pub fn card_height() -> f32 { BASE_CARD_HEIGHT * scale() }
pub fn center_view_gap() -> f32 { BASE_CENTER_VIEW_GAP * scale() }
pub fn pop_offset() -> f32 { BASE_POP_OFFSET * scale() }
pub fn empty_card_padding() -> f32 { BASE_EMPTY_CARD_PADDING * scale() }