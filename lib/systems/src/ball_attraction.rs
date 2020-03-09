use crate::{BallAttractionVfxEvent, StopBallAttractionEvent};

use bundle::bindings::{ActionBinding, ArkanoidBindings};
use components::{AttractionLine, Ball, Paddle, StickyBall};

use amethyst::{
    core::{
        math::{Unit, Vector2, Vector3},
        Time, Transform,
    },
    derive::SystemDesc,
    ecs::prelude::*,
    input::InputHandler,
    renderer::palette::rgb::Srgba,
    shrev::{EventChannel, ReaderId},
};

#[derive(SystemDesc)]
pub struct BallAttractionSystem {
    reader: ReaderId<StopBallAttractionEvent>,
    last_collision_time: f64,
    time_accelerated: f64,
    timeout: f64,
}

impl BallAttractionSystem {
    pub fn new(world: &mut World) -> Self {
        <Self as System>::SystemData::setup(world);
        Self {
            reader: world.write_resource::<EventChannel<StopBallAttractionEvent>>().register_reader(),
            last_collision_time: 0.0,
            time_accelerated: 0.0,
            timeout: 0.25,
        }
    }
}

impl<'s> System<'s> for BallAttractionSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Ball>,
        ReadStorage<'s, StickyBall>,
        ReadStorage<'s, AttractionLine>,
        ReadStorage<'s, Paddle>,
        ReadStorage<'s, Transform>,
        Read<'s, Time>,
        Read<'s, InputHandler<ArkanoidBindings>>,
        Read<'s, EventChannel<StopBallAttractionEvent>>,
        Write<'s, EventChannel<BallAttractionVfxEvent>>,
    );

    fn run(&mut self, (entities, mut balls, sticky_balls, attraction_lines, paddles, transforms, time, input, stop_ball_attraction_event_channel, mut ball_attraction_vfx_event_channel): Self::SystemData) {
        if (&mut balls).join().any(|x| x.velocity_mult > 1.0) {
            for StopBallAttractionEvent { collision_time } in stop_ball_attraction_event_channel.read(&mut self.reader) {
                if self.last_collision_time < self.time_accelerated {
                    self.last_collision_time = *collision_time;
                }
            }

            if self.last_collision_time >= self.time_accelerated {
                if time.absolute_time_seconds() < self.last_collision_time + self.timeout {
                    return;
                }
                for val in (&entities, &mut balls, !&sticky_balls)
                    .join()
                    .zip((&entities, &attraction_lines).join())
                    .map(|((ball_entity, ball, _), (attraction_line_entity, _))| (ball_entity, ball, attraction_line_entity))
                {
                    let (ball_entity, ball, attraction_line_entity): (Entity, &mut Ball, Entity) = val;
                    ball.velocity_mult = 1.0;

                    ball_attraction_vfx_event_channel.single_write(BallAttractionVfxEvent {
                        ball_entity,
                        ball_color: Srgba::new(1.0, 1.0, 1.0, 1.0),
                        attraction_line_entity,
                        attraction_line_color: Srgba::new(1.0, 1.0, 1.0, 0.0),
                    });
                }
            }
        } else {
            // Consume events in channel
            stop_ball_attraction_event_channel.read(&mut self.reader).for_each(|_| ());
        }

        if let Some(val) = (&paddles, &transforms).join().next().map(|(paddle, paddle_transform)| (paddle.height, paddle_transform.translation())) {
            let (paddle_height, paddle_translation): (f32, &Vector3<f32>) = val;

            for val in (&entities, &mut balls, !&sticky_balls, &transforms)
                .join()
                .zip((&entities, &attraction_lines).join())
                .map(|((ball_entity, ball, _, ball_transform), (attraction_line_entity, _))| (ball_entity, ball, ball_transform, attraction_line_entity))
            {
                let (ball_entity, ball, ball_transform, attraction_line_entity): (Entity, &mut Ball, &Transform, Entity) = val;

                let ball_source: Vector2<f32> = [ball_transform.translation().x, ball_transform.translation().y].into();
                let paddle_target: Vector2<f32> = [paddle_translation.x, paddle_translation.y + paddle_height / 2.0 + ball.radius].into();

                if let Some(true) = input.action_is_down(&ActionBinding::BallAttraction) {
                    self.time_accelerated = time.absolute_time_seconds();
                    ball.direction = Unit::new_normalize(paddle_target - ball_source);
                    ball.velocity_mult = 3.0;

                    ball_attraction_vfx_event_channel.single_write(BallAttractionVfxEvent {
                        ball_entity,
                        ball_color: Srgba::new(0.9, 0.3, 0.2, 1.0),
                        attraction_line_entity,
                        attraction_line_color: Srgba::new(1.0, 1.0, 1.0, 1.0),
                    });
                } else if ball.velocity_mult > 1.0 && self.last_collision_time < self.time_accelerated {
                    ball.velocity_mult = 1.0;

                    ball_attraction_vfx_event_channel.single_write(BallAttractionVfxEvent {
                        ball_entity,
                        ball_color: Srgba::new(1.0, 1.0, 1.0, 1.0),
                        attraction_line_entity,
                        attraction_line_color: Srgba::new(1.0, 1.0, 1.0, 0.0),
                    });
                }
            }
        }
    }
}
