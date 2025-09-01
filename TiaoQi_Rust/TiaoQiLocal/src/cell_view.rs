use macroquad::color::{BLACK, BLUE, GREEN, LIME, ORANGE, PURPLE, RED, WHITE, YELLOW};
use macroquad::input::mouse_position;
use macroquad::shapes::{draw_circle, draw_circle_lines};
use crate::board::PieceColor;
use crate::display_constants::RADIUS;

pub struct CellView {
    index: usize,
    pub(crate) color: Option<PieceColor>,
    x: f32,
    y: f32,
    clickable: bool,
    selected: bool
}

impl CellView {
    pub fn new(index: usize, color: Option<PieceColor>, x: f32, y: f32, clickable: bool) -> Self {
        Self { index, color, x, y, clickable, selected: false }
    }
    
    pub fn draw(&self) {
        if self.is_hovered() {
            draw_circle_lines(self.x, self.y, RADIUS + 1.5, 3.0, LIME);
        }
        draw_circle_lines(self.x, self.y, RADIUS, 2.0, BLACK);

        if let Some(color) = &self.color {
            if self.selected {
                draw_circle(self.x, self.y, RADIUS, ORANGE)
            } else {
                draw_circle(self.x, self.y, RADIUS, color.get_display_color());
            }
        }
    }
    
    pub fn set_color(&mut self, color: Option<PieceColor>) {
        self.color = color;
    }

    pub fn is_hovered(&self) -> bool {
        let (mx, my) = mouse_position();
        self.clickable && (mx - self.x).powf(2.0) + (my - self.y).powf(2.0) <= RADIUS.powf(2.0)
    }
    
    pub fn set_clickable(&mut self, clickable: bool) {
        self.clickable = clickable;
    }

    pub fn set_selected(&mut self, select: bool) {
        if self.color.is_none() && select {
            panic!("Attempted to select an empty cell");
        }

        self.selected = select;
    }

    pub fn index(&self) -> usize {
        self.index
    }
}