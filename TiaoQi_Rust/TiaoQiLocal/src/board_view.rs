use std::collections::HashSet;
use macroquad::prelude::screen_width;
use macroquad::window::screen_height;
use crate::board::{Board, Cell, PieceColor};
use crate::cell_view::CellView;
use crate::display_constants::{R3, CELL_LOCATION_SCALE};

pub struct BoardView {
    cells: [CellView; 121],
}

impl BoardView {
    pub fn new(board: &Board) -> Self {
        let cells = std::array::from_fn(|i| {
            let color = board.cells[i].color;
            CellView::new(
                i,
                color,
                screen_width() / 2.0 + CELL_LOCATIONS[i][0] * CELL_LOCATION_SCALE,
                screen_height() / 2.0 + CELL_LOCATIONS[i][1] * CELL_LOCATION_SCALE,
                false
            )
        });
        Self { cells }
    }

    pub fn draw(&self) {
        for cell in &self.cells {
            cell.draw();
        }
    }

    pub fn update_board(&mut self, board: &Board, clickable_cells: HashSet<usize>, selected_piece: Option<usize>) {
        for i in 0..121 {
            self.cells[i].set_color(board.cells[i].color);
            self.cells[i].set_clickable(clickable_cells.contains(&i));
            self.cells[i].set_selected(false);
        }

        if let Some(piece) = selected_piece {
            if self.cells[piece].color.is_some() {
                self.cells[piece].set_selected(true);
            }
        }
    }

    pub fn get_hovered_cell(&self) -> Option<usize> {
        self.cells.iter().find(|c| c.is_hovered()).map(|c| c.index())
    }
}

static CELL_LOCATIONS: [[f32; 2]; 121] = [
    [0.0, 8.0 * R3],
    [-1.0, 7.0 * R3],
    [1.0, 7.0 * R3],
    [-2.0, 6.0 * R3],
    [0.0, 6.0 * R3],
    [2.0, 6.0 * R3],
    [-3.0, 5.0 * R3],
    [-1.0, 5.0 * R3],
    [1.0, 5.0 * R3],
    [3.0, 5.0 * R3],
    [-12.0, 4.0 * R3],
    [-10.0, 4.0 * R3],
    [-8.0, 4.0 * R3],
    [-6.0, 4.0 * R3],
    [-4.0, 4.0 * R3],
    [-2.0, 4.0 * R3],
    [0.0, 4.0 * R3],
    [2.0, 4.0 * R3],
    [4.0, 4.0 * R3],
    [6.0, 4.0 * R3],
    [8.0, 4.0 * R3],
    [10.0, 4.0 * R3],
    [12.0, 4.0 * R3],
    [-11.0, 3.0 * R3],
    [-9.0, 3.0 * R3],
    [-7.0, 3.0 * R3],
    [-5.0, 3.0 * R3],
    [-3.0, 3.0 * R3],
    [-1.0, 3.0 * R3],
    [1.0, 3.0 * R3],
    [3.0, 3.0 * R3],
    [5.0, 3.0 * R3],
    [7.0, 3.0 * R3],
    [9.0, 3.0 * R3],
    [11.0, 3.0 * R3],
    [-10.0, 2.0 * R3],
    [-8.0, 2.0 * R3],
    [-6.0, 2.0 * R3],
    [-4.0, 2.0 * R3],
    [-2.0, 2.0 * R3],
    [0.0, 2.0 * R3],
    [2.0, 2.0 * R3],
    [4.0, 2.0 * R3],
    [6.0, 2.0 * R3],
    [8.0, 2.0 * R3],
    [10.0, 2.0 * R3],
    [-9.0, R3],
    [-7.0, R3],
    [-5.0, R3],
    [-3.0, R3],
    [-1.0, R3],
    [1.0, R3],
    [3.0, R3],
    [5.0, R3],
    [7.0, R3],
    [9.0, R3],
    [-8.0, 0.0],
    [-6.0, 0.0],
    [-4.0, 0.0],
    [-2.0, 0.0],
    [0.0, 0.0],
    [2.0, 0.0],
    [4.0, 0.0],
    [6.0, 0.0],
    [8.0, 0.0],
    [-9.0, -R3],
    [-7.0, -R3],
    [-5.0, -R3],
    [-3.0, -R3],
    [-1.0, -R3],
    [1.0, -R3],
    [3.0, -R3],
    [5.0, -R3],
    [7.0, -R3],
    [9.0, -R3],
    [-10.0, -2.0 * R3],
    [-8.0, -2.0 * R3],
    [-6.0, -2.0 * R3],
    [-4.0, -2.0 * R3],
    [-2.0, -2.0 * R3],
    [0.0, -2.0 * R3],
    [2.0, -2.0 * R3],
    [4.0, -2.0 * R3],
    [6.0, -2.0 * R3],
    [8.0, -2.0 * R3],
    [10.0, -2.0 * R3],
    [-11.0, -3.0 * R3],
    [-9.0, -3.0 * R3],
    [-7.0, -3.0 * R3],
    [-5.0, -3.0 * R3],
    [-3.0, -3.0 * R3],
    [-1.0, -3.0 * R3],
    [1.0, -3.0 * R3],
    [3.0, -3.0 * R3],
    [5.0, -3.0 * R3],
    [7.0, -3.0 * R3],
    [9.0, -3.0 * R3],
    [11.0, -3.0 * R3],
    [-12.0, -4.0 * R3],
    [-10.0, -4.0 * R3],
    [-8.0, -4.0 * R3],
    [-6.0, -4.0 * R3],
    [-4.0, -4.0 * R3],
    [-2.0, -4.0 * R3],
    [0.0, -4.0 * R3],
    [2.0, -4.0 * R3],
    [4.0, -4.0 * R3],
    [6.0, -4.0 * R3],
    [8.0, -4.0 * R3],
    [10.0, -4.0 * R3],
    [12.0, -4.0 * R3],
    [-3.0, -5.0 * R3],
    [-1.0, -5.0 * R3],
    [1.0, -5.0 * R3],
    [3.0, -5.0 * R3],
    [-2.0, -6.0 * R3],
    [0.0, -6.0 * R3],
    [2.0, -6.0 * R3],
    [-1.0, -7.0 * R3],
    [1.0, -7.0 * R3],
    [0.0, -8.0 * R3]
];