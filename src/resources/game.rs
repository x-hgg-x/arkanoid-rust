pub enum GameEvent {
    GameOver,
    LevelComplete,
}

pub struct Game {
    pub lifes: i32,
    pub score: u32,
    pub event: Option<GameEvent>,
}

pub const NUM_LIFES: i32 = 5;

impl Default for Game {
    fn default() -> Self {
        Self {
            lifes: NUM_LIFES,
            score: 0,
            event: None,
        }
    }
}
