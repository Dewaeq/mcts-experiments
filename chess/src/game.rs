use shakmaty::{Chess, Move, Outcome, Position};

use crate::state::State;

pub struct Game {
    pub pos: Chess,
}

impl Game {
    pub fn new(_players: [usize; 2]) -> Self {
        Game { pos: Chess::new() }
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
            pos: self.pos.clone().play(&action).unwrap(),
        }
    }

    fn reward(&self) -> f32 {
        match self.pos.outcome() {
            Some(Outcome::Draw) => 0.,
            Some(Outcome::Decisive { winner: _ }) => todo!(),
            _ => panic!(),
        }
    }

    fn is_terminal(&self) -> bool {
        self.pos.is_game_over()
    }
}
