use bevy::prelude::*;

use crate::{
    game::{
        components::{AnimateWithSpeed, BoxType, Cart, CartCrate},
        AnimationState,
    },
    loader::{AnimationAssets, TextureAssets},
    WIDTH,
};

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
            target: Vec2::new(-0.75 * WIDTH, location.y),
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
