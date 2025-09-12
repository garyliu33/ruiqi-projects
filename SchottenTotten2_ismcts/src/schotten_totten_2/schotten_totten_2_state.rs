use crate::game_state::GameState;
use crate::schotten_totten_2::card::{Card, Color};
use crate::schotten_totten_2::com_st_proto;
use crate::schotten_totten_2::r#move::SchottenTotten2Move;
use crate::schotten_totten_2::player::Player;
use crate::schotten_totten_2::types::{FormationType, Role, WinningType};
use crate::schotten_totten_2::wall_tile::{WallPattern, WallTile};
use rand::rng;
use rand::seq::SliceRandom;
use std::fmt;

const HAND_SIZE: usize = 6;

#[derive(Debug, Clone)]
pub struct SchottenTotten2State {
    pub deck: Vec<Card>,
    pub discard_pile: Vec<Card>,
    pub players: [Player; 2],
    pub wall_tiles: Vec<WallTile>,
    pub player_to_move_index: usize,
    pub attacker_damaged_tiles: u8,
    pub is_client_turn: bool,
    pub last_played_card: Option<Card>,
}

impl SchottenTotten2State {
    pub fn from_proto(proto: &com_st_proto::GameStateProto) -> Self {
        let mut discard = vec![];
        for (_, card_list_proto) in &proto.discard {
            for card_proto in &card_list_proto.card_list {
                discard.push(Card::from_proto(card_proto));
            }
        }
        let client_hand = Card::from_proto_array(&proto.client_hand);
        let (attacker, defender) = if proto.is_client_attacker {
            (
                Player {
                    hand: client_hand,
                    role: Role::Attacker,
                    oil_cauldrons: 0,
                },
                Player {
                    hand: vec![],
                    role: Role::Defender,
                    oil_cauldrons: proto.cauldron_count as u8,
                },
            )
        } else {
            (
                Player {
                    hand: vec![],
                    role: Role::Attacker,
                    oil_cauldrons: 0,
                },
                Player {
                    hand: client_hand,
                    role: Role::Defender,
                    oil_cauldrons: proto.cauldron_count as u8,
                },
            )
        };
        let player_to_move = match (proto.is_client_turn, proto.is_client_attacker) {
            (true, true) => 0,
            (true, false) => 1,
            (false, true) => 1,
            (false, false) => 0,
        };
        let wall_tiles = WallTile::from_proto_array(&proto.walls);
        let damaged_tile_count = wall_tiles.iter().filter(|t| t.is_damaged).count() as u8;
        SchottenTotten2State {
            deck: vec![],
            discard_pile: discard,
            players: [attacker, defender],
            wall_tiles: wall_tiles,
            player_to_move_index: player_to_move,
            attacker_damaged_tiles: damaged_tile_count,
            is_client_turn: proto.is_client_turn,
            last_played_card: proto
                .last_played_card
                .map(|card_proto| Card::from_proto(&card_proto)),
        }
    }

    pub fn get_current_player(&self) -> &Player {
        &self.players[self.player_to_move_index]
    }

    fn check_chicken_vs_chef(
        &self,
        played_card: Card,
        opponent_cards: &Vec<Card>,
    ) -> Option<usize> {
        if played_card.value != 0 && played_card.value != 11 {
            return None;
        }
        let expected_opp_value = if played_card.value == 0 { 11_i8 } else { 0_i8 };

        if let Some(opp_card) = opponent_cards
            .iter()
            .find(|c| c.value == expected_opp_value)
        {
            if opp_card.color == played_card.color {
                let index = opponent_cards
                    .iter()
                    .position(|c| c.value == expected_opp_value)
                    .unwrap();
                return Some(index);
            }
        }
        return None;
    }

    fn check_attacker_control(&self, tile_index: usize) -> bool {
        let tile = &self.wall_tiles[tile_index];

        assert!(tile.attacker_cards.len() <= tile.get_length());
        assert!(tile.defender_cards.len() <= tile.get_length());

        let attacker_formation_complete = tile.attacker_cards.len() == tile.get_length();
        let defender_formation_complete = tile.defender_cards.len() == tile.get_length();

        if !attacker_formation_complete || !defender_formation_complete {
            return false;
        }

        let attacker_eval = FormationType::evaluate_formation(&tile.attacker_cards);
        let defender_eval = FormationType::evaluate_formation(&tile.defender_cards);

        assert!(attacker_eval.0.is_some());
        assert!(defender_eval.0.is_some());

        let attacker_formation = attacker_eval.0.as_ref().unwrap();
        let defender_formation = defender_eval.0.as_ref().unwrap();
        let wall_pattern = tile.get_wall_pattern();

        match wall_pattern {
            WallPattern::None => {
                if attacker_formation > defender_formation {
                    true // Stronger formation type
                } else if attacker_formation == defender_formation {
                    attacker_eval.1 > defender_eval.1 // Same type, higher sum
                } else {
                    false
                }
            }
            WallPattern::Plus => {
                attacker_eval.1 > defender_eval.1 // higher sum
            }
            WallPattern::Minus => {
                attacker_eval.1 < defender_eval.1 // lower sum
            }
            WallPattern::Run => {
                if attacker_formation == defender_formation {
                    attacker_eval.1 > defender_eval.1 // Same type, higher sum
                } else if *attacker_formation == FormationType::ColorRun {
                    true
                } else if *defender_formation == FormationType::ColorRun {
                    false
                } else if *attacker_formation == FormationType::Run {
                    true
                } else if *defender_formation == FormationType::Run {
                    false
                } else {
                    attacker_eval.1 > defender_eval.1
                }
            }
            WallPattern::Color => {
                if attacker_formation == defender_formation {
                    attacker_eval.1 > defender_eval.1 // Same type, higher sum
                } else if *attacker_formation == FormationType::ColorRun {
                    true
                } else if *defender_formation == FormationType::ColorRun {
                    false
                } else if *attacker_formation == FormationType::Color {
                    true
                } else if *defender_formation == FormationType::Color {
                    false
                } else {
                    attacker_eval.1 > defender_eval.1
                }
            }
            WallPattern::Equal => {
                if attacker_formation == defender_formation {
                    attacker_eval.1 > defender_eval.1 // Same type, higher sum
                } else if *attacker_formation == FormationType::SameStrength {
                    true
                } else if *defender_formation == FormationType::SameStrength {
                    false
                } else {
                    attacker_eval.1 > defender_eval.1
                }
            }
        }
    }

