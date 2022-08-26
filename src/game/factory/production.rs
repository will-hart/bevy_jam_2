use bevy::prelude::*;

use crate::game::components::BoxType;

use super::{recipes::Recipes, OnDropInFactoryInput};

#[derive(Default, Debug)]
pub struct Factory {
    pub inputs: [Option<BoxType>; 2],
}

impl Factory {
    pub fn drop(&mut self, box_type: BoxType) {
        if self.inputs[0].is_none() {
            self.inputs[0] = Some(box_type);
        } else {
            self.inputs[1] = Some(box_type);
        }
    }

    pub fn reset(&mut self) {
        self.inputs[0] = None;
        self.inputs[1] = None;
    }

    pub fn is_complete(&self) -> bool {
        self.inputs[0].is_some() && self.inputs[1].is_some()
    }
}

pub fn add_item_to_factory(
    recipes: Res<Recipes>,
    mut factory: ResMut<Factory>,
    mut drop_events: EventReader<OnDropInFactoryInput>,
) {
    for drop_event in drop_events.iter() {
        factory.drop(drop_event.box_type);

        if factory.is_complete() {
            let bt = if let Some(bt) = recipes.get_output(&factory.inputs) {
                bt
            } else {
                info!("No recipe available yet, factory contains {:?}", factory);
                return;
            };

            info!(
                "Factory receive two inputs {:?} and {:?} which has a recipe of {:?}",
                factory.inputs[0], factory.inputs[1], bt
            );

            factory.reset();
        }
    }
}
