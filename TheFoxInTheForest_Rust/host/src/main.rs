use common::deck::Deck;
use crate::game_controller::GameController;
use common::player::Player;
use crate::network::Network;

mod game_controller;
pub mod game_state;
mod network;

fn main() {
    let mut deck = Deck::new();
    deck.shuffle();

    let player1 = Player::new();
    let player2 = Player::new();

    let mut network = Network::new();
    network.start_server().expect("Failed to start server");

    let mut game_controller = GameController::new(vec![player1, player2], 0, network);
    game_controller.start_game();
}