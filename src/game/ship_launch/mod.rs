use bevy::{prelude::*, utils::HashSet};
use iyes_loopless::prelude::IntoConditionalSystem;

use crate::{
    game::{
        actions::ShipSlotType,
        components::AnimateWithSpeed,
        spawners::{SHIP_SAILING_POSITION_Y, SHIP_SPEED},
    },
    loader::AnimationAssets,
    GameState, WIDTH,
};

use super::{
    actions::ShipSlots,
    animation::OnShipArrivedAtDestination,
    components::{Ship, ShipArriving, ShipDemandItemMarker, ShipHold, Wave},
    Animation, AnimationState, SystemLabels,
};

pub struct OnLaunchShip {
    pub slot_id: usize,
}

pub struct OnShipScore {
    pub ship_hold: ShipHold,
}

pub struct LaunchShipPlugin;

impl Plugin for LaunchShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OnLaunchShip>()
            .add_event::<OnShipScore>()
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
    mut launch_events: EventReader<OnLaunchShip>,
    mut score_events: EventWriter<OnShipScore>,
    animations: Res<AnimationAssets>,
    mut slots: ResMut<ShipSlots>,
    mut waves: Query<Entity, With<Wave>>,
    mut ships: Query<
        (
            &mut Handle<Animation>,
            &mut AnimationState,
            &ShipHold,
            &Parent,
            &Children,
        ),
        With<Ship>,
    >,
    markers: Query<Entity, &ShipDemandItemMarker>,
) {
    let mut launched = HashSet::<usize>::new();

    for evt in launch_events.iter() {
        if launched.contains(&evt.slot_id) {
            continue;
        }
        launched.insert(evt.slot_id);

        info!(
            "Handling launch of ship from slot {}, which is ship {:?}",
            evt.slot_id, slots.slots[evt.slot_id]
        );

        match slots.slots[evt.slot_id] {
            ShipSlotType::Occupied(se) => {
                let (mut ship_anim, mut anim_state, hold, parent, children) =
                    ships.get_mut(se).expect("Should find ship");

                // update the animation
                *ship_anim = animations.ship_unfurl.clone();
                anim_state.reset();

                // get the parent and add an animation to it to depart
                let wave_entity = waves
                    .get_mut(parent.get())
                    .expect("Ship needs a parent wave");
                commands.entity(wave_entity).insert(AnimateWithSpeed {
                    speed: SHIP_SPEED,
                    target: vec![Vec3::new(
                        0.7 * WIDTH,
                        SHIP_SAILING_POSITION_Y,
                        6.0 + evt.slot_id as f32 * 3.0,
                    )],
                });

                // empty the slot
                slots.slots[evt.slot_id] = ShipSlotType::Empty;

                // remove the demand markers
                for child in children.iter() {
                    if markers.contains(*child) {
                        commands.entity(*child).despawn();
                    }
                }

                score_events.send(OnShipScore {
                    ship_hold: hold.clone(),
                });
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
    mut slots: ResMut<ShipSlots>,
    animations: Res<AnimationAssets>,
    mut events: EventReader<OnShipArrivedAtDestination>,
    waves: Query<&Children, With<Wave>>,
    mut ships: Query<(
        Entity,
        &mut Handle<Animation>,
        &mut AnimationState,
        Option<&ShipArriving>,
    )>,
) {
    for evt in events.iter() {
        match waves.get(evt.0) {
            Ok(wave_children) => {
                for child in wave_children.iter() {
                    let (ship_ent, mut anim, mut anim_state, arriving) =
                        ships.get_mut(*child).expect("Should have a ship hold");

                    match arriving {
                        Some(arr) => {
                            info!("Ship arrived at slot {}", arr.0);

                            *anim = animations.ship_furl.clone();
                            anim_state.reset();
                            commands.entity(ship_ent).remove::<ShipArriving>();

                            slots.slots[arr.0] = ShipSlotType::Occupied(ship_ent);
                        }
                        None => {
                            info!("Despawning ship",);
                            commands.entity(evt.0).despawn_recursive();
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
