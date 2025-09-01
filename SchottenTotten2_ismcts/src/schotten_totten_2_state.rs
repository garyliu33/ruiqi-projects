use crate::game_state::GameState;
use rand::rng;
use rand::seq::SliceRandom;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WinningType {
    DamagedTwice,
    DamagedFourTiles,
    EmptyDeck,
    NoSpace,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Color {
    Red,
    Blue,
    Yellow,
    Green,
    Gray,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Card {
    pub value: u8,
    pub color: Color,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {:?})", self.value, self.color)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub enum FormationType {
    Sum,
    Run,
    Color,
    SameStrength,
    ColorRun,
}

impl FormationType {
    // Logic to evaluate a formation based on the rules.
    pub fn evaluate_formation(cards: &[Card]) -> (Option<FormationType>, u32) {
        if cards.len() < 2 {
            return (None, 0);
        }

        let sum: u32 = cards.iter().map(|c| c.value as u32).sum();

        let first_color = cards[0].color;
        let same_color = cards.iter().all(|c| c.color == first_color);

        let first_strength = cards[0].value;
        let same_strength = cards.iter().all(|c| c.value == first_strength);

        let mut sorted_cards = cards.to_vec();
        sorted_cards.sort_by_key(|c| c.value);
        let mut is_run = true;
        for i in 0..sorted_cards.len() - 1 {
            if sorted_cards[i].value + 1 != sorted_cards[i + 1].value {
                is_run = false;
                break;
            }
        }

        if is_run && same_color {
            return (Some(FormationType::ColorRun), sum);
        }
        if same_strength {
            return (Some(FormationType::SameStrength), sum);
        }
        if same_color {
            return (Some(FormationType::Color), sum);
        }
        if is_run {
            return (Some(FormationType::Run), sum);
        }
        return (Some(FormationType::Sum), sum);
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd)]
pub enum WallPattern {
    Color,
    Run,
    Equal,
    Plus,
    Minus,
    None,
}

#[derive(Debug, Clone)]
pub struct WallTile {
    pub id: usize,
    pub required_cards: usize,
    pub wall_pattern: WallPattern,
    pub damaged_required_cards: usize,
    pub damaged_wall_pattern: WallPattern,
    pub is_damaged: bool,
    pub is_damaged_twice: bool,
    pub attacker_cards: Vec<Card>,
    pub defender_cards: Vec<Card>,
}

impl fmt::Display for WallTile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Tile {}: Pattern: {:?}, Attacker: {:?}, Defender: {:?}, isDamaged: {}",
            self.id,
            if self.is_damaged {
                self.damaged_wall_pattern
            } else {
                self.wall_pattern
            },
            self.attacker_cards,
            self.defender_cards,
            self.is_damaged,
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Role {
    Attacker,
    Defender,
}

#[derive(Debug, Clone)]
pub struct Player {
    pub hand: Vec<Card>,
    pub role: Role,
    pub oil_cauldrons: u8,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}: {} cards in hand", self.role, self.hand.len())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SchottenTotten2Move {
    PlayCard { card: Card, tile_index: usize },
    Retreat { tile_index: usize },
    ThrowOilCauldron { tile_index: usize },
}

impl fmt::Display for SchottenTotten2Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SchottenTotten2Move::PlayCard { card, tile_index } => {
                write!(f, "Play card {} on tile {}", card, tile_index)
            }
            SchottenTotten2Move::Retreat { tile_index } => {
                write!(f, "Retreat from tile {}", tile_index)
            }
            SchottenTotten2Move::ThrowOilCauldron { tile_index } => {
                write!(f, "Throw oil cauldron on tile {}", tile_index)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct SchottenTotten2State {
    pub deck: Vec<Card>,
    pub discard_pile: Vec<Card>,
    pub players: [Player; 2],
    pub wall_tiles: Vec<WallTile>,
    pub player_to_move_index: usize,
    pub attacker_damaged_tiles: u8,
}

impl SchottenTotten2State {
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
        let expected_opp_value = if played_card.value == 0 { 11_u8 } else { 0_u8 };

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

        assert!(tile.attacker_cards.len() <= tile.required_cards);
        assert!(tile.defender_cards.len() <= tile.required_cards);

        let attacker_formation_complete = tile.attacker_cards.len() == tile.required_cards;
        let defender_formation_complete = tile.defender_cards.len() == tile.required_cards;

        if !attacker_formation_complete || !defender_formation_complete {
            return false;
        }

        let attacker_eval = FormationType::evaluate_formation(&tile.attacker_cards);
        let defender_eval = FormationType::evaluate_formation(&tile.defender_cards);

        assert!(attacker_eval.0.is_some());
        assert!(defender_eval.0.is_some());

        let wall_pattern = if tile.is_damaged {
            &tile.damaged_wall_pattern
        } else {
            &tile.wall_pattern
        };
        let attacker_formation = attacker_eval.0.as_ref().unwrap();
        let defender_formation = defender_eval.0.as_ref().unwrap();
        let result = match wall_pattern {
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
        };
        result
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
            let mut has_empty_space = false;
            for wall in &self.wall_tiles {
                if wall.defender_cards.len() < wall.required_cards {
                    has_empty_space = true;
                    break;
                }
            }
            if !has_empty_space {
                return (true, 0.0, WinningType::NoSpace);
            }
        }

        (false, 0.0, WinningType::None)
    }
}

