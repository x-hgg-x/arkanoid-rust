use crate::components::{Ball, StickyBall};

use amethyst::{
    core::{Time, Transform},
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData as _, WriteStorage},
};

#[derive(SystemDesc)]
pub struct MoveBallSystem;

type SystemData<'s> = (ReadStorage<'s, Ball>, ReadStorage<'s, StickyBall>, WriteStorage<'s, Transform>, Read<'s, Time>);

impl<'s> System<'s> for MoveBallSystem {
    type SystemData = SystemData<'s>;

    fn run(&mut self, (balls, sticky_balls, mut transforms, time): SystemData) {
        for val in (&balls, !&sticky_balls, &mut transforms).join() {
            let (ball, _, ball_transform): (&Ball, (), &mut Transform) = val;

            ball_transform.prepend_translation_x(ball.velocity * ball.direction.x * time.delta_seconds());
            ball_transform.prepend_translation_y(ball.velocity * ball.direction.y * time.delta_seconds());
        }
    }
}
