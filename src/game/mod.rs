pub mod actions;
mod animation;
pub mod components;
mod day_night_cycle;
mod spawners;

mod custom_sprite;
#[cfg(feature = "debug_system")]
mod debug;
mod ui;

pub use animation::{Animation, AnimationState};

use bevy::prelude::*;
use iyes_loopless::prelude::AppLooplessStateExt;

use crate::{
    game::{
        actions::ActionPlugin,
        components::BoxType,
        custom_sprite::CustomSpritePlugin,
        day_night_cycle::DayNightCyclePlugin,
        spawners::{spawn_cart, spawn_ship, spawn_torch},
        ui::UiPlugin,
    },
    loader::{AnimationAssets, FontAssets, TextureAssets},
    GameState, GRID_SIZE, WIDTH,
};

#[cfg(feature = "debug_system")]
use crate::game::debug::DebugPlugin;

use animation::AnimationPlugin;

use self::{actions::ShipSlots, custom_sprite::CustomSpriteMaterial};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        info!("Mounting GamePlugin");
        app.add_plugin(CustomSpritePlugin)
            .add_plugin(AnimationPlugin)
            .add_plugin(ActionPlugin)
            .add_plugin(DayNightCyclePlugin)
            .add_plugin(UiPlugin)
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
    fonts: Res<FontAssets>,
    mut ship_slots: ResMut<ShipSlots>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomSpriteMaterial>>,
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
        &fonts,
        &mut meshes,
        &mut materials,
        Vec3::new(-GRID_SIZE * 10., -GRID_SIZE * 4., 0.6),
    ));

    ship_slots.slots[1] = Some(spawn_ship(
        &mut commands,
        &textures,
        &fonts,
        &mut meshes,
        &mut materials,
        Vec3::new(0., -GRID_SIZE * 4., 0.6),
    ));

    ship_slots.slots[2] = Some(spawn_ship(
        &mut commands,
        &textures,
        &fonts,
        &mut meshes,
        &mut materials,
        Vec3::new(GRID_SIZE * 10., -GRID_SIZE * 4., 0.6),
    ));

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
