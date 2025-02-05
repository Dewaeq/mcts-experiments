#![allow(dead_code)]

use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    },
    thread,
};

use game::{Game, GameState};
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

    let uct_score = Arc::new(AtomicUsize::new(0));
    let non_uct_score = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];

    for _ in 0..8 {
        let uct_score = uct_score.clone();
        let non_uct_score = non_uct_score.clone();

        let handle = thread::spawn(move || {
            for i in 0..20 {
                let uct_player = fastrand::usize(0..=1);
                let mut players = [
                    Player::Ai(false, search_time),
                    Player::Ai(false, search_time),
                ];

                players[uct_player] = Player::Ai(true, search_time);
                let mut game = Game::new(players);
                game.main_loop(false);

                if let GameState::Win(winner) = game.get_state() {
                    if winner == uct_player {
                        uct_score.fetch_add(1, Ordering::Relaxed);
                    } else {
                        non_uct_score.fetch_add(1, Ordering::Relaxed);
                    }

                    println!("uct won game {i}: {}", winner == uct_player as usize);
                }
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("score:");
    println!("uct={:?}\tnon-uct={:?}", uct_score, non_uct_score);

    return;

    let mut players = [Player::Human, Player::Ai(true, search_time)];

    for (player, c) in players.iter_mut().zip(args[1].chars()) {
        *player = if c == 'a' {
            Player::Ai(true, search_time)
        } else {
            Player::Human
        };
    }

    let mut game = Game::new(players);
    game.main_loop(true);
}
