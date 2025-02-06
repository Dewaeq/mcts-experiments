use crate::{state::State, tree::Tree};

pub struct Mcts<T>
where
    T: State,
{
    _phantom: std::marker::PhantomData<T>,
}

impl<T> Mcts<T>
where
    T: State,
{
    pub fn simulate(&self, node_id: usize, tree: Tree<T>) -> f32
    where
        T: Clone,
    {
        let mut state = tree.get_state(node_id);

        while !state.is_terminal() {
            let action = fastrand::choice(state.possible_actions()).unwrap();
            state = state.apply_action(action);
        }

        state.reward()
    }
}
