use bundle::bindings::{ArkanoidBindings, AxisBinding};
use components::Paddle;
use resources::ARENA_WIDTH;

use amethyst::{
    core::{Time, Transform},
    derive::SystemDesc,
    ecs::prelude::*,
    input::{
        Axis::{Emulated, Mouse},
        InputHandler,
    },
    window::ScreenDimensions,
};

#[derive(SystemDesc)]
pub struct MovePaddleSystem;

impl<'s> System<'s> for MovePaddleSystem {
    type SystemData = (
        ReadStorage<'s, Paddle>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
        ReadExpect<'s, ScreenDimensions>,
        Read<'s, InputHandler<ArkanoidBindings>>,
    );

    fn run(&mut self, (paddles, mut transforms, time, dimensions, input): <Self as System>::SystemData) {
        for val in (&paddles, &mut transforms).join() {
            let (paddle, paddle_transform): (&Paddle, &mut Transform) = val;
            let old_x = paddle_transform.translation().x;

            let new_x = match input.bindings.axis(&AxisBinding::Paddle) {
                Some(Mouse { .. }) => input.mouse_position().map(|pos| pos.0 / dimensions.width() * ARENA_WIDTH).unwrap_or(old_x),
                Some(Emulated { .. }) => old_x + input.axis_value(&AxisBinding::Paddle).map(|movement| ARENA_WIDTH * time.delta_seconds() * movement).unwrap_or(0.0),
                _ => old_x,
            };

            paddle_transform.set_translation_x(new_x.min(ARENA_WIDTH - paddle.width / 2.0).max(paddle.width / 2.0));
        }
    }
}
