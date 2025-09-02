use macroquad::color::*;
use crate::piece_color::PieceColor::{Red, Orange, Green, Blue, Purple, Cyan};

#[derive(Copy, Clone, PartialEq)]
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
            0 => Red,
            1 => Orange,
            2 => Green,
            3 => Cyan,
            4 => Blue,
            5 => Purple,
            _ => unreachable!()
        }
    }

    pub fn get_display_color(&self) -> Color {
        match self {
            Red => RED,
            Orange => ORANGE,
            Green => GREEN,
            Cyan => Color::from_rgba(13, 212, 252, 255),
            Blue => Color::from_rgba(0, 55, 255, 255),
            Purple => PURPLE,
        }
    }
}