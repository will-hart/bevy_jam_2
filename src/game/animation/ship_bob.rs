use bevy::prelude::*;

use crate::game::components::{Ship, ShipHold};

const BOB_AMOUNT: f32 = 4.;
const ROTATION_FACTOR: f32 = 0.02;

pub fn ship_bob(time: Res<Time>, mut ships: Query<(&mut Transform, &Ship, &ShipHold)>) {
    let t = time.time_since_startup().as_secs_f32();
    let dt = time.delta_seconds();

    for (mut tx, ship, hold) in ships.iter_mut() {
        let amt = (t + ship.phase).sin();
        tx.translation.y =
            ship.y_offset + BOB_AMOUNT * amt - (hold.crates.len() as f32 * 3.0).clamp(0.0, 21.0);

        tx.rotate_z(t.cos() * ROTATION_FACTOR * dt);
    }
}
