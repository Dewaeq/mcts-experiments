use shakmaty::{Chess, Move, Outcome, Position};

use crate::state::State;

#[derive(Default, Clone)]
pub struct Game {
    pub pos: Chess,
    pub last_action: Option<Move>,
    cached_is_terminal: Option<bool>,
}

impl Game {
    pub fn new(pos: Chess) -> Self {
        let is_terminal = pos.is_game_over();

        Game {
            pos,
            last_action: None,
            cached_is_terminal: Some(is_terminal),
        }
    }
}

impl State for Game {
    type Action = Move;

    fn possible_actions(&self) -> Vec<Self::Action> {
        let mut actions = self.pos.legal_moves().to_vec();
        fastrand::shuffle(&mut actions);

        actions
    }

    fn apply_action(&self, action: Self::Action) -> Self {
        let mut pos = self.pos.clone();
        pos.play_unchecked(&action);

        let mut game = Game::new(pos);
        game.last_action = Some(action);

        game
    }

    fn last_action(&self) -> Option<Self::Action> {
        self.last_action.clone()
    }

    fn reward(&self, perspective: &Self) -> f32 {
        match self.pos.outcome() {
            Some(Outcome::Draw) => 0.5,
            Some(Outcome::Decisive { winner }) => {
                if winner == perspective.pos.turn().other() {
                    1.
                } else {
                    0.
                }
            }
            // treat too long playouts as a loss
            _ => 0.,
        }
    }

    fn is_terminal(&mut self, depth: usize) -> bool {
        self.cached_is_terminal.unwrap() || depth >= 30
    }
}
