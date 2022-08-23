use bevy::prelude::*;
use rand::{seq::SliceRandom, thread_rng, Rng};

use crate::{
    game::{
        components::{AnimateWithSpeed, BoxType, Cart, CartCrate, BOX_TYPES},
        ui::CurrentTutorialLevel,
        AnimationState,
    },
    loader::{AnimationAssets, TextureAssets},
    GRID_SIZE, WIDTH,
};

pub const CART_Z_POS: f32 = 0.4;

pub fn spawn_cart(
    commands: &mut Commands,
    textures: &TextureAssets,
    animations: &AnimationAssets,
    location: Vec3,
    box_types: [BoxType; 2],
) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: textures.horse_and_cart.clone(),
            transform: Transform::from_translation(location),
            ..Default::default()
        })
        .insert(AnimateWithSpeed {
            speed: 30.,
            target: vec![Vec3::new(-0.75 * WIDTH, location.y, CART_Z_POS)],
        })
        .insert(Cart {
            front: Some(box_types[0]),
            back: Some(box_types[1]),
        })
        .insert(animations.cart.clone())
        .insert(AnimationState::default())
        .with_children(|parent| {
            parent
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: textures.cart_boxes.clone(),
                    sprite: TextureAtlasSprite {
                        index: box_types[0] as usize * 2,
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
                        index: box_types[1] as usize * 2 + 1,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(CartCrate {
                    is_front_slot: false,
                });
        });
}

#[derive(Default)]
pub struct NextSpawnTime(pub u64);

pub fn cart_spawning_system(
    mut commands: Commands,
    tutorial_level: Res<CurrentTutorialLevel>,
    time: Res<Time>,
    textures: Res<TextureAssets>,
    animations: Res<AnimationAssets>,
    mut spawns: Local<NextSpawnTime>,
) {
    let t = time.time_since_startup().as_secs();
    if t > spawns.0 {
        let mut rng = thread_rng();

        // set the next spawn time
        spawns.0 += rng.gen_range(5..15);

        // spawn a cart
        spawn_cart(
            &mut commands,
            &textures,
            &animations,
            Vec3::new(WIDTH / 2.0 + GRID_SIZE * 5.0, -GRID_SIZE * 1.5, CART_Z_POS),
            if tutorial_level.0 < 3 {
                [BoxType::Fruit, BoxType::Fruit]
            } else {
                [
                    *BOX_TYPES.choose(&mut rng).unwrap(),
                    *BOX_TYPES.choose(&mut rng).unwrap(),
                ]
            },
        );
    }
}