    fn check_game_over(&self) -> (bool, f64, WinningType) {
        // Attacker wins if they damage a 4th tile or one tile twice.
        if self.attacker_damaged_tiles >= 4 {
            return (true, 1.0, WinningType::DamagedFourTiles); // Attacker wins
        }
        let damaged_twice = self.wall_tiles.iter().any(|t| t.is_damaged_twice);
        if damaged_twice {
            return (true, 1.0, WinningType::DamagedTwice); // Attacker wins
        }

        // Defender wins if the deck is empty and the attacker hasn't won.
        if self.deck.is_empty() {
            return (true, 0.0, WinningType::EmptyDeck); // Defender wins
        }

        // Defender wins if it has no space left to play.
        if self.player_to_move_index == 1 {
            let is_all_tiles_full = self
                .wall_tiles
                .iter()
                .filter(|w| w.defender_cards.len() < w.get_length())
                .count()
                == 0;
            if is_all_tiles_full {
                return (true, 0.0, WinningType::NoSpace);
            }
        }

        (false, 0.0, WinningType::None)
    }
}

impl GameState<SchottenTotten2Move> for SchottenTotten2State {
    fn player_to_move(&self) -> usize {
        self.player_to_move_index
    }

    fn get_next_player(&self, player: usize) -> usize {
        (player + 1) % self.number_of_players()
    }

    fn clone_state(&self) -> impl GameState<SchottenTotten2Move> {
        self.clone()
    }

    fn clone_and_randomize(&self, player_index: usize) -> impl GameState<SchottenTotten2Move> {
        let mut new_state = self.clone();

        // Unknown cards are all cards minus hand cards, tile cards and discarded cards.
        let mut unknown_cards = vec![];
        let colors = [
            Color::Red,
            Color::Blue,
            Color::Green,
            Color::Yellow,
            Color::Gray,
        ];
        for &color in &colors {
            for value in 0..=11 {
                unknown_cards.push(Card { value, color });
            }
        }
        unknown_cards.retain(|c| !self.players[player_index].hand.contains(c));
        for wall in &self.wall_tiles {
            unknown_cards.retain(|c| !wall.attacker_cards.contains(c));
            unknown_cards.retain(|c| !wall.defender_cards.contains(c));
        }
        unknown_cards.retain(|c| !self.discard_pile.contains(c));

        let mut rng = rng();
        unknown_cards.shuffle(&mut rng);

        // Deal out the unknown cards.
        let opponent_index = self.get_next_player(player_index);
        new_state.players[opponent_index].hand = unknown_cards.drain(0..HAND_SIZE).collect();
        new_state.deck = unknown_cards;

        new_state
    }

