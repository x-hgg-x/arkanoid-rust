use crate::components::PrefabHandles;
use crate::states::MainMenuState;

use amethyst::{assets::ProgressCounter, prelude::*, ui::UiLoader};

#[derive(Default)]
pub struct LoadingState {
    progress_counter: ProgressCounter,
}

impl SimpleState for LoadingState {
    fn on_start(&mut self, data: StateData<GameData>) {
        let world = data.world;

        let main_menu = world.exec(|loader: UiLoader| loader.load("ui/main_menu.ron", &mut self.progress_counter));

        world.insert(PrefabHandles { main_menu });
    }

    fn update(&mut self, _data: &mut StateData<GameData>) -> SimpleTrans {
        println!("Loading: {}%", 100 * self.progress_counter.num_finished() / self.progress_counter.num_assets());

        if self.progress_counter.num_failed() > 0 {
            println!("Error when loading assets.");
            return Trans::Quit;
        }

        if self.progress_counter.is_complete() {
            println!("Loading complete.");
            return Trans::Switch(Box::new(MainMenuState::default()));
        }
        Trans::None
    }
}
