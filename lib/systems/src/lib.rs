#![allow(clippy::type_complexity)]

mod ball_attraction;
mod ball_attraction_vfx;
mod block_health;
mod collision;
mod life;
mod move_ball;
mod move_paddle;
mod score;
mod sticky_ball;

pub use ball_attraction::*;
pub use ball_attraction_vfx::*;
pub use block_health::*;
pub use collision::*;
pub use life::*;
pub use move_ball::*;
pub use move_paddle::*;
pub use score::*;
pub use sticky_ball::*;

use resources::CurrentState;

use amethyst::{
    core::SystemBundle,
    ecs::{DispatcherBuilder, Entity, World},
    prelude::*,
    renderer::palette::rgb::Srgba,
    utils::ortho_camera::CameraOrthoSystem,
    Error,
};

pub struct BlockCollisionEvent {
    pub entity: Entity,
}

pub struct LifeEvent;

pub struct ScoreEvent {
    pub score: i32,
}

pub struct StopBallAttractionEvent {
    pub collision_time: f64,
}

pub struct BallAttractionVfxEvent {
    pub ball_entity: Entity,
    pub ball_color: Srgba,
    pub attraction_line_entity: Entity,
    pub attraction_line_color: Srgba,
}

pub struct ArkanoidBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for ArkanoidBundle {
    fn build(self, world: &mut World, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<(), Error> {
        builder.add_thread_local(MovePaddleSystem.pausable(CurrentState::Running));
        builder.add(CameraOrthoSystem::default(), "", &[]);
        builder.add(StickyBallSystem::default().pausable(CurrentState::Running), "sticky_ball_system", &[]);
        builder.add(BallAttractionSystem::new(world).pausable(CurrentState::Running), "ball_attraction_system", &["sticky_ball_system"]);
        builder.add(BallAttractionVfxSystem::new(world).pausable(CurrentState::Running), "ball_attraction_vfx_system", &["ball_attraction_system"]);
        builder.add(MoveBallSystem.pausable(CurrentState::Running), "move_ball_system", &["ball_attraction_system"]);
        builder.add(CollisionSystem.pausable(CurrentState::Running), "collision_system", &["move_ball_system"]);
        builder.add(BlockHealthSystem::new(world).pausable(CurrentState::Running), "block_health_system", &["collision_system"]);
        builder.add(LifeSystem::new(world).pausable(CurrentState::Running), "life_system", &["collision_system"]);
        builder.add(ScoreSystem::new(world).pausable(CurrentState::Running), "score_system", &["collision_system"]);
        Ok(())
    }
}
