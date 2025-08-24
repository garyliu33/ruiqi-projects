use macroquad::prelude::*;

pub struct PlayerStatView {
    score: usize,
    tricks: usize,
    x: f32,
    y: f32
}

impl PlayerStatView {
    pub fn new(score: usize, tricks:usize, x: f32, y: f32) -> Self {
        Self { score, tricks, x, y }
    }

    pub fn draw(&self) {
        let lines = vec![
            format!("Points: {}", self.score),
            format!("Tricks: {}", self.tricks),
        ];

        let font_size = 30;
        let font = None;
        let line_spacing = 5.0; // extra spacing between lines

        // measure single line height
        let sample = measure_text("Ag", font, font_size, 1.0);
        let line_height = sample.height;

        // total block height
        let total_height = lines.len() as f32 * line_height + (lines.len() as f32 - 1.0) * line_spacing;

        // starting y so that the whole block is centered
        let mut y = self.y - total_height / 2.0 + line_height;

        for line in lines {
            let dims = measure_text(&line, font, font_size, 1.0);
            let x = self.x - dims.width / 2.0;

            draw_text_ex(
                &line,
                x,
                y,
                TextParams {
                    font_size,
                    color: BLACK,
                    ..Default::default()
                },
            );

            y += line_height + line_spacing;
        }
    }
}