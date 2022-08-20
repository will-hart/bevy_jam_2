use bevy::prelude::*;

use crate::{game::components::AnimateX, GRID_SIZE, WIDTH};

/// moves waves and other things across the screen
pub fn animate_x(time: Res<Time>, mut waves: Query<(&AnimateX, &mut Transform)>) {
    let dt = time.delta_seconds();

    for (anim, mut wave) in waves.iter_mut() {
        wave.translation.x += dt * anim.speed;
        if anim.looped && wave.translation.x > WIDTH / 2.0 + GRID_SIZE {
            info!("Resetting an x-animated item");
            wave.translation.x = -WIDTH / 2.0 - 5.0 * GRID_SIZE;
        }
    }
}
