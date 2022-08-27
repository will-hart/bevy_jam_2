use bevy::prelude::*;

use crate::{
    game::{
        components::{AnimateWithSpeed, BoxType, Cart, CartCrate},
        // ui::CurrentTutorialLevel,
        AnimationState,
    },
    loader::{AnimationAssets, TextureAssets},
    GRID_SIZE, WIDTH,
};

pub struct OnCartSpawned;

pub const CART_SPAWN_DELAY: f32 = 5.0;
pub const CART_Z_POS: f32 = 0.4;

pub struct CartSpawningState {
    pub items: Vec<BoxType>,
    pub active_carts: usize,
    max_carts: usize,
}

impl Default for CartSpawningState {
    fn default() -> Self {
        Self {
            items: vec![],
            active_carts: 0,
            max_carts: 5,
        }
    }
}

/// Not a system - this is a helper function used to spawn carts
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
    // tutorial_level: Res<CurrentTutorialLevel>, // TODO
    time: Res<Time>,
    textures: Res<TextureAssets>,
    animations: Res<AnimationAssets>,
    mut spawning_state: ResMut<CartSpawningState>,
    mut cart_spawn_events: EventWriter<OnCartSpawned>,
    mut last_spawn: Local<f32>,
) {
    // only spawn if we have capacity + both items are filled
    if spawning_state.active_carts >= spawning_state.max_carts || spawning_state.items.len() < 2 {
        return;
    }

    let elapsed = time.time_since_startup().as_secs_f32();

    // only spawn if its been 8 seconds since the last spawn
    if elapsed - *last_spawn < CART_SPAWN_DELAY {
        return;
    }

    let cart_items = spawning_state.items.drain(0..2).collect::<Vec<_>>();

    // spawn a cart
    spawn_cart(
        &mut commands,
        &textures,
        &animations,
        Vec3::new(WIDTH / 2.0 + GRID_SIZE * 5.0, -GRID_SIZE * 1.5, CART_Z_POS),
        [cart_items[0], cart_items[1]], // TODO: spawn these based on the tutorial
    );
    cart_spawn_events.send(OnCartSpawned);

    *last_spawn = elapsed;
    spawning_state.active_carts += 1;
}
