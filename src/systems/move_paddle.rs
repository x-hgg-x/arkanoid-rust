use crate::components::Paddle;
use crate::states::ARENA_WIDTH;

use precompile::bindings::{ArkanoidBindings, AxisBinding};

use amethyst::{
    core::{Time, Transform},
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData as _, WriteStorage},
    input::InputHandler,
};

#[derive(SystemDesc)]
pub struct MovePaddleSystem;

type SystemData<'s> = (ReadStorage<'s, Paddle>, WriteStorage<'s, Transform>, Read<'s, Time>, Read<'s, InputHandler<ArkanoidBindings>>);

impl<'s> System<'s> for MovePaddleSystem {
    type SystemData = SystemData<'s>;

    fn run(&mut self, (paddles, mut transforms, time, input): SystemData) {
        for val in (&paddles, &mut transforms).join() {
            let (paddle, paddle_transform): (&Paddle, &mut Transform) = val;

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
