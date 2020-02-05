use crate::components::{Ball, Paddle, StickyBall};
use crate::states::ARENA_WIDTH;

use precompile::bindings::{ActionBinding, ArkanoidBindings};

use amethyst::{
    core::{
        math::{RealField, Unit, Vector2},
        Time, Transform,
    },
    derive::SystemDesc,
    ecs::{Entities, Join, Read, ReadStorage, System, SystemData as _, WriteStorage},
    input::InputHandler,
};

#[derive(SystemDesc, Default)]
pub struct StickyBallSystem {
    time: f32,
}

type SystemData<'s> = (
    Entities<'s>,
    ReadStorage<'s, Paddle>,
    WriteStorage<'s, Ball>,
    WriteStorage<'s, StickyBall>,
    WriteStorage<'s, Transform>,
    Read<'s, Time>,
    Read<'s, InputHandler<ArkanoidBindings>>,
);

impl<'s> System<'s> for StickyBallSystem {
    type SystemData = SystemData<'s>;

    fn run(&mut self, (entities, paddles, mut balls, mut sticky_balls, mut transforms, time, input): SystemData) {
        if let Some(val) = (&paddles, &transforms).join().next().map(|(paddle, paddle_transform)| (paddle.width, paddle_transform.translation().x)) {
            let (paddle_width, paddle_x): (f32, f32) = val;

            for val in (&mut balls, &sticky_balls, &mut transforms).join() {
                let (ball, sticky_ball, ball_transform): (&mut Ball, &StickyBall, &mut Transform) = val;

                // Follow paddle
                ball_transform.set_translation_x(paddle_x.min(ARENA_WIDTH - ball.radius / 2.0).max(ball.radius / 2.0));

                // Add oscillation
                ball_transform.prepend_translation_x(sticky_ball.width_extent / 2.0 * (2.0 * f32::pi() * self.time / sticky_ball.period).sin());

                // Set ball direction
                let angle = ((paddle_x - ball_transform.translation().x) / paddle_width * f32::pi()).min(f32::pi() / 3.0).max(-f32::pi() / 3.0);
                ball.direction = Unit::new_unchecked(Vector2::new(-angle.sin(), angle.cos()));
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
