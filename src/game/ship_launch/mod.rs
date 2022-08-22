use bevy::{prelude::*, utils::HashSet};
use iyes_loopless::prelude::IntoConditionalSystem;

use crate::{
    game::{
        actions::ShipSlotType,
        components::{AnimateWithSpeed, ShipRespawnTimer},
    },
    loader::AnimationAssets,
    GameState, GRID_SIZE, WIDTH,
};

use super::{
    actions::ShipSlots,
    animation::ShipArrivedAtDestination,
    components::{Ship, ShipArriving, ShipHold, ShipText, Wave},
    Animation, AnimationState, SystemLabels,
};

pub struct LaunchShipEvent {
    pub slot_id: usize,
}

pub struct LaunchShipPlugin;

impl Plugin for LaunchShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LaunchShipEvent>()
            .add_system(trigger_launch.run_not_in_state(GameState::Loading))
            .add_system(
                ship_despawn
                    .run_in_state(GameState::Playing)
                    .after(SystemLabels::ShipAnimationAndDespawn),
            );
    }
}

fn trigger_launch(
    mut commands: Commands,
    mut events: EventReader<LaunchShipEvent>,
    animations: Res<AnimationAssets>,
    mut slots: ResMut<ShipSlots>,
    mut waves: Query<Entity, With<Wave>>,
    mut ships: Query<
        (
            &mut Handle<Animation>,
            &mut AnimationState,
            &Parent,
            &Children,
        ),
        With<Ship>,
    >,
    ship_texts: Query<Entity, With<ShipText>>,
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

        match slots.slots[evt.slot_id] {
            ShipSlotType::Occupied(se) => {
                let (mut ship_anim, mut anim_state, parent, children) =
                    ships.get_mut(se).expect("Should find ship");

                // update the animation
                *ship_anim = animations.ship_unfurl.clone();
                anim_state.reset();

                // get the parent and add an animation to it to depart
                let wave_entity = waves
                    .get_mut(parent.get())
                    .expect("Ship needs a parent wave");
                commands.entity(wave_entity).insert(AnimateWithSpeed {
                    speed: 20.0,
                    target: vec![Vec3::new(
                        0.7 * WIDTH,
                        -10.0 * GRID_SIZE,
                        6.0 + evt.slot_id as f32 * 3.0,
                    )],
                });

                // hide the ship text
                for child in children.iter() {
                    if let Ok(text_ent) = ship_texts.get(*child) {
                        commands.entity(text_ent).despawn()
                    }
                }

                // empty the slot
                slots.slots[evt.slot_id] = ShipSlotType::Empty;
            }
            _ => {
                warn!("Attempted to depart ship that has already left");
            }
        }
    }
}

/// Despawns ships and sets them for respawn
pub fn ship_despawn(
    mut commands: Commands,
    time: Res<Time>,
    mut slots: ResMut<ShipSlots>,
    animations: Res<AnimationAssets>,
    mut events: EventReader<ShipArrivedAtDestination>,
    waves: Query<&Children, With<Wave>>,
    mut ships: Query<(
        Entity,
        &Ship,
        &ShipHold,
        &mut Handle<Animation>,
        &mut AnimationState,
        Option<&ShipArriving>,
    )>,
) {
    for evt in events.iter() {
        match waves.get(evt.0) {
            Ok(wave_children) => {
                for child in wave_children.iter() {
                    let (ship_ent, ship, hold, mut anim, mut anim_state, arriving) =
                        ships.get_mut(*child).expect("Should have a ship hold");

                    match arriving {
                        Some(arr) => {
                            info!("Ship arrived at slot {}", arr.0);

                            *anim = animations.ship_furl.clone();
                            anim_state.reset();

                            slots.slots[arr.0] = ShipSlotType::Occupied(ship_ent);
                        }
                        None => {
                            info!(
                                "Despawning ship, setting it to respawn in {} seconds",
                                hold.destination.get_travel_duration()
                            );
                            commands.entity(evt.0).despawn_recursive();
                            commands.spawn().insert(ShipRespawnTimer {
                                ship_to_respawn: *ship,
                                respawn_at: time.time_since_startup().as_secs_f32()
                                    + hold.destination.get_travel_duration(),
                            });
                        }
                    }
                }
            }
            _ => {
                // not a wave
                warn!("Somehow ship despawn event was called on not a ship :confused:")
            }
        }
    }
}
