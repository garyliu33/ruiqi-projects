use std::f32::consts::PI;
use macroquad::prelude::*;
use common::client_move::ClientMove;
use crate::cell_view::CellView;
use crate::display_constants::*;
use common::piece_color::PieceColor;
use common::server_message::ClientGameState;

pub struct BoardView {
    cells: [CellView; 121],
    rotation: f32
}

impl BoardView {
    pub fn new(rotation: f32) -> Self {
        let cells = std::array::from_fn(|i| {
            CellView::new(i, None, 0.0, 0.0, false)
        });
        let mut view = Self { cells, rotation };

        let constants = DISPLAY_CONSTANTS.get().unwrap().read().unwrap();
        view.update_positions(&constants);
        view
    }

    pub fn update_positions(&mut self, constants: &DisplayConstants) {
        let center_x = constants.screen_width / 2.0;
        let center_y = constants.screen_height / 2.0;
        let scale = constants.cell_location_scale;

        for (i, cell_view) in self.cells.iter_mut().enumerate() {
            let p = rotate(CELL_LOCATIONS[i], self.rotation);
            cell_view.set_position(center_x + p.x * scale, center_y + p.y * scale);
        }
    }

    pub fn draw(&self) {
        // for i in 0..6 {
        //     self.draw_target_marker(i);
        // }

        for cell in &self.cells {
            cell.draw();
        }
    }

    fn draw_target_marker(&self, i: usize) {
        let scale = DISPLAY_CONSTANTS.get().unwrap().read().unwrap().cell_location_scale;
        let mut color = PieceColor::get_color(i).get_display_color();
        color.a = 0.8;
        let center = vec2(screen_width() / 2.0, screen_height() / 2.0);
        let [c1, c2, c3] = TRIANGLE_CORNERS[(i + 3) % 6];
        draw_triangle(center + c1 * scale,
                      center + c2 * scale,
                      center + c3 * scale,
                      color);
    }

    pub fn update_board(&mut self, state: &ClientGameState) {
        for i in 0..121 {
            self.cells[i].set_color(state.cells[i]);
            self.cells[i].set_clickable(state.clickable_cells.contains(&i));
            self.cells[i].set_selected(false);
            self.cells[i].set_was_previous_move(match &state.previous_move_path {
                Some(path) => path.contains(&i),
                None => false
            });
            self.cells[i].enable_hover(state.is_your_turn);
        }

        if let Some(piece) = state.selected_piece {
            if self.cells[piece].color.is_some() {
                self.cells[piece].set_selected(true);
            }
        }
    }

    pub fn get_hovered_cell(&self) -> Option<usize> {
        self.cells.iter().find(|c| c.is_hovered()).map(|c| c.index())
    }

    pub fn draw_with_message(&self, msg: String) {
        self.draw();
        let font_size = 80.0;
        let text_dims = measure_text(&msg, None, font_size as u16, 1.0);
        draw_text(&msg, (screen_width() - text_dims.width) / 2.0, (screen_height() + text_dims.height) / 2.0, font_size, WHITE);
    }

    pub fn handle_click(&self) -> Option<ClientMove> {
        if let Some(clicked) = self.get_hovered_cell() {
            Some(ClientMove::new(clicked))
        } else {
            None
        }
    }
}

