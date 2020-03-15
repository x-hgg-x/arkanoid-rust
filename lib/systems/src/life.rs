use crate::LifeEvent;

use resources::{Game, GameEvent, LIFE_TEXT_ID};

use amethyst::{
    derive::SystemDesc,
    ecs::prelude::*,
    shrev::{EventChannel, ReaderId},
    ui::{UiFinder, UiText},
};

#[derive(SystemDesc)]
pub struct LifeSystem {
    reader: ReaderId<LifeEvent>,
}

impl LifeSystem {
    pub fn new(world: &mut World) -> Self {
        <Self as System>::SystemData::setup(world);
        Self {
            reader: world.write_resource::<EventChannel<LifeEvent>>().register_reader(),
        }
    }
}

impl<'s> System<'s> for LifeSystem {
    type SystemData = (Write<'s, Game>, WriteStorage<'s, UiText>, UiFinder<'s>, Read<'s, EventChannel<LifeEvent>>);

    fn run(&mut self, (mut game, mut ui_texts, ui_finder, life_event_channel): Self::SystemData) {
        for _event in life_event_channel.read(&mut self.reader) {
            game.lives -= 1;

            if let Some(ui_text) = ui_finder.find(LIFE_TEXT_ID).and_then(|entity| ui_texts.get_mut(entity)) {
                ui_text.text = format!("LIVES: {}", game.lives);
            }

            if game.lives <= 0 {
                game.event = Some(GameEvent::GameOver);
            }
        }
    }
}