    fn do_move(&mut self, m: &SchottenTotten2Move) {
        match m {
            SchottenTotten2Move::PlayCard { card, tile_index } => {
                let card_index = self.players[self.player_to_move_index]
                    .hand
                    .iter()
                    .position(|c| c == card)
                    .unwrap();
                let card_to_play = self.players[self.player_to_move_index]
                    .hand
                    .remove(card_index);

                let current_player = &self.players[self.player_to_move_index];

                let wall_tile = &self.wall_tiles[*tile_index];
                let opponent_cards = if current_player.role == Role::Attacker {
                    &wall_tile.defender_cards
                } else {
                    &wall_tile.attacker_cards
                };

                let opponent_card_index = self.check_chicken_vs_chef(card_to_play, opponent_cards);
                if let Some(card_index) = opponent_card_index {
                    let opponent_cards = if current_player.role == Role::Attacker {
                        &mut self.wall_tiles[*tile_index].defender_cards
                    } else {
                        &mut self.wall_tiles[*tile_index].attacker_cards
                    };
                    let opp_card = opponent_cards.remove(card_index);
                    self.discard_pile.push(opp_card);
                    self.discard_pile.push(card_to_play);
                } else {
                    if current_player.role == Role::Attacker {
                        self.wall_tiles[*tile_index]
                            .attacker_cards
                            .push(card_to_play);
                    } else {
                        self.wall_tiles[*tile_index]
                            .defender_cards
                            .push(card_to_play);
                    }

                    if self.check_attacker_control(*tile_index) {
                        if self.wall_tiles[*tile_index].is_damaged {
                            self.wall_tiles[*tile_index].is_damaged_twice = true;
                        } else {
                            self.attacker_damaged_tiles += 1;
                            self.wall_tiles[*tile_index].is_damaged = true;
                        }
                        self.discard_pile
                            .extend(&mut self.wall_tiles[*tile_index].attacker_cards.drain(..));
                        self.discard_pile
                            .extend(&mut self.wall_tiles[*tile_index].defender_cards.drain(..));
                    }
                }

                // Draw a card
                if !self.deck.is_empty() {
                    let new_card = self.deck.pop().unwrap();
                    self.players[self.player_to_move_index].hand.push(new_card);
                }

                self.player_to_move_index = self.get_next_player(self.player_to_move_index);
            }
            SchottenTotten2Move::Retreat { tile_index } => {
                let tile = &mut self.wall_tiles[*tile_index];
                // Attacker discards all cards from the specified tile.
                self.discard_pile.extend(tile.attacker_cards.drain(..));
            }
            SchottenTotten2Move::ThrowOilCauldron { tile_index } => {
                let defender = &mut self.players[self.player_to_move_index];
                let tile = &mut self.wall_tiles[*tile_index];

                assert!(!tile.attacker_cards.is_empty());
                assert!(defender.oil_cauldrons > 0);

                if defender.oil_cauldrons > 0 && !tile.attacker_cards.is_empty() {
                    let removed_card = tile.attacker_cards.remove(0); // The card closest to the wall.
                    self.discard_pile.push(removed_card);
                    defender.oil_cauldrons -= 1;
                }
            }
        }

        assert_eq!(
            5 * 12,
            self.discard_pile.len()
                + self
                    .wall_tiles
                    .iter()
                    .map(|w| w.attacker_cards.len() + w.defender_cards.len())
                    .sum::<usize>()
                + self.deck.len()
                + self.players[0].hand.len()
                + self.players[1].hand.len()
        );
    }

    fn get_moves(&self) -> Vec<SchottenTotten2Move> {
        let mut moves = Vec::new();
        let current_player = &self.players[self.player_to_move_index];

        // Play card moves
        for tile_index in 0..self.wall_tiles.len() {
            let tile = &self.wall_tiles[tile_index];
            let player_cards = if current_player.role == Role::Attacker {
                &tile.attacker_cards
            } else {
                &tile.defender_cards
            };
            if player_cards.len() < tile.get_length() {
                moves.extend(
                    current_player
                        .hand
                        .iter()
                        .map(|c| SchottenTotten2Move::PlayCard {
                            card: *c,
                            tile_index: tile_index,
                        }),
                );
            }
        }

        // Attacker-specific moves.
        if current_player.role == Role::Attacker {
            for (i, tile) in self.wall_tiles.iter().enumerate() {
                if !tile.attacker_cards.is_empty() {
                    moves.push(SchottenTotten2Move::Retreat { tile_index: i });
                }
            }
        }

        // Defender-specific moves.
        if current_player.role == Role::Defender
            && current_player.oil_cauldrons > 0
            && !self
                .last_played_card
                .map(|card| card.color == Color::ACTION && card.value == -2)
                .unwrap_or(false)
        {
            for (i, tile) in self.wall_tiles.iter().enumerate() {
                if !tile.attacker_cards.is_empty() {
                    moves.push(SchottenTotten2Move::ThrowOilCauldron { tile_index: i });
                }
            }
        }

        moves
    }

    fn get_result(&self, player: usize) -> Option<f64> {
        let (is_game_over, reward, _) = self.check_game_over();
        if is_game_over {
            return if player == 0 && self.players[0].role == Role::Attacker
                || player == 1 && self.players[1].role == Role::Attacker
            {
                Some(reward)
            } else {
                Some(1.0 - reward) // Defender perspective
            };
        } else {
            None // Game not over
        }
    }

    fn number_of_players(&self) -> usize {
        2
    }
}

impl fmt::Display for SchottenTotten2State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for p in 0..self.number_of_players() {
            let player = &self.players[p];
            write!(f, "player {} cards: {:?}", p, player.hand)?;
        }
        writeln!(f, "")?;
        for tile in &self.wall_tiles {
            write!(f, "Tile {}:", tile.id)?;
            write!(f, " Attacker: {:?}", tile.attacker_cards)?;
            write!(f, " Defender: {:?}", tile.defender_cards)?;
            writeln!(f, " Damaged: {}", tile.is_damaged)?;
        }
        writeln!(f, "deck size: {}", self.deck.len())?;
        writeln!(f, "Player to move: {}", self.player_to_move())?;
        Ok(())
    }
}

