use crate::{game::Game, search_state::SearchState};

#[derive(Default)]
pub struct SearchTree {
    pub states: Vec<SearchState>,
    pub nodes: Vec<Node>,
    pub current_id: usize,
}

impl SearchTree {
    pub fn new() -> Self {
        SearchTree {
            states: vec![],
            nodes: vec![],
            current_id: 0,
        }
    }

    pub fn add(&mut self, state: SearchState, parent_id: Option<usize>) -> usize {
        let id = self.current_id;
        let node = Node {
            id,
            parent_id,
            child_ids: vec![],
        };

        if let Some(parent_id) = parent_id {
            self.nodes[parent_id].child_ids.push(id);
        }

        self.nodes.push(node);
        self.states.push(state);
        self.current_id += 1;

        id
    }

    /// Generate all the children of this state and add them
    /// to the tree
    pub fn add_children(&mut self, node_id: usize) {
        if !self.nodes[node_id].child_ids.is_empty() {
            return;
        }

        if self.states[node_id].game.is_terminal() {
            return;
        }

        let mut child_states = vec![];
        let game = self.states[node_id].game;
        let (_, ncols) = game.shape();

        for col in 0..ncols {
            if !game.is_filled_col(col) {
                let mut new_game = game;
                new_game.do_move(col);
                child_states.push(SearchState::new(new_game));
            }
        }

        for child_state in child_states {
            self.add(child_state, Some(node_id));
        }
    }

    pub fn is_fully_expanded(&self, node_id: usize) -> bool {
        if self.nodes[node_id].child_ids.is_empty() {
            return false;
        }

        self.get_child_states(node_id)
            .all(|state| state.num_simulations != 0)
    }

    pub fn is_terminal(&self, node_id: usize) -> bool {
        self.states[node_id].game.is_terminal()
    }

    pub fn get_child_states(&self, node_id: usize) -> impl Iterator<Item = SearchState> + use<'_> {
        let child_ids = &self.nodes[node_id].child_ids;

        child_ids.iter().map(|&id| self.states[id])
    }

    pub fn random_child(&self, node_id: usize) -> usize {
        *fastrand::choice(self.nodes[node_id].child_ids.iter()).unwrap_or(&node_id)
    }

    pub fn get_parent_id(&self, node_id: usize) -> Option<usize> {
        self.nodes[node_id].parent_id
    }

    pub fn get_game(&self, node_id: usize) -> Game {
        self.states[node_id].game
    }

    pub fn update_state(&mut self, node_id: usize, result: f32) {
        self.states[node_id].num_simulations += 1;
        self.states[node_id].score += result;
    }

    pub fn best_move(&self, node_id: usize) -> (usize, f32) {
        let best_state = self
            .get_child_states(node_id)
            .max_by(|x, y| x.mean_score().partial_cmp(&y.mean_score()).unwrap())
            .unwrap();

        (best_state.game.last_move(), best_state.mean_score())
    }
}

#[derive(Debug)]
pub struct Node {
    id: usize,
    parent_id: Option<usize>,
    child_ids: Vec<usize>,
}
