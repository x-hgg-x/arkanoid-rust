use crate::{GameOverState, LevelCompleteState, PausedState};

use components::PrefabHandles;
use resources::{CurrentState, Game, GameEvent};

use amethyst::{
    controls::HideCursor,
    ecs::Join,
    input::{is_key_down, VirtualKeyCode},
    prelude::*,
};

#[derive(Default)]
pub struct GameplayState;

impl SimpleState for GameplayState {
    fn on_start(&mut self, data: StateData<GameData>) {
        let world = data.world;

        let game_handles = world.read_resource::<PrefabHandles>().game.clone();

        world.create_entity().with(game_handles.camera).build();
        world.create_entity().with(game_handles.background).build();
        world.create_entity().with(game_handles.level).build();
        world.create_entity().with(game_handles.score).build();
        world.create_entity().with(game_handles.life).build();

        *world.write_resource() = CurrentState::Running;
        *world.write_resource() = HideCursor { hide: true };
    }

    fn on_pause(&mut self, data: StateData<GameData>) {
        let world = data.world;
        *world.write_resource() = CurrentState::Paused;
        *world.write_resource() = HideCursor { hide: false };
    }

    fn on_resume(&mut self, data: StateData<GameData>) {
        let world = data.world;
        *world.write_resource() = CurrentState::Running;
        *world.write_resource() = HideCursor { hide: true };
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        let world = data.world;
        let entities: Vec<_> = world.entities().join().collect();
        world.delete_entities(&entities).expect("Failed to delete entities.");

        *world.write_resource() = CurrentState::Paused;
        *world.write_resource::<Game>() = Default::default();
        *world.write_resource() = HideCursor { hide: false };
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        let mut game = data.world.write_resource::<Game>();

        match game.event.take() {
            Some(GameEvent::GameOver) => Trans::Switch(Box::new(GameOverState::new(game.score))),
            Some(GameEvent::LevelComplete) => Trans::Switch(Box::new(LevelCompleteState::new(game.score))),
            _ => Trans::None,
        }
    }

    fn handle_event(&mut self, _data: StateData<GameData>, event: StateEvent) -> SimpleTrans {
        if let StateEvent::Window(event) = event {
            if is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Push(Box::new(PausedState::default()));
            }
        }
        Trans::None
    }
}
