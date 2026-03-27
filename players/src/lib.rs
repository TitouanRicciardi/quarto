pub mod bot_human;
pub mod minimax;
pub mod minimax_alpha_beta;
pub mod random;

use quarto_core::{Game, Piece};

/// A trait that defines what is needed to play Quarto.
pub trait Player {
    /// Given a game state, returns what piece to give your opponent.
    fn give_piece(&mut self, game: &Game) -> Piece;

    /// Given a game state and a piece, returns the coordinates of where to put the piece.
    fn play_piece(&mut self, game: &Game, given_piece: Piece) -> (usize, usize);
}
