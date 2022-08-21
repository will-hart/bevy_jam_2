use bevy::prelude::*;

use crate::game::components::{AnimateWithSpeed, Wave};

pub struct DespawnEntity(pub Entity);

/// moves waves and other things across the screen
pub fn animate_entity(
    mut commands: Commands,
    time: Res<Time>,
    mut despawn_events: EventWriter<DespawnEntity>,
    mut animated_entities: Query<(Entity, &AnimateWithSpeed, &mut Transform, Option<&Wave>)>,
) {
    let dt = time.delta_seconds();

    for (ent, anim, mut item, wave) in animated_entities.iter_mut() {
        let delta = item.translation.truncate() - anim.target;

        if delta.length_squared() < 0.01 {
            info!("Despawning an animated item");
            if wave.is_some() {
                despawn_events.send(DespawnEntity(ent));
            } else {
                commands.entity(ent).despawn_recursive();
            }
            continue;
        }

        let dx = -delta.x.signum() * (dt * anim.speed).clamp(0.0, delta.x.abs());
        let dy = -delta.y.signum() * (dt * anim.speed).clamp(0.0, delta.y.abs());
        item.translation += Vec3::new(dx, dy, 0.0);
    }
}
