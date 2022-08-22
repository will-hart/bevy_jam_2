use bevy::{prelude::*, utils::HashSet};
use iyes_loopless::prelude::IntoConditionalSystem;

use crate::{
    game::components::{AnimateWithSpeed, ShipRespawnTimer},
    loader::AnimationAssets,
    GameState, GRID_SIZE, WIDTH,
};

use super::{
    actions::ShipSlots,
    animation::DespawnEntity,
    components::{Ship, ShipHold, ShipText, Wave},
    Animation, SystemLabels,
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
    mut waves: Query<(Entity, &mut Transform), With<Wave>>,
    mut ships: Query<(&mut Handle<Animation>, &Parent, &Children), With<Ship>>,
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

        match slots.slots[evt.slot_id].take() {
            Some(se) => {
                let (mut ship_anim, parent, children) =
                    ships.get_mut(se).expect("Should find ship");

                // update the animation
                *ship_anim = animations.ship_unfurl.clone();

                // get the parent and add an animation to it to depart
                let (wave_entity, mut wave_tx) = waves
                    .get_mut(parent.get())
                    .expect("Ship needs a parent wave");
                commands.entity(wave_entity).insert(AnimateWithSpeed {
                    speed: 20.0,
                    target: Vec2::new(0.7 * WIDTH, -10.0 * GRID_SIZE),
                });
                wave_tx.translation.z = 6.0 + evt.slot_id as f32 * 3.0; // not sure why this does prevent overlapping of waves + other ships :thinking:

                // hide the ship text
                for child in children.iter() {
                    if let Ok(text_ent) = ship_texts.get(*child) {
                        commands.entity(text_ent).despawn()
                    }
                }
            }
            None => {
                warn!("Attempted to depart ship that has already left");
            }
        }
    }
}

/// Despawns ships and sets them for respawn
pub fn ship_despawn(
    mut commands: Commands,
    time: Res<Time>,
    mut events: EventReader<DespawnEntity>,
    waves: Query<&Children, With<Wave>>,
    ships: Query<(&Ship, &ShipHold)>,
) {
    for evt in events.iter() {
        match waves.get(evt.0) {
            Ok(wave_children) => {
                for child in wave_children.iter() {
                    let (ship, hold) = ships.get(*child).expect("Should have a ship hold");
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
            _ => {
                // not a wave
                warn!("Somehow ship despawn event was called on not a ship :confused:")
            }
        }
    }
}
