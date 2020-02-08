use crate::*;

use amethyst::{
    assets::{Handle, Prefab},
    renderer::sprite::prefab::SpriteScenePrefab,
    ui::UiPrefab,
};

#[derive(Clone)]
pub struct MenuPrefabHandles {
    pub main_menu: Handle<UiPrefab>,
    pub pause_menu: Handle<UiPrefab>,
    pub game_over_menu: Handle<UiPrefab>,
    pub level_complete_menu: Handle<UiPrefab>,
}

#[derive(Clone)]
pub struct GamePrefabHandles {
    pub camera: Handle<Prefab<CameraPrefabData>>,
    pub background: Handle<Prefab<SpriteScenePrefab>>,
    pub level: Handle<Prefab<ArkanoidPrefabData>>,
    pub score: Handle<UiPrefab>,
    pub life: Handle<UiPrefab>,
}

#[derive(Clone)]
pub struct PrefabHandles {
    pub menu: MenuPrefabHandles,
    pub game: GamePrefabHandles,
}
