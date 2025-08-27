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

    let mut network = Network::new().expect("Failed to start server");

    while !network.all_players_connected() {
        network.accept_new_players();
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    println!("All players connected");
    let mut game_controller = GameController::new(vec![player1, player2], 0, network);
    game_controller.start_game();
}