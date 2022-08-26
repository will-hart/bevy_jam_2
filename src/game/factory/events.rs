use crate::game::components::BoxType;
use bevy::prelude::*;

pub struct OnDropInFactoryInput {
    pub box_type: BoxType,
}

pub struct OnFactoryStartProducing {
    pub box_type: BoxType,
}

pub struct OnFactoryFinishProducing {
    pub box_type: BoxType,
}

pub fn handle_drop_factory_input(mut drop_events: EventReader<OnDropInFactoryInput>) {
    for event in drop_events.iter() {
        info!(
            "Handling crate dropped in factory input: {:?}",
            event.box_type
        );
    }
}
