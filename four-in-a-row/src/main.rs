#![allow(dead_code)]

use game::Game;
use player::Player;

pub mod game;
pub mod player;
pub mod search_state;
pub mod tree;

fn main() {
    let mut game = Game::new([Player::Human, Player::Ai]);
    game.main_loop();
}
