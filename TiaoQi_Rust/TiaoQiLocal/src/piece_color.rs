use macroquad::color::{Color, BLUE, GRAY, GREEN, PURPLE, RED, YELLOW};
use crate::piece_color::PieceColor::{Red, Yellow, Green, Blue, Purple, Gray};

#[derive(Copy, Clone, PartialEq)]
pub enum PieceColor {
    Red,
    Yellow,
    Green,
    Blue,
    Purple,
    Gray
}

impl PieceColor {
    pub fn get_color(i: usize) -> PieceColor {
        match i {
            0 => Red,
            1 => Yellow,
            2 => Green,
            3 => Blue,
            4 => Purple,
            5 => Gray,
            _ => unreachable!()
        }
    }

    pub fn get_index(&self) -> usize {
        match self {
            Red => 0,
            Yellow => 1,
            Green => 2,
            Blue => 3,
            Purple => 4,
            Gray => 5
        }
    }

    pub fn get_display_color(&self) -> Color {
        match self.get_index() {
            0 => RED,
            1 => YELLOW,
            2 => GREEN,
            3 => BLUE,
            4 => PURPLE,
            5 => GRAY,
            _ => unreachable!()
        }
    }
}