use bevy::prelude::*;

use crate::game::components::VisualEffect;

pub fn despawn_visual_effects(
    mut commands: Commands,
    time: Res<Time>,
    effects: Query<(Entity, &VisualEffect)>,
) {
    if effects.is_empty() {
        return;
    }

    let current_time = time.time_since_startup().as_secs_f32();
    for (ent, effect) in effects.iter() {
        if current_time > effect.0 {
            commands.entity(ent).despawn_recursive();
        }
    }
}
