use std::time::Instant;

use crate::{
    game::{Game, GameState},
    search_state::SearchState,
    tree::SearchTree,
};

#[derive(Clone, Copy)]
pub enum Player {
    Human,
    Ai,
}

impl Player {
    pub fn get_move(self, game: &Game) -> usize {
        match self {
            Player::Human => HumanPlayer::get_move(game),
            Player::Ai => AiPlayer::get_move(game),
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
    fn get_move(game: &Game) -> usize {
        let mut tree = SearchTree::new();
        let root_state = SearchState::new(*game);

        let root_id = tree.add(root_state, None);
        let mut i = 0;

        let timer = Instant::now();
        while timer.elapsed().as_millis() < 1000 {
            // selection
            let node_id = AiPlayer::select(root_id, &tree);

            // expansion
            let node_id = AiPlayer::expand(node_id, &mut tree);

            // simulation
            let result = AiPlayer::simulate(node_id, &tree);

            // backpropagation
            AiPlayer::backpropagate(result, Some(node_id), &mut tree);
            i += 1;
        }

        let (best_move, mean_score) = tree.best_move(root_id);
        println!("ran {i} simulations, mean: {mean_score}");

        best_move
    }

    fn select(mut node_id: usize, tree: &SearchTree) -> usize {
        while tree.is_fully_expanded(node_id) && !tree.is_terminal(node_id) {
            node_id = tree.random_child(node_id);
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

        while matches!(game_state, GameState::Playing) {
            game.do_move(AiPlayer::random_action(&game));
            game_state = game.get_state();
        }

        match game_state {
            GameState::Draw => 0.5,
            GameState::Win(player) => player as f32,
            GameState::Playing => panic!(),
        }
    }

    fn random_action(game: &Game) -> usize {
        let mut col = fastrand::usize(0..game.shape().1);

        while game.is_filled_col(col) {
            col = fastrand::usize(0..game.shape().1);
        }

        col
    }

    fn backpropagate(result: f32, mut node_id: Option<usize>, tree: &mut SearchTree) {
        while let Some(id) = node_id {
            tree.update_state(id, result);
            node_id = tree.get_parent_id(id);
        }
    }
}
