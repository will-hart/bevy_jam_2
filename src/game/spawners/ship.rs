use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use heron::{CollisionLayers, CollisionShape, RigidBody};
use rand::{seq::SliceRandom, thread_rng, Rng};

use crate::{
    game::{
        components::{
            AnimateWithSpeed, BoxType, CountDownTimer, Ship, ShipDemandItemMarker, ShipHold,
            SpawnShipRequest, TopUiBar, TutorialMarker, Wave, BOX_DEMANDS,
        },
        custom_sprite::CustomSpriteMaterial,
        rng::RandomSpawnTimer,
        spawners::request::spawn_ship_request_icon,
        ui::tutorial::CurrentTutorialLevel,
        AnimationState,
    },
    loader::{AnimationAssets, FontAssets, TextureAssets},
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
#[allow(clippy::too_many_arguments)]
pub fn ship_spawn_on_timer_expiry(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    animations: Res<AnimationAssets>,
    fonts: Res<FontAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomSpriteMaterial>>,
    requests: Query<(&Parent, &CountDownTimer, &SpawnShipRequest)>,
) {
    for (parent_entity, timer, request) in requests.iter() {
        if timer.0.just_finished() {
            info!("Launching a ship due to timer");
            spawn_ship(
                &mut commands,
                0, // TODO
                &textures,
                &animations,
                &fonts,
                &mut meshes,
                &mut materials,
                request.clone(),
            );

            // despawn the spawn indicator
            commands.entity(parent_entity.get()).despawn_recursive();
        }
    }
}

/// Spawns a ship in teh game world based on a RequestShip
#[allow(clippy::too_many_arguments)]
pub fn spawn_ship(
    commands: &mut Commands,
    tutorial_level: u8,
    textures: &TextureAssets,
    animations: &AnimationAssets,
    fonts: &FontAssets,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<CustomSpriteMaterial>,
    request: SpawnShipRequest,
) -> Entity {
    let mut rng = thread_rng();
    let mut entity: Option<Entity> = None;

    let text_style = TextStyle {
        font: fonts.default_font.clone(),
        font_size: 12.0,
        color: Color::WHITE,
    };

    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad::new(Vec2::new(288.0, 64.0))))
                .into(),
            transform: Transform::from_translation(SHIP_SPAWN_OFFSCREEN_POSITION),
            material: materials.add(textures.waves.clone().into()),
            ..Default::default()
        })
        .insert_bundle((
            CollisionShape::Cuboid {
                half_extends: Vec3::new(SHIP_WIDTH / 2.2, 10.0, 30.0),
                border_radius: None,
            },
            RigidBody::Sensor,
            CollisionLayers::none()
                .with_group(GamePhysicsLayer::Ship)
                .with_mask(GamePhysicsLayer::Crate),
        ))
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
                    .insert(animations.ship_unfurl.clone())
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

                        if tutorial_level == 1 {
                            ship_child_commands
                                .spawn_bundle(Text2dBundle {
                                    text: Text::from_section(
                                        "^^^ Drag these crates on to the ship",
                                        text_style,
                                    ),
                                    transform: Transform::from_xyz(-130., -140., 2.0),
                                    ..default()
                                })
                                .insert(TutorialMarker(1));
                        }
                    })
                    .id(),
            )
        });

    entity.unwrap()
}
