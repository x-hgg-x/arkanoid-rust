use crate::components::Block;
use crate::systems::BlockCollisionEvent;

use amethyst::{
    core::SystemDesc,
    derive::SystemDesc,
    ecs::{Entities, Read, System, SystemData, World, WriteStorage},
    prelude::*,
    renderer::SpriteRender,
    shrev::{EventChannel, ReaderId},
};

#[derive(SystemDesc)]
pub struct BlockHealthSystem {
    reader: ReaderId<BlockCollisionEvent>,
}

impl BlockHealthSystem {
    pub fn new(world: &mut World) -> Self {
        <Self as System>::SystemData::setup(world);
        Self {
            reader: world.write_resource::<EventChannel<BlockCollisionEvent>>().register_reader(),
        }
    }
}

impl<'s> System<'s> for BlockHealthSystem {
    type SystemData = (Entities<'s>, WriteStorage<'s, Block>, WriteStorage<'s, SpriteRender>, Read<'s, EventChannel<BlockCollisionEvent>>);

    fn run(&mut self, (entities, mut blocks, mut sprites, block_collision_event_channel): Self::SystemData) {
        for BlockCollisionEvent { entity } in block_collision_event_channel.read(&mut self.reader) {
            if let (Some(block), Some(sprite)) = (blocks.get_mut(*entity), sprites.get_mut(*entity)) {
                block.health -= 1.0;
                if block.health > 0.0 {
                    sprite.sprite_number += 6;
                } else {
                    entities.delete(*entity).expect("Failed to delete entity.");
                }
            }
        }
    }
}
