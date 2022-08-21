pub mod actions;
mod animation;
pub mod components;
mod day_night_cycle;
mod spawners;

#[cfg(feature = "debug_system")]
mod debug;

pub use animation::{Animation, AnimationState};

use bevy::prelude::*;
use iyes_loopless::prelude::AppLooplessStateExt;

use crate::{
    game::{
        actions::ActionPlugin,
        components::{AnimateX, BoxType},
        day_night_cycle::DayNightCyclePlugin,
        spawners::{spawn_cart, spawn_ship, spawn_torch},
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
        spawn_torch(
            &mut commands,
            &textures,
            &animations,
            Vec3::new(*x, 0., 0.1),
            i == 0,
            i > 1,
        );
    });

    /* SHIPS */
    ship_slots.slots[0] = Some(spawn_ship(
        &mut commands,
        &textures,
        Vec3::new(-GRID_SIZE * 10., -GRID_SIZE * 4., 0.6),
        -4.0 * GRID_SIZE,
    ));

    ship_slots.slots[1] = Some(spawn_ship(
        &mut commands,
        &textures,
        Vec3::new(0., -GRID_SIZE * 4., 0.6),
        -4.0 * GRID_SIZE,
    ));

    ship_slots.slots[2] = Some(spawn_ship(
        &mut commands,
        &textures,
        Vec3::new(GRID_SIZE * 10., -GRID_SIZE * 4., 0.6),
        -4.0 * GRID_SIZE,
    ));

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
    spawn_cart(
        &mut commands,
        &textures,
        &animations,
        Vec3::new(WIDTH / 2.0 + GRID_SIZE * 5.0, -GRID_SIZE * 1.5, 0.4),
        [BoxType::Cotton, BoxType::Apples],
    );
    spawn_cart(
        &mut commands,
        &textures,
        &animations,
        Vec3::new(WIDTH / 2.0 + GRID_SIZE * 15.0, -GRID_SIZE * 1.5, 0.4),
        [BoxType::Bannanas, BoxType::Bannanas],
    );
}
