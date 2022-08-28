use bevy::prelude::*;
use heron::Collisions;

use crate::{
    game::{
        components::{FactoryInput, PhysicsCrate, ShipHold, SplashCatcher, Wave},
        factory::OnDropInFactoryInput,
    },
    HEIGHT, WIDTH,
};

use super::dropping::OnDropCrateOnShip;

pub struct OnCrateSplashedInWater(pub Vec2);

/// Handles collisions between physics crates and ships
#[allow(clippy::too_many_arguments)]
pub fn detect_crate_drop_on_ship(
    mut commands: Commands,
    mut factory_event: EventWriter<OnDropInFactoryInput>,
    mut drop_on_ship_event: EventWriter<OnDropCrateOnShip>,
    mut splash_event: EventWriter<OnCrateSplashedInWater>,
    box_collisions: Query<(Entity, &Collisions, &PhysicsCrate, &Transform)>,
    ship_entities: Query<&Children, With<Wave>>,
    factory_inputs: Query<&FactoryInput>,
    splashers: Query<&SplashCatcher>,
    mut ship_holds: Query<(Entity, &mut ShipHold, &GlobalTransform)>,
) {
    for (crate_entity, crate_collisions, physics_crate, crate_tx) in box_collisions.iter() {
        for collision in crate_collisions.entities() {
            if factory_inputs.get(collision).is_ok() {
                info!(
                    "Dropped {:?} in factory input, raising event and despawning",
                    physics_crate.box_type
                );
                factory_event.send(OnDropInFactoryInput {
                    box_type: physics_crate.box_type,
                });
                commands.entity(crate_entity).despawn_recursive();
                continue;
            }

            if splashers.get(collision).is_ok() {
                info!("Crate splashed down!");
                splash_event.send(OnCrateSplashedInWater(crate_tx.translation.truncate()));
                commands.entity(crate_entity).despawn_recursive();
                continue;
            }

            if let Ok(children) = ship_entities.get(collision) {
                // add the crate to the ship hold and despawn the physics crate
                for child in children.iter() {
                    if let Ok((ship_entity, mut ship_hold, tx)) = ship_holds.get_mut(*child) {
                        info!("Crate {:?} dropped on ship {:?}!", crate_entity, ship_hold);

                        let unmet_demands = ship_hold.get_unmet_demands();
                        drop_on_ship_event.send(OnDropCrateOnShip {
                            ship_entity,
                            box_type: physics_crate.box_type,
                            location: tx.translation(),
                            was_demanded: unmet_demands.contains(&physics_crate.box_type),
                        });

                        ship_hold.crates.push(physics_crate.box_type);

                        commands.entity(crate_entity).despawn_recursive();
                    }
                }
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
