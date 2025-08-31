use macroquad::color::{BLUE, GREEN, ORANGE, PURPLE, RED, YELLOW};

#[derive(Copy, Clone)]
pub enum PieceColor {
    Red,
    Yellow,
    Green,
    Blue,
    Purple,
    Gray
}

static DISPLAY_COLORS: [macroquad::color::Color; 6] = [RED, ORANGE, YELLOW, GREEN, BLUE, PURPLE];

impl PieceColor {
    pub fn get_color(i: usize) -> PieceColor {
        match i {
            0 => PieceColor::Red,
            1 => PieceColor::Yellow,
            2 => PieceColor::Green,
            3 => PieceColor::Blue,
            4 => PieceColor::Purple,
            5 => PieceColor::Gray,
            _ => unreachable!()
        }
    }

    pub fn get_index(&self) -> usize {
        match self {
            PieceColor::Red => 0,
            PieceColor::Yellow => 1,
            PieceColor::Green => 2,
            PieceColor::Blue => 3,
            PieceColor::Purple => 4,
            PieceColor::Gray => 5
        }
    }

    pub fn get_display_color(&self) -> macroquad::color::Color {
        DISPLAY_COLORS[self.get_index()]
    }
}