#[derive(PartialEq)]
pub enum CurrentState {
    Paused,
    Running,
}

impl Default for CurrentState {
    fn default() -> Self {
        CurrentState::Paused
    }
}
