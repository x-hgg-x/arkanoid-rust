use amethyst::{
    assets::{PrefabData, ProgressCounter},
    core::math::{Unit, Vector2},
    derive::PrefabData,
    ecs::{Component, DenseVecStorage, Entity, NullStorage, WriteStorage},
    renderer::{resources::Tint, sprite::prefab::SpriteScenePrefab},
    Error,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Component, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
#[serde(deny_unknown_fields)]
pub struct Paddle {
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone, Component, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
#[serde(deny_unknown_fields)]
pub struct Ball {
    pub radius: f32,
    pub velocity: f32,
    pub velocity_mult: f32,
    pub direction: Unit<Vector2<f32>>,
}

#[derive(Debug, Clone, Component, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
#[serde(deny_unknown_fields)]
pub struct StickyBall {
    pub period: f32,
}

#[derive(Debug, Default, Clone, Component, Deserialize, Serialize, PrefabData)]
#[storage(NullStorage)]
#[prefab(Component)]
#[serde(deny_unknown_fields)]
pub struct AttractionLine;

#[derive(Debug, Clone, Component, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
#[serde(deny_unknown_fields)]
pub struct Block {
    pub width: f32,
    pub height: f32,
    pub health: f32,
}

#[derive(Debug, Clone, Deserialize, Serialize, PrefabData)]
#[serde(deny_unknown_fields)]
pub enum ArkanoidElementPrefab {
    Paddle(Paddle),
    Ball { ball: Ball, sticky: Option<StickyBall> },
    AttractionLine(AttractionLine),
    Block(Block),
}

#[derive(Debug, Clone, Deserialize, Serialize, PrefabData)]
#[serde(deny_unknown_fields, transparent)]
pub struct TintPrefab {
    #[prefab(Component)]
    tint: Tint,
}

#[derive(Debug, Clone, Deserialize, Serialize, PrefabData)]
#[serde(deny_unknown_fields)]
pub struct ArkanoidPrefabData {
    sprite_scene: SpriteScenePrefab,
    tint: Option<TintPrefab>,
    element: ArkanoidElementPrefab,
}