#[cfg(test)]
impl SchottenTotten2State {
    fn new(_: usize) -> SchottenTotten2State {
        let mut siege_cards = Vec::new();
        let colors = [
            Color::Red,
            Color::Blue,
            Color::Green,
            Color::Yellow,
            Color::Gray,
        ];
        for &color in &colors {
            for value in 0..=11 {
                siege_cards.push(Card { value, color });
            }
        }

        let mut rng = rng();
        siege_cards.shuffle(&mut rng);

        let p1 = Player {
            hand: siege_cards.drain(0..6).collect(),
            role: Role::Attacker,
            oil_cauldrons: 0,
        };
        let p2 = Player {
            hand: siege_cards.drain(0..6).collect(),
            role: Role::Defender,
            oil_cauldrons: 3,
        };

        let wall_tiles = vec![
            WallTile {
                id: 0,
                intact_length: 3,
                intact_wall_pattern: WallPattern::Plus,
                damaged_length: 3,
                damaged_wall_pattern: WallPattern::Run,
                is_damaged: false,
                is_damaged_twice: false,
                attacker_cards: Vec::new(),
                defender_cards: Vec::new(),
            },
            WallTile {
                id: 1,
                intact_length: 4,
                intact_wall_pattern: WallPattern::None,
                damaged_length: 2,
                damaged_wall_pattern: WallPattern::Equal,
                is_damaged: false,
                is_damaged_twice: false,
                attacker_cards: Vec::new(),
                defender_cards: Vec::new(),
            },
            WallTile {
                id: 2,
                intact_length: 3,
                intact_wall_pattern: WallPattern::None,
                damaged_length: 3,
                damaged_wall_pattern: WallPattern::Color,
                is_damaged: false,
                is_damaged_twice: false,
                attacker_cards: Vec::new(),
                defender_cards: Vec::new(),
            },
            WallTile {
                id: 3,
                intact_length: 2,
                intact_wall_pattern: WallPattern::None,
                damaged_length: 4,
                damaged_wall_pattern: WallPattern::Minus,
                is_damaged: false,
                is_damaged_twice: false,
                attacker_cards: Vec::new(),
                defender_cards: Vec::new(),
            },
            WallTile {
                id: 4,
                intact_length: 3,
                intact_wall_pattern: WallPattern::None,
                damaged_length: 3,
                damaged_wall_pattern: WallPattern::Color,
                is_damaged: false,
                is_damaged_twice: false,
                attacker_cards: Vec::new(),
                defender_cards: Vec::new(),
            },
            WallTile {
                id: 5,
                intact_length: 4,
                intact_wall_pattern: WallPattern::None,
                damaged_length: 2,
                damaged_wall_pattern: WallPattern::Equal,
                is_damaged: false,
                is_damaged_twice: false,
                attacker_cards: Vec::new(),
                defender_cards: Vec::new(),
            },
            WallTile {
                id: 6,
                intact_length: 3,
                intact_wall_pattern: WallPattern::Minus,
                damaged_length: 3,
                damaged_wall_pattern: WallPattern::Run,
                is_damaged: false,
                is_damaged_twice: false,
                attacker_cards: Vec::new(),
                defender_cards: Vec::new(),
            },
        ];

        SchottenTotten2State {
            deck: siege_cards,
            discard_pile: Vec::new(),
            players: [p1, p2],
            wall_tiles: wall_tiles,
            player_to_move_index: 0,
            attacker_damaged_tiles: 0,
            is_client_turn: false,
            last_played_card: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::os::linux::raw::stat;

    use super::*;

    // Helper function to create a new game state for testing
    fn setup_game_state() -> SchottenTotten2State {
        SchottenTotten2State::new(2)
    }

    // --- GameState Setup and Basic Functions Tests ---

    #[test]
    fn test_get_next_player() {
        let state = setup_game_state();
        assert_eq!(state.get_next_player(0), 1);
        assert_eq!(state.get_next_player(1), 0);
    }

    #[test]
    fn test_number_of_players() {
        let state = setup_game_state();
        assert_eq!(state.number_of_players(), 2);
    }

    // --- Move Execution Tests (do_move) ---

    #[test]
    fn test_do_move_play_card() {
        let mut state = setup_game_state();
        let initial_deck_size = state.deck.len();
        let initial_hand_size = state.players[0].hand.len();
        let card_to_play = state.players[0].hand[0];
        let tile_index = 0;

        let play_move = SchottenTotten2Move::PlayCard {
            card: card_to_play,
            tile_index,
        };
        state.do_move(&play_move);

        assert_eq!(state.players[0].hand.len(), initial_hand_size); // Hand size should remain the same after drawing a new card
        assert_eq!(state.deck.len(), initial_deck_size - 1); // One card should be drawn from the deck
        assert_eq!(state.wall_tiles[tile_index].attacker_cards.len(), 1);
        assert_eq!(state.wall_tiles[tile_index].attacker_cards[0], card_to_play);
        assert_eq!(state.player_to_move_index, 1);
        assert!(
            state.players[0]
                .hand
                .iter()
                .find(|c| **c == card_to_play)
                .is_none()
        );
    }

    #[test]
    fn test_do_move_play_card_2() {
        let mut state = setup_game_state();
        let initial_deck_size = state.deck.len();
        let initial_hand_size = state.players[0].hand.len();
        let card_to_play = state.players[0].hand[2];
        let tile_index = 3;

        let play_move = SchottenTotten2Move::PlayCard {
            card: card_to_play,
            tile_index,
        };
        state.do_move(&play_move);

        assert_eq!(state.players[0].hand.len(), initial_hand_size); // Hand size should remain the same after drawing a new card
        assert_eq!(state.deck.len(), initial_deck_size - 1); // One card should be drawn from the deck
        assert_eq!(state.wall_tiles[tile_index].attacker_cards.len(), 1);
        assert_eq!(state.wall_tiles[tile_index].attacker_cards[0], card_to_play);
        assert_eq!(state.player_to_move_index, 1);
        assert!(
            state.players[0]
                .hand
                .iter()
                .find(|c| **c == card_to_play)
                .is_none()
        );
    }

    #[test]
    fn test_do_move_attacker_wins_by_damaged_twice() {
        let mut state = setup_game_state();
        let tile_index = 0;
        state.wall_tiles[tile_index].is_damaged = true;
        state.attacker_damaged_tiles = 1;

        // Make attacker win the same tile again
        // Attacker formation
        state.wall_tiles[tile_index].attacker_cards = vec![
            Card {
                value: 10,
                color: Color::Red,
            },
            Card {
                value: 11,
                color: Color::Red,
            },
        ];
        // Defender formation
        state.wall_tiles[tile_index].defender_cards = vec![
            Card {
                value: 1,
                color: Color::Blue,
            },
            Card {
                value: 0,
                color: Color::Yellow,
            },
            Card {
                value: 0,
                color: Color::Blue,
            },
        ];
        // The wall tile above added 5 cards, so remove 5 cards from the deck.
        state.deck.drain(0..5);

        // Ensure the wall pattern is one where attacker can win
        state.wall_tiles[tile_index].damaged_wall_pattern = WallPattern::Plus;
        state.wall_tiles[tile_index].intact_length = 3;

        // Player to move must be the attacker
        state.player_to_move_index = 0;
        let card_to_play = state.players[0].hand[0];
        let play_move = SchottenTotten2Move::PlayCard {
            card: card_to_play,
            tile_index,
        };
        state.do_move(&play_move);

        assert_eq!(6, state.players[0].hand.len());
        assert_eq!(6, state.players[1].hand.len());
        assert_eq!(6, state.discard_pile.len());
        assert_eq!(42, state.deck.len());

        // This test case assumes that `do_move` will trigger a check that leads to a win
        // but the current implementation of `do_move` doesn't do a full game-over check.
        // It only checks if the tile is controlled.
        // Therefore, we must call the `check_game_over` function to assert the outcome.
        let (is_over, _, winning_type) = state.check_game_over();
        assert!(is_over);
        assert_eq!(winning_type, WinningType::DamagedTwice);
    }

    #[test]
    fn test_do_move_attacker_wins_by_four_damaged_tiles() {
        let mut state = setup_game_state();
        // Pre-damage three tiles
        state.wall_tiles[0].is_damaged = true;
        state.wall_tiles[1].is_damaged = true;
        state.wall_tiles[2].is_damaged = true;
        state.attacker_damaged_tiles = 3;

        // Attacker's turn
        state.player_to_move_index = 0;
        let card_to_play = state.players[0].hand[0];
        let tile_index = 3; // This will be the 4th damaged tile

        // Make attacker win the tile
        state.wall_tiles[tile_index].attacker_cards = vec![Card {
            value: 11,
            color: Color::Red,
        }];
        state.wall_tiles[tile_index].defender_cards = vec![
            Card {
                value: 0,
                color: Color::Blue,
            },
            Card {
                value: 1,
                color: Color::Blue,
            },
        ];

        // the wall tile above added 3 cards, so remove 3 cards from the deck.
        state.deck.drain(0..3);

        state.wall_tiles[tile_index].intact_wall_pattern = WallPattern::Plus;
        state.wall_tiles[tile_index].intact_length = 2;

        let play_move = SchottenTotten2Move::PlayCard {
            card: card_to_play,
            tile_index,
        };
        state.do_move(&play_move);

        // Attacker should now have 4 damaged tiles
        assert_eq!(state.attacker_damaged_tiles, 4);
        assert_eq!(6, state.players[0].hand.len());
        assert_eq!(6, state.players[1].hand.len());
        assert_eq!(4, state.discard_pile.len());
        assert_eq!(44, state.deck.len());

        // Check if the game is over
        let (is_over, _, winning_type) = state.check_game_over();
        assert!(is_over);
        assert_eq!(winning_type, WinningType::DamagedFourTiles);
    }

    #[test]
    fn test_do_move_play_chicken_vs_chef() {
        let mut state = setup_game_state();
        state.player_to_move_index = 0; // Attacker's turn
        let tile_index = 0;

        // Defender places a "Chef" card (value 11)
        let chef_card = Card {
            value: 11,
            color: Color::Red,
        };
        state.wall_tiles[tile_index].defender_cards.push(chef_card);

        // Attacker plays a "Chicken" card (value 0) of the same color
        let chicken_card = Card {
            value: 0,
            color: Color::Red,
        };
        let card_removed = state.players[0].hand.remove(0);
        state.discard_pile.push(card_removed);
        state.players[0].hand.push(chicken_card);

        state.deck.drain(0..2);

        let play_move = SchottenTotten2Move::PlayCard {
            card: chicken_card,
            tile_index,
        };
        state.do_move(&play_move);

        // The defender's card should be removed, and the attacker's card should not be placed
        assert!(state.wall_tiles[tile_index].defender_cards.is_empty());
        assert!(state.wall_tiles[tile_index].attacker_cards.is_empty());
        assert_eq!(6, state.players[0].hand.len());
        assert_eq!(6, state.players[1].hand.len());
    }

    #[test]
    fn test_do_move_play_chicken_vs_chef_more_cards() {
        let mut state = setup_game_state();
        state.player_to_move_index = 0; // Attacker's turn
        let tile_index = 0;

        // Defender places a "Chef" card (value 11)
        let chef_card = Card {
            value: 11,
            color: Color::Red,
        };
        state.wall_tiles[tile_index].defender_cards.push(chef_card);
        state.wall_tiles[tile_index].defender_cards.push(Card {
            value: 10,
            color: Color::Blue,
        });

        // Attacker plays a "Chicken" card (value 0) of the same color
        let chicken_card = Card {
            value: 0,
            color: Color::Red,
        };
        let card_removed = state.players[0].hand.remove(0);
        state.discard_pile.push(card_removed);
        state.players[0].hand.push(chicken_card);
        state.deck.drain(0..3);

        let play_move = SchottenTotten2Move::PlayCard {
            card: chicken_card,
            tile_index,
        };
        state.do_move(&play_move);

        // The defender's card should be removed, and the attacker's card should not be placed
        assert!(state.wall_tiles[tile_index].defender_cards.len() == 1);
        assert!(state.wall_tiles[tile_index].attacker_cards.is_empty());
        assert_eq!(6, state.players[0].hand.len());
        assert_eq!(6, state.players[1].hand.len());
    }

    #[test]
    fn test_do_move_throw_oil_cauldron() {
        let mut state = setup_game_state();
        state.player_to_move_index = 1; // Defender's turn
        let initial_cauldrons = state.players[1].oil_cauldrons;
        let tile_index = 0;
        let card_to_add = Card {
            value: 5,
            color: Color::Red,
        };

        state.wall_tiles[tile_index]
            .attacker_cards
            .push(card_to_add);
        state.deck.drain(0..1);

        let throw_move = SchottenTotten2Move::ThrowOilCauldron { tile_index };
        state.do_move(&throw_move);

        assert_eq!(state.players[1].oil_cauldrons, initial_cauldrons - 1);
        assert!(state.wall_tiles[tile_index].attacker_cards.is_empty());
        assert_eq!(state.discard_pile.len(), 1);
        assert_eq!(state.discard_pile[0], card_to_add);
        assert_eq!(6, state.players[0].hand.len());
        assert_eq!(6, state.players[1].hand.len());
    }

    #[test]
    fn test_do_move_retreat() {
        let mut state = setup_game_state();
        let tile_index = 0;
        state.wall_tiles[tile_index].attacker_cards.push(Card {
            value: 5,
            color: Color::Red,
        });
        state.wall_tiles[tile_index].attacker_cards.push(Card {
            value: 6,
            color: Color::Red,
        });
        state.deck.drain(0..2);

        let retreat_move = SchottenTotten2Move::Retreat { tile_index };
        state.do_move(&retreat_move);

        assert!(state.wall_tiles[tile_index].attacker_cards.is_empty());
        assert_eq!(state.discard_pile.len(), 2);
        assert_eq!(
            state.discard_pile[1],
            Card {
                value: 6,
                color: Color::Red
            }
        );
    }

    // --- Game Over Condition Tests (check_game_over) ---

    #[test]
    fn test_check_game_over_attacker_wins_by_damaged_tiles() {
        let mut state = setup_game_state();
        state.attacker_damaged_tiles = 3;

        // Damage the 4th tile to trigger a win
        state.wall_tiles[3].is_damaged = true;
        state.attacker_damaged_tiles += 1;

        let (is_over, result, winning_type) = state.check_game_over();
        assert!(is_over);
        assert_eq!(result, 1.0);
        assert_eq!(WinningType::DamagedFourTiles, winning_type);
    }

    #[test]
    fn test_check_game_over_attacker_wins_by_damaging_twice() {
        let mut state = setup_game_state();

        // Damage tile 0 for the first time
        state.attacker_damaged_tiles = 1;

        state.wall_tiles.push(WallTile {
            id: 0,
            intact_length: 3,
            intact_wall_pattern: WallPattern::Plus,
            damaged_length: 3,
            damaged_wall_pattern: WallPattern::Run,
            is_damaged: true, // This marks the tile as "damaged twice"
            is_damaged_twice: true,
            attacker_cards: Vec::new(),
            defender_cards: Vec::new(),
        });

        let (is_over, result, winning_type) = state.check_game_over();
        assert!(is_over);
        assert_eq!(result, 1.0);
        assert_eq!(WinningType::DamagedTwice, winning_type);
    }

    #[test]
    fn test_check_game_over_defender_wins_deck_empty() {
        let mut state = setup_game_state();
        state.deck.clear();
        let (is_over, result, winning_type) = state.check_game_over();
        assert!(is_over);
        assert_eq!(result, 0.0);
        assert_eq!(WinningType::EmptyDeck, winning_type);
    }

    #[test]
    fn test_check_game_over_defender_wins_no_space_to_play() {
        let mut state = setup_game_state();
        state.player_to_move_index = 1; // Defender's turn
        for tile in &mut state.wall_tiles {
            for _ in 0..tile.intact_length {
                tile.defender_cards.push(Card {
                    value: 1,
                    color: Color::Red,
                });
            }
        }
        let (is_over, result, winning_type) = state.check_game_over();
        assert!(is_over);
        assert_eq!(result, 0.0);
        assert_eq!(WinningType::NoSpace, winning_type);
    }

    #[test]
    fn test_check_game_over_not_over() {
        let state = setup_game_state();
        let (is_over, result, winning_type) = state.check_game_over();
        assert!(!is_over);
        assert_eq!(result, 0.0);
        assert_eq!(WinningType::None, winning_type);
    }

    // --- get_moves() Tests ---

    #[test]
    fn test_get_moves_attacker_turn() {
        let mut state = setup_game_state();
        state.player_to_move_index = 0; // Attacker's turn
        let initial_hand_size = state.players[0].hand.len();

        let moves = state.get_moves();

        // Attacker can play any card from their hand on any tile with space.
        // There are 7 tiles, and all have space initially.
        // 6 cards in hand * 7 tiles = 42 potential PlayCard moves.
        // Additionally, the attacker can retreat from any tile they have cards on.
        // Initially, the attacker has no cards on any tile, so there should be no retreat moves.
        let expected_play_moves = initial_hand_size * state.wall_tiles.len();
        let actual_play_moves = moves
            .iter()
            .filter(|m| matches!(m, SchottenTotten2Move::PlayCard { .. }))
            .count();
        let actual_retreat_moves = moves
            .iter()
            .filter(|m| matches!(m, SchottenTotten2Move::Retreat { .. }))
            .count();

        assert_eq!(actual_play_moves, expected_play_moves);
        assert_eq!(actual_retreat_moves, 0);

        // Add a card to a tile to test retreat moves
        state.wall_tiles[0].attacker_cards.push(Card {
            value: 5,
            color: Color::Red,
        });
        let moves_with_retreat = state.get_moves();
        let actual_retreat_moves_after = moves_with_retreat
            .iter()
            .filter(|m| matches!(m, SchottenTotten2Move::Retreat { .. }))
            .count();
        assert_eq!(actual_retreat_moves_after, 1);
    }

    #[test]
    fn test_get_moves_defender_turn() {
        let mut state = setup_game_state();
        state.player_to_move_index = 1; // Defender's turn
        let initial_hand_size = state.players[1].hand.len();

        let moves = state.get_moves();

        // Defender can play any card from their hand on any tile with space.
        // There are 7 tiles, and all have space initially.
        // 6 cards in hand * 7 tiles = 42 potential PlayCard moves.
        // The defender can also throw oil cauldrons. Initially, there are no attacker cards on tiles,
        // so there should be no ThrowOilCauldron moves.
        let expected_play_moves = initial_hand_size * state.wall_tiles.len();
        let actual_play_moves = moves
            .iter()
            .filter(|m| matches!(m, SchottenTotten2Move::PlayCard { .. }))
            .count();
        let actual_oil_cauldron_moves = moves
            .iter()
            .filter(|m| matches!(m, SchottenTotten2Move::ThrowOilCauldron { .. }))
            .count();

        assert_eq!(actual_play_moves, expected_play_moves);
        assert_eq!(actual_oil_cauldron_moves, 0);

        // Add a card to a tile to test ThrowOilCauldron moves
        state.wall_tiles[0].attacker_cards.push(Card {
            value: 5,
            color: Color::Red,
        });
        let moves_with_oil = state.get_moves();
        let actual_oil_cauldron_moves_after = moves_with_oil
            .iter()
            .filter(|m| matches!(m, SchottenTotten2Move::ThrowOilCauldron { .. }))
            .count();
        assert_eq!(actual_oil_cauldron_moves_after, 1);
    }

    #[test]
    fn test_get_moves_full_tile() {
        let mut state = setup_game_state();
        state.player_to_move_index = 0; // Attacker's turn
        let tile_index = 0;
        let tile = &mut state.wall_tiles[tile_index];

        // Fill the tile with the maximum number of cards for the attacker
        for _ in 0..tile.intact_length {
            tile.attacker_cards.push(Card {
                value: 1,
                color: Color::Red,
            });
        }

        let moves = state.get_moves();

        // The number of play card moves should be reduced by the cards that would go on the now-full tile.
        let num_play_moves = moves
            .iter()
            .filter(|m| matches!(m, SchottenTotten2Move::PlayCard { .. }))
            .count();
        assert_eq!(
            num_play_moves,
            state.players[0].hand.len() * (state.wall_tiles.len() - 1)
        );

        // The attacker should still be able to retreat from this tile
        let retreat_moves = moves
            .iter()
            .filter(|m| matches!(m, SchottenTotten2Move::Retreat { .. }))
            .count();
        assert_eq!(retreat_moves, 1);
    }

    // --- check_attacker_control() Tests ---

    #[test]
    fn test_check_attacker_control_attacker_wins_on_plus_tile() {
        let mut state = setup_game_state();
        let tile_index = 0;

        // Attacker has a higher sum (10 + 11 + 9 = 30)
        state.wall_tiles[tile_index].attacker_cards = vec![
            Card {
                value: 10,
                color: Color::Red,
            },
            Card {
                value: 11,
                color: Color::Red,
            },
            Card {
                value: 9,
                color: Color::Blue,
            },
        ];

        // Defender has a lower sum (1 + 2 + 3 = 6)
        state.wall_tiles[tile_index].defender_cards = vec![
            Card {
                value: 1,
                color: Color::Blue,
            },
            Card {
                value: 2,
                color: Color::Blue,
            },
            Card {
                value: 3,
                color: Color::Blue,
            },
        ];
        state.deck.drain(0..6);

        state.wall_tiles[tile_index].intact_wall_pattern = WallPattern::Plus;
        state.wall_tiles[tile_index].intact_length = 3;

        let result = state.check_attacker_control(tile_index);
        assert!(result);
    }

    #[test]
    fn test_check_attacker_control_defender_wins_on_plus_tile() {
        let mut state = setup_game_state();
        let tile_index = 0;

        // Attacker has a lower sum (1 + 2 + 3 = 6)
        state.wall_tiles[tile_index].attacker_cards = vec![
            Card {
                value: 1,
                color: Color::Red,
            },
            Card {
                value: 2,
                color: Color::Red,
            },
            Card {
                value: 3,
                color: Color::Red,
            },
        ];

        // Defender has a higher sum (10 + 11 + 9 = 30)
        state.wall_tiles[tile_index].defender_cards = vec![
            Card {
                value: 10,
                color: Color::Blue,
            },
            Card {
                value: 11,
                color: Color::Blue,
            },
            Card {
                value: 9,
                color: Color::Yellow,
            },
        ];
        state.deck.drain(0..6);

        state.wall_tiles[tile_index].intact_wall_pattern = WallPattern::Plus;
        state.wall_tiles[tile_index].intact_length = 3;

        let result = state.check_attacker_control(tile_index);
        assert!(!result);
    }

    #[test]
    fn test_check_attacker_control_draw_on_plus_tile() {
        let mut state = setup_game_state();
        let tile_index = 0;

        // Both have the same sum (1 + 2 + 3 = 6)
        state.wall_tiles[tile_index].attacker_cards = vec![
            Card {
                value: 1,
                color: Color::Red,
            },
            Card {
                value: 2,
                color: Color::Red,
            },
            Card {
                value: 3,
                color: Color::Red,
            },
        ];
        state.wall_tiles[tile_index].defender_cards = vec![
            Card {
                value: 1,
                color: Color::Yellow,
            },
            Card {
                value: 2,
                color: Color::Blue,
            },
            Card {
                value: 3,
                color: Color::Blue,
            },
        ];
        state.deck.drain(0..6);

        state.wall_tiles[tile_index].intact_wall_pattern = WallPattern::Plus;
        state.wall_tiles[tile_index].intact_length = 3;

        let result = state.check_attacker_control(tile_index);
        assert!(!result); // Defender wins in a tie
    }

    #[test]
    fn test_check_attacker_control_attacker_wins_on_run_tile() {
        let mut state = setup_game_state();
        let tile_index = 0;

        // Attacker has a run (which is a higher formation type than a sum)
        state.wall_tiles[tile_index].attacker_cards = vec![
            Card {
                value: 2,
                color: Color::Red,
            },
            Card {
                value: 3,
                color: Color::Blue,
            },
            Card {
                value: 4,
                color: Color::Green,
            },
        ];

        // Defender has only a sum
        state.wall_tiles[tile_index].defender_cards = vec![
            Card {
                value: 10,
                color: Color::Red,
            },
            Card {
                value: 10,
                color: Color::Blue,
            },
            Card {
                value: 10,
                color: Color::Green,
            },
        ];
        state.deck.drain(0..6);

        state.wall_tiles[tile_index].intact_wall_pattern = WallPattern::Run;
        state.wall_tiles[tile_index].intact_length = 3;

        let result = state.check_attacker_control(tile_index);
        assert!(result);
    }

    #[test]
    fn test_check_attacker_control_defender_wins_on_run_tile_by_stronger_formation() {
        let mut state = setup_game_state();
        let tile_index = 0;

        // Attacker has a run
        state.wall_tiles[tile_index].attacker_cards = vec![
            Card {
                value: 10,
                color: Color::Red,
            },
            Card {
                value: 8,
                color: Color::Blue,
            },
            Card {
                value: 9,
                color: Color::Green,
            },
        ];

        // Defender has a color run (stronger than a run)
        state.wall_tiles[tile_index].defender_cards = vec![
            Card {
                value: 2,
                color: Color::Red,
            },
            Card {
                value: 4,
                color: Color::Red,
            },
            Card {
                value: 3,
                color: Color::Red,
            },
        ];
        state.deck.drain(0..6);

        state.wall_tiles[tile_index].intact_wall_pattern = WallPattern::Run;
        state.wall_tiles[tile_index].intact_length = 3;

        let result = state.check_attacker_control(tile_index);
        assert!(!result);
    }

    #[test]
    fn test_get_moves_defender_cant_throw_oil_after_spy_move() {
        let mut state = setup_game_state();
        state.player_to_move_index = 1; // Defender's turn
        state.players[1].oil_cauldrons = 1;

        // Set the last played card to be the "Spy" card
        state.last_played_card = Some(Card {
            color: Color::ACTION,
            value: -2,
        });

        // Place an attacker card on a tile to make it a potential target for the oil cauldron
        let tile_index = 0;
        state.wall_tiles[tile_index].attacker_cards.push(Card {
            value: 5,
            color: Color::Red,
        });
        state.deck.drain(0..1);

        let moves = state.get_moves();

        // The defender should not be able to throw an oil cauldron because the last card played was a "Spy" card
        let actual_oil_cauldron_moves = moves
            .iter()
            .filter(|m| matches!(m, SchottenTotten2Move::ThrowOilCauldron { .. }))
            .count();

        assert_eq!(actual_oil_cauldron_moves, 0);
    }
}
