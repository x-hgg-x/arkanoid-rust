use crate::{BallAttractionVfxEvent, BlockCollisionEvent, LifeEvent, ScoreEvent, StopBallAttractionEvent};

use components::{AttractionLine, Ball, Block, Paddle, StickyBall};
use resources::{ARENA_HEIGHT, ARENA_WIDTH};

use amethyst::{
    core::{
        math::{Isometry2, RealField, Rotation2, Unit, Vector3},
        Time, Transform,
    },
    derive::SystemDesc,
    ecs::prelude::*,
    renderer::palette::rgb::Srgba,
    shrev::EventChannel,
};

use ncollide2d::{
    query,
    shape::{Ball as BallShape, Compound, Cuboid, ShapeHandle},
};

#[derive(SystemDesc)]
pub struct CollisionSystem;

impl<'s> System<'s> for CollisionSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Ball>,
        WriteStorage<'s, StickyBall>,
        ReadStorage<'s, AttractionLine>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Paddle>,
        ReadStorage<'s, Block>,
        Read<'s, Time>,
        Write<'s, EventChannel<BlockCollisionEvent>>,
        Write<'s, EventChannel<LifeEvent>>,
        Write<'s, EventChannel<ScoreEvent>>,
        Write<'s, EventChannel<StopBallAttractionEvent>>,
        Write<'s, EventChannel<BallAttractionVfxEvent>>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut balls,
            mut sticky_balls,
            attraction_lines,
            mut transforms,
            paddles,
            blocks,
            time,
            mut block_collision_event_channel,
            mut life_event_channel,
            mut score_event_channel,
            mut stop_ball_attraction_event_channel,
            mut ball_attraction_vfx_event_channel,
        ): Self::SystemData,
    ) {
        // Compute union of blocks
        let block_compound: Compound<f32> = Compound::new(
            (&blocks, &transforms)
                .join()
                .map(|(block, block_transform): (&Block, &Transform)| {
                    (
                        Isometry2::new([block_transform.translation().x, block_transform.translation().y].into(), 0.0),
                        ShapeHandle::new(Cuboid::new([block.width / 2.0, block.height / 2.0].into())),
                    )
                })
                .collect(),
        );

        // Get block entities
        let block_entities: Vec<Entity> = (&entities, &blocks).join().map(|(entity, _)| entity).collect();

        if let Some(val) = (&paddles, &transforms).join().next().map(|(paddle, paddle_transform)| (paddle, paddle_transform.translation())) {
            let (paddle, paddle_translation): (&Paddle, &Vector3<f32>) = val;
            let paddle_x = paddle_translation.x;
            let paddle_y = paddle_translation.y;

            let ball_lines: Vec<(Entity, &mut Ball, &mut Transform, Entity)> = (&entities, &mut balls, !&sticky_balls, &mut transforms)
                .join()
                .zip((&entities, &attraction_lines).join())
                .map(|((ball_entity, ball, _, ball_transform), (attraction_line_entity, _))| (ball_entity, ball, ball_transform, attraction_line_entity))
                .collect();

            for (ball_entity, ball, ball_transform, attraction_line_entity) in ball_lines {
                let ball_x = ball_transform.translation().x;
                let ball_y = ball_transform.translation().y;

                // Bounce at the top, left and right of the arena
                if ball_x <= ball.radius {
                    ball.direction.as_mut_unchecked().x = ball.direction.x.abs();
                }
                if ball_x >= ARENA_WIDTH - ball.radius {
                    ball.direction.as_mut_unchecked().x = -ball.direction.x.abs();
                }
                if ball_y >= ARENA_HEIGHT - ball.radius {
                    ball.direction.as_mut_unchecked().y = -ball.direction.y.abs();
                }

                // Bounce at the paddle
                let mut bounced = false;
                let ball_shape = BallShape::new(ball.radius);
                let ball_pos = Isometry2::new([ball_x, ball_y].into(), 0.0);

                let paddle_shape = Cuboid::new([paddle.width / 2.0, paddle.height / 2.0].into());
                let paddle_pos = Isometry2::new([paddle_x, paddle_y].into(), 0.0);

                if query::contact(&paddle_pos, &paddle_shape, &ball_pos, &ball_shape, 0.0).is_some() {
                    bounced = true;
                    let angle = ((paddle_x - ball_transform.translation().x) / paddle.width * f32::pi()).min(f32::pi() / 3.0).max(-f32::pi() / 3.0);
                    ball.direction = Unit::new_unchecked([-angle.sin(), angle.cos()].into());

                    stop_ball_attraction_event_channel.single_write(StopBallAttractionEvent {
                        collision_time: time.absolute_time_seconds(),
                    });
                }

                // Lose a life when ball reach the bottom of the arena
                if ball_y <= ball.radius && !bounced {
                    ball.velocity_mult = 1.0;
                    ball_transform.set_translation_x(paddle_x);
                    ball_transform.set_translation_y(paddle.height + ball.radius);

                    sticky_balls.insert(ball_entity, StickyBall { period: 2.0 }).expect("Unable to add entity to storage.");

                    life_event_channel.single_write(LifeEvent);
                    score_event_channel.single_write(ScoreEvent { score: -1000 });

                    ball_attraction_vfx_event_channel.single_write(BallAttractionVfxEvent {
                        ball_entity,
                        ball_color: Srgba::new(1.0, 1.0, 1.0, 1.0),
                        attraction_line_entity,
                        attraction_line_color: Srgba::new(1.0, 1.0, 1.0, 0.0),
                    });
                }

                // Bounce at the blocks
                if let Some(contact) = query::contact(&Isometry2::identity(), &block_compound, &ball_pos, &ball_shape, 0.0) {
                    stop_ball_attraction_event_channel.single_write(StopBallAttractionEvent {
                        collision_time: time.absolute_time_seconds(),
                    });

                    let angle = (-ball.direction.perp(&contact.normal)).atan2(-ball.direction.dot(&contact.normal));
                    ball.direction = -(Rotation2::new(2.0 * angle) * ball.direction);
                    ball.direction.renormalize();

                    // Get individual collided blocks
                    if block_compound.shapes().len() == block_entities.len() {
                        for (index, shape) in block_compound.shapes().iter().enumerate() {
                            let (block_isometry, block): (&Isometry2<f32>, &Cuboid<f32>) = (&shape.0, shape.1.downcast_ref().unwrap());

                            if query::contact(block_isometry, block, &ball_pos, &ball_shape, 0.0).is_some() {
                                block_collision_event_channel.single_write(BlockCollisionEvent { entity: block_entities[index] });
                                score_event_channel.single_write(ScoreEvent { score: 50 });
                            }
                        }
                    }
                }
            }
        }
    }
}
