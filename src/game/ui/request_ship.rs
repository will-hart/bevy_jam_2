use bevy::prelude::*;

use crate::{
    game::{
        actions::{ShipSlotType, ShipSlots},
        components::{RequestShip, TutorialMarker},
        custom_sprite::CustomSpriteMaterial,
        spawners::spawn_ship,
    },
    loader::{AnimationAssets, TextureAssets},
};

pub struct OnRequestShipSpawn(pub Entity, pub RequestShip);

pub fn ship_spawn_handler(
    mut commands: Commands,
    mut slots: ResMut<ShipSlots>,
    textures: Res<TextureAssets>,
    animations: Res<AnimationAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomSpriteMaterial>>,
    mut spawn_events: EventReader<OnRequestShipSpawn>,
    tutorial_markers: Query<Entity, With<TutorialMarker>>,
) {
    let mut spawned = false;

    for evt in spawn_events.iter() {
        // find a free slot
        let (slot_id, _) = match slots.slots.iter().enumerate().find(|(_, s)| match s {
            ShipSlotType::Empty => true,
            _ => false,
        }) {
            Some(s) => s,
            None => {
                info!("No free slots to spawn");
                return;
            }
        };

        // spawn the ship
        slots.slots[slot_id] = ShipSlotType::Arriving(spawn_ship(
            &mut commands,
            &textures,
            &animations,
            &mut meshes,
            &mut materials,
            slot_id,
            evt.1.clone(),
        ));

        // tidy up the button
        commands.entity(evt.0).despawn_recursive();

        spawned = true;
    }

    if spawned {
        for ent in tutorial_markers.iter() {
            commands.entity(ent).despawn_recursive();
        }
    }
}
