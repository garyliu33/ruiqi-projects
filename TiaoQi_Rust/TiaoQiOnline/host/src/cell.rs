use common::piece_color::PieceColor;

pub struct Cell {
    pub color: Option<PieceColor>,
    pub neighbors: [Option<usize>; 6]
}

impl Cell {
    pub fn new(right: Option<usize>, top_right: Option<usize>, top_left: Option<usize>, left: Option<usize>, bottom_left: Option<usize>, bottom_right: Option<usize>) -> Self {
        Self { color: None, neighbors: [right, top_right, top_left, left, bottom_left, bottom_right] }
    }
}