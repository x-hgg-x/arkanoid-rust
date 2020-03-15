use crate::MainMenuState;

use components::{ArkanoidPrefabData, CameraPrefabData, GamePrefabHandles, MenuPrefabHandles, PrefabHandles};

use amethyst::{
    assets::{PrefabLoader, ProgressCounter, RonFormat},
    controls::HideCursor,
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

        // Show cursor
        *world.write_resource() = HideCursor { hide: false };

        type SystemData<'s> = (UiLoader<'s>, PrefabLoader<'s, CameraPrefabData>, PrefabLoader<'s, SpriteScenePrefab>, PrefabLoader<'s, ArkanoidPrefabData>);

        let prefab_handles = world.exec(|(ui_loader, camera_loader, sprite_loader, arkanoid_loader): SystemData| PrefabHandles {
            menu: MenuPrefabHandles {
                main_menu: ui_loader.load("ui/main_menu.ron", &mut self.progress_counter),
                pause_menu: ui_loader.load("ui/pause_menu.ron", &mut self.progress_counter),
                game_over_menu: ui_loader.load("ui/game_over_menu.ron", &mut self.progress_counter),
                level_complete_menu: ui_loader.load("ui/level_complete_menu.ron", &mut self.progress_counter),
            },
            game: GamePrefabHandles {
                camera: camera_loader.load("prefabs/camera.ron", RonFormat, &mut self.progress_counter),
                background: sprite_loader.load("prefabs/background.ron", RonFormat, &mut self.progress_counter),
                level: arkanoid_loader.load("prefabs/level.ron", RonFormat, &mut self.progress_counter),
                score: ui_loader.load("ui/score.ron", &mut self.progress_counter),
                life: ui_loader.load("ui/life.ron", &mut self.progress_counter),
            },
        });

        world.insert(prefab_handles);
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
