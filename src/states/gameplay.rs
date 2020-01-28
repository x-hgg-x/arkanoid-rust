use crate::components::PrefabHandles;
use crate::resources::{CurrentState, Game, GameEvent};
use crate::states::{GameOverState, PausedState};

use amethyst::{
    ecs::Join,
    input::{is_key_down, VirtualKeyCode},
    prelude::*,
};

pub const ARENA_WIDTH: f32 = 720.0;
pub const ARENA_HEIGHT: f32 = 600.0;

pub const LIFE_TEXT_ID: &str = "life";

#[derive(Default)]
pub struct GameplayState {}

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
    }

    fn on_pause(&mut self, data: StateData<GameData>) {
        *data.world.write_resource() = CurrentState::Paused;
    }

    fn on_resume(&mut self, data: StateData<GameData>) {
        *data.world.write_resource() = CurrentState::Running;
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        let world = data.world;
        let entities: Vec<_> = world.entities().join().collect();
        world.delete_entities(&entities).expect("Failed to delete entity.");

        *world.write_resource() = CurrentState::Paused;
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        match data.world.write_resource::<Game>().event.take() {
            Some(GameEvent::GameOver) => Trans::Switch(Box::new(GameOverState::default())),
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
