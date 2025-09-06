/// A trait representing the state of a game.
/// Parameterized over `M`, the move type (e.g., Card, (row,col), etc.).
use std::fmt::Display;

pub trait GameState<M: Clone + PartialEq + Display>: Clone + Display {
    /// The current player to move.
    fn player_to_move(&self) -> usize;

    /// Returns the next player.
    /// Player starts with 0.
    fn get_next_player(&self, player: usize) -> usize;

    /// Creates a deep clone of this game state.
    fn clone_state(&self) -> impl GameState<M>;

    /// Creates a deep clone of this game state, randomizing hidden info
    /// from the viewpoint of the player.
    fn clone_and_randomize(&self, player: usize) -> impl GameState<M>;

    /// Updates the state by carrying out the given move.
    fn do_move(&mut self, m: &M);

    /// Returns all possible moves from this state.
    fn get_moves(&self) -> Vec<M>;

    /// Gets the game result from the viewpoint of a player.
    fn get_result(&self, player: usize) -> Option<f64>;

    /// Returns for the number of players in the game.
    fn number_of_players(&self) -> usize;
}