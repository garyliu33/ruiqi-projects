use macroquad::color::{BLACK, BLUE, GREEN, ORANGE, PURPLE, RED, WHITE, YELLOW};
use macroquad::input::mouse_position;
use macroquad::shapes::{draw_circle, draw_circle_lines};
use crate::board::PieceColor;

pub struct CellView {
    color: Option<PieceColor>,
    x: f32,
    y: f32,
    clickable: bool
}

const RADIUS: f32 = 5.0;

impl CellView {
    pub fn new(color: Option<PieceColor>, x: f32, y: f32, clickable: bool) -> Self {
        Self { color, x, y, clickable }
    }
    
    pub fn draw(&self) {
        if self.clickable && self.is_hovered() {
            draw_circle_lines(self.x, self.y, RADIUS, 2.0, GREEN);
        } else {
            draw_circle_lines(self.x, self.y, RADIUS, 2.0, BLACK);
        }
        
        if let Some(color) = &self.color {
            draw_circle(self.x, self.y, RADIUS, color.get_display_color());
        }
    }

    pub fn is_hovered(&self) -> bool {
        let (mx, my) = mouse_position();
        (mx - self.x).powf(2.0) + (my - self.x).powf(2.0) <= 5.0
    }
}