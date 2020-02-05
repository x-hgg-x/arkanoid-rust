use crate::components::{Ball, Paddle, StickyBall};
use crate::systems::StopBallAttractionEvent;

use precompile::bindings::{ActionBinding, ArkanoidBindings};

use amethyst::{
    core::{
        math::{Unit, Vector2, Vector3},
        Time, Transform,
    },
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData as _, World, WriteStorage},
    input::InputHandler,
    prelude::*,
    shrev::{EventChannel, ReaderId},
};

#[derive(SystemDesc)]
pub struct BallAttractionSystem {
    reader: ReaderId<StopBallAttractionEvent>,
    last_collision_time: f64,
    timeout: f64,
    is_accelerated: bool,
    time_accelerated: f64,
}

impl BallAttractionSystem {
    pub fn new(world: &mut World) -> Self {
        <Self as System>::SystemData::setup(world);
        Self {
            reader: world.write_resource::<EventChannel<StopBallAttractionEvent>>().register_reader(),
            last_collision_time: 0.0,
            timeout: 0.25,
            is_accelerated: false,
            time_accelerated: 0.0,
        }
    }
}

type SystemData<'s> = (
    WriteStorage<'s, Ball>,
    ReadStorage<'s, StickyBall>,
    ReadStorage<'s, Paddle>,
    ReadStorage<'s, Transform>,
    Read<'s, Time>,
    Read<'s, InputHandler<ArkanoidBindings>>,
    Read<'s, EventChannel<StopBallAttractionEvent>>,
);

impl<'s> System<'s> for BallAttractionSystem {
    type SystemData = SystemData<'s>;

    fn run(&mut self, (mut balls, sticky_balls, paddles, transforms, time, input, stop_ball_attraction_event_channel): SystemData) {
        if self.is_accelerated {
            for StopBallAttractionEvent { collision_time } in stop_ball_attraction_event_channel.read(&mut self.reader) {
                if self.last_collision_time < self.time_accelerated {
                    self.last_collision_time = *collision_time;
                }
            }

            if self.last_collision_time >= self.time_accelerated {
                if time.absolute_time_seconds() < self.last_collision_time + self.timeout {
                    return;
                }

                if time.absolute_time_seconds() >= self.last_collision_time + self.timeout {
                    for ball in (&mut balls).join() {
                        ball.velocity_mult = 1.0;
                    }
                }
            }
        } else {
            // Consume events in channel
            stop_ball_attraction_event_channel.read(&mut self.reader).for_each(|_| ());
        }

        if let Some(val) = (&paddles, &transforms).join().next().map(|(paddle, paddle_transform)| (paddle.height, paddle_transform.translation())) {
            let (paddle_height, paddle_translation): (f32, &Vector3<f32>) = val;

            for val in (&mut balls, !&sticky_balls, &transforms).join() {
                let (ball, _, ball_transform): (&mut Ball, (), &Transform) = val;

                if let Some(true) = input.action_is_down(&ActionBinding::BallAttraction) {
                    let ball_source: Vector2<f32> = Vector2::new(ball_transform.translation().x, ball_transform.translation().y);
                    let paddle_target: Vector2<f32> = Vector2::new(paddle_translation.x, paddle_translation.y + paddle_height / 2.0 + ball.radius);
                    ball.direction = Unit::new_normalize(paddle_target - ball_source);
                    ball.velocity_mult = 3.0;

                    self.is_accelerated = true;
                    self.time_accelerated = time.absolute_time_seconds();
                } else if self.is_accelerated && self.last_collision_time < self.time_accelerated {
                    ball.velocity_mult = 1.0;
                }
            }
        }
    }
}
