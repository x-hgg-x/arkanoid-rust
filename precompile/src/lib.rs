pub mod bindings;

use bindings::ArkanoidBindings;

use amethyst::{
    core::{SystemBundle, TransformBundle},
    ecs::DispatcherBuilder,
    error::Error,
    input::InputBundle,
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::fps_counter::FpsCounterBundle,
};

pub struct PrecompiledBundle {
    pub bindings_config_path: String,
    pub display_config_path: String,
}

impl<'a, 'b> SystemBundle<'a, 'b> for PrecompiledBundle {
    fn build(self, world: &mut World, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<(), Error> {
        RenderingBundle::<DefaultBackend>::new()
            .with_plugin(RenderToWindow::from_config_path(self.display_config_path)?.with_clear([0.0, 0.0, 0.0, 1.0]))
            .with_plugin(RenderFlat2D::default())
            .with_plugin(RenderUi::default())
            .build(world, builder)?;

        FpsCounterBundle::default().build(world, builder)?;
        TransformBundle::default().build(world, builder)?;
        InputBundle::<ArkanoidBindings>::new().with_bindings_from_file(self.bindings_config_path)?.build(world, builder)?;
        UiBundle::<ArkanoidBindings>::new().build(world, builder)?;
        Ok(())
    }
}
