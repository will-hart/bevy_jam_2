use bevy::prelude::*;

use crate::game::components::{AnimateWithSpeed, Wave};

pub struct ShipArrivedAtDestination(pub Entity);

/// moves waves and other things across the screen
pub fn animate_entity(
    mut commands: Commands,
    time: Res<Time>,
    mut arrival_events: EventWriter<ShipArrivedAtDestination>,
    mut animated_entities: Query<(Entity, &mut AnimateWithSpeed, &mut Transform, Option<&Wave>)>,
) {
    let dt = time.delta_seconds();

    for (ent, mut anim, mut item, wave) in animated_entities.iter_mut() {
        let current_waypoint = match anim.target.first() {
            Some(t) => t,
            None => {
                continue;
            }
        };

        let delta = item.translation - *current_waypoint;

        if delta.length_squared() < 0.01 {
            anim.target.remove(0);

            if anim.target.is_empty() {
                info!("Despawning an animated item");
                if wave.is_some() {
                    arrival_events.send(ShipArrivedAtDestination(ent));
                } else {
                    commands.entity(ent).despawn_recursive();
                }
            }

            continue;
        }

        let dx = -delta.x.signum() * (dt * anim.speed).clamp(0.0, delta.x.abs());
        let dy = -delta.y.signum() * (dt * anim.speed).clamp(0.0, delta.y.abs());
        let dz = -delta.z.signum() * (dt * anim.speed).clamp(0.0, delta.z.abs());
        item.translation += Vec3::new(dx, dy, dz);
    }
}
