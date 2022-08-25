pub mod actions;
mod animation;
pub mod components;
mod day_night_cycle;
mod spawners;

mod custom_sprite;
#[cfg(feature = "debug_system")]
mod debug;

pub mod rng;
mod ui;

pub use animation::{Animation, AnimationState};
pub use ui::OnCoinsReceived;

use bevy::prelude::*;
use heron::prelude::*;
use iyes_loopless::prelude::AppLooplessStateExt;

use crate::{
    game::{
        actions::ActionPlugin,
        custom_sprite::CustomSpritePlugin,
        day_night_cycle::DayNightCyclePlugin,
        spawners::{spawn_torch, GamePhysicsLayer, SpawningPlugin},
        ui::UiPlugin,
    },
    loader::{AnimationAssets, TextureAssets},
    GameState, GRID_SIZE, WIDTH,
};

#[cfg(feature = "debug_system")]
use crate::game::debug::DebugPlugin;

use animation::AnimationPlugin;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemLabel)]
pub enum SystemLabels {
    ShipAnimationAndDespawn,
    ScoreDisplay,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        info!("Mounting GamePlugin");
        app.add_plugin(CustomSpritePlugin)
            .add_plugin(PhysicsPlugin::default()) // Add the plugin
            .insert_resource(Gravity::from(Vec3::new(0.0, -500.0, 0.0)))
            .add_plugin(AnimationPlugin)
            .add_plugin(ActionPlugin)
            .add_plugin(DayNightCyclePlugin)
            .add_plugin(UiPlugin)
            .add_plugin(SpawningPlugin)
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

    /* WAREHOUSE */
    commands
        .spawn_bundle((
            RigidBody::Static,
            CollisionShape::Cuboid {
                half_extends: Vec3::new(100.0, 10.0, GRID_SIZE / 2.0),
                border_radius: None,
            },
            CollisionLayers::none()
                .with_group(GamePhysicsLayer::Ship)
                .with_mask(GamePhysicsLayer::Crate),
        ))
        .insert_bundle(SpriteBundle::default())
        .insert_bundle(TransformBundle {
            local: Transform::from_xyz(-0.25 * WIDTH, -1.5 * GRID_SIZE, 0.0),
            ..default()
        });
}