static CELL_LOCATIONS: [Vec2; 121] = [
    vec2(0.0, 8.0 * R3),
    vec2(-1.0, 7.0 * R3),
    vec2(1.0, 7.0 * R3),
    vec2(-2.0, 6.0 * R3),
    vec2(0.0, 6.0 * R3),
    vec2(2.0, 6.0 * R3),
    vec2(-3.0, 5.0 * R3),
    vec2(-1.0, 5.0 * R3),
    vec2(1.0, 5.0 * R3),
    vec2(3.0, 5.0 * R3),
    vec2(-12.0, 4.0 * R3),
    vec2(-10.0, 4.0 * R3),
    vec2(-8.0, 4.0 * R3),
    vec2(-6.0, 4.0 * R3),
    vec2(-4.0, 4.0 * R3),
    vec2(-2.0, 4.0 * R3),
    vec2(0.0, 4.0 * R3),
    vec2(2.0, 4.0 * R3),
    vec2(4.0, 4.0 * R3),
    vec2(6.0, 4.0 * R3),
    vec2(8.0, 4.0 * R3),
    vec2(10.0, 4.0 * R3),
    vec2(12.0, 4.0 * R3),
    vec2(-11.0, 3.0 * R3),
    vec2(-9.0, 3.0 * R3),
    vec2(-7.0, 3.0 * R3),
    vec2(-5.0, 3.0 * R3),
    vec2(-3.0, 3.0 * R3),
    vec2(-1.0, 3.0 * R3),
    vec2(1.0, 3.0 * R3),
    vec2(3.0, 3.0 * R3),
    vec2(5.0, 3.0 * R3),
    vec2(7.0, 3.0 * R3),
    vec2(9.0, 3.0 * R3),
    vec2(11.0, 3.0 * R3),
    vec2(-10.0, 2.0 * R3),
    vec2(-8.0, 2.0 * R3),
    vec2(-6.0, 2.0 * R3),
    vec2(-4.0, 2.0 * R3),
    vec2(-2.0, 2.0 * R3),
    vec2(0.0, 2.0 * R3),
    vec2(2.0, 2.0 * R3),
    vec2(4.0, 2.0 * R3),
    vec2(6.0, 2.0 * R3),
    vec2(8.0, 2.0 * R3),
    vec2(10.0, 2.0 * R3),
    vec2(-9.0, R3),
    vec2(-7.0, R3),
    vec2(-5.0, R3),
    vec2(-3.0, R3),
    vec2(-1.0, R3),
    vec2(1.0, R3),
    vec2(3.0, R3),
    vec2(5.0, R3),
    vec2(7.0, R3),
    vec2(9.0, R3),
    vec2(-8.0, 0.0),
    vec2(-6.0, 0.0),
    vec2(-4.0, 0.0),
    vec2(-2.0, 0.0),
    vec2(0.0, 0.0),
    vec2(2.0, 0.0),
    vec2(4.0, 0.0),
    vec2(6.0, 0.0),
    vec2(8.0, 0.0),
    vec2(-9.0, -R3),
    vec2(-7.0, -R3),
    vec2(-5.0, -R3),
    vec2(-3.0, -R3),
    vec2(-1.0, -R3),
    vec2(1.0, -R3),
    vec2(3.0, -R3),
    vec2(5.0, -R3),
    vec2(7.0, -R3),
    vec2(9.0, -R3),
    vec2(-10.0, -2.0 * R3),
    vec2(-8.0, -2.0 * R3),
    vec2(-6.0, -2.0 * R3),
    vec2(-4.0, -2.0 * R3),
    vec2(-2.0, -2.0 * R3),
    vec2(0.0, -2.0 * R3),
    vec2(2.0, -2.0 * R3),
    vec2(4.0, -2.0 * R3),
    vec2(6.0, -2.0 * R3),
    vec2(8.0, -2.0 * R3),
    vec2(10.0, -2.0 * R3),
    vec2(-11.0, -3.0 * R3),
    vec2(-9.0, -3.0 * R3),
    vec2(-7.0, -3.0 * R3),
    vec2(-5.0, -3.0 * R3),
    vec2(-3.0, -3.0 * R3),
    vec2(-1.0, -3.0 * R3),
    vec2(1.0, -3.0 * R3),
    vec2(3.0, -3.0 * R3),
    vec2(5.0, -3.0 * R3),
    vec2(7.0, -3.0 * R3),
    vec2(9.0, -3.0 * R3),
    vec2(11.0, -3.0 * R3),
    vec2(-12.0, -4.0 * R3),
    vec2(-10.0, -4.0 * R3),
    vec2(-8.0, -4.0 * R3),
    vec2(-6.0, -4.0 * R3),
    vec2(-4.0, -4.0 * R3),
    vec2(-2.0, -4.0 * R3),
    vec2(0.0, -4.0 * R3),
    vec2(2.0, -4.0 * R3),
    vec2(4.0, -4.0 * R3),
    vec2(6.0, -4.0 * R3),
    vec2(8.0, -4.0 * R3),
    vec2(10.0, -4.0 * R3),
    vec2(12.0, -4.0 * R3),
    vec2(-3.0, -5.0 * R3),
    vec2(-1.0, -5.0 * R3),
    vec2(1.0, -5.0 * R3),
    vec2(3.0, -5.0 * R3),
    vec2(-2.0, -6.0 * R3),
    vec2(0.0, -6.0 * R3),
    vec2(2.0, -6.0 * R3),
    vec2(-1.0, -7.0 * R3),
    vec2(1.0, -7.0 * R3),
    vec2(0.0, -8.0 * R3)
];

static TRIANGLE_CORNERS: [[Vec2; 3]; 6] = [
    [vec2(0.0, 10.0 * R3 - 2.0), vec2(-6.0 + R3, 4.0 * R3 + 1.0), vec2(6.0 - R3, 4.0 * R3 + 1.0)],
    [vec2(9.0, -1.0 * R3 + 2.0), vec2(3.0 + R3, 5.0 * R3 - 1.0), vec2(15.0 - R3, 5.0 * R3 - 1.0)],
    [vec2(9.0, 1.0 * R3 - 2.0), vec2(3.0 + R3, -5.0 * R3 + 1.0), vec2(15.0 - R3, -5.0 * R3 + 1.0)],
    [vec2(0.0, -10.0 * R3 + 2.0), vec2(-6.0 + R3, -4.0 * R3 - 1.0), vec2(6.0 - R3, -4.0 * R3 - 1.0)],
    [vec2(-9.0, 1.0 * R3 - 2.0), vec2(-3.0 - R3, -5.0 * R3 + 1.0), vec2(-15.0 + R3, -5.0 * R3 + 1.0)],
    [vec2(-9.0, -1.0 * R3 + 2.0), vec2(-3.0 - R3, 5.0 * R3 - 1.0), vec2(-15.0 + R3, 5.0 * R3 - 1.0)]
];

fn rotate(p: Vec2, deg: f32) -> Vec2 {
    let rad = deg * PI / 180.0;
    let sin = rad.sin();
    let cos = rad.cos();
    vec2(p.x * cos - p.y * sin, p.x * sin + p.y * cos)
}