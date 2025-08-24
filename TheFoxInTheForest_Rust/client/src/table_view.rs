use macroquad::input::{is_mouse_button_pressed, MouseButton};
use macroquad::window::{screen_height, screen_width};
use common::client_game_state::ClientGameState;
use common::client_move::ClientMove;
use crate::center_view::CenterView;
use crate::display_constants::*;
use crate::hand_view::HandView;
use crate::opponent_hand_view::OpponentHandView;
use crate::opponent_played_card_view::OpponentPlayedCardView;
use crate::played_card_view::PlayedCardView;
use crate::player_stat_view::PlayerStatView;

pub struct TableView {
    opponent_hand_view: OpponentHandView,
    opponent_stat_view: PlayerStatView,
    opponent_played_card_view: OpponentPlayedCardView,
    center_view: CenterView,
    your_played_card_view: PlayedCardView,
    your_hand_view: HandView,
    your_stat_view: PlayerStatView,
    pub is_your_turn: bool
}

impl TableView {
    pub fn new(state: &ClientGameState) -> Self {
        Self {
            opponent_hand_view: OpponentHandView::new(state.opponent_hand_size, screen_width() / 2.0, card_height() / 2.0, !state.is_your_turn),
            opponent_played_card_view: OpponentPlayedCardView::new(state.opponent_card, screen_width() / 2.0, screen_height() / 4.0),
            opponent_stat_view: PlayerStatView::new(state.opponent_points, state.opponent_tricks, 50.0, 50.0), // TODO change x and y
            center_view: CenterView::new(state.center_card, screen_width() / 2.0, screen_height() / 2.0),
            your_played_card_view: PlayedCardView::new(state.your_card, screen_width() / 2.0, 3.0 * screen_height() / 4.0, state.is_your_turn),
            your_hand_view: HandView::new(&state.your_hand, &state.your_playable_cards, screen_width() / 2.0, screen_height() - card_height() / 2.0, state.is_your_turn),
            your_stat_view: PlayerStatView::new(state.your_points, state.your_tricks, 50.0, screen_height() - 50.0),
            is_your_turn: state.is_your_turn
        }
    }

    pub fn draw(&mut self) {
        self.opponent_hand_view.draw();
        self.opponent_played_card_view.draw();
        self.opponent_stat_view.draw();
        self.center_view.draw();
        self.your_played_card_view.draw();
        self.your_hand_view.draw();
        self.your_stat_view.draw();
    }

    pub fn handle_click(&mut self) -> Option<ClientMove> {
        if self.is_your_turn && is_mouse_button_pressed(MouseButton::Left) {
            if self.your_played_card_view.is_hovered() {
                if let Some(card) = self.your_hand_view.get_selected_card() {
                    return Some(ClientMove::new(card))
                }
            } else {
                self.your_hand_view.update();
            }
        }
        None
    }
}