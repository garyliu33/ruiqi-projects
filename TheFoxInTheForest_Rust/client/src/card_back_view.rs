use macroquad::prelude::*;
use crate::display_constants::*;
use std::sync::OnceLock;

pub struct CardBackView {
    pub x: f32,
    pub y: f32,
}

static CARD_BACK_TEXTURE: OnceLock<Texture2D> = OnceLock::new();

impl CardBackView {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn draw(&self) {
        let texture = CARD_BACK_TEXTURE.get().expect("CardBack texture not loaded");
        draw_texture_ex(texture, self.x - card_width() / 2.0, self.y - card_height() / 2.0, WHITE, DrawTextureParams {
            dest_size: Some(vec2(card_width(), card_height())),
            ..Default::default()
        })
    }
}

pub async fn load_card_back_texture() {
    let tex = load_texture("client/assets/cardback.png").await.unwrap();
    tex.set_filter(FilterMode::Linear);
    CARD_BACK_TEXTURE.set(tex).unwrap();
}