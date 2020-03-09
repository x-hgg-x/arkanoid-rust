use bundle::bindings::{ActionBinding, ArkanoidBindings};
use components::{Ball, Paddle, StickyBall};
use resources::ARENA_WIDTH;

use amethyst::{
    core::{
        math::{RealField, Unit},
        Time, Transform,
    },
    derive::SystemDesc,
    ecs::prelude::*,
    input::InputHandler,
};

#[derive(SystemDesc, Default)]
pub struct StickyBallSystem {
    time: f32,
}

impl<'s> System<'s> for StickyBallSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Paddle>,
        WriteStorage<'s, Ball>,
        WriteStorage<'s, StickyBall>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
        Read<'s, InputHandler<ArkanoidBindings>>,
    );

    fn run(&mut self, (entities, paddles, mut balls, mut sticky_balls, mut transforms, time, input): Self::SystemData) {
        if let Some(val) = (&paddles, &transforms).join().next().map(|(paddle, paddle_transform)| (paddle.width, paddle_transform.translation().x)) {
            let (paddle_width, paddle_x): (f32, f32) = val;

            for val in (&mut balls, &sticky_balls, &mut transforms).join() {
                let (ball, sticky_ball, ball_transform): (&mut Ball, &StickyBall, &mut Transform) = val;

                // Follow paddle
                ball_transform.set_translation_x(paddle_x.min(ARENA_WIDTH - ball.radius / 2.0).max(ball.radius / 2.0));

                // Add oscillation
                ball_transform.prepend_translation_x(paddle_width / 4.0 * (2.0 * f32::pi() * self.time / sticky_ball.period).sin());

                // Set ball direction
                let angle = (paddle_x - ball_transform.translation().x) / paddle_width * f32::pi();
                ball.direction = Unit::new_unchecked([-angle.sin(), angle.cos()].into());

                // Reset ball velocity
                ball.velocity_mult = 1.0
            }

            if let Some(true) = input.action_is_down(&ActionBinding::ReleaseBall) {
                let sticky_ball_entities: Vec<_> = (&entities, &balls, &sticky_balls).join().map(|(entity, _, _)| entity).collect();

                for entity in sticky_ball_entities {
                    sticky_balls.remove(entity);
                }
            }
        }

        // Increment time
        self.time += time.delta_seconds();
    }
}
