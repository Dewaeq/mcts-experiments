use game::Game;
use mcts::Mcts;
use shakmaty::{fen::Fen, CastlingMode, Chess};

pub mod game;
pub mod mcts;
pub mod node;
pub mod state;
pub mod tree;

fn main() {
    let mut args = std::env::args();
    args.next();

    let search_time = args.next().unwrap().parse::<u128>().unwrap();
    let fen = args.next().unwrap();
    let pos: Chess = fen
        .parse::<Fen>()
        .unwrap()
        .into_position(CastlingMode::Standard)
        .unwrap();
    let game = Game::new(pos);

    let mut searcher: Mcts<Game> = Mcts::default();
    let m = searcher.search(search_time * 1000, game);
    println!("{m:?}");
}
