pub mod actions;
mod animation;
pub mod components;
mod day_night_cycle;
mod spawners;

mod custom_sprite;
#[cfg(feature = "debug_system")]
mod debug;

pub mod factory;
pub mod rng;
mod ui;

pub use animation::{Animation, AnimationState};
pub use spawners::OnShipSpawned;
pub use ui::OnCoinsReceived;

use bevy::prelude::*;
use heron::prelude::*;
use iyes_loopless::prelude::AppLooplessStateExt;

use crate::{
    game::{
        actions::ActionPlugin,
        components::{FactoryInput, SplashCatcher, WorldEntity},
        custom_sprite::CustomSpritePlugin,
        day_night_cycle::DayNightCyclePlugin,
        factory::FactoryPlugin,
        spawners::{spawn_torch, GamePhysicsLayer, SpawningPlugin},
        ui::UiPlugin,
    },
    loader::{AnimationAssets, TextureAssets},
    GameState, GRID_SIZE, HEIGHT, WIDTH,
};

#[cfg(feature = "debug_system")]
use crate::game::debug::DebugPlugin;

use animation::AnimationPlugin;

use self::{components::TutorialMarker, ui::Score};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemLabel)]
pub enum SystemLabels {
    ShipAnimationAndDespawn,
    ScoreDisplay,
    FactoryProduction,
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
            .add_plugin(FactoryPlugin)
            .add_enter_system(GameState::Playing, setup_world)
            .add_exit_system(GameState::Playing, teardown_world);

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
    commands
        .spawn_bundle(SpriteBundle {
            texture: textures.background.clone(),
            ..Default::default()
        })
        .insert(WorldEntity);

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
        })
        .insert(WorldEntity);

    /* FACTORY */
    commands
        .spawn_bundle((
            RigidBody::Sensor,
            CollisionShape::Cuboid {
                half_extends: Vec3::new(60.0, 10.0, GRID_SIZE / 2.0),
                border_radius: None,
            },
            CollisionLayers::none()
                .with_group(GamePhysicsLayer::Ship)
                .with_mask(GamePhysicsLayer::Crate),
        ))
        .insert_bundle(SpriteBundle::default())
        .insert_bundle(TransformBundle {
            local: Transform::from_xyz(-1.0, 1.5 * GRID_SIZE, 0.0),
            ..default()
        })
        .insert(FactoryInput)
        .insert(WorldEntity);

    /* FACTORY OUTPUT */
    commands
        .spawn_bundle((
            RigidBody::Static,
            CollisionShape::Cuboid {
                half_extends: Vec3::new(60.0, 10.0, GRID_SIZE / 2.0),
                border_radius: None,
            },
            CollisionLayers::none()
                .with_group(GamePhysicsLayer::Ship)
                .with_mask(GamePhysicsLayer::Crate),
        ))
        .insert_bundle(SpriteBundle::default())
        .insert_bundle(TransformBundle {
            local: Transform::from_xyz(5.0 * GRID_SIZE, 0.0, 0.0),
            ..default()
        })
        .insert(WorldEntity);

    /* SPLASH SECTION */
    commands
        .spawn_bundle((
            RigidBody::Sensor,
            CollisionShape::Cuboid {
                half_extends: Vec3::new(WIDTH / 2.0, 10.0, GRID_SIZE / 2.0),
                border_radius: None,
            },
            CollisionLayers::none()
                .with_group(GamePhysicsLayer::Ship)
                .with_mask(GamePhysicsLayer::Crate),
        ))
        .insert_bundle(SpriteBundle::default())
        .insert_bundle(TransformBundle {
            local: Transform::from_xyz(0.0, -HEIGHT / 2.0 + GRID_SIZE, 0.0),
            ..default()
        })
        .insert(SplashCatcher)
        .insert(WorldEntity);
}

fn teardown_world(
    mut commands: Commands,
    items: Query<Entity, With<WorldEntity>>,
    mut score: ResMut<Score>,
    tutorial_markers: Query<Entity, With<TutorialMarker>>,
) {
    for ent in items.iter() {
        commands.entity(ent).despawn_recursive();
    }

    for ent in tutorial_markers.iter() {
        commands.entity(ent).despawn_recursive();
    }

    score.0 = 0.0;
}
