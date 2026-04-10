use quarto_core::{Board, Game, Piece, Stack};
use quarto_players::{
    Player,
    // minimax::MinimaxBot,
    minimax_alpha_beta::MinimaxAlphaBetaPlayer,
    random::RandomBot
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct QuartoEngine {
    game: Game,
    players: [Box<dyn Player>; 2],
    current_player: u8,
    winner: Option<u8>,
}

#[wasm_bindgen]
impl QuartoEngine {
    #[wasm_bindgen(constructor)]
    pub fn new() -> QuartoEngine {
        QuartoEngine {
            game: Game {
                board: Board::new(),
                stack: Stack::new(),
            },
            players: [
                Box::new(RandomBot::default()),
                Box::new(MinimaxAlphaBetaPlayer {}),
            ],
            current_player: 0,
            winner: None,
        }
    }

    pub fn get_board(&self) -> Vec<u8> {
        let mut flat = Vec::with_capacity(16);
        for y in 0..4 {
            for x in 0..4 {
                flat.push(match self.game.board.get_piece(x, y) {
                    Some(p) => p.0,
                    None => 255,
                });
            }
        }
        flat
    }

    pub fn get_stack(&self) -> u16 {
        self.game.stack.0
    }

    pub fn place_piece(&mut self, x: usize, y: usize, piece_id: u8) -> bool {
        if self.winner.is_some() {
            return false;
        }

        if self.game.board.get_piece(x, y).is_some() {
            return false;
        }

        self.game.board.set_piece(x, y, Some(Piece(piece_id)));

        if self.game.board.is_win(x, y) {
            self.winner = Some(self.current_player);
        }

        true
    }

    pub fn pick_piece(&mut self, piece_id: u8) -> bool {
        if self.winner.is_some() {
            return false;
        }

        if self.game.stack.pick(Piece(piece_id)) {
            self.current_player = (self.current_player + 1) % 2;
            return true;
        }
        false
    }

    pub fn give_piece(&mut self) -> u8 {
        let player = match self.players.get_mut(self.current_player as usize) {
            Some(player) => player,
            None => unreachable!(),
        };

        player.give_piece(&self.game).0
    }

    pub fn play_piece(&mut self, piece: u8) -> Vec<u8> {
        let mut res = vec![0, 0];

        let player = match self.players.get_mut(self.current_player as usize) {
            Some(player) => player,
            None => unreachable!(),
        };

        let (x, y) = player.play_piece(&self.game, Piece(piece));
        res[0] = x as u8;
        res[1] = y as u8;
        res
    }

    pub fn get_current_player(&self) -> u8 {
        self.current_player
    }

    pub fn get_winner(&self) -> Option<u8> {
        self.winner
    }
}

impl Default for QuartoEngine {
    fn default() -> Self {
        Self::new()
    }
}
