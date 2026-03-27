use quarto_core::{Board, Game, Piece, Stack};
use quarto_players::{Player, bot_human::Human, minimax::MinimaxBot, minimax_alpha_beta::MinimaxAlphaBetaPlayer, random::RandomBot};

const PLAYER_COUNT: usize = 2;

pub enum Outcome {
    Win(usize),
    Draw,
    Illegal(usize),
}

/// Play a single move. Returns the game end state (if any).
pub fn game_iter(
    game: &mut Game,
    giving_player_id: usize,
    players: &mut [Box<dyn Player>; PLAYER_COUNT],
) -> Option<Outcome> {
    // No pieces left to give
    if game.stack.0 == 0 {
        return Some(Outcome::Draw);
    }

    let giving_player = players.get_mut(giving_player_id).unwrap();
    let given_piece = giving_player.give_piece(game);

    // Piece has already been played
    if !game.stack.pick(given_piece) {
        return Some(Outcome::Illegal(giving_player_id));
    }

    let placing_player_id = (giving_player_id + 1) % PLAYER_COUNT;
    let placing_player = players.get_mut(placing_player_id).unwrap();
    let (placed_piece_x, placed_piece_y) = placing_player.play_piece(game, given_piece);

    // Coordinates weren't empty
    if game
        .board
        .get_piece(placed_piece_x, placed_piece_y)
        .is_some()
    {
        return Some(Outcome::Illegal(placing_player_id));
    }

    game.board
        .set_piece(placed_piece_x, placed_piece_y, Some(given_piece));

    if game.board.is_win(placed_piece_x, placed_piece_y) {
        return Some(Outcome::Win(placing_player_id));
    }

    None
}

/// Play an entire game with the given players.
pub fn game_loop(players: &mut [Box<dyn Player>; 2]) -> Outcome {
    let player_count = players.len();

    let mut game = Game {
        board: Board::new(),
        stack: Stack::new(),
    };

    loop {
        for i in 0..player_count {
            if let Some(outcome) = game_iter(&mut game, i, players) {
                return outcome;
            }
        }
    }
}

pub fn main() {
    println!("hello, world!");

    for piece in 0..16 {
        let piece = Piece(piece as u8);
        println!(
            "{}: bright = {}, square = {}, tall = {}, hollow = {}",
            piece,
            piece.is_bright(),
            piece.is_square(),
            piece.is_tall(),
            piece.is_hollow()
        );
    }

    let p1 = Human::new("Human".to_string());
    let p2 = MinimaxAlphaBetaPlayer{};

    let mut players: [Box<dyn Player>; 2] = [Box::new(p1), Box::new(p2)];

    let mut game = Game {
        board: Board::new(),
        stack: Stack::new(),
    };

    let outcome = 'outer: loop {
        for i in 0..PLAYER_COUNT {
            println!("{}", game.board);
            println!("player {i}'s pick");
            if let Some(outcome) = game_iter(&mut game, i, &mut players) {
                break 'outer outcome;
            }
        }
    };

    match outcome {
        Outcome::Win(i) => println!("player {i} won"),
        Outcome::Draw => println!("draw"),
        Outcome::Illegal(i) => println!("player {i} attempted an illegal move"),
    }

    println!("{}", game.board);
}
