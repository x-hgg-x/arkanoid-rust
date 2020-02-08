use crate::components::{Ball, Paddle, StickyBall};
use crate::systems::StopBallAttractionEvent;

use precompile::bindings::{ActionBinding, ArkanoidBindings};

use amethyst::{
    core::{
        math::{Unit, Vector2, Vector3},
        Time, Transform,
    },
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData as _, World, Write, WriteStorage},
    input::InputHandler,
    prelude::*,
    renderer::{
        debug_drawing::DebugLines,
        palette::rgb::{Rgb, Srgba},
        resources::Tint,
    },
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

type SystemData<'s> = (
    WriteStorage<'s, Ball>,
    ReadStorage<'s, StickyBall>,
    WriteStorage<'s, Tint>,
    ReadStorage<'s, Paddle>,
    ReadStorage<'s, Transform>,
    Read<'s, Time>,
    Write<'s, DebugLines>,
    Read<'s, InputHandler<ArkanoidBindings>>,
    Read<'s, EventChannel<StopBallAttractionEvent>>,
);

impl<'s> System<'s> for BallAttractionSystem {
    type SystemData = SystemData<'s>;

    fn run(&mut self, (mut balls, sticky_balls, mut tints, paddles, transforms, time, mut debug_lines, input, stop_ball_attraction_event_channel): SystemData) {
        let mut is_timeout = false;

        if (&mut balls).join().any(|x| x.velocity_mult > 1.0) {
            for StopBallAttractionEvent { collision_time } in stop_ball_attraction_event_channel.read(&mut self.reader) {
                if self.last_collision_time < self.time_accelerated {
                    self.last_collision_time = *collision_time;
                }
            }

            if self.last_collision_time >= self.time_accelerated {
                if time.absolute_time_seconds() < self.last_collision_time + self.timeout {
                    is_timeout = true;
                } else {
                    for val in (&mut balls, &mut tints).join() {
                        let (ball, ball_tint): (&mut Ball, &mut Tint) = val;
                        ball.velocity_mult = 1.0;
                        ball_tint.0.color = Rgb::new(1.0, 1.0, 1.0);
                    }
                }
            }
        } else {
            // Consume events in channel
            stop_ball_attraction_event_channel.read(&mut self.reader).for_each(|_| ());
        }

        if let Some(val) = (&paddles, &transforms).join().next().map(|(paddle, paddle_transform)| (paddle.height, paddle_transform.translation())) {
            let (paddle_height, paddle_translation): (f32, &Vector3<f32>) = val;

            for val in (&mut balls, !&sticky_balls, &mut tints, &transforms).join() {
                let (ball, _, ball_tint, ball_transform): (&mut Ball, (), &mut Tint, &Transform) = val;

                let ball_source: Vector2<f32> = [ball_transform.translation().x, ball_transform.translation().y].into();
                let paddle_target: Vector2<f32> = [paddle_translation.x, paddle_translation.y + paddle_height / 2.0 + ball.radius].into();

                if ball.velocity_mult > 1.0 {
                    debug_lines.draw_line([ball_source.x, ball_source.y, 0.1].into(), [paddle_target.x, paddle_target.y, 0.1].into(), Srgba::default());
                }

                if !is_timeout {
                    if let Some(true) = input.action_is_down(&ActionBinding::BallAttraction) {
                        self.time_accelerated = time.absolute_time_seconds();
                        ball.direction = Unit::new_normalize(paddle_target - ball_source);
                        ball.velocity_mult = 3.0;
                        ball_tint.0.color = Rgb::new(0.9, 0.3, 0.2);
                    } else if ball.velocity_mult > 1.0 && self.last_collision_time < self.time_accelerated {
                        ball.velocity_mult = 1.0;
                        ball_tint.0.color = Rgb::new(1.0, 1.0, 1.0);
                    }
                }
            }
        }
    }
}
