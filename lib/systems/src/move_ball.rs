use components::{Ball, StickyBall};

use amethyst::{
    core::{Time, Transform},
    derive::SystemDesc,
    ecs::prelude::*,
};

#[derive(SystemDesc)]
pub struct MoveBallSystem;

impl<'s> System<'s> for MoveBallSystem {
    type SystemData = (ReadStorage<'s, Ball>, ReadStorage<'s, StickyBall>, WriteStorage<'s, Transform>, Read<'s, Time>);

    fn run(&mut self, (balls, sticky_balls, mut transforms, time): Self::SystemData) {
        for val in (&balls, !&sticky_balls, &mut transforms).join() {
            let (ball, _, ball_transform): (&Ball, (), &mut Transform) = val;

            ball_transform.prepend_translation_x(ball.velocity * ball.velocity_mult * ball.direction.x * time.delta_seconds());
            ball_transform.prepend_translation_y(ball.velocity * ball.velocity_mult * ball.direction.y * time.delta_seconds());
        }
    }
}
