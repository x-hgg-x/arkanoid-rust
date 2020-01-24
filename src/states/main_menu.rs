use crate::components::PrefabHandles;
use crate::states::GameplayState;

use amethyst::{
    ecs::Entity,
    input::{is_key_down, VirtualKeyCode},
    prelude::*,
    ui::{UiFinder, UiText},
};

const NUM_MENU_ITEMS: i32 = 2;
const CURSOR_MAIN_MENU_NEW_GAME_ID: &str = "cursor_new_game";
const CURSOR_MAIN_MENU_NEW_GAME_POS: i32 = 0;
const CURSOR_MAIN_MENU_EXIT_ID: &str = "cursor_exit";
const CURSOR_MAIN_MENU_EXIT_POS: i32 = 1;

#[derive(Default)]
pub struct MainMenuState {
    main_menu: Option<Entity>,
    selection: i32,
}

impl SimpleState for MainMenuState {
    fn on_start(&mut self, data: StateData<GameData>) {
        let world = data.world;

        let main_menu = world.read_resource::<PrefabHandles>().main_menu.clone();

        self.main_menu = Some(world.create_entity().with(main_menu).build());
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        if let Some(entity) = self.main_menu {
            data.world.delete_entity(entity).expect("Failed to delete entity.");
        }
    }

    fn handle_event(&mut self, data: StateData<GameData>, event: StateEvent) -> SimpleTrans {
        if let StateEvent::Window(event) = event {
            if is_key_down(&event, VirtualKeyCode::Escape) || is_key_down(&event, VirtualKeyCode::Q) {
                return Trans::Quit;
            }
            if is_key_down(&event, VirtualKeyCode::Return) || is_key_down(&event, VirtualKeyCode::Space) {
                match self.selection.rem_euclid(NUM_MENU_ITEMS) {
                    CURSOR_MAIN_MENU_NEW_GAME_POS => {
                        return Trans::Switch(Box::new(GameplayState::default()));
                    }
                    CURSOR_MAIN_MENU_EXIT_POS => {
                        return Trans::Quit;
                    }
                    _ => unreachable!(),
                }
            }
            if is_key_down(&event, VirtualKeyCode::Down) {
                self.selection = (self.selection + 1).rem_euclid(NUM_MENU_ITEMS);
            }
            if is_key_down(&event, VirtualKeyCode::Up) {
                self.selection = (self.selection - 1).rem_euclid(NUM_MENU_ITEMS);
            }

            let world = data.world;

            let cursor_new_game = world.exec(|ui_finder: UiFinder| ui_finder.find(CURSOR_MAIN_MENU_NEW_GAME_ID));
            let cursor_exit = world.exec(|ui_finder: UiFinder| ui_finder.find(CURSOR_MAIN_MENU_EXIT_ID));

            if let (Some(cursor_new_game), Some(cursor_exit)) = (cursor_new_game, cursor_exit) {
                let mut ui_texts = world.write_storage::<UiText>();

                match self.selection.rem_euclid(NUM_MENU_ITEMS) {
                    CURSOR_MAIN_MENU_NEW_GAME_POS => {
                        write_cursor_alpha_color(ui_texts.get_mut(cursor_new_game), 1.0);
                        write_cursor_alpha_color(ui_texts.get_mut(cursor_exit), 0.0);
                    }
                    CURSOR_MAIN_MENU_EXIT_POS => {
                        write_cursor_alpha_color(ui_texts.get_mut(cursor_new_game), 0.0);
                        write_cursor_alpha_color(ui_texts.get_mut(cursor_exit), 1.0);
                    }
                    _ => unreachable!(),
                }
            }
        }
        Trans::None
    }
}

fn write_cursor_alpha_color(cursor: Option<&mut UiText>, alpha_color: f32) {
    if let Some(cursor) = cursor {
        cursor.color[3] = alpha_color;
    }
}
