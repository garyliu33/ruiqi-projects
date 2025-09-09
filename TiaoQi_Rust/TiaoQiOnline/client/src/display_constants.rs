use std::sync::{OnceLock, RwLock};

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
    pub thin_highlight: f32
}

impl DisplayConstants {
    pub fn new(screen_width: f32, screen_height: f32) -> Self {
        let min = screen_width.min(screen_height);
        let cell_location_scale = min / 35.0;
        let radius = cell_location_scale / 2.0;
        let thickness = radius / 4.0;
        let thick_highlight = thickness * 8.0;
        let thin_highlight = thickness * 6.0;

        Self { screen_width, screen_height, cell_location_scale, radius, thickness, thick_highlight, thin_highlight }
    }
}