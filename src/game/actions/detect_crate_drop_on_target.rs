use bevy::prelude::*;
use heron::Collisions;

use crate::{
    game::components::{PhysicsCrate, ShipHold, Wave},
    HEIGHT, WIDTH,
};

/// Handles collisions between physics crates and ships
pub fn detect_crate_drop_on_ship(
    mut commands: Commands,
    box_collisions: Query<(Entity, &Collisions, &PhysicsCrate)>,
    ship_entities: Query<&Children, With<Wave>>,
    mut ship_holds: Query<&mut ShipHold>,
) {
    for (crate_entity, crate_collisions, physics_crate) in box_collisions.iter() {
        for collision in crate_collisions.entities() {
            match ship_entities.get(collision) {
                Ok(children) => {
                    // add the crate to the ship hold and despawn the physics crate
                    for child in children.iter() {
                        if let Ok(mut ship_hold) = ship_holds.get_mut(*child) {
                            ship_hold.crates.push(physics_crate.box_type);
                            commands.entity(crate_entity).despawn_recursive();
                            info!("Crate {:?} dropped on ship {:?}!", crate_entity, ship_hold);
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

pub fn detect_physics_crate_out_of_bounds(
    mut commands: Commands,
    crates: Query<(Entity, &Transform), With<PhysicsCrate>>,
) {
    for (crate_ent, crate_tx) in crates.iter() {
        if crate_tx.translation.x < -0.6 * WIDTH
            || crate_tx.translation.x > 0.6 * WIDTH
            || crate_tx.translation.y < -0.75 * HEIGHT
        {
            info!("Crate {:?} fell out of bounds, despawning", crate_ent);
            commands.entity(crate_ent).despawn_recursive();
        }
    }
}
