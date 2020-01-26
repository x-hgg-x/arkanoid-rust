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
        builder.add(MovePaddleSystem.pausable(CurrentState::Running), "paddle_system", &[]);
        Ok(())
    }
}
