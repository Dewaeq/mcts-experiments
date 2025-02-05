use crate::game::Game;

#[derive(Clone, Copy)]
pub struct SearchState {
    pub score: f32,
    pub num_simulations: usize,
    pub game: Game,
}

impl SearchState {
    pub fn new(game: Game) -> Self {
        SearchState {
            score: 0.,
            num_simulations: 0,
            game,
        }
    }

    pub fn mean_score(&self) -> f32 {
        self.score / self.num_simulations as f32
    }

    pub fn uct_score(&self, parent_sims: usize, c: f32) -> f32 {
        self.mean_score()
            + c * (2. * (parent_sims as f32).ln() / self.num_simulations as f32).sqrt()
    }
}
