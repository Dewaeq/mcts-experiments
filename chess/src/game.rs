use std::num::NonZero;

use shakmaty::{Chess, Move, Outcome, Position};

use crate::state::State;

#[derive(Default, Clone)]
pub struct Game {
    pub pos: Chess,
    pub last_action: Option<Move>,
}

impl Game {
    pub fn new(pos: Chess) -> Self {
        Game {
            pos,
            last_action: None,
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
        Game {
            pos: {
                let mut pos = self.pos.clone();
                pos.play_unchecked(&action);
                pos
            },
            last_action: Some(action),
        }
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
            // treat too long playouts as draws
            _ => 0.5,
        }
    }

    fn is_terminal(&self) -> bool {
        //self.pos.is_game_over()
        self.pos.fullmoves() > NonZero::new(200).unwrap() || self.pos.is_game_over()
    }
}
