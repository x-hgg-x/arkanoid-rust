use crate::components::Paddle;
use crate::states::ARENA_WIDTH;

use precompile::bindings::{ArkanoidBindings, AxisBinding};

use amethyst::{
    core::{Time, Transform},
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData as _, WriteStorage},
    input::{
        Axis::{Emulated, Mouse},
        InputHandler,
    },
};

#[derive(SystemDesc)]
pub struct MovePaddleSystem;

type SystemData<'s> = (ReadStorage<'s, Paddle>, WriteStorage<'s, Transform>, Read<'s, Time>, Read<'s, InputHandler<ArkanoidBindings>>);

impl<'s> System<'s> for MovePaddleSystem {
    type SystemData = SystemData<'s>;

    fn run(&mut self, (paddles, mut transforms, time, input): SystemData) {
        for val in (&paddles, &mut transforms).join() {
            let (paddle, paddle_transform): (&Paddle, &mut Transform) = val;
            let old_x = paddle_transform.translation().x;

            let new_x = match input.bindings.axis(&AxisBinding::Paddle) {
                Some(Mouse { .. }) => input.mouse_position().map(|pos| pos.0).unwrap_or(old_x),
                Some(Emulated { .. }) => old_x + input.axis_value(&AxisBinding::Paddle).map(|movement| ARENA_WIDTH * time.delta_seconds() * movement).unwrap_or(0.0),
                _ => old_x,
            };

            paddle_transform.set_translation_x(new_x.min(ARENA_WIDTH - paddle.width / 2.0).max(paddle.width / 2.0));
        }
    }
}
