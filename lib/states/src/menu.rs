use amethyst::{
    input::{is_key_down, VirtualKeyCode},
    prelude::*,
    ui::{UiEventType, UiFinder, UiText},
};

use std::convert::TryInto;

pub trait Menu {
    fn get_selection(&self) -> i32;
    fn set_selection(&mut self, selection: i32);
    fn confirm_selection(&self) -> SimpleTrans;
    fn get_menu_ids(&self) -> &[&str];
    fn get_cursor_menu_ids(&self) -> &[&str];

    fn update_menu(&mut self, world: &mut World, event: &StateEvent) -> SimpleTrans {
        let selection: i32 = self.get_selection();
        let num_items = self.get_cursor_menu_ids().len();

        match event {
            StateEvent::Window(event) => {
                if is_key_down(&event, VirtualKeyCode::Down) {
                    self.set_selection((selection + 1).rem_euclid(num_items.try_into().unwrap()));
                }
                if is_key_down(&event, VirtualKeyCode::Up) {
                    self.set_selection((selection - 1).rem_euclid(num_items.try_into().unwrap()));
                }
                if is_key_down(&event, VirtualKeyCode::Return) || is_key_down(&event, VirtualKeyCode::Space) {
                    return self.confirm_selection();
                }
            }
            StateEvent::Ui(ui_event) => {
                if let UiEventType::HoverStart | UiEventType::Click = ui_event.event_type {
                    if let Some(position) = world.exec(|ui_finder: UiFinder| self.get_menu_ids().iter().filter_map(|id| ui_finder.find(id)).position(|entity| entity == ui_event.target)) {
                        self.set_selection(position.try_into().unwrap());

                        if let UiEventType::Click = ui_event.event_type {
                            return self.confirm_selection();
                        }
                    }
                }
            }
            _ => {}
        }

        let cursor_entities: Vec<_> = world.exec(|ui_finder: UiFinder| self.get_cursor_menu_ids().iter().filter_map(|id| ui_finder.find(id)).collect());
        if cursor_entities.len() == num_items {
            let mut ui_texts = world.write_storage::<UiText>();

            let new_selection: usize = self.get_selection().try_into().unwrap();
            for entity in &cursor_entities {
                self.write_cursor_alpha_color(ui_texts.get_mut(*entity), 0.0);
            }
            self.write_cursor_alpha_color(ui_texts.get_mut(cursor_entities[new_selection]), 1.0);
        }
        Trans::None
    }

    fn write_cursor_alpha_color(&self, cursor: Option<&mut UiText>, alpha_color: f32) {
        if let Some(cursor) = cursor {
            cursor.color[3] = alpha_color;
        }
    }
}
