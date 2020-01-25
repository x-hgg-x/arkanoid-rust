use crate::components::{GamePrefabHandles, PrefabHandles};

use amethyst::prelude::*;

#[derive(Default)]
pub struct GameplayState {}

impl SimpleState for GameplayState {
    fn on_start(&mut self, data: StateData<GameData>) {
        let world = data.world;

        let GamePrefabHandles { camera, background } = world.read_resource::<PrefabHandles>().game.clone();

        world.create_entity().with(camera).build();
        world.create_entity().with(background).build();
    }
}
