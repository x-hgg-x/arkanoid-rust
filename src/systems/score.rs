use crate::resources::Game;
use crate::states::SCORE_TEXT_ID;
use crate::systems::ScoreEvent;

use amethyst::{
    derive::SystemDesc,
    ecs::{Read, System, SystemData as _, World, Write, WriteStorage},
    prelude::*,
    shrev::{EventChannel, ReaderId},
    ui::{UiFinder, UiText},
};

#[derive(SystemDesc)]
pub struct ScoreSystem {
    reader: ReaderId<ScoreEvent>,
}

impl ScoreSystem {
    pub fn new(world: &mut World) -> Self {
        <Self as System>::SystemData::setup(world);
        Self {
            reader: world.write_resource::<EventChannel<ScoreEvent>>().register_reader(),
        }
    }
}

type SystemData<'s> = (Write<'s, Game>, WriteStorage<'s, UiText>, UiFinder<'s>, Read<'s, EventChannel<ScoreEvent>>);

impl<'s> System<'s> for ScoreSystem {
    type SystemData = SystemData<'s>;

    fn run(&mut self, (mut game, mut ui_texts, ui_finder, score_event_channel): SystemData) {
        for ScoreEvent { score } in score_event_channel.read(&mut self.reader) {
            game.score += score;

            if let Some(ui_text) = ui_finder.find(SCORE_TEXT_ID).and_then(|entity| ui_texts.get_mut(entity)) {
                ui_text.text = format!("SCORE: {}", game.score);
            }
        }
    }
}
