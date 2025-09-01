use common::card::*;
use common::deck::Deck;
use crate::game_state::GameState;
use common::player::Player;
use crate::network::Network;

pub const HAND_SIZE: usize = 13;

pub struct GameController {
    pub state: GameState,
    pub network: Network
}

impl GameController {
    pub fn new(players: Vec<Player>, initial_starting_player: usize, network: Network) -> Self {
        Self {
            state: GameState::new(players, initial_starting_player),
            network
        }
    }

    pub fn start_game(&mut self) {
        loop {
            self.state.deck.shuffle();
            for _ in 0..HAND_SIZE {
                self.state.players[self.state.starting_player].draw_card(self.state.deck.pop());
                self.state.players[1 - self.state.starting_player].draw_card(self.state.deck.pop());
            }
            self.state.center_card.set_card(self.state.deck.pop());
            self.run_game();
            self.add_points();
            if self.is_game_over() {
                self.network.send_win_message(&self.state, self.get_winner());
                break;
            }
            self.reset_game();
        }
    }

    fn is_game_over(&self) -> bool {
        for player in &self.state.players {
            if player.get_score() >= 21 {
                return true;
            }
        }
        false
    }

    fn get_winner(&self) -> usize {
        if self.state.players[0].get_score() > self.state.players[1].get_score() {
            0
        } else if self.state.players[0].get_score() < self.state.players[1].get_score() {
            1
        } else {
            2
        }
    }

    fn run_game(&mut self) {
        for _ in 0..HAND_SIZE {
            self.take_turn(self.state.starting_player);
            self.take_turn(1 - self.state.starting_player);
            self.state.starting_player = self.process_trick();
            self.reset_round();
        }
    }

    fn reset_round(&mut self) {
        self.state.trick_cards.clear();
        self.state.leading_suit = None;
    }

    fn add_points(&mut self) {
        for player in &mut self.state.players {
            match player.num_tricks_won() {
                0..=3 => player.add_points(6),
                4 => player.add_points(1),
                5 => player.add_points(2),
                6 => player.add_points(3),
                7..=9 => player.add_points(6),
                10..=12 => (),
                _ => unreachable!(),
            }
        }
    }

    fn reset_game(&mut self) {
        self.state.deck = Deck::new();
        self.state.players[0].reset_tricks();
        self.state.players[1].reset_tricks();
        self.state.initial_starting_player = 1 - self.state.initial_starting_player;
        self.state.starting_player = self.state.initial_starting_player;
    }

    fn take_turn(&mut self, player_index: usize) {
        loop {
            self.network.send_game_state(&self.state, player_index);
            let card = self.wait_for_move(player_index);
            if self.state.can_play_card(card, player_index) {
                self.state.players[player_index].remove_card(&card);
                self.play_card(card, player_index);
                break;
            }
        }
    }

    fn play_card(&mut self, card: Card, player_index: usize) {
        if self.state.trick_cards.is_empty() {
            self.state.leading_suit = Some(card.suit());
        }
        self.state.trick_cards.push(card);

        match card.rank() {
            Rank::Three => {
                self.state.players[player_index].draw_card(self.state.center_card.remove_card());
                self.network.send_game_state(&self.state, player_index);
                let card = self.wait_for_move(player_index);
                self.state.players[player_index].remove_card(&card);
                self.state.center_card.set_card(card);
            }
            Rank::Five => {
                self.state.players[player_index].draw_card(self.state.deck.pop());
                self.network.send_game_state(&self.state, player_index);
                let card = self.wait_for_move(player_index);
                self.state.players[player_index].remove_card(&card);
                self.state.deck.add_to_bottom(card);
            }
            _ => (),
        }
    }

    fn wait_for_move(&mut self, player_index: usize) -> Card {
        loop {
            if let Some(client_move) = self.network.wait_for_move(player_index) {
                return client_move.card;
            } else {
                println!("Player {} disconnected. Pausing game until they reconnect.", player_index);

                while self.network.player_streams[player_index].is_none() {
                    self.network.accept_new_players();
                    std::thread::sleep(std::time::Duration::from_millis(500));
                }

                println!("Player {} reconnected! Resuming game.", player_index);
                self.network.send_game_state(&self.state, player_index);
            }
        }
    }

    fn process_trick(&mut self) -> usize {
        let leading_card = self.state.trick_cards[0];
        let following_card = self.state.trick_cards[1];

        let leading_strength;
        let following_strength;
        if (leading_card.rank() == Rank::Nine && following_card.rank() == Rank::Nine)
            || (leading_card.rank() != Rank::Nine && following_card.rank() != Rank::Nine)
        {
            leading_strength = self.card_strength(leading_card);
            following_strength = self.card_strength(following_card);
        } else {
            if leading_card.rank() == Rank::Nine {
                leading_strength = self.card_strength(Card::new(
                    Rank::Nine,
                    self.state.center_card.get_card().unwrap().suit()
                ));
                following_strength = self.card_strength(following_card);
            } else {
                leading_strength = self.card_strength(leading_card);
                following_strength = self.card_strength(Card::new(
                    Rank::Nine,
                    self.state.center_card.get_card().unwrap().suit()
                ));
            }
        }

        let trick_winner;
        let next_trick_leader;
        if leading_strength > following_strength {
            trick_winner = self.state.starting_player;
            next_trick_leader = match following_card.rank() {
                Rank::One => 1 - trick_winner,
                _ => trick_winner,
            };
        } else if leading_strength < following_strength {
            trick_winner = 1 - self.state.starting_player;
            next_trick_leader = match leading_card.rank() {
                Rank::One => 1 - trick_winner,
                _ => trick_winner,
            };
        } else {
            panic!("Trick processing went wrong")
        }

        for card in &self.state.trick_cards {
            if card.rank() == Rank::Seven {
                self.state.players[trick_winner].add_points(1);
            }
        }

        self.state.players[trick_winner].win_trick();
        next_trick_leader
    }

    fn card_strength(&mut self, card: Card) -> usize {
        match self.state.leading_suit {
            Some(lead) => {
                if card.suit() == self.state.center_card.get_card().unwrap().suit() {
                    100 + card.rank().value()
                } else if card.suit() == lead {
                    card.rank().value()
                } else {
                    0
                }
            }
            None => {
                if card.suit() == self.state.center_card.get_card().unwrap().suit() {
                    100 + card.rank().value()
                } else {
                    card.rank().value()
                }
            }
        }
    }
}
