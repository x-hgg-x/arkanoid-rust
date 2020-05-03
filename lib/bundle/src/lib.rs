pub mod bindings;

use bindings::ArkanoidBindings;

use amethyst::{
    assets::Processor,
    controls::{CursorHideSystemDesc, MouseFocusUpdateSystemDesc},
    core::{SystemBundle, TransformBundle},
    ecs::{prelude::*, DispatcherBuilder, ReadExpect},
    error::Error,
    input::{Bindings, InputSystemDesc},
    prelude::*,
    renderer::types::DefaultBackend,
    renderer::{
        pass::DrawFlat2DDesc,
        rendy::{
            graph::present::PresentNode,
            hal::{
                command::{ClearDepthStencil, ClearValue},
                window::PresentMode,
            },
        },
        sprite_visibility::SpriteVisibilitySortingSystem,
        Factory, Format, GraphBuilder, GraphCreator, Kind, RenderGroupDesc, RenderingSystem, SpriteSheet, SubpassBuilder, TextureProcessorSystem,
    },
    ui::{
        BlinkSystem, CacheSelectionOrderSystem, DragWidgetSystemDesc, DrawUiDesc, FontAsset, NoCustomUi, ResizeSystemDesc, SelectionKeyboardSystemDesc, SelectionMouseSystemDesc, TextEditingInputSystemDesc,
        TextEditingMouseSystemDesc, ToNativeWidget, UiButtonActionRetriggerSystemDesc, UiButtonSystemDesc, UiGlyphsSystemDesc, UiLoaderSystemDesc, UiMouseSystem, UiSoundRetriggerSystemDesc, UiSoundSystemDesc,
        UiTransformSystemDesc,
    },
    window::{ScreenDimensions, Window, WindowBundle},
};

pub struct StartingBundle {
    pub bindings_config_path: String,
    pub display_config_path: String,
}

impl<'a, 'b> SystemBundle<'a, 'b> for StartingBundle {
    fn build(self, world: &mut World, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<(), Error> {
        // TransformBundle
        TransformBundle::default().build(world, builder)?;

        // UiBundle systems
        builder.add(UiLoaderSystemDesc::<<NoCustomUi as ToNativeWidget>::PrefabData, u32>::default().build(world), "ui_loader", &[]);
        builder.add(UiTransformSystemDesc::default().build(world), "ui_transform", &["transform_system"]);
        builder.add(Processor::<FontAsset>::new(), "font_processor", &["ui_loader"]);
        builder.add(CacheSelectionOrderSystem::<()>::new(), "selection_order_cache", &[]);
        builder.add(SelectionMouseSystemDesc::<(), ArkanoidBindings>::default().build(world), "ui_mouse_selection", &[]);
        builder.add(SelectionKeyboardSystemDesc::<()>::default().build(world), "ui_keyboard_selection", &["ui_mouse_selection"]);
        builder.add(TextEditingMouseSystemDesc::default().build(world), "ui_text_editing_mouse_system", &["ui_mouse_selection", "ui_keyboard_selection"]);
        builder.add(TextEditingInputSystemDesc::default().build(world), "ui_text_editing_input_system", &["ui_mouse_selection", "ui_keyboard_selection"]);
        builder.add(ResizeSystemDesc::default().build(world), "ui_resize_system", &[]);
        builder.add(UiButtonSystemDesc::default().build(world), "ui_button_system", &[]);
        builder.add(DragWidgetSystemDesc::<ArkanoidBindings>::default().build(world), "ui_drag_system", &[]);
        builder.add(UiButtonActionRetriggerSystemDesc::default().build(world), "ui_button_action_retrigger_system", &["ui_button_system"]);
        builder.add(UiSoundSystemDesc::default().build(world), "ui_sound_system", &[]);
        builder.add(UiSoundRetriggerSystemDesc::default().build(world), "ui_sound_retrigger_system", &["ui_sound_system"]);
        builder.add(BlinkSystem, "blink_system", &[]);

        // Other various systems
        builder.add(UiGlyphsSystemDesc::<DefaultBackend>::default().build(world), "ui_glyphs_system", &[]);
        builder.add(Processor::<SpriteSheet>::new(), "sprite_sheet_processor", &[]);
        builder.add(SpriteVisibilitySortingSystem::new(), "sprite_visibility_sorting_system", &[]);
        builder.add(TextureProcessorSystem::<DefaultBackend>::default(), "texture_processor", &[]);
        builder.add(MouseFocusUpdateSystemDesc::default().build(world), "mouse_focus", &[]);
        builder.add(CursorHideSystemDesc::default().build(world), "cursor_hide", &["mouse_focus"]);
        WindowBundle::from_config_path(self.display_config_path)?.build(world, builder)?;
        builder.add_thread_local(RenderingSystem::<DefaultBackend, _>::new(RenderGraph::default()));

        // Input system and depending systems
        builder.add_thread_local(InputSystemDesc::<ArkanoidBindings>::new(Some(Bindings::load(self.bindings_config_path)?)).build(world));
        builder.add_thread_local(UiMouseSystem::<ArkanoidBindings>::new());

        Ok(())
    }
}

#[derive(Default)]
struct RenderGraph {
    dimensions: Option<ScreenDimensions>,
    dirty: bool,
}

impl GraphCreator<DefaultBackend> for RenderGraph {
    fn rebuild(&mut self, world: &World) -> bool {
        // Rebuild when dimensions change, but wait until at least two frames have the same.
        let new_dimensions = world.try_fetch::<ScreenDimensions>();
        use std::ops::Deref;
        if self.dimensions.as_ref() != new_dimensions.as_deref() {
            self.dirty = true;
            self.dimensions = new_dimensions.map(|d| d.deref().clone());
            return false;
        }
        self.dirty
    }

    fn builder(&mut self, factory: &mut Factory<DefaultBackend>, world: &World) -> GraphBuilder<DefaultBackend, World> {
        self.dirty = false;

        // Retrieve a reference to the target window, which is created by the WindowBundle
        let window = <ReadExpect<Window>>::fetch(world);
        let dimensions = self.dimensions.as_ref().unwrap();
        let window_kind = Kind::D2(dimensions.width() as u32, dimensions.height() as u32, 1, 1);

        // Create a new drawing surface in our window
        let surface = factory.create_surface(&window);
        let surface_format = factory.get_surface_format(&surface);

        // Begin building our RenderGraph
        let mut graph_builder = GraphBuilder::new();
        let color = graph_builder.create_image(window_kind, 1, surface_format, Some(ClearValue::Color([0.0, 0.0, 0.0, 1.0].into())));

        let depth = graph_builder.create_image(window_kind, 1, Format::D32Sfloat, Some(ClearValue::DepthStencil(ClearDepthStencil(1.0, 0))));

        let pass = graph_builder.add_node(
            SubpassBuilder::new()
                // Add DrawFlat2DDesc for drawing sprites
                .with_group(DrawFlat2DDesc::default().builder())
                // Add DrawUiDesc for drawing UI
                .with_group(DrawUiDesc::default().builder())
                .with_color(color)
                .with_depth_stencil(depth)
                .into_pass(),
        );

        let mut present_builder = PresentNode::builder(factory, surface, color);

        // Choose present mode priority (set Mailbox to highest priority)
        present_builder = present_builder.with_present_modes_priority(|mode| match mode {
            PresentMode::Mailbox => Some(1),
            PresentMode::Fifo => Some(0),
            _ => None,
        });

        // Finally, add the pass to the graph
        let _present = graph_builder.add_node(present_builder.with_dependency(pass));

        graph_builder
    }
}
