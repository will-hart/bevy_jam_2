mod animation;
pub mod components;
mod day_night_cycle;

pub use animation::{Animation, AnimationState};

use bevy::prelude::*;
use iyes_loopless::{condition::IntoConditionalSystem, prelude::AppLooplessStateExt};

use crate::{
    game::{
        components::{AnimateX, Ship, Torch},
        day_night_cycle::{SkyColourCycles, SunEvent, TimeOfDay},
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
            .insert_resource(TimeOfDay { time_of_day: 5.8 })
            .add_event::<SunEvent>()
            .add_plugin(AnimationPlugin)
            .add_enter_system(GameState::Playing, setup_world)
            .add_system(day_night_cycle::day_night_cycle.run_not_in_state(GameState::Loading))
            .add_system(day_night_cycle::torch_visibility.run_not_in_state(GameState::Loading))
            .add_system(day_night_cycle::star_and_sun_spawner.run_not_in_state(GameState::Loading))
            .add_system(day_night_cycle::sun_movement.run_not_in_state(GameState::Loading))
            .add_system(day_night_cycle::star_movement.run_not_in_state(GameState::Loading));
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
    [
        -GRID_SIZE * 9.25,
        GRID_SIZE * 3.25,
        6.5 * GRID_SIZE,
        8.5 * GRID_SIZE,
    ]
    .iter()
    .enumerate()
    .for_each(|(i, x)| {
        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: if i <= 1 {
                    textures.torch.clone()
                } else {
                    textures.torch_upright.clone()
                },
                transform: Transform::from_xyz(*x, 0., 0.1).with_scale(Vec3::new(
                    if i == 0 { -0.5 } else { 0.5 },
                    0.5,
                    0.5,
                )),
                ..Default::default()
            })
            .insert(animations.torch_off.clone())
            .insert(AnimationState::default())
            .insert(Torch);
    });

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
    let mut x = -WIDTH / 2.0 - 5. * GRID_SIZE;

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
            transform: Transform::from_xyz(WIDTH / 2.0 + GRID_SIZE * 5.0, -GRID_SIZE * 1.5, 0.4),
            ..Default::default()
        })
        .insert(AnimateX {
            looped: false,
            speed: -20.,
        })
        .insert(animations.cart.clone())
        .insert(AnimationState::default())
        .with_children(|parent| {
            parent.spawn_bundle(SpriteSheetBundle {
                texture_atlas: textures.cart_boxes.clone(),
                sprite: TextureAtlasSprite {
                    index: 0,
                    ..Default::default()
                },
                ..Default::default()
            });
            parent.spawn_bundle(SpriteSheetBundle {
                texture_atlas: textures.cart_boxes.clone(),
                sprite: TextureAtlasSprite {
                    index: 3,
                    ..Default::default()
                },
                ..Default::default()
            });
        });
}
