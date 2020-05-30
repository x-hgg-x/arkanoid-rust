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
    utils::ortho_camera::CameraOrtho,
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
        ReadStorage<'s, CameraOrtho>,
        Read<'s, InputHandler<ArkanoidBindings>>,
    );

    fn run(&mut self, (paddles, mut transforms, time, dimensions, ortho_cameras, input): Self::SystemData) {
        if let Some(ortho_camera) = ortho_cameras.join().next() {
            let (left, right, _, _) = ortho_camera.camera_offsets(dimensions.aspect_ratio());
            let camera_width = right - left;

            for val in (&paddles, &mut transforms).join() {
                let (paddle, paddle_transform): (&Paddle, &mut Transform) = val;
                let old_x = paddle_transform.translation().x;

                let new_x = match input.bindings.axis(&AxisBinding::Paddle) {
                    Some(Mouse { .. }) => input.mouse_position().map(|pos| pos.0 * camera_width / dimensions.width() - (camera_width - ARENA_WIDTH) / 2.0).unwrap_or(old_x),

                    Some(Emulated { .. }) => old_x + input.axis_value(&AxisBinding::Paddle).map(|movement| ARENA_WIDTH * time.delta_seconds() * movement).unwrap_or(0.0),
                    _ => old_x,
                };

                paddle_transform.set_translation_x(new_x.min(ARENA_WIDTH - paddle.width / 2.0).max(paddle.width / 2.0));
            }
        }
    }
}
