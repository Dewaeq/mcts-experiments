/// State's should contain all game logic/information
pub trait State {
    type Action;

    /// An already shuffled list of possible actions from
    /// this state
    fn possible_actions(&self) -> Vec<Self::Action>;

    fn apply_action(&self, action: Self::Action) -> Self;

    fn last_action(&self) -> Option<Self::Action>;

    /// Get the reward for reaching this state,
    /// from the perspective of another state
    fn reward(&self, perspective: &Self) -> f32;

    fn is_terminal(&self) -> bool;
}
