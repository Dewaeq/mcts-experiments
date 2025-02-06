/// State's should contain all game logic/information
pub trait State {
    type Action;

    fn possible_actions(&self) -> Vec<Self::Action>;

    fn apply_action(&self, action: Self::Action) -> Self;

    fn reward(&self) -> f32;

    fn is_terminal(&self) -> bool;
}
