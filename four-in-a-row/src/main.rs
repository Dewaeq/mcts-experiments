#![allow(dead_code)]

use game::Game;
use player::Player;

pub mod game;
pub mod player;
pub mod search_state;
pub mod tree;

fn main() {
    let mut args = std::env::args().collect::<Vec<_>>();
    if args.len() == 1 {
        args.push("ha".to_owned());
        args.push("1000".to_owned());
    } else if args.len() < 3 {
        args.push("1000".to_owned());
    }

    let search_time = args[2].parse::<u128>().unwrap();

    let mut players = [Player::Human, Player::Ai(search_time)];

    for (player, c) in players.iter_mut().zip(args[1].chars()) {
        *player = if c == 'a' {
            Player::Ai(search_time)
        } else {
            Player::Human
        };
    }

    let mut game = Game::new(players);
    game.main_loop();
}
