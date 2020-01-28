use crate::components::{ArkanoidPrefabData, CameraPrefabData, GamePrefabHandles, MenuPrefabHandles, PrefabHandles};
use crate::states::MainMenuState;

use amethyst::{
    assets::{PrefabLoader, ProgressCounter, RonFormat},
    prelude::*,
    renderer::sprite::prefab::SpriteScenePrefab,
    ui::UiLoader,
};

#[derive(Default)]
pub struct LoadingState {
    progress_counter: ProgressCounter,
}

impl SimpleState for LoadingState {
    fn on_start(&mut self, data: StateData<GameData>) {
        let world = data.world;

        let main_menu = world.exec(|loader: UiLoader| loader.load("ui/main_menu.ron", &mut self.progress_counter));
        let pause_menu = world.exec(|loader: UiLoader| loader.load("ui/pause_menu.ron", &mut self.progress_counter));
        let game_over_menu = world.exec(|loader: UiLoader| loader.load("ui/game_over_menu.ron", &mut self.progress_counter));
        let camera = world.exec(|loader: PrefabLoader<CameraPrefabData>| loader.load("prefabs/camera.ron", RonFormat, &mut self.progress_counter));
        let background = world.exec(|loader: PrefabLoader<SpriteScenePrefab>| loader.load("prefabs/background.ron", RonFormat, &mut self.progress_counter));
        let level = world.exec(|loader: PrefabLoader<ArkanoidPrefabData>| loader.load("prefabs/level.ron", RonFormat, &mut self.progress_counter));
        let score = world.exec(|loader: UiLoader| loader.load("ui/score.ron", &mut self.progress_counter));
        let life = world.exec(|loader: UiLoader| loader.load("ui/life.ron", &mut self.progress_counter));

        world.insert(PrefabHandles {
            menu: MenuPrefabHandles {
                main_menu,
                pause_menu,
                game_over_menu,
            },
            game: GamePrefabHandles {
                camera,
                background,
                level,
                score,
                life,
            },
        });
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
