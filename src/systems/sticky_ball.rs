use crate::components::{Ball, Paddle, StickyBall};
use crate::states::ARENA_WIDTH;

use arkanoid_precompile::bindings::{ActionBinding, ArkanoidBindings};

use amethyst::{
    core::{SystemDesc, Time, Transform},
    derive::SystemDesc,
    ecs::{Entities, Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
    input::InputHandler,
};

use std::f32::consts::PI;

#[derive(SystemDesc, Default)]
pub struct StickyBallSystem {
    time: f32,
}

impl<'s> System<'s> for StickyBallSystem {
    #[allow(clippy::type_complexity)]
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
        if let Some((paddle_width, paddle_x)) = (&paddles, &transforms).join().next().map(|(paddle, paddle_transform)| (paddle.width, paddle_transform.translation().x)) {
            for (ball, sticky_ball, ball_transform) in (&mut balls, &sticky_balls, &mut transforms).join() {
                // Follow paddle
                ball_transform.set_translation_x(paddle_x.min(ARENA_WIDTH - ball.radius / 2.0).max(ball.radius / 2.0));

                // Add oscillation
                ball_transform.prepend_translation_x(sticky_ball.width_extent / 2.0 * (2.0 * PI * self.time / sticky_ball.period).sin());

                // Set ball direction
                let angle = ((paddle_x - ball_transform.translation().x) / paddle_width * PI).min(PI / 3.0).max(-PI / 3.0);
                ball.direction.x = -angle.sin();
                ball.direction.y = angle.cos();
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
