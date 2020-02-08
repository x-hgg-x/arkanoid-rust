use crate::{MainMenuState, Menu};

use components::PrefabHandles;

use amethyst::{
    ecs::Entity,
    input::{is_key_down, VirtualKeyCode},
    prelude::*,
};

#[derive(Default)]
pub struct PausedState {
    pause_menu: Option<Entity>,
    selection: i32,
}

impl Menu for PausedState {
    fn get_selection(&self) -> i32 {
        self.selection
    }

    fn set_selection(&mut self, selection: i32) {
        self.selection = selection;
    }

    fn confirm_selection(&self) -> SimpleTrans {
        match self.selection {
            // Resume
            0 => Trans::Pop,
            // Main Menu
            1 => Trans::Replace(Box::new(MainMenuState::default())),
            // Exit
            2 => Trans::Quit,
            _ => unreachable!(),
        }
    }

    fn get_menu_ids(&self) -> &[&str] {
        &["resume", "main_menu", "exit"]
    }

    fn get_cursor_menu_ids(&self) -> &[&str] {
        &["cursor_resume", "cursor_main_menu", "cursor_exit"]
    }
}

impl SimpleState for PausedState {
    fn on_start(&mut self, data: StateData<GameData>) {
        let world = data.world;

        let pause_menu = world.read_resource::<PrefabHandles>().menu.pause_menu.clone();
        self.pause_menu = Some(world.create_entity().with(pause_menu).build());
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        if let Some(entity) = self.pause_menu {
            data.world.delete_entity(entity).expect("Failed to delete entity.");
        }
    }

    fn handle_event(&mut self, data: StateData<GameData>, event: StateEvent) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Pop;
            }
        }
        self.update_menu(data.world, &event)
    }
}
