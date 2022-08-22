use bevy::prelude::*;
use bevy_prototype_debug_lines::{DebugLines, DebugLinesPlugin};

use crate::WIDTH;

use super::actions::{CART_MAX_Y, CART_MIN_Y, SHIP_MAX_Y, SHIP_MIN_Y, SHIP_ZONES};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(DebugLinesPlugin::default())
            .add_system(draw_drop_regions);
    }
}

fn draw_drop_regions(mut lines: ResMut<DebugLines>) {
    lines.line(
        Vec3::new(-WIDTH / 2.0, SHIP_MIN_Y, 0.1),
        Vec3::new(WIDTH / 2.0, SHIP_MIN_Y, 0.1),
        0.0,
    );

    lines.line(
        Vec3::new(-WIDTH / 2.0, SHIP_MAX_Y, 0.1),
        Vec3::new(WIDTH / 2.0, SHIP_MAX_Y, 0.1),
        0.0,
    );

    lines.line(
        Vec3::new(-WIDTH / 2.0, CART_MIN_Y, 0.1),
        Vec3::new(WIDTH / 2.0, CART_MIN_Y, 0.1),
        0.0,
    );

    lines.line(
        Vec3::new(-WIDTH / 2.0, CART_MAX_Y, 0.1),
        Vec3::new(WIDTH / 2.0, CART_MAX_Y, 0.1),
        0.0,
    );

    for r in SHIP_ZONES.iter() {
        lines.line(
            Vec3::new(r.start, SHIP_MIN_Y, 0.1),
            Vec3::new(r.start, SHIP_MAX_Y, 0.1),
            0.0,
        );

        lines.line(
            Vec3::new(r.end, SHIP_MIN_Y, 0.1),
            Vec3::new(r.end, SHIP_MAX_Y, 0.1),
            0.0,
        );
    }
}
