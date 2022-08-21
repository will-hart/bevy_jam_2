use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::{game::components::Ship, loader::TextureAssets};

pub fn spawn_ship(
    commands: &mut Commands,
    textures: &TextureAssets,
    location: Vec3,
    y: f32,
) -> Entity {
    let mut rng = thread_rng();

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: textures.ship.clone(),
            transform: Transform::from_translation(location),
            ..Default::default()
        })
        .insert(Ship {
            y,
            phase: rng.gen_range(-3.1..3.1),
            crates: vec![],
        })
        .id()
}
