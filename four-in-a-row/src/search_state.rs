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
}
