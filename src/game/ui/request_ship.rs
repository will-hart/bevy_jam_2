use bevy::prelude::*;

use crate::{
    game::{
        actions::{ShipSlotType, ShipSlots},
        components::{CountDownTimer, RequestShip, TutorialMarker},
        custom_sprite::CustomSpriteMaterial,
        spawners::spawn_ship,
    },
    loader::{AnimationAssets, FontAssets, TextureAssets},
};

use super::CurrentTutorialLevel;

pub struct OnRequestShipSpawn {
    pub request_button_entity: Entity,
    pub request_ship: RequestShip,
}

/// Expires ship requests when their timer is out, setting their destination to None so that we
/// know they're an unmet ship
#[allow(clippy::too_many_arguments)]
pub fn ship_request_expiry_system(
    mut commands: Commands,
    tutorial: Res<CurrentTutorialLevel>,
    requests: Query<(&CountDownTimer, &Parent)>,
) {
    if tutorial.0 < 3 {
        return;
    }

    for (timer, parent) in requests.iter() {
        if timer.0.just_finished() {
            info!("Expiring a ship spawn request");
            commands.entity(parent.get()).despawn_recursive();
        }
    }
}

/// handles spawning a ship - triggered by the [OnRequestShipSpawn] event sent from
/// the [crate::game::ui::launch_ships::button_interaction] system
#[allow(clippy::too_many_arguments)]
pub fn ship_spawn_handler(
    mut commands: Commands,
    tutorial: Res<CurrentTutorialLevel>,
    textures: Res<TextureAssets>,
    animations: Res<AnimationAssets>,
    fonts: Res<FontAssets>,
    mut slots: ResMut<ShipSlots>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomSpriteMaterial>>,
    mut spawn_events: EventReader<OnRequestShipSpawn>,
    tutorial_markers: Query<(Entity, &TutorialMarker)>,
) {
    let mut spawned = false;

    for evt in spawn_events.iter() {
        // find a free slot
        let (slot_id, _) = match slots
            .slots
            .iter()
            .enumerate()
            .find(|(_, s)| matches!(s, ShipSlotType::Empty))
        {
            Some(s) => s,
            None => {
                info!("No free slots to spawn");
                return;
            }
        };

        // spawn the ship
        slots.slots[slot_id] = ShipSlotType::Arriving(spawn_ship(
            &mut commands,
            tutorial.0,
            &textures,
            &animations,
            &fonts,
            &mut meshes,
            &mut materials,
            slot_id,
            evt.request_ship.clone(),
        ));

        // tidy up the button
        commands
            .entity(evt.request_button_entity)
            .despawn_recursive();

        spawned = true;
    }

    if spawned {
        for (ent, marker) in tutorial_markers.iter() {
            // ensure level 0 tutorial is hidden
            if marker.0 == 0 {
                commands.entity(ent).despawn_recursive();
            }
        }
    }
}