impl GameState<SchottenTotten2Move> for SchottenTotten2State {
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
                required_cards: 3,
                wall_pattern: WallPattern::Plus,
                damaged_required_cards: 3,
                damaged_wall_pattern: WallPattern::Run,
                is_damaged: false,
                is_damaged_twice: false,
                attacker_cards: Vec::new(),
                defender_cards: Vec::new(),
            },
            WallTile {
                id: 1,
                required_cards: 4,
                wall_pattern: WallPattern::None,
                damaged_required_cards: 2,
                damaged_wall_pattern: WallPattern::Equal,
                is_damaged: false,
                is_damaged_twice: false,
                attacker_cards: Vec::new(),
                defender_cards: Vec::new(),
            },
            WallTile {
                id: 2,
                required_cards: 3,
                wall_pattern: WallPattern::None,
                damaged_required_cards: 3,
                damaged_wall_pattern: WallPattern::Color,
                is_damaged: false,
                is_damaged_twice: false,
                attacker_cards: Vec::new(),
                defender_cards: Vec::new(),
            },
            WallTile {
                id: 3,
                required_cards: 2,
                wall_pattern: WallPattern::None,
                damaged_required_cards: 4,
                damaged_wall_pattern: WallPattern::Minus,
                is_damaged: false,
                is_damaged_twice: false,
                attacker_cards: Vec::new(),
                defender_cards: Vec::new(),
            },
            WallTile {
                id: 4,
                required_cards: 3,
                wall_pattern: WallPattern::None,
                damaged_required_cards: 3,
                damaged_wall_pattern: WallPattern::Color,
                is_damaged: false,
                is_damaged_twice: false,
                attacker_cards: Vec::new(),
                defender_cards: Vec::new(),
            },
            WallTile {
                id: 5,
                required_cards: 4,
                wall_pattern: WallPattern::None,
                damaged_required_cards: 2,
                damaged_wall_pattern: WallPattern::Equal,
                is_damaged: false,
                is_damaged_twice: false,
                attacker_cards: Vec::new(),
                defender_cards: Vec::new(),
            },
            WallTile {
                id: 6,
                required_cards: 3,
                wall_pattern: WallPattern::Minus,
                damaged_required_cards: 3,
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
            wall_tiles,
            player_to_move_index: 0,
            attacker_damaged_tiles: 0,
        }
    }

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
        let mut rng = rng();
        let mut new_state = self.clone();

        // Unknown cards are the opponent's hand and the deck.
        let mut unknown_cards: Vec<Card> = Vec::new();
        let opponent_index = self.get_next_player(player_index);

        unknown_cards.extend_from_slice(&self.players[opponent_index].hand);
        unknown_cards.extend_from_slice(&self.deck);
        unknown_cards.shuffle(&mut rng);

        // Deal out the unknown cards.
        let opponent_hand_size = self.players[opponent_index].hand.len();
        new_state.players[opponent_index].hand =
            unknown_cards.drain(0..opponent_hand_size).collect();
        new_state.deck = unknown_cards;

        new_state
    }

    fn do_move(&mut self, m: &SchottenTotten2Move) {
        match m {
            SchottenTotten2Move::PlayCard { card, tile_index } => {
                let card_index = self.players[self.player_to_move_index]
                    .hand
                    .iter()
                    .position(|c| c == card);
                let card_to_play = if let Some(index) = card_index {
                    Some(self.players[self.player_to_move_index].hand.remove(index))
                } else {
                    None
                };
                let current_player = &self.players[self.player_to_move_index];

                if let Some(card_to_play) = card_to_play {
                    let wall_tile = &self.wall_tiles[*tile_index];
                    let opponent_cards = if current_player.role == Role::Attacker {
                        &wall_tile.defender_cards
                    } else {
                        &wall_tile.attacker_cards
                    };

                    let opponent_index = self.check_chicken_vs_chef(card_to_play, opponent_cards);
                    if let Some(index) = opponent_index {
                        let opponent_cards = if current_player.role == Role::Attacker {
                            &mut self.wall_tiles[*tile_index].defender_cards
                        } else {
                            &mut self.wall_tiles[*tile_index].attacker_cards
                        };
                        opponent_cards.remove(index);
                        return;
                    }

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
                    }
                }

                // Draw a card
                if !self.deck.is_empty() {
                    let new_card = self.deck.pop().unwrap();
                    self.players[self.player_to_move_index].hand.push(new_card);
                }

                // Attacker can declare control after playing a card.
                if self.players[self.player_to_move_index].role == Role::Attacker {
                    // This is handled as a separate move in get_moves
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

                if defender.oil_cauldrons > 0 && !tile.attacker_cards.is_empty() {
                    let removed_card = tile.attacker_cards.remove(0); // The card closest to the wall.
                    self.discard_pile.push(removed_card);
                    defender.oil_cauldrons -= 1;
                }
            }
        }
    }

    fn get_moves(&self) -> Vec<SchottenTotten2Move> {
        let mut moves = Vec::new();
        let current_player = &self.players[self.player_to_move_index];

        // Play card moves
        for card in &current_player.hand {
            for tile_index in 0..self.wall_tiles.len() {
                let tile = &self.wall_tiles[tile_index];
                let player_cards = if current_player.role == Role::Attacker {
                    &tile.attacker_cards
                } else {
                    &tile.defender_cards
                };
                if player_cards.len() < tile.required_cards {
                    moves.push(SchottenTotten2Move::PlayCard {
                        card: *card,
                        tile_index,
                    });
                }
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
        if current_player.role == Role::Defender {
            for (i, tile) in self.wall_tiles.iter().enumerate() {
                if current_player.oil_cauldrons > 0 && !tile.attacker_cards.is_empty() {
                    moves.push(SchottenTotten2Move::ThrowOilCauldron { tile_index: i });
                }
            }
        }

        moves
    }

    fn get_result(&self, player: usize) -> Option<f64> {
        let (is_game_over, reward, winning_type) = self.check_game_over();
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
        writeln!(f, "Player to move: {}", self.player_to_move())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to create a new game state for testing
    fn setup_game_state() -> SchottenTotten2State {
        SchottenTotten2State::new(2)
    }

    // --- GameState Setup and Basic Functions Tests ---

    #[test]
    fn test_new_game_state() {
        let state = setup_game_state();
        assert_eq!(state.deck.len(), 60 - 12);
        assert_eq!(state.players[0].hand.len(), 6);
        assert_eq!(state.players[1].hand.len(), 6);
        assert_eq!(state.players[0].role, Role::Attacker);
        assert_eq!(state.players[1].role, Role::Defender);
        assert_eq!(state.players[1].oil_cauldrons, 3);
        assert_eq!(state.wall_tiles.len(), 7);
        assert_eq!(state.player_to_move_index, 0);
    }

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

        // Ensure the wall pattern is one where attacker can win
        state.wall_tiles[tile_index].damaged_wall_pattern = WallPattern::Plus;
        state.wall_tiles[tile_index].required_cards = 3;

        // Player to move must be the attacker
        state.player_to_move_index = 0;
        let card_to_play = state.players[0].hand[0];
        let play_move = SchottenTotten2Move::PlayCard {
            card: card_to_play,
            tile_index,
        };
        state.do_move(&play_move);

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
        state.wall_tiles[tile_index].wall_pattern = WallPattern::Plus;
        state.wall_tiles[tile_index].required_cards = 2;

        let play_move = SchottenTotten2Move::PlayCard {
            card: card_to_play,
            tile_index,
        };
        state.do_move(&play_move);

        // Attacker should now have 4 damaged tiles
        assert_eq!(state.attacker_damaged_tiles, 4);

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
        state.players[0].hand.clear();
        state.players[0].hand.push(chicken_card);

        let play_move = SchottenTotten2Move::PlayCard {
            card: chicken_card,
            tile_index,
        };
        state.do_move(&play_move);

        // The defender's card should be removed, and the attacker's card should not be placed
        assert!(state.wall_tiles[tile_index].defender_cards.is_empty());
        assert!(state.wall_tiles[tile_index].attacker_cards.is_empty());
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
        state.players[0].hand.clear();
        state.players[0].hand.push(chicken_card);

        let play_move = SchottenTotten2Move::PlayCard {
            card: chicken_card,
            tile_index,
        };
        state.do_move(&play_move);

        // The defender's card should be removed, and the attacker's card should not be placed
        assert!(state.wall_tiles[tile_index].defender_cards.len() == 1);
        assert!(state.wall_tiles[tile_index].attacker_cards.is_empty());
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

        let throw_move = SchottenTotten2Move::ThrowOilCauldron { tile_index };
        state.do_move(&throw_move);

        assert_eq!(state.players[1].oil_cauldrons, initial_cauldrons - 1);
        assert!(state.wall_tiles[tile_index].attacker_cards.is_empty());
        assert_eq!(state.discard_pile.len(), 1);
        assert_eq!(state.discard_pile[0], card_to_add);
    }

    #[test]
    fn test_do_move_retreat() {
        let mut state = setup_game_state();
        let tile_index = 0;
        let card_to_add = Card {
            value: 5,
            color: Color::Red,
        };
        state.wall_tiles[tile_index]
            .attacker_cards
            .push(card_to_add);
        let card_to_add = Card {
            value: 6,
            color: Color::Red,
        };
        state.wall_tiles[tile_index]
            .attacker_cards
            .push(card_to_add);

        let retreat_move = SchottenTotten2Move::Retreat { tile_index };
        state.do_move(&retreat_move);

        assert!(state.wall_tiles[tile_index].attacker_cards.is_empty());
        assert_eq!(state.discard_pile.len(), 2);
        assert_eq!(state.discard_pile[1], card_to_add);
    }

    // --- Game Over Condition Tests (check_game_over) ---

    #[test]
    fn test_check_game_over_attacker_wins_by_damaged_tiles() {
        let mut state = setup_game_state();
        state.attacker_damaged_tiles = 3;

        // Damage the 4th tile to trigger a win
        state.wall_tiles[3].is_damaged = true;
        state.attacker_damaged_tiles += 1;

        let (is_over, result, _) = state.check_game_over();
        assert!(is_over);
        assert_eq!(result, 1.0);
    }

    #[test]
    fn test_check_game_over_attacker_wins_by_damaging_twice() {
        let mut state = setup_game_state();

        // Damage tile 0 for the first time
        state.attacker_damaged_tiles = 1;

        state.wall_tiles.push(WallTile {
            id: 0,
            required_cards: 3,
            wall_pattern: WallPattern::Plus,
            damaged_required_cards: 3,
            damaged_wall_pattern: WallPattern::Run,
            is_damaged: true, // This marks the tile as "damaged twice"
            is_damaged_twice: true,
            attacker_cards: Vec::new(),
            defender_cards: Vec::new(),
        });

        let (is_over, result, _) = state.check_game_over();
        assert!(is_over);
        assert_eq!(result, 1.0);
    }

    #[test]
    fn test_check_game_over_defender_wins_deck_empty() {
        let mut state = setup_game_state();
        state.deck.clear();
        let (is_over, result, _) = state.check_game_over();
        assert!(is_over);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_check_game_over_defender_wins_no_space_to_play() {
        let mut state = setup_game_state();
        state.player_to_move_index = 1; // Defender's turn
        for tile in &mut state.wall_tiles {
            for _ in 0..tile.required_cards {
                tile.defender_cards.push(Card {
                    value: 1,
                    color: Color::Red,
                });
            }
        }
        let (is_over, result, _) = state.check_game_over();
        assert!(is_over);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_check_game_over_not_over() {
        let state = setup_game_state();
        let (is_over, _, _) = state.check_game_over();
        assert!(!is_over);
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
        for _ in 0..tile.required_cards {
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

    // --- FormationType::evaluate_formation() Tests ---

    #[test]
    fn test_evaluate_formation_less_than_two_cards() {
        let cards = vec![Card {
            value: 1,
            color: Color::Red,
        }];
        assert_eq!(FormationType::evaluate_formation(&cards), (None, 0));
    }

    #[test]
    fn test_evaluate_formation_color_run() {
        let cards = vec![
            Card {
                value: 3,
                color: Color::Red,
            },
            Card {
                value: 4,
                color: Color::Red,
            },
            Card {
                value: 5,
                color: Color::Red,
            },
        ];
        let (formation_type, sum) = FormationType::evaluate_formation(&cards);
        assert_eq!(formation_type, Some(FormationType::ColorRun));
        assert_eq!(sum, 12);
    }

    #[test]
    fn test_evaluate_formation_same_strength() {
        let cards = vec![
            Card {
                value: 7,
                color: Color::Red,
            },
            Card {
                value: 7,
                color: Color::Blue,
            },
            Card {
                value: 7,
                color: Color::Green,
            },
        ];
        let (formation_type, sum) = FormationType::evaluate_formation(&cards);
        assert_eq!(formation_type, Some(FormationType::SameStrength));
        assert_eq!(sum, 21);
    }

    #[test]
    fn test_evaluate_formation_color() {
        let cards = vec![
            Card {
                value: 2,
                color: Color::Yellow,
            },
            Card {
                value: 8,
                color: Color::Yellow,
            },
            Card {
                value: 10,
                color: Color::Yellow,
            },
        ];
        let (formation_type, sum) = FormationType::evaluate_formation(&cards);
        assert_eq!(formation_type, Some(FormationType::Color));
        assert_eq!(sum, 20);
    }

    #[test]
    fn test_evaluate_formation_run() {
        let cards = vec![
            Card {
                value: 9,
                color: Color::Green,
            },
            Card {
                value: 10,
                color: Color::Red,
            },
            Card {
                value: 11,
                color: Color::Blue,
            },
        ];
        let (formation_type, sum) = FormationType::evaluate_formation(&cards);
        assert_eq!(formation_type, Some(FormationType::Run));
        assert_eq!(sum, 30);
    }

    #[test]
    fn test_evaluate_formation_sum() {
        let cards = vec![
            Card {
                value: 1,
                color: Color::Green,
            },
            Card {
                value: 5,
                color: Color::Red,
            },
            Card {
                value: 9,
                color: Color::Blue,
            },
        ];
        let (formation_type, sum) = FormationType::evaluate_formation(&cards);
        assert_eq!(formation_type, Some(FormationType::Sum));
        assert_eq!(sum, 15);
    }

    #[test]
    fn test_evaluate_formation_unshuffled_order_does_not_matter() {
        // Test that the order of cards doesn't change the outcome for run or color run.
        let cards = vec![
            Card {
                value: 5,
                color: Color::Red,
            },
            Card {
                value: 3,
                color: Color::Red,
            },
            Card {
                value: 4,
                color: Color::Red,
            },
        ];
        let (formation_type, sum) = FormationType::evaluate_formation(&cards);
        assert_eq!(formation_type, Some(FormationType::ColorRun));
        assert_eq!(sum, 12);
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
                color: Color::Red,
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

        state.wall_tiles[tile_index].wall_pattern = WallPattern::Plus;
        state.wall_tiles[tile_index].required_cards = 3;

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
                color: Color::Blue,
            },
        ];

        state.wall_tiles[tile_index].wall_pattern = WallPattern::Plus;
        state.wall_tiles[tile_index].required_cards = 3;

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

        state.wall_tiles[tile_index].wall_pattern = WallPattern::Plus;
        state.wall_tiles[tile_index].required_cards = 3;

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

        state.wall_tiles[tile_index].wall_pattern = WallPattern::Run;
        state.wall_tiles[tile_index].required_cards = 3;

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

        // Defender has a color run (stronger than a run)
        state.wall_tiles[tile_index].defender_cards = vec![
            Card {
                value: 8,
                color: Color::Red,
            },
            Card {
                value: 9,
                color: Color::Red,
            },
            Card {
                value: 10,
                color: Color::Red,
            },
        ];

        state.wall_tiles[tile_index].wall_pattern = WallPattern::Run;
        state.wall_tiles[tile_index].required_cards = 3;

        let result = state.check_attacker_control(tile_index);
        assert!(!result);
    }
}
