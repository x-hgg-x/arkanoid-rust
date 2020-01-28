use amethyst::{
    input::{is_key_down, VirtualKeyCode},
    prelude::*,
    ui::{UiFinder, UiText},
    winit::Event,
};

pub trait Menu {
    fn get_selection(&self) -> i32;
    fn set_selection(&mut self, selection: i32);
    fn get_cursor_menu_ids(&self) -> &[&str];

    fn change_menu(&mut self, world: &mut World, event: &Event) {
        let selection = self.get_selection();
        let num_items = self.get_cursor_menu_ids().len();

        if is_key_down(&event, VirtualKeyCode::Down) {
            self.set_selection((selection + 1).rem_euclid(num_items as i32));
        }
        if is_key_down(&event, VirtualKeyCode::Up) {
            self.set_selection((selection - 1).rem_euclid(num_items as i32));
        }

        let cursor_entities: Vec<_> = world.exec(|ui_finder: UiFinder| self.get_cursor_menu_ids().iter().filter_map(|id| ui_finder.find(id)).collect());
        if cursor_entities.len() == num_items {
            let mut ui_texts = world.write_storage::<UiText>();

            let new_selection = self.get_selection();
            for entity in &cursor_entities {
                self.write_cursor_alpha_color(ui_texts.get_mut(*entity), 0.0);
            }
            self.write_cursor_alpha_color(ui_texts.get_mut(cursor_entities[new_selection as usize]), 1.0);
        }
    }

    fn write_cursor_alpha_color(&self, cursor: Option<&mut UiText>, alpha_color: f32) {
        if let Some(cursor) = cursor {
            cursor.color[3] = alpha_color;
        }
    }
}
