use crate::{GameplayState, MainMenuState, Menu};

use components::PrefabHandles;
use resources::SCORE_TEXT_ID;

use amethyst::{
    ecs::{Entity, WriteStorage},
    prelude::*,
    ui::{UiFinder, UiText},
};

pub struct GameOverState {
    game_over_menu: Option<Entity>,
    selection: i32,
    score: i32,
    score_updated: bool,
}

impl GameOverState {
    pub fn new(score: i32) -> Self {
        Self {
            game_over_menu: None,
            selection: 0,
            score,
            score_updated: false,
        }
    }
}

impl Menu for GameOverState {
    fn get_selection(&self) -> i32 {
        self.selection
    }

    fn set_selection(&mut self, selection: i32) {
        self.selection = selection;
    }

    fn confirm_selection(&self) -> SimpleTrans {
        match self.selection {
            // Restart
            0 => Trans::Switch(Box::new(GameplayState::default())),
            // Main Menu
            1 => Trans::Switch(Box::new(MainMenuState::default())),
            // Exit
            2 => Trans::Quit,
            _ => unreachable!(),
        }
    }

    fn get_menu_ids(&self) -> &[&str] {
        &["restart", "main_menu", "exit"]
    }

    fn get_cursor_menu_ids(&self) -> &[&str] {
        &["cursor_restart", "cursor_main_menu", "cursor_exit"]
    }
}

impl SimpleState for GameOverState {
    fn on_start(&mut self, data: StateData<GameData>) {
        let world = data.world;

        let game_over_menu = world.read_resource::<PrefabHandles>().menu.game_over_menu.clone();
        self.game_over_menu = Some(world.create_entity().with(game_over_menu).build());
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        if let Some(entity) = self.game_over_menu {
            data.world.delete_entity(entity).expect("Failed to delete entity.");
        }
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        if !self.score_updated {
            data.world.exec(|(ui_finder, mut ui_texts): (UiFinder, WriteStorage<UiText>)| {
                if let Some(ui_text) = ui_finder.find(SCORE_TEXT_ID).and_then(|entity| ui_texts.get_mut(entity)) {
                    ui_text.text = format!("SCORE: {}", self.score);
                    self.score_updated = true;
                }
            });
        }
        Trans::None
    }

    fn handle_event(&mut self, data: StateData<GameData>, event: StateEvent) -> SimpleTrans {
        self.update_menu(data.world, &event)
    }
}
