use crate::{BlockCollisionEvent, ScoreEvent};

use components::Block;
use resources::{Game, GameEvent, NUM_LIVES};

use amethyst::{
    derive::SystemDesc,
    ecs::prelude::*,
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
    type SystemData = (
        Write<'s, Game>,
        Entities<'s>,
        WriteStorage<'s, Block>,
        WriteStorage<'s, SpriteRender>,
        Read<'s, EventChannel<BlockCollisionEvent>>,
        Write<'s, EventChannel<ScoreEvent>>,
    );

    fn run(&mut self, (mut game, entities, mut blocks, mut sprites, block_collision_event_channel, mut score_event_channel): Self::SystemData) {
        for BlockCollisionEvent { entity } in block_collision_event_channel.read(&mut self.reader) {
            if let (Some(block), Some(sprite)) = (blocks.get_mut(*entity), sprites.get_mut(*entity)) {
                block.health -= 1.0;
                if block.health > 0.0 {
                    sprite.sprite_number += 6;
                } else {
                    entities.delete(*entity).expect("Failed to delete entity.");
                    score_event_channel.single_write(ScoreEvent { score: 50 });
                }
            }
        }

        let game_beginning = game.lives == NUM_LIVES && game.score == 0;
        if (&blocks).join().next().is_none() && !game_beginning {
            game.event = Some(GameEvent::LevelComplete);
        }
    }
}
