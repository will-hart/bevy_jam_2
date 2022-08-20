mod actions;
mod animation;
pub mod components;
mod day_night_cycle;

#[cfg(feature = "debug_system")]
mod debug;

pub use animation::{Animation, AnimationState};

use bevy::prelude::*;
use iyes_loopless::prelude::AppLooplessStateExt;

use crate::{
    game::{
        actions::ActionPlugin,
        components::{AnimateX, BoxType, Cart, CartCrate, Ship, Torch},
        day_night_cycle::DayNightCyclePlugin,
    },
    loader::{AnimationAssets, TextureAssets},
    GameState, GRID_SIZE, WIDTH,
};

#[cfg(feature = "debug_system")]
use crate::game::debug::DebugPlugin;

use animation::AnimationPlugin;

use self::actions::ShipSlots;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        info!("Mounting GamePlugin");
        app.add_plugin(AnimationPlugin)
            .add_plugin(ActionPlugin)
            .add_plugin(DayNightCyclePlugin)
            .add_enter_system(GameState::Playing, setup_world);

        #[cfg(feature = "debug_system")]
        {
            app.add_plugin(DebugPlugin);
        }
    }
}

fn setup_world(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    animations: Res<AnimationAssets>,
    mut ship_slots: ResMut<ShipSlots>,
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
    ship_slots.slots[0] = Some(
        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: textures.ship.clone(),
                transform: Transform::from_xyz(-GRID_SIZE * 10., -GRID_SIZE * 4., 0.6),
                ..Default::default()
            })
            .insert(Ship {
                y: -GRID_SIZE * 4.,
                phase: 0.25,
                crates: vec![],
            })
            .id(),
    );
    ship_slots.slots[1] = Some(
        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: textures.ship.clone(),
                transform: Transform::from_xyz(0., -GRID_SIZE * 4., 0.6),
                ..Default::default()
            })
            .insert(Ship {
                y: -GRID_SIZE * 4.,
                phase: 1.9,
                crates: vec![],
            })
            .id(),
    );
    ship_slots.slots[2] = Some(
        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: textures.ship.clone(),
                transform: Transform::from_xyz(GRID_SIZE * 10., -GRID_SIZE * 4., 0.6),
                ..Default::default()
            })
            .insert(Ship {
                y: -GRID_SIZE * 4.,
                phase: 2.5,
                crates: vec![],
            })
            .id(),
    );

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

    /* CARTS */
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: textures.horse_and_cart.clone(),
            transform: Transform::from_xyz(WIDTH / 2.0 + GRID_SIZE * 5.0, -GRID_SIZE * 1.5, 0.4),
            ..Default::default()
        })
        .insert(AnimateX {
            looped: false,
            speed: -30.,
        })
        .insert(Cart {
            front: Some(BoxType::Cotton),
            back: Some(BoxType::Apples),
        })
        .insert(animations.cart.clone())
        .insert(AnimationState::default())
        .with_children(|parent| {
            parent
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: textures.cart_boxes.clone(),
                    sprite: TextureAtlasSprite {
                        index: 0,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(CartCrate {
                    is_front_slot: true,
                });
            parent
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: textures.cart_boxes.clone(),
                    sprite: TextureAtlasSprite {
                        index: 3,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(CartCrate {
                    is_front_slot: false,
                });
        });

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: textures.horse_and_cart.clone(),
            transform: Transform::from_xyz(WIDTH / 2.0 + GRID_SIZE * 15.0, -GRID_SIZE * 1.5, 0.4),
            ..Default::default()
        })
        .insert(AnimateX {
            looped: false,
            speed: -30.,
        })
        .insert(Cart {
            front: Some(BoxType::Bannanas),
            back: Some(BoxType::Bannanas),
        })
        .insert(animations.cart.clone())
        .insert(AnimationState::default())
        .with_children(|parent| {
            parent
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: textures.cart_boxes.clone(),
                    sprite: TextureAtlasSprite {
                        index: 4,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(CartCrate {
                    is_front_slot: true,
                });
            parent
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: textures.cart_boxes.clone(),
                    sprite: TextureAtlasSprite {
                        index: 5,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(CartCrate {
                    is_front_slot: false,
                });
        });
}
