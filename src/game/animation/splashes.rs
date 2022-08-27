use bevy::prelude::*;

use crate::{
    game::{actions::OnCrateSplashedInWater, components::VisualEffect},
    loader::{AnimationAssets, TextureAssets},
    GRID_SIZE,
};

use super::AnimationState;

pub fn splash_when_hitting_water(
    mut commands: Commands,
    time: Res<Time>,
    textures: Res<TextureAssets>,
    animations: Res<AnimationAssets>,
    mut splash_event: EventReader<OnCrateSplashedInWater>,
) {
    for evt in splash_event.iter() {
        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: textures.splashes.clone(),
                transform: Transform::from_translation(
                    evt.0.extend(3.0) + Vec3::new(0.0, -0.5 * GRID_SIZE, 0.0),
                ),
                ..default()
            })
            .insert(VisualEffect(time.time_since_startup().as_secs_f32() + 3.0))
            .insert(AnimationState::default())
            .insert(animations.splashes.clone());
    }
}
