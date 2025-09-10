use std::f32::consts::PI;
use std::sync::{OnceLock, RwLock};
use macroquad::color::Color;
use macroquad::math::{vec2, Vec2};
use macroquad::models::{Mesh, Vertex};

pub const R3: f32 = 1.73205080757;

pub static DISPLAY_CONSTANTS: OnceLock<RwLock<DisplayConstants>> = OnceLock::new();

#[derive(Debug)]
pub struct DisplayConstants {
    pub screen_width: f32,
    pub screen_height: f32,
    pub cell_location_scale: f32,
    pub radius: f32,
    pub thickness: f32,
    pub thick_highlight: f32,
    pub thin_highlight: f32,
    pub target_marker_radius: f32
}

impl DisplayConstants {
    pub fn new(screen_width: f32, screen_height: f32) -> Self {
        let min = screen_width.min(screen_height);
        let cell_location_scale = min / 35.0;
        let radius = cell_location_scale / 2.0;
        let thickness = radius / 4.0;
        let thick_highlight = thickness * 8.0;
        let thin_highlight = thickness * 6.0;
        let target_marker_radius = thickness * 4.0;

        Self { screen_width, screen_height, cell_location_scale, radius, thickness, thick_highlight, thin_highlight, target_marker_radius }
    }
}

pub fn gradient_ring_mesh(x: f32, y: f32, radius: f32, thickness: f32, inner_color: Color, outer_color: Color) -> Mesh {
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

pub fn transparent(color: Color) -> Color {
    Color::new(color.r, color.g, color.b, 0.0)
}

pub fn rotate(p: Vec2, deg: f32) -> Vec2 {
    let rad = deg * PI / 180.0;
    let sin = rad.sin();
    let cos = rad.cos();
    vec2(p.x * cos - p.y * sin, p.x * sin + p.y * cos)
}