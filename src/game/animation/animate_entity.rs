use bevy::prelude::*;

use crate::game::components::AnimateWithSpeed;

/// moves waves and other things across the screen
pub fn animate_entity(
    mut commands: Commands,
    time: Res<Time>,
    mut animated_entities: Query<(Entity, &AnimateWithSpeed, &mut Transform)>,
) {
    let dt = time.delta_seconds();

    for (ent, anim, mut item) in animated_entities.iter_mut() {
        let delta = item.translation.truncate() - anim.target;

        if delta.length_squared() < 0.01 {
            info!("Despawning an animated item");
            commands.entity(ent).despawn_recursive();
            continue;
        }

        let dx = -delta.x.signum() * (dt * anim.speed).clamp(0.0, delta.x.abs());
        let dy = -delta.y.signum() * (dt * anim.speed).clamp(0.0, delta.y.abs());
        item.translation += Vec3::new(dx, dy, 0.0);
    }
}
