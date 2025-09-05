use std::collections::HashSet;
use macroquad::prelude::*;
use crate::board::Board;
use crate::cell_view::CellView;
use crate::display_constants::*;
use crate::piece_color::PieceColor;

pub struct BoardView {
    cells: [CellView; 121]
}

impl BoardView {
    pub fn new(board: &Board) -> Self {
        let cells = std::array::from_fn(|i| {
            CellView::new(i, board.cells[i].color, 0.0, 0.0, false)
        });
        let mut view = Self { cells };

        let constants = DISPLAY_CONSTANTS.get().unwrap().read().unwrap();
        view.update_positions(&constants);
        view
    }

    pub fn update_positions(&mut self, constants: &DisplayConstants) {
        let center_x = constants.screen_width / 2.0;
        let center_y = constants.screen_height / 2.0;
        let scale = constants.cell_location_scale;

        for (i, cell_view) in self.cells.iter_mut().enumerate() {
            let [x, y] = CELL_LOCATIONS[i];
            cell_view.set_position(center_x + x * scale, center_y + y * scale);
        }
    }

    pub fn draw(&self) {
        // for i in 0..6 {
        //     self.draw_triangle(i);
        // }

        for cell in &self.cells {
            cell.draw();
        }
    }

    fn draw_triangle(&self, i: usize) {
        let scale = DISPLAY_CONSTANTS.get().unwrap().read().unwrap().cell_location_scale;
        let color = PieceColor::get_color(i).get_display_color();
        let center = vec2(screen_width() / 2.0, screen_height() / 2.0);
        let [c1, c2, c3] = TRIANGLE_CORNERS[(i + 3) % 6];
        draw_triangle(center + c1 * scale,
                            center + c2 * scale,
                            center + c3 * scale,
                            color);
    }

    pub fn update_board(&mut self, board: &Board, clickable_cells: HashSet<usize>, selected_piece: Option<usize>, previous_move_path: Vec<usize>) {
        for i in 0..121 {
            self.cells[i].set_color(board.cells[i].color);
            self.cells[i].set_clickable(clickable_cells.contains(&i));
            self.cells[i].set_selected(false);
            self.cells[i].set_was_previous_move(previous_move_path.contains(&i));
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

    pub fn display_winner(&self, winner: PieceColor) {
        let text = match winner {
            PieceColor::Red => "Red wins!",
            PieceColor::Orange => "Orange wins!",
            PieceColor::Green => "Green wins!",
            PieceColor::Cyan => "Cyan wins!",
            PieceColor::Blue => "Blue wins!",
            PieceColor::Purple => "Purple wins!"
        };

        let font_size = 80.0;
        let text_dims = measure_text(text, None, font_size as u16, 1.0);
        draw_text(text, (screen_width() - text_dims.width) / 2.0, (screen_height() + text_dims.height) / 2.0, font_size, WHITE);
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

static TRIANGLE_CORNERS: [[Vec2; 3]; 6] = [
    [vec2(0.0, 10.0 * R3 - 2.0), vec2(-6.0 + R3, 4.0 * R3 + 1.0), vec2(6.0 - R3, 4.0 * R3 + 1.0)],
    [vec2(9.0, -1.0 * R3 + 2.0), vec2(3.0 + R3, 5.0 * R3 - 1.0), vec2(15.0 - R3, 5.0 * R3 - 1.0)],
    [vec2(9.0, 1.0 * R3 - 2.0), vec2(3.0 + R3, -5.0 * R3 + 1.0), vec2(15.0 - R3, -5.0 * R3 + 1.0)],
    [vec2(0.0, -10.0 * R3 + 2.0), vec2(-6.0 + R3, -4.0 * R3 - 1.0), vec2(6.0 - R3, -4.0 * R3 - 1.0)],
    [vec2(-9.0, 1.0 * R3 - 2.0), vec2(-3.0 - R3, -5.0 * R3 + 1.0), vec2(-15.0 + R3, -5.0 * R3 + 1.0)],
    [vec2(-9.0, -1.0 * R3 + 2.0), vec2(-3.0 - R3, 5.0 * R3 - 1.0), vec2(-15.0 + R3, 5.0 * R3 - 1.0)]
];