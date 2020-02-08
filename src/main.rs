use bundle::StartingBundle;
use components::{ArkanoidPrefabData, CameraPrefabData};
use resources::CurrentState;
use states::LoadingState;
use systems::ArkanoidBundle;

use amethyst::{assets::PrefabLoaderSystemDesc, core::frame_limiter::FrameRateLimitConfig, prelude::*, renderer::sprite::prefab::SpriteScenePrefab, LoggerConfig};

fn main() -> amethyst::Result<()> {
    amethyst::Logger::from_config(LoggerConfig::load("config/logger.ron")?).start();

    let game_data = GameDataBuilder::new()
        .with_bundle(StartingBundle {
            bindings_config_path: String::from("config/bindings.ron"),
            display_config_path: String::from("config/display.ron"),
        })?
        .with_bundle(ArkanoidBundle)?
        .with_system_desc(PrefabLoaderSystemDesc::<CameraPrefabData>::default(), "", &[])
        .with_system_desc(PrefabLoaderSystemDesc::<SpriteScenePrefab>::default(), "", &[])
        .with_system_desc(PrefabLoaderSystemDesc::<ArkanoidPrefabData>::default(), "", &[]);

    Application::build("assets", LoadingState::default())?
        .with_frame_limit_config(FrameRateLimitConfig::load("config/frame_limiter.ron")?)
        .with_resource(CurrentState::default())
        .build(game_data)?
        .run();

    Ok(())
}
