use crate::components::{GamePrefabHandles, PrefabHandles};
use crate::states::PausedState;

use amethyst::{
    ecs::Join,
    input::{is_key_down, VirtualKeyCode},
    prelude::*,
};

#[derive(Default)]
pub struct GameplayState {}

impl SimpleState for GameplayState {
    fn on_start(&mut self, data: StateData<GameData>) {
        let world = data.world;

        let GamePrefabHandles { camera, background, level, score } = world.read_resource::<PrefabHandles>().game.clone();

        world.create_entity().with(camera).build();
        world.create_entity().with(background).build();
        world.create_entity().with(level).build();
        world.create_entity().with(score).build();
    }

    fn handle_event(&mut self, _data: StateData<GameData>, event: StateEvent) -> SimpleTrans {
        if let StateEvent::Window(event) = event {
            if is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Push(Box::new(PausedState::default()));
            }
        }
        Trans::None
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        let world = data.world;
        let entities: Vec<_> = world.entities().join().collect();
        world.delete_entities(&entities).expect("Failed to delete entity.");
    }
}
