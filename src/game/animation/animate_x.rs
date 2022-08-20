use bevy::prelude::*;

use crate::{game::components::AnimateX, GRID_SIZE, WIDTH};

/// moves waves and other things across the screen
pub fn animate_x(
    mut commands: Commands,
    time: Res<Time>,
    mut waves: Query<(Entity, &AnimateX, &mut Transform)>,
) {
    let dt = time.delta_seconds();

    for (ent, anim, mut item) in waves.iter_mut() {
        item.translation.x += dt * anim.speed;

        let out_of_bounds = if anim.speed < 0.0 {
            item.translation.x < -WIDTH / 2.0 - 3.0 * GRID_SIZE
        } else {
            item.translation.x > WIDTH / 2.0 + 3.0 * GRID_SIZE
        };

        if out_of_bounds {
            if anim.looped {
                info!("Resetting an x-animated item");
                item.translation.x = -WIDTH / 2.0 - 5.0 * GRID_SIZE;
            } else {
                info!("Despawning an animated item");
                commands.entity(ent).despawn();
            }
        }
    }
}
