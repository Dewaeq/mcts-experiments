use std::time::Instant;

use crate::{
    game::{Game, GameState},
    search_state::SearchState,
    tree::SearchTree,
};

#[derive(Clone, Copy)]
pub enum Player {
    Human,
    Ai(u128),
}

impl Player {
    pub fn get_move(self, game: &Game) -> usize {
        match self {
            Player::Human => HumanPlayer::get_move(game),
            Player::Ai(search_time) => AiPlayer::get_move(game, search_time),
        }
    }
}

struct HumanPlayer;
struct AiPlayer;

impl HumanPlayer {
    fn get_move(game: &Game) -> usize {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();

        let col = buffer.trim_end().parse::<usize>().unwrap();

        if !(0..game.shape().1).contains(&col) || game.is_filled_col(col) {
            println!("invalid column index!");
            HumanPlayer::get_move(game)
        } else {
            col
        }
    }
}

impl AiPlayer {
    fn get_move(game: &Game, search_time: u128) -> usize {
        let mut tree = SearchTree::new();
        let root_state = SearchState::new(*game);

        let root_id = tree.add(root_state, None);
        let mut iterations = 0;

        let timer = Instant::now();
        loop {
            if iterations % 2048 == 0 && timer.elapsed().as_millis() >= search_time {
                break;
            }

            // selection
            let selected_id = AiPlayer::select(root_id, &tree);

            // expansion
            let child_id = AiPlayer::expand(selected_id, &mut tree);

            // simulation
            let reward = AiPlayer::simulate(child_id, &tree);

            // backpropagation
            AiPlayer::backpropagate(reward, Some(child_id), &mut tree);
            iterations += 1;
        }

        for state in tree.get_child_states(root_id) {
            println!(
                "{}\t{}\t{}",
                state.game.last_move(),
                state.num_simulations,
                state.mean_score()
            );
        }

        let (best_move, mean_score) = tree.best_move(root_id);
        println!("ran {iterations} simulations, mean: {mean_score}");

        best_move
    }

    fn select(mut node_id: usize, tree: &SearchTree) -> usize {
        while tree.is_fully_expanded(node_id) && !tree.is_terminal(node_id) {
            node_id = tree.uct_child(node_id, 1.);
        }

        node_id
    }

    fn expand(node_id: usize, tree: &mut SearchTree) -> usize {
        tree.add_children(node_id);
        tree.random_child(node_id)
    }

    fn simulate(node_id: usize, tree: &SearchTree) -> f32 {
        let mut game = tree.get_game(node_id);
        let mut game_state = game.get_state();
        let me = 1 - game.turn();

        while matches!(game_state, GameState::Playing) {
            game.do_move(AiPlayer::random_action(&game));
            game_state = game.get_state();
        }

        match game_state {
            GameState::Draw => 0.5,
            GameState::Win(winner) if winner == me => 1.,
            GameState::Win(winner) if winner != me => 0.,
            _ => panic!(),
        }
    }

    fn random_action(game: &Game) -> usize {
        let mut col = fastrand::usize(0..game.shape().1);

        while game.is_filled_col(col) {
            col = fastrand::usize(0..game.shape().1);
        }

        col
    }

    fn backpropagate(mut reward: f32, mut node_id: Option<usize>, tree: &mut SearchTree) {
        while let Some(id) = node_id {
            node_id = tree.get_parent_id(id);
            tree.update_state(id, reward);

            reward = 1. - reward;
        }
    }
}
