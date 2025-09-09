use macroquad::color::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum PieceColor {
    Red,
    Orange,
    Green,
    Cyan,
    Blue,
    Purple
}

impl PieceColor {
    pub fn get_color(i: usize) -> PieceColor {
        match i {
            0 => PieceColor::Red,
            1 => PieceColor::Orange,
            2 => PieceColor::Green,
            3 => PieceColor::Cyan,
            4 => PieceColor::Blue,
            5 => PieceColor::Purple,
            _ => unreachable!()
        }
    }

    pub fn get_display_color(&self) -> Color {
        match self {
            PieceColor::Red => RED,
            PieceColor::Orange => ORANGE,
            PieceColor::Green => GREEN,
            PieceColor::Cyan => Color::from_rgba(13, 212, 252, 255),
            PieceColor::Blue => Color::from_rgba(0, 55, 255, 255),
            PieceColor::Purple => PURPLE,
        }
    }
}