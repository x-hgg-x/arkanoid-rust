use crate::components::PrefabHandles;
use crate::states::{GameplayState, MainMenuState, Menu};

use amethyst::{
    ecs::Entity,
    input::{is_key_down, VirtualKeyCode},
    prelude::*,
};

#[derive(Default)]
pub struct GameOverState {
    game_over_menu: Option<Entity>,
    selection: i32,
}

impl Menu for GameOverState {
    fn get_selection(&self) -> i32 {
        self.selection
    }

    fn set_selection(&mut self, selection: i32) {
        self.selection = selection;
    }

    fn get_menu_ids(&self) -> &[&str] {
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

    fn handle_event(&mut self, data: StateData<GameData>, event: StateEvent) -> SimpleTrans {
        if let StateEvent::Window(event) = event {
            if is_key_down(&event, VirtualKeyCode::Return) || is_key_down(&event, VirtualKeyCode::Space) {
                match self.selection {
                    // Restart
                    0 => {
                        return Trans::Switch(Box::new(GameplayState::default()));
                    }
                    // Main Menu
                    1 => {
                        return Trans::Switch(Box::new(MainMenuState::default()));
                    }
                    // Exit
                    2 => {
                        return Trans::Quit;
                    }
                    _ => unreachable!(),
                }
            }
            self.change_menu(data.world, &event);
        }
        Trans::None
    }
}
