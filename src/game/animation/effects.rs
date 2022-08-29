use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::{
    game::components::{AnimateWithSpeed, VisualEffect},
    loader::TextureAssets,
    HEIGHT, WIDTH,
};

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

pub fn spawn_rain_effects(mut commands: Commands, textures: Res<TextureAssets>) {
    let mut rng = thread_rng();

    // Probably should spawn these over time but :shrug:
    // I hope it doesn't bork web builds
    for _ in 0..150 {
        let x = rng.gen_range(0.0..WIDTH) - WIDTH / 2.0;
        let y = rng.gen_range(0.0..2.0 * HEIGHT) + 0.5 * HEIGHT;

        commands
            .spawn_bundle(SpriteBundle {
                texture: textures.rain_drop.clone(),
                transform: Transform::from_xyz(x, y, 8.0),
                ..default()
            })
            .insert(AnimateWithSpeed {
                speed: 100.,
                target: vec![Vec3::new(x, -HEIGHT * 0.6, 8.0)],
            });
    }
}
