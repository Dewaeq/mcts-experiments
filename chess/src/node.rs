use crate::state::State;

/// Nodes contain the required data to create a tree,
/// as well as parameters used by the MCTS algorithm
pub struct Node<T>
where
    T: State,
{
    pub state: T,
    pub id: usize,
    pub parent_id: Option<usize>,
    pub child_ids: Vec<usize>,
    pub actions: Vec<T::Action>,

    num_sims: usize,
    score: f32,
}

impl<T> Node<T>
where
    T: State,
{
    pub fn new(state: T, id: usize, parent_id: Option<usize>) -> Self {
        let actions = state.possible_actions();

        Node {
            state,
            id,
            parent_id,
            child_ids: vec![],
            actions,
            num_sims: 0,
            score: 0.,
        }
    }

    pub fn is_explored(&self) -> bool {
        self.num_sims != 0
    }

    pub fn uct_score(&self) -> f32 {
        todo!()
    }
}
