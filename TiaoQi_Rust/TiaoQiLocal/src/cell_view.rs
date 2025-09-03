use std::f32::consts::PI;
use macroquad::color::*;
use macroquad::input::mouse_position;
use macroquad::prelude::*;
use macroquad::shapes::{draw_circle, draw_circle_lines};
use crate::piece_color::PieceColor;
use crate::display_constants::RADIUS;

pub struct CellView {
    index: usize,
    pub color: Option<PieceColor>,
    x: f32,
    y: f32,
    clickable: bool,
    selected: bool,
    was_previous_move: bool
}

impl CellView {
    pub fn new(index: usize, color: Option<PieceColor>, x: f32, y: f32, clickable: bool) -> Self {
        Self { index, color, x, y, clickable, selected: false, was_previous_move: false }
    }
    
    pub fn draw(&self) {
        match self.color {
            Some(piece_color) => {
                let mut color = piece_color.get_display_color();
                draw_circle(self.x, self.y, RADIUS, color);
                draw_circle_lines(self.x, self.y, RADIUS, 2.0, LIGHTGRAY);

                if self.selected {
                    color = Color::new(1.0, 1.0, 1.0, 0.9);
                    draw_mesh(&gradient_ring_mesh(self.x, self.y, RADIUS + 2.0, 16.0, color, transparent(color)));
                } else if self.is_hovered() {
                    color = Color::new(1.0, 1.0, 1.0, 0.7);
                    draw_mesh(&gradient_ring_mesh(self.x, self.y, RADIUS + 2.0, 12.0, color, transparent(color)));
                } else if self.clickable {
                    color.a = 0.7;
                    draw_mesh(&gradient_ring_mesh(self.x, self.y, RADIUS + 2.0, 12.0, color, transparent(color)));
                } else if self.was_previous_move {
                    color = YELLOW;
                    color.a = 0.6;
                    draw_mesh(&gradient_ring_mesh(self.x, self.y, RADIUS + 2.0, 12.0, color, transparent(color)));
                }
            }
            None => {
                draw_circle_lines(self.x, self.y, RADIUS, 2.0, LIGHTGRAY);

                let mut color = WHITE;
                if self.is_hovered() {
                    draw_mesh(&gradient_ring_mesh(self.x, self.y, RADIUS + 2.0, 12.0, color, transparent(color)));
                } else if self.clickable {
                    color.a = 0.4;
                    draw_mesh(&gradient_ring_mesh(self.x, self.y, RADIUS + 2.0, 12.0, color, transparent(color)));
                }

                if self.was_previous_move {
                    draw_circle(self.x, self.y, 2.0, LIGHTGRAY);
                }
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

    pub fn set_was_previous_move(&mut self, b: bool) {
        self.was_previous_move = b;
    }

    pub fn index(&self) -> usize {
        self.index
    }
}

fn gradient_ring_mesh(x: f32, y: f32, radius: f32, thickness: f32, inner_color: Color, outer_color: Color) -> Mesh {
    let mut vertices = vec![];
    let mut indices = vec![];

    let segments = 128;
    for i in 0..=segments {
        let angle = (i as f32 / segments as f32) * 2.0 * PI;
        let cos = angle.cos();
        let sin = angle.sin();

        vertices.push(Vertex::new(
            x + (radius + thickness) * cos,
            y + (radius + thickness) * sin,
            0.0,
            0.0,
            0.0,
            outer_color,
        ));

        vertices.push(Vertex::new(
            x + radius * cos,
            y + radius * sin,
            0.0,
            0.0,
            0.0,
            inner_color,
        ));
    }

    for i in 0..segments {
        let current = i * 2;
        let next = (i + 1) * 2;

        let v0 = current;
        let v1 = current + 1;
        let v2 = next + 1;
        let v3 = next;

        indices.push(v0);
        indices.push(v1);
        indices.push(v2);

        indices.push(v0);
        indices.push(v2);
        indices.push(v3);
    }

    Mesh{ vertices, indices, texture: None }
}

fn transparent(color: Color) -> Color {
    Color::new(color.r, color.g, color.b, 0.0)
}