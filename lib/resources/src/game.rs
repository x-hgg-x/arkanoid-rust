pub enum GameEvent {
    GameOver,
    LevelComplete,
}

pub struct Game {
    pub lives: i32,
    pub score: i32,
    pub event: Option<GameEvent>,
}

pub const NUM_LIVES: i32 = 5;

impl Default for Game {
    fn default() -> Self {
        Self { lives: NUM_LIVES, score: 0, event: None }
    }
}
