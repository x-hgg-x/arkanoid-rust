pub enum GameEvent {
    GameOver,
    LevelComplete,
}

pub struct Game {
    pub lifes: i32,
    pub score: u32,
    pub event: Option<GameEvent>,
}

impl Default for Game {
    fn default() -> Self {
        Self { lifes: 5, score: 0, event: None }
    }
}
