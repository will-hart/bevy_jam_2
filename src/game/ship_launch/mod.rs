use bevy::{prelude::*, utils::HashSet};
use iyes_loopless::prelude::IntoConditionalSystem;

use crate::{
    game::components::AnimateWithSpeed, loader::AnimationAssets, GameState, GRID_SIZE, WIDTH,
};

use super::{
    actions::ShipSlots,
    components::{Ship, Wave},
    Animation,
};

pub struct LaunchShipEvent {
    pub slot_id: usize,
}

pub struct LaunchShipPlugin;

impl Plugin for LaunchShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LaunchShipEvent>()
            .add_system(trigger_launch.run_not_in_state(GameState::Loading));
    }
}

fn trigger_launch(
    mut commands: Commands,
    mut events: EventReader<LaunchShipEvent>,
    animations: Res<AnimationAssets>,
    mut slots: ResMut<ShipSlots>,
    mut waves: Query<(Entity, &mut Transform), With<Wave>>,
    mut ships: Query<(&mut Handle<Animation>, &Parent), With<Ship>>,
) {
    let mut launched = HashSet::<usize>::new();

    for evt in events.iter() {
        if launched.contains(&evt.slot_id) {
            continue;
        }
        launched.insert(evt.slot_id);

        info!(
            "Launching ship from slot {}, which is ship {:?}",
            evt.slot_id, slots.slots[evt.slot_id]
        );

        match slots.slots[evt.slot_id].take() {
            Some(se) => {
                let (mut ship_anim, parent) = ships.get_mut(se).expect("Should find ship");

                // update the animation
                *ship_anim = animations.ship_unfurl.clone();

                // get the parent and add an animation to it to depart
                let (wave_entity, mut wave_tx) = waves
                    .get_mut(parent.get())
                    .expect("Ship needs a parent wave");
                commands.entity(wave_entity).insert(AnimateWithSpeed {
                    speed: 20.0,
                    target: Vec2::new(WIDTH, -10.0 * GRID_SIZE),
                });
                wave_tx.translation.z = 6.0 + evt.slot_id as f32 * 3.0; // not sure why this does prevent overlapping of waves + other ships :thinking:
            }
            None => {
                warn!("Attempted to depart ship that has already left");
            }
        }
    }
}
