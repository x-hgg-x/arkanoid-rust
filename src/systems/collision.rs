use crate::components::{Ball, Block, Paddle, StickyBall};
use crate::states::{ARENA_HEIGHT, ARENA_WIDTH};

use amethyst::{
    core::{
        math::{self, Isometry2, Rotation2, Vector2, Vector3},
        Transform,
    },
    derive::SystemDesc,
    ecs::{Entities, Entity, Join, ReadStorage, System, SystemData as _, Write, WriteStorage},
    shrev::EventChannel,
};

use ncollide2d::{query, shape::Cuboid};

use std::f32::consts::PI;

pub struct BlockCollisionEvent {
    pub entity: Entity,
}

pub struct LifeEvent;

pub struct ScoreEvent {
    pub score: i32,
}

#[derive(SystemDesc)]
pub struct CollisionSystem;

type SystemData<'s> = (
    Entities<'s>,
    WriteStorage<'s, Ball>,
    WriteStorage<'s, StickyBall>,
    WriteStorage<'s, Transform>,
    ReadStorage<'s, Paddle>,
    ReadStorage<'s, Block>,
    Write<'s, EventChannel<BlockCollisionEvent>>,
    Write<'s, EventChannel<LifeEvent>>,
    Write<'s, EventChannel<ScoreEvent>>,
);

impl<'s> System<'s> for CollisionSystem {
    type SystemData = SystemData<'s>;

    fn run(&mut self, (entities, mut balls, mut sticky_balls, mut transforms, paddles, blocks, mut block_collision_event_channel, mut life_event_channel, mut score_event_channel): SystemData) {
        // Get blocks with translation and entity
        let blocks_entities_translations: Vec<_> = (&blocks, &entities, &transforms)
            .join()
            .map(|(block, entity, block_transform)| (block, entity, *block_transform.translation()))
            .collect();

        if let Some(val) = (&paddles, &transforms).join().next() {
            let (paddle, paddle_transform): (&Paddle, &Transform) = val;

            let paddle_x = paddle_transform.translation().x;
            let paddle_y = paddle_transform.translation().y;

            for val in (&entities, &mut balls, &mut transforms).join() {
                let (entity, ball, ball_transform): (Entity, &mut Ball, &mut Transform) = val;

                let ball_x = ball_transform.translation().x;
                let ball_y = ball_transform.translation().y;

                // Bounce at the top, left and right of the arena
                if ball_x <= ball.radius {
                    ball.direction.x = ball.direction.x.abs();
                }
                if ball_x >= ARENA_WIDTH - ball.radius {
                    ball.direction.x = -ball.direction.x.abs();
                }
                if ball_y >= ARENA_HEIGHT - ball.radius {
                    ball.direction.y = -ball.direction.y.abs();
                }

                // Lose a life when ball reach the bottom of the arena
                if ball_y <= ball.radius {
                    let sticky = StickyBall {
                        width_extent: paddle.width / 2.0,
                        period: 2.0,
                    };

                    sticky_balls.insert(entity, sticky).expect("Unable to add entity to storage.");
                    ball_transform.set_translation_xyz(paddle_x, paddle.height + ball.radius, 0.0);
                    life_event_channel.single_write(LifeEvent);
                    score_event_channel.single_write(ScoreEvent { score: -1000 });
                }

                // Bounce at the paddle
                let ball_shape = Cuboid::new(Vector2::new(ball.radius, ball.radius));
                let ball_pos = Isometry2::new(Vector2::new(ball_x, ball_y), math::zero());

                let paddle_shape = Cuboid::new(Vector2::new(paddle.width / 2.0, paddle.height / 2.0));
                let paddle_pos = Isometry2::new(Vector2::new(paddle_x, paddle_y), math::zero());

                if query::contact(&paddle_pos, &paddle_shape, &ball_pos, &ball_shape, 0.0).is_some() {
                    let angle = ((paddle_x - ball_transform.translation().x) / paddle.width * PI).min(PI / 3.0).max(-PI / 3.0);
                    ball.direction.x = -angle.sin();
                    ball.direction.y = angle.cos();
                }

                // Bounce at the blocks
                for val in &blocks_entities_translations {
                    let (block, entity, block_translation): &(&Block, Entity, Vector3<f32>) = val;
                    let block_shape = Cuboid::new(Vector2::new(block.width / 2.0, block.height / 2.0));
                    let block_pos = Isometry2::new(Vector2::new(block_translation.x, block_translation.y), math::zero());

                    if let Some(mut contact) = query::contact(&block_pos, &block_shape, &ball_pos, &ball_shape, 0.0) {
                        contact.normal.renormalize();
                        let angle = (-ball.direction.perp(&contact.normal)).atan2(-ball.direction.dot(&contact.normal));
                        ball.direction = -(Rotation2::new(2.0 * angle) * ball.direction).normalize();
                        block_collision_event_channel.single_write(BlockCollisionEvent { entity: *entity });
                        score_event_channel.single_write(ScoreEvent { score: 50 });
                        break;
                    }
                }
            }
        }
    }
}
