use common::card::{Card, Rank, Suit};
use common::center_card::CenterCard;
use common::deck::Deck;
use common::player::Player;
use common::client_game_state::ClientGameState;

pub struct GameState {
    pub leading_suit: Option<Suit>,
    pub trick_cards: Vec<Card>,
    pub deck: Deck,
    pub center_card: CenterCard,
    pub players: Vec<Player>,
    pub starting_player: usize,
    pub initial_starting_player: usize,
}

impl GameState {
    pub fn new(players: Vec<Player>, initial_starting_player: usize) -> Self {
        Self {
            leading_suit: None,
            trick_cards: Vec::new(),
            deck: Deck::new(),
            center_card: CenterCard::new(),
            players,
            starting_player: initial_starting_player,
            initial_starting_player,
        }
    }

    pub fn create_client_game_state(&'_ self, player_id: usize, is_your_turn: bool) -> ClientGameState {
        let your_card;
        let opponent_card;
        if self.trick_cards.is_empty() {
            your_card = None;
            opponent_card = None;
        } else if self.trick_cards.len() == 1 {
            if player_id == self.starting_player {
                your_card = Some(self.trick_cards[0]);
                opponent_card = None;
            } else {
                your_card = None;
                opponent_card = Some(self.trick_cards[0]);
            }
        } else if self.trick_cards.len() == 2 {
            if player_id == self.starting_player {
                your_card = Some(self.trick_cards[0]);
                opponent_card = Some(self.trick_cards[1]);
            } else {
                your_card = Some(self.trick_cards[1]);
                opponent_card = Some(self.trick_cards[0]);
            }
        } else {
            unreachable!();
        }

        let center_card = self.center_card.get_card();

        let your_hand = self.players[player_id].get_hand();
        let mut your_playable_cards = Vec::new();
        for card in self.players[player_id].get_hand() {
            if self.can_play_card(card, player_id) {
                your_playable_cards.push(card);
            }
        }
        
        let opponent_hand_size = self.players[1 - player_id].get_hand().len();
        let your_tricks = self.players[player_id].num_tricks_won();
        let opponent_tricks = self.players[1 - player_id].num_tricks_won();
        let your_points = self.players[player_id].get_score();
        let opponent_points = self.players[1 - player_id].get_score();

        ClientGameState {
            your_card,
            opponent_card,
            center_card,
            your_hand,
            your_playable_cards,
            opponent_hand_size,
            your_tricks,
            opponent_tricks,
            your_points,
            opponent_points,
            is_your_turn
        }
    }

    pub fn can_play_card(&self, card: Card, player_index: usize) -> bool {
        let player = &self.players[player_index];
        match self.leading_suit {
            Some(suit) => {
                let leading_card = self.trick_cards[0];
                if leading_card.rank() == Rank::Eleven {
                    match player.highest_card(suit) {
                        Some(high_card) => card == *high_card || card.rank() == Rank::One,
                        None => true,
                    }
                } else {
                    match player.highest_card(suit) {
                        Some(high_card) => card.suit() == high_card.suit(),
                        None => true,
                    }
                }
            }
            None => true,
        }
    }
}