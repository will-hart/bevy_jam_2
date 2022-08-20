mod animation;
pub mod components;
mod day_night_cycle;

pub use animation::{Animation, AnimationState};

use bevy::prelude::*;
use iyes_loopless::{condition::IntoConditionalSystem, prelude::AppLooplessStateExt};

use crate::{
    game::{components::Torch, day_night_cycle::SkyColourCycles},
    loader::{AnimationAssets, TextureAssets},
    GameState,
};
use animation::AnimationPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        info!("Mounting GamePlugin");
        app.insert_resource(SkyColourCycles::default())
            .add_plugin(AnimationPlugin)
            .add_enter_system(GameState::Playing, setup_world)
            .add_system(day_night_cycle::day_night_cycle.run_not_in_state(GameState::Loading))
            .add_system(day_night_cycle::torch_visibility.run_not_in_state(GameState::Loading));
    }
}

fn setup_world(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    animations: Res<AnimationAssets>,
) {
    info!("Setting up game world");
    commands.spawn_bundle(SpriteBundle {
        texture: textures.background.clone(),
        ..Default::default()
    });

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: textures.torch.clone(),
            transform: Transform::from_xyz(32. * 3.5, 32. * -2., 0.1),
            ..Default::default()
        })
        .insert(animations.torch_off.clone())
        .insert(AnimationState::default())
        .insert(Torch);

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: textures.torch.clone(),
            transform: Transform::from_xyz(-32. * 9.5, 32. * -2., 0.1)
                .with_scale(Vec3::new(-1.0, 1.0, 1.0)),
            ..Default::default()
        })
        .insert(animations.torch_off.clone())
        .insert(AnimationState::default())
        .insert(Torch);
}
