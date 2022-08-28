use bevy::prelude::*;
use heron::{CollisionLayers, CollisionShape, RigidBody};
use rand::{seq::SliceRandom, thread_rng, Rng};

use crate::{
    game::{
        components::{
            AnimateWithSpeed, BoxType, CountDownTimer, Ship, ShipDemandItemMarker, ShipHold,
            SpawnShipRequest, TopUiBar, Wave, WorldEntity, BOX_DEMANDS,
        },
        rng::RandomSpawnTimer,
        spawners::request::spawn_ship_request_icon,
        ui::tutorial::CurrentTutorialLevel,
        AnimationState,
    },
    loader::{AnimationAssets, TextureAssets},
    GRID_SIZE, WIDTH,
};

use super::GamePhysicsLayer;

pub const SHIP_SAILING_POSITION_Y: f32 = -10.0 * GRID_SIZE;
pub const SHIP_SPAWN_OFFSCREEN_POSITION: Vec3 =
    Vec3::new(-0.7 * WIDTH, SHIP_SAILING_POSITION_Y, 8.0);

pub const MAX_SPAWN_REQUESTS: usize = 5;

pub const SHIP_SPEED: f32 = 40.0;

pub const SPAWN_SPEED_UP_RATE_PER_SECOND: f64 = 0.05;

pub const SHIP_WIDTH: f32 = 288.0;

pub struct OnShipSpawned;

/// Periodically queues up a RequestShip component and button in the ship bar
/// When the timer gets to 0, the ship spawns and sails across the screen.
#[allow(clippy::too_many_arguments)]
pub fn ship_queuing_system(
    mut commands: Commands,
    time: Res<Time>,
    textures: Res<TextureAssets>,
    mut tutorial_level: ResMut<CurrentTutorialLevel>,
    mut event_test: Local<RandomSpawnTimer>,
    mut next_test: Local<f64>,
    spawn_requests: Query<&SpawnShipRequest>,
    top_bar_query: Query<Entity, With<TopUiBar>>,
) {
    if tutorial_level.0 == 5 {
        info!("Spawning level 5 tutorial ship");

        let top_bar = top_bar_query.single();

        commands.entity(top_bar).with_children(|layout| {
            spawn_ship_request_icon(
                layout,
                &textures,
                vec![BoxType::Cider],
                (time.seconds_since_startup() + 8.0) as f32,
            );
        });

        tutorial_level.0 = 6;
        return;
    }

    if tutorial_level.0 < 9 {
        return;
    }

    let elapsed = time.seconds_since_startup();
    if elapsed < *next_test {
        return;
    }

    if spawn_requests.iter().count() >= MAX_SPAWN_REQUESTS {
        return;
    }

    *next_test = elapsed + 1.0;

    // speed up ship spawns,
    let new_min = (event_test.spawn_range.start - SPAWN_SPEED_UP_RATE_PER_SECOND).max(8.0); // reduce by 0.05 / second (every 20 seconds reduce gap by 1s), up to 8s
    let new_max =
        (new_min + (event_test.spawn_range.end - event_test.spawn_range.start) - 0.01).max(9.0);
    event_test.spawn_range = new_min..new_max;

    let mut rng = thread_rng();
    if event_test.tick(&mut rng, elapsed) {
        info!(
            "Spawning a ship request, next spawn range is {:?}s",
            event_test.spawn_range
        );

        let top_bar = top_bar_query.single();

        let mut demands = vec![];
        for _ in 0..rng.gen_range(1..=3) {
            demands.push(*BOX_DEMANDS.choose(&mut rng).unwrap());
        }

        commands.entity(top_bar).with_children(|layout| {
            spawn_ship_request_icon(
                layout,
                &textures,
                demands,
                (time.seconds_since_startup() + event_test.spawn_range.start) as f32,
            );
        });
    }
}

/// Launches ships when their timer runs out
pub fn ship_spawn_on_timer_expiry(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    animations: Res<AnimationAssets>,
    mut spawn_events: EventWriter<OnShipSpawned>,
    requests: Query<(&Parent, &CountDownTimer, &SpawnShipRequest)>,
) {
    for (parent_entity, timer, request) in requests.iter() {
        if timer.0.finished() {
            info!("Launching a ship due to timer");
            spawn_ship(&mut commands, &textures, &animations, request.clone());

            // despawn the spawn indicator
            commands.entity(parent_entity.get()).despawn_recursive();
            spawn_events.send(OnShipSpawned);
        }
    }
}

/// Spawns a ship in the game world based on a RequestShip
pub fn spawn_ship(
    commands: &mut Commands,
    textures: &TextureAssets,
    animations: &AnimationAssets,
    request: SpawnShipRequest,
) -> Entity {
    let mut rng = thread_rng();
    let mut entity: Option<Entity> = None;

    commands
        .spawn_bundle(SpriteBundle {
            texture: textures.waves.clone(),
            transform: Transform::from_translation(SHIP_SPAWN_OFFSCREEN_POSITION),
            ..default()
        })
        .insert_bundle((
            CollisionShape::Cuboid {
                half_extends: Vec3::new(SHIP_WIDTH / 2.2, 18.0, 30.0),
                border_radius: None,
            },
            RigidBody::Sensor,
            CollisionLayers::none()
                .with_group(GamePhysicsLayer::Ship)
                .with_mask(GamePhysicsLayer::Crate),
        ))
        .insert(WorldEntity)
        .insert(AnimateWithSpeed {
            speed: SHIP_SPEED,
            target: vec![Vec3::new(
                -SHIP_SPAWN_OFFSCREEN_POSITION.x,
                SHIP_SPAWN_OFFSCREEN_POSITION.y,
                SHIP_SPAWN_OFFSCREEN_POSITION.z,
            )],
        })
        .insert(Wave)
        .with_children(|child_commands| {
            let ship_hold = ShipHold {
                crates: vec![],
                demands: request.demands.clone(),
            };

            entity = Some(
                child_commands
                    .spawn_bundle(SpriteSheetBundle {
                        texture_atlas: textures.ship.clone(),
                        transform: Transform::from_xyz(0.5 * GRID_SIZE, 0.0, -0.5),
                        ..Default::default()
                    })
                    .insert(Ship::new(&mut rng))
                    .insert(animations.ship_idle.clone())
                    .insert(AnimationState::default())
                    .insert(ship_hold.clone())
                    .with_children(|ship_child_commands| {
                        for (idx, demand) in ship_hold.demands.iter().enumerate() {
                            ship_child_commands
                                .spawn_bundle(SpriteBundle {
                                    texture: demand.get_image(textures),
                                    transform: Transform::from_xyz(
                                        -110. + (idx as f32 * GRID_SIZE),
                                        -120.,
                                        2.,
                                    ),
                                    ..default()
                                })
                                .insert(ShipDemandItemMarker(*demand));
                        }
                    })
                    .id(),
            )
        });

    entity.unwrap()
}
