use crate::BallAttractionVfxEvent;

use components::{AttractionLine, Ball, Paddle, StickyBall};

use amethyst::{
    core::{
        math::{Vector2, Vector3},
        Transform,
    },
    derive::SystemDesc,
    ecs::prelude::*,
    renderer::resources::Tint,
    shrev::{EventChannel, ReaderId},
};

#[derive(SystemDesc)]
pub struct BallAttractionVfxSystem {
    reader: ReaderId<BallAttractionVfxEvent>,
}

impl BallAttractionVfxSystem {
    pub fn new(world: &mut World) -> Self {
        <Self as System>::SystemData::setup(world);
        Self {
            reader: world.write_resource::<EventChannel<BallAttractionVfxEvent>>().register_reader(),
        }
    }
}

impl<'s> System<'s> for BallAttractionVfxSystem {
    type SystemData = (
        ReadStorage<'s, Ball>,
        ReadStorage<'s, StickyBall>,
        ReadStorage<'s, AttractionLine>,
        ReadStorage<'s, Paddle>,
        WriteStorage<'s, Tint>,
        WriteStorage<'s, Transform>,
        Read<'s, EventChannel<BallAttractionVfxEvent>>,
    );

    fn run(&mut self, (balls, sticky_balls, attraction_lines, paddles, mut tints, mut transforms, ball_attraction_vfx_event_channel): Self::SystemData) {
        for BallAttractionVfxEvent {
            ball_entity,
            ball_color,
            attraction_line_entity,
            attraction_line_color,
        } in ball_attraction_vfx_event_channel.read(&mut self.reader)
        {
            if let Some(ball_tint) = tints.get_mut(*ball_entity) {
                ball_tint.0 = *ball_color;
            }

            if let Some(attraction_line_tint) = tints.get_mut(*attraction_line_entity) {
                attraction_line_tint.0 = *attraction_line_color;
            }
        }

        if let Some(val) = (&paddles, &transforms).join().next().map(|(paddle, paddle_transform)| (paddle.height, *paddle_transform.translation())) {
            let (paddle_height, paddle_translation): (f32, Vector3<f32>) = val;

            let balls: Vec<(&Ball, Vector3<f32>)> = (&balls, !&sticky_balls, &transforms).join().map(|(ball, _, ball_transform)| (ball, *ball_transform.translation())).collect();

            for val in balls
                .into_iter()
                .zip((&attraction_lines, &mut transforms).join())
                .map(|((ball, ball_transform), (_, attraction_line_transform))| (ball, ball_transform, attraction_line_transform))
            {
                let (ball, ball_translation, attraction_line_transform): (&Ball, Vector3<f32>, &mut Transform) = val;

                let ball_source: Vector2<f32> = [ball_translation.x, ball_translation.y].into();
                let paddle_target: Vector2<f32> = [paddle_translation.x, paddle_translation.y + paddle_height / 2.0 + ball.radius].into();

                let middle: Vector2<f32> = (paddle_target + ball_source) / 2.0;
                let diff: Vector2<f32> = paddle_target - ball_source;

                attraction_line_transform.set_translation_x(middle.x);
                attraction_line_transform.set_translation_y(middle.y);
                attraction_line_transform.set_rotation_z_axis((-Vector2::y_axis().perp(&diff)).atan2(-Vector2::y_axis().dot(&diff)));
                attraction_line_transform.scale_mut().y = diff.norm();
            }
        }
    }
}
