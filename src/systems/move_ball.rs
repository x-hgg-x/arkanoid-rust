use crate::components::{Ball, StickyBall};

use amethyst::{
    core::{SystemDesc, Time, Transform},
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
};

#[derive(SystemDesc)]
pub struct MoveBallSystem;

impl<'s> System<'s> for MoveBallSystem {
    type SystemData = (ReadStorage<'s, Ball>, ReadStorage<'s, StickyBall>, WriteStorage<'s, Transform>, Read<'s, Time>);

    fn run(&mut self, (balls, sticky_balls, mut transforms, time): Self::SystemData) {
        for (ball, _sticky_balls, ball_transform) in (&balls, !&sticky_balls, &mut transforms).join() {
            ball_transform.prepend_translation_x(ball.velocity * ball.direction.x * time.delta_seconds());
            ball_transform.prepend_translation_y(ball.velocity * ball.direction.y * time.delta_seconds());
        }
    }
}
