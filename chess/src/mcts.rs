use std::{fmt::Display, time::Instant};

use crate::{state::State, tree::Tree};

#[derive(Default)]
pub struct Mcts<T>
where
    T: State + Clone,
{
    tree: Tree<T>,
}

impl<T> Mcts<T>
where
    T: State + Clone,
    T::Action: Display,
{
    pub fn search(&mut self, search_time: u128, root_state: T) -> T::Action {
        let started = Instant::now();
        let mut iterations = 0;

        let root_id = self.tree.add_state(root_state, None);

        loop {
            if iterations % 2048 == 0 && started.elapsed().as_millis() >= search_time {
                break;
            }

            let node_id = self.tree.select_child(root_id);
            let child_id = self.tree.expand(node_id);
            let reward = self.simulate(child_id);
            self.backpropagate(child_id, reward);

            iterations += 1;
        }

        let n = self.tree.nodes[root_id].num_sims();
        for &child_id in &self.tree.nodes[root_id].child_ids {
            let node = &self.tree.nodes[child_id];
            let state = self.tree.get_state_ref(child_id);
            println!(
                "{}:\t{}\t{}\t{}",
                state.last_action().unwrap(),
                node.num_sims(),
                node.mean_score(),
                node.uct_score(n)
            );
        }

        println!("{} its/sec", iterations as f32 / search_time as f32 * 1000.);
        println!("tree size: {}", self.tree.size());
        println!("continuation: ");
        for action in self.tree.continuation(root_id) {
            print!("{action}, ");
        }
        println!();

        self.tree.best_action(root_id)
    }

    pub fn simulate(&self, node_id: usize) -> f32 {
        let mut state = self.tree.get_state(node_id);
        let mut depth = 0;

        while !state.is_terminal(depth) {
            let action = state.possible_actions().pop().unwrap();
            state = state.apply_action(action);
            depth += 1;
        }

        state.reward(self.tree.get_state_ref(node_id))
    }

    pub fn backpropagate(&mut self, node_id: usize, mut reward: f32) {
        let mut node_id = Some(node_id);

        while let Some(id) = node_id {
            self.tree.update_node(id, reward);
            node_id = self.tree.get_parent_id(id);

            reward = 1. - reward;
        }
    }
}
