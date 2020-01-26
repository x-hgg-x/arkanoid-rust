use crate::resources::CurrentState;
use crate::systems::*;

use amethyst::{
    core::SystemBundle,
    ecs::{DispatcherBuilder, World},
    prelude::*,
    Error,
};

pub struct ArkanoidBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for ArkanoidBundle {
    fn build(self, _world: &mut World, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<(), Error> {
        builder.add(MovePaddleSystem.pausable(CurrentState::Running), "paddle_system", &["input_system"]);
        builder.add(StickyBallSystem::default().pausable(CurrentState::Running), "sticky_ball_system", &["paddle_system"]);
        builder.add(MoveBallSystem.pausable(CurrentState::Running), "move_ball_system", &["sticky_ball_system"]);
        builder.add(CollisionSystem.pausable(CurrentState::Running), "collision_system", &["paddle_system", "move_ball_system"]);
        Ok(())
    }
}
