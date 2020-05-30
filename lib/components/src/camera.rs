use amethyst::{
    assets::{PrefabData, ProgressCounter},
    core::Transform,
    derive::PrefabData,
    ecs::Entity,
    renderer::camera::CameraPrefab,
    utils::ortho_camera::CameraOrtho,
    Error,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PrefabData)]
#[serde(deny_unknown_fields)]
pub struct CameraPrefabData {
    camera: CameraPrefab,
    camera_ortho: CameraOrtho,
    transform: Transform,
}
