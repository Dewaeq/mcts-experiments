use crate::node::Node;
use crate::state::State;
use fastrand::Rng;

#[derive(Default)]
pub struct Tree<T>
where
    T: State,
{
    pub nodes: Vec<Node<T>>,
    index: usize,

    rng: Rng,
}

impl<T> Tree<T>
where
    T: State,
{
    pub fn size(&self) -> usize {
        self.nodes.len()
    }

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

    pub fn select_child(&mut self, mut node_id: usize) -> usize {
        while self.is_fully_expanded(node_id) && !self.is_terminal(node_id) {
            node_id = self.uct_select_child(node_id).unwrap();
        }

        node_id
    }

    fn uct_select_child(&self, node_id: usize) -> Option<usize> {
        let n = self.nodes[node_id].num_sims();

        self.nodes[node_id]
            .child_ids
            .iter()
            .max_by(|&&x, &&y| {
                self.nodes[x]
                    .uct_score(n)
                    .partial_cmp(&self.nodes[y].uct_score(n))
                    .unwrap()
            })
            .cloned()
    }

    /// Will panic if this node does not have any possible actions left,
    /// i.e. all it's children (possibly none) have already been added
    /// to the tree
    pub fn expand(&mut self, node_id: usize) -> usize {
        if self.nodes[node_id].actions.is_empty() {
            return node_id;
        }

        let action = self.nodes[node_id].actions.pop().unwrap();
        let new_state = self.nodes[node_id].state.apply_action(action);

        self.add_state(new_state, Some(node_id))
    }

    pub fn get_parent_id(&self, node_id: usize) -> Option<usize> {
        self.nodes[node_id].parent_id
    }

    pub fn get_state(&self, node_id: usize) -> T
    where
        T: Clone,
    {
        self.nodes[node_id].state.clone()
    }

    pub fn get_state_ref(&self, node_id: usize) -> &T {
        &self.nodes[node_id].state
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

    fn most_visited_child(&self, node_id: usize) -> Option<usize> {
        self.nodes[node_id]
            .child_ids
            .iter()
            .max_by_key(|&&x| self.nodes[x].num_sims())
            .cloned()
    }

    pub fn random_child(&mut self, node_id: usize) -> usize {
        *self
            .rng
            .choice(self.nodes[node_id].child_ids.iter())
            .unwrap_or(&node_id)
    }

    pub fn is_fully_expanded(&self, node_id: usize) -> bool {
        self.nodes[node_id].actions.is_empty()
        //self.nodes[node_id]
        //    .child_ids
        //    .iter()
        //    .all(|&child_id| self.nodes[child_id].is_explored())
    }

    pub fn is_terminal(&mut self, node_id: usize) -> bool {
        self.nodes[node_id].state.is_terminal()
    }

    pub fn update_node(&mut self, node_id: usize, reward: f32) {
        self.nodes[node_id].update(reward);
    }

    pub fn best_action(&self, node_id: usize) -> T::Action {
        let child_id = self.most_visited_child(node_id).unwrap();

        self.nodes[child_id].state.last_action().unwrap()
    }

    pub fn continuation(&self, mut node_id: usize) -> Vec<T::Action> {
        let mut actions = vec![];

        while let Some(id) = self.most_visited_child(node_id) {
            actions.push(self.get_state_ref(id).last_action().unwrap());
            node_id = id;
        }

        actions
    }
}
