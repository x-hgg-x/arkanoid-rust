use crate::components::Paddle;
use crate::states::ARENA_WIDTH;

use arkanoid_precompile::bindings::{ArkanoidBindings, AxisBinding};

use amethyst::{
    core::{SystemDesc, Time, Transform},
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
    input::InputHandler,
};

#[derive(SystemDesc)]
pub struct MovePaddleSystem;

impl<'s> System<'s> for MovePaddleSystem {
    type SystemData = (ReadStorage<'s, Paddle>, WriteStorage<'s, Transform>, Read<'s, Time>, Read<'s, InputHandler<ArkanoidBindings>>);

    fn run(&mut self, (paddles, mut transforms, time, input): Self::SystemData) {
        for (paddle, paddle_transform) in (&paddles, &mut transforms).join() {
            if let Some(movement) = input.axis_value(&AxisBinding::Paddle) {
                paddle_transform.set_translation_x(
                    (paddle_transform.translation().x + paddle.velocity * time.delta_seconds() * movement)
                        .min(ARENA_WIDTH - paddle.width / 2.0)
                        .max(paddle.width / 2.0),
                );
            }
        }
    }
}
