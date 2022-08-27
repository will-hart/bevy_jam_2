use bevy::prelude::*;

use crate::{
    game::{
        components::{Torch, WorldEntity},
        AnimationState,
    },
    loader::{AnimationAssets, TextureAssets},
};

pub fn spawn_torch(
    commands: &mut Commands,
    textures: &TextureAssets,
    animations: &AnimationAssets,
    location: Vec3,
    flip_x: bool,
    upright: bool,
) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: if upright {
                textures.torch_upright.clone()
            } else {
                textures.torch.clone()
            },
            transform: Transform::from_translation(location).with_scale(Vec3::new(
                if flip_x { -0.5 } else { 0.5 },
                0.5,
                0.5,
            )),
            ..Default::default()
        })
        .insert(animations.torch_off.clone())
        .insert(AnimationState::default())
        .insert(Torch)
        .insert(WorldEntity);
}
