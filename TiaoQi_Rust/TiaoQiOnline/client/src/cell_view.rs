use macroquad::prelude::*;
use common::piece_color::PieceColor;
use crate::display_assets::*;

pub struct CellView {
    index: usize,
    pub color: Option<PieceColor>,
    x: f32,
    y: f32,
    clickable: bool,
    selected: bool,
    was_previous_move: bool,
    enable_hover: bool
}

impl CellView {
    pub fn new(index: usize, color: Option<PieceColor>, x: f32, y: f32, clickable: bool) -> Self {
        Self { index, color, x, y, clickable, selected: false, was_previous_move: false, enable_hover: false }
    }

    pub fn draw(&self) {
        let constants = DISPLAY_CONSTANTS.get().unwrap().read().unwrap();
        let r = constants.radius + constants.thickness;

        match self.color {
            Some(piece_color) => {
                let mut color = piece_color.get_display_color();
                draw_circle(self.x, self.y, constants.radius, color);
                draw_circle_lines(self.x, self.y, constants.radius, constants.thickness, LIGHTGRAY);

                if self.selected {
                    color = Color::new(1.0, 1.0, 1.0, 0.9);
                    draw_mesh(&gradient_ring_mesh(self.x, self.y, r, constants.thick_highlight, color, transparent(color)));
                } else if self.is_hovered() {
                    color = Color::new(1.0, 1.0, 1.0, 0.7);
                    draw_mesh(&gradient_ring_mesh(self.x, self.y, r, constants.thin_highlight, color, transparent(color)));
                } else if self.clickable {
                    color.a = 0.7;
                    draw_mesh(&gradient_ring_mesh(self.x, self.y, r, constants.thin_highlight, color, transparent(color)));
                } else if self.was_previous_move {
                    color.a = 0.9;
                    draw_mesh(&gradient_ring_mesh(self.x, self.y, r, constants.thin_highlight, color, transparent(color)));
                }
            }
            None => {
                draw_circle_lines(self.x, self.y, constants.radius, constants.thickness, LIGHTGRAY);

                if self.is_hovered() {
                    let color = Color::new(1.0, 1.0, 1.0, 0.9);
                    draw_mesh(&gradient_ring_mesh(self.x, self.y, r, constants.thin_highlight, color, transparent(color)));
                } else if self.clickable {
                    let color = Color::new(1.0, 1.0, 1.0, 0.7);
                    draw_mesh(&gradient_ring_mesh(self.x, self.y, r, constants.thin_highlight, color, transparent(color)));
                }

                if self.was_previous_move {
                    draw_circle(self.x, self.y, constants.thickness, LIGHTGRAY);
                }
            }
        }
    }

    pub fn is_hovered(&self) -> bool {
        let radius = DISPLAY_CONSTANTS.get().unwrap().read().unwrap().radius;
        let (mx, my) = mouse_position();
        self.enable_hover && self.clickable && (mx - self.x).powf(2.0) + (my - self.y).powf(2.0) <= radius.powf(2.0)
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    pub fn set_color(&mut self, color: Option<PieceColor>) {
        self.color = color;
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

    pub fn set_was_previous_move(&mut self, b: bool) {
        self.was_previous_move = b;
    }

    pub fn enable_hover(&mut self, b: bool) {
        self.enable_hover = b;
    }

    pub fn index(&self) -> usize {
        self.index
    }
}