use crate::{MainMenuState, Menu};

use components::PrefabHandles;
use resources::SCORE_TEXT_ID;

use amethyst::{
    ecs::{Entity, WriteStorage},
    prelude::*,
    ui::{UiFinder, UiText},
};

pub struct LevelCompleteState {
    level_complete_menu: Option<Entity>,
    selection: i32,
    score: i32,
    score_updated: bool,
}

impl LevelCompleteState {
    pub fn new(score: i32) -> Self {
        Self {
            level_complete_menu: None,
            selection: 0,
            score,
            score_updated: false,
        }
    }
}

impl Menu for LevelCompleteState {
    fn get_selection(&self) -> i32 {
        self.selection
    }

    fn set_selection(&mut self, selection: i32) {
        self.selection = selection;
    }

    fn confirm_selection(&self) -> SimpleTrans {
        match self.selection {
            // Main Menu
            0 => Trans::Switch(Box::new(MainMenuState::default())),
            _ => unreachable!(),
        }
    }

    fn get_menu_ids(&self) -> &[&str] {
        &["main_menu"]
    }

    fn get_cursor_menu_ids(&self) -> &[&str] {
        &["cursor_main_menu"]
    }
}

impl SimpleState for LevelCompleteState {
    fn on_start(&mut self, data: StateData<GameData>) {
        let world = data.world;

        let level_complete_menu = world.read_resource::<PrefabHandles>().menu.level_complete_menu.clone();
        self.level_complete_menu = Some(world.create_entity().with(level_complete_menu).build());
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        if let Some(entity) = self.level_complete_menu {
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
