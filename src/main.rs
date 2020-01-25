mod components;
mod states;

use components::CameraPrefabData;
use states::LoadingState;

use arkanoid_precompile::PrecompiledBundle;

use amethyst::{assets::PrefabLoaderSystemDesc, core::frame_limiter::FrameRateLimitConfig, prelude::*, renderer::sprite::prefab::SpriteScenePrefab};

fn main() -> amethyst::Result<()> {
    amethyst::Logger::from_config(Default::default()).level_for("gfx_backend_vulkan", amethyst::LogLevelFilter::Off).start();

    let game_data = GameDataBuilder::new()
        .with_bundle(PrecompiledBundle {
            bindings_config_path: String::from("config/bindings.ron"),
            display_config_path: String::from("config/display.ron"),
        })?
        .with_system_desc(PrefabLoaderSystemDesc::<CameraPrefabData>::default(), "", &[])
        .with_system_desc(PrefabLoaderSystemDesc::<SpriteScenePrefab>::default(), "", &[]);

    let frame_limiter_config = FrameRateLimitConfig::load("config/frame_limiter.ron");

    Application::build("assets", LoadingState::default())?
        .with_frame_limit_config(frame_limiter_config)
        .build(game_data)?
        .run();

    Ok(())
}
