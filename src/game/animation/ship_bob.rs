use bevy::prelude::*;

use crate::game::components::Ship;

const BOB_AMOUNT: f32 = 4.;
const ROTATION_FACTOR: f32 = 0.02;

pub fn ship_bob(time: Res<Time>, mut ships: Query<(&mut Transform, &Ship)>) {
    let t = time.time_since_startup().as_secs_f32();
    let dt = time.delta_seconds();

    for (mut tx, ship) in ships.iter_mut() {
        let amt = (t + ship.phase).sin();
        tx.translation.y = ship.y + BOB_AMOUNT * amt;

        tx.rotate_z(t.cos() * ROTATION_FACTOR * dt);
    }
}
