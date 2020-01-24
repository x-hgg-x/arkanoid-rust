mod components;
mod states;

use states::LoadingState;

use arkanoid_precompile::PrecompiledBundle;

use amethyst::{core::frame_limiter::FrameRateLimitConfig, prelude::*};

fn main() -> amethyst::Result<()> {
    amethyst::Logger::from_config(Default::default()).level_for("gfx_backend_vulkan", amethyst::LogLevelFilter::Off).start();

    let game_data = GameDataBuilder::new().with_bundle(PrecompiledBundle {
        bindings_config_path: String::from("config/bindings.ron"),
        display_config_path: String::from("config/display.ron"),
    })?;

    let frame_limiter_config = FrameRateLimitConfig::load("config/frame_limiter.ron");

    Application::build("assets", LoadingState::default())?
        .with_frame_limit_config(frame_limiter_config)
        .build(game_data)?
        .run();

    Ok(())
}
