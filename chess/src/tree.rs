use crate::node::{self, Node};
use crate::state::State;
use fastrand::Rng;

#[derive(Default)]
pub struct Tree<T>
where
    T: State,
{
    nodes: Vec<Node<T>>,
    index: usize,

    rng: Rng,
}

impl<T> Tree<T>
where
    T: State,
{
    pub fn add_state(&mut self, state: T, parent_id: Option<usize>) -> usize {
        let id = self.index;
        let node = Node::new(state, id, parent_id);

        if let Some(parent) = parent_id {
            self.nodes[parent].child_ids.push(id);
        }

        self.nodes.push(node);
        self.index += 1;

        id
    }

    pub fn select(&mut self, mut node_id: usize) -> usize {
        while self.is_fully_expanded(node_id) && !self.is_terminal(node_id) {
            node_id = self.uct_select_child(node_id);
        }

        node_id
    }

    fn uct_select_child(&self, node_id: usize) -> usize {
        *self.nodes[node_id]
            .child_ids
            .iter()
            .max_by(|&&x, &&y| {
                self.nodes[x]
                    .uct_score()
                    .partial_cmp(&self.nodes[y].uct_score())
                    .unwrap()
            })
            .unwrap()
    }

    /// Will panic if this node does not have any possible actions left,
    /// i.e. all it's children (possibly none) have already been added
    /// to the tree
    pub fn expand(&mut self, node_id: usize) -> usize {
        let action = self.nodes[node_id].actions.pop().unwrap();
        let new_state = self.nodes[node_id].state.apply_action(action);

        self.add_state(new_state, Some(node_id))
    }

    pub fn get_state(&self, node_id: usize) -> T
    where
        T: Clone,
    {
        self.nodes[node_id].state.clone()
    }

    pub fn get_child_states(&self, node_id: usize) -> impl Iterator<Item = T> + '_
    where
        T: Copy,
    {
        self.nodes[node_id]
            .child_ids
            .iter()
            .map(|&child_id| self.nodes[child_id].state)
    }

    pub fn random_child(&mut self, node_id: usize) -> usize {
        *self
            .rng
            .choice(self.nodes[node_id].child_ids.iter())
            .unwrap_or(&node_id)
    }

    pub fn is_fully_expanded(&self, node_id: usize) -> bool {
        self.nodes[node_id]
            .child_ids
            .iter()
            .all(|&child_id| self.nodes[child_id].is_explored())
    }

    pub fn is_terminal(&self, node_id: usize) -> bool {
        self.nodes[node_id].state.is_terminal()
    }
}
