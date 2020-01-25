use crate::components::*;

use amethyst::{
    assets::{Handle, Prefab},
    renderer::sprite::prefab::SpriteScenePrefab,
    ui::UiPrefab,
};

#[derive(Clone)]
pub struct MenuPrefabHandles {
    pub main_menu: Handle<UiPrefab>,
}

#[derive(Clone)]
pub struct GamePrefabHandles {
    pub camera: Handle<Prefab<CameraPrefabData>>,
    pub background: Handle<Prefab<SpriteScenePrefab>>,
    pub level: Handle<Prefab<SpriteScenePrefab>>,
}

#[derive(Clone)]
pub struct PrefabHandles {
    pub menu: MenuPrefabHandles,
    pub game: GamePrefabHandles,
}
