mod animation;
pub mod components;
mod day_night_cycle;

pub use animation::{Animation, AnimationState};

use bevy::prelude::*;
use iyes_loopless::{condition::IntoConditionalSystem, prelude::AppLooplessStateExt};

use crate::{
    game::{
        components::{AnimateX, Ship, Torch},
        day_night_cycle::SkyColourCycles,
    },
    loader::{AnimationAssets, TextureAssets},
    GameState, GRID_SIZE, WIDTH,
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

    /* BACKGROUNDS */
    commands.spawn_bundle(SpriteBundle {
        texture: textures.background.clone(),
        ..Default::default()
    });

    /* TORCHES */
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: textures.torch.clone(),
            transform: Transform::from_xyz(GRID_SIZE * 3.25, 0., 0.1)
                .with_scale(Vec3::new(0.5, 0.5, 0.5)),
            ..Default::default()
        })
        .insert(animations.torch_off.clone())
        .insert(AnimationState::default())
        .insert(Torch);

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: textures.torch.clone(),
            transform: Transform::from_xyz(-GRID_SIZE * 9.25, 0., 0.1)
                .with_scale(Vec3::new(-0.5, 0.5, 0.5)),
            ..Default::default()
        })
        .insert(animations.torch_off.clone())
        .insert(AnimationState::default())
        .insert(Torch);

    /* SHIPS */
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: textures.ship.clone(),
            transform: Transform::from_xyz(-GRID_SIZE * 10., -GRID_SIZE * 4., 0.6),
            ..Default::default()
        })
        .insert(Ship {
            y: -GRID_SIZE * 4.,
            phase: 0.25,
        });
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: textures.ship.clone(),
            transform: Transform::from_xyz(0., -GRID_SIZE * 4., 0.6),
            ..Default::default()
        })
        .insert(Ship {
            y: -GRID_SIZE * 4.,
            phase: 1.9,
        });
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: textures.ship.clone(),
            transform: Transform::from_xyz(GRID_SIZE * 10., -GRID_SIZE * 4., 0.6),
            ..Default::default()
        })
        .insert(Ship {
            y: -GRID_SIZE * 4.,
            phase: 2.5,
        });

    /* WAVES */
    let mut x = -WIDTH / 2.0 - 20. * GRID_SIZE;

    while x < WIDTH / 2.0 + 5. * GRID_SIZE {
        commands
            .spawn_bundle(SpriteBundle {
                texture: textures.waves.clone(),
                transform: Transform::from_xyz(x, -GRID_SIZE * 8., 0.9),
                ..Default::default()
            })
            .insert(AnimateX {
                looped: true,
                speed: 15.0,
            });
        x += 160.;
    }

    /* CART */
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: textures.horse_and_cart.clone(),
            transform: Transform::from_xyz(WIDTH / 2.0 + GRID_SIZE * 5.0, -GRID_SIZE, 0.4),
            ..Default::default()
        })
        .insert(AnimateX {
            looped: false,
            speed: -12.,
        })
        .insert(animations.cart.clone())
        .insert(AnimationState::default())
        .with_children(|parent| {
            parent.spawn_bundle(SpriteBundle {
                texture: textures.cart_boxes.clone(),
                ..Default::default()
            });
            parent.spawn_bundle(SpriteBundle {
                texture: textures.cart_boxes2.clone(),
                ..Default::default()
            });
        });
}
