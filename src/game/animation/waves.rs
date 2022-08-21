use bevy::prelude::*;

use crate::game::{components::Wave, custom_sprite::CustomSpriteMaterial};

pub fn scroll_wave_texture(
    time: Res<Time>,
    mut materials: ResMut<Assets<CustomSpriteMaterial>>,
    instances: Query<&mut Handle<CustomSpriteMaterial>, With<Wave>>,
) {
    let dt = time.delta_seconds();
    for instance in instances.iter() {
        let material = materials.get_mut(instance).unwrap();
        material.x_offset -= 0.1 * dt;
    }
}
