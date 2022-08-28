use crate::{
    game::{
        components::{BoxType, FactoryGraphic},
        Animation,
    },
    loader::AnimationAssets,
};
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

pub struct OnFactoryQueueItem {
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

pub fn show_factory_on_animation(
    animations: Res<AnimationAssets>,
    mut items: Query<&mut Handle<Animation>, With<FactoryGraphic>>,
) {
    for mut item in items.iter_mut() {
        *item = animations.factory_on.clone().into();
    }
}

pub fn show_factory_off_animation(
    animations: Res<AnimationAssets>,
    mut items: Query<&mut Handle<Animation>, With<FactoryGraphic>>,
) {
    for mut item in items.iter_mut() {
        *item = animations.factory_off.clone().into();
    }
}
