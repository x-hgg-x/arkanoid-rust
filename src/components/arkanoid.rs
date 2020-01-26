use amethyst::{
    assets::{PrefabData, ProgressCounter},
    core::math::Vector2,
    derive::PrefabData,
    ecs::{Component, DenseVecStorage, Entity, WriteStorage},
    renderer::sprite::prefab::SpriteScenePrefab,
    Error,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Component, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
#[serde(deny_unknown_fields)]
pub struct PlayerPaddle {
    pub width: f32,
    pub height: f32,
    pub velocity: f32,
    pub health: f32,
}

#[derive(Debug, Clone, Component, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
#[serde(deny_unknown_fields)]
pub struct Ball {
    pub velocity: Vector2<f32>,
    pub radius: f32,
}

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
    PlayerPaddle(PlayerPaddle),
    Ball(Ball),
    Block(Block),
}

#[derive(Debug, Clone, Deserialize, Serialize, PrefabData)]
#[serde(deny_unknown_fields)]
pub struct ArkanoidPrefabData {
    sprite_scene: SpriteScenePrefab,
    element: ArkanoidElementPrefab,
}
