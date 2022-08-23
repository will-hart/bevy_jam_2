use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::{seq::SliceRandom, thread_rng, Rng};

use crate::{
    game::{
        components::{
            AnimateWithSpeed, RequestShip, Ship, ShipArriving, ShipDemandItemMarker, ShipHold,
            TopUiBar, TutorialMarker, Wave, BOX_TYPES, DESTINATIONS,
        },
        custom_sprite::CustomSpriteMaterial,
        rng::RandomEvent,
        ui::{spawn_ship_request_button, CurrentTutorialLevel},
        AnimationState,
    },
    loader::{AnimationAssets, FontAssets, TextureAssets},
    GRID_SIZE, WIDTH,
};

pub const SHIP_SLOTS_POSITIONS: [Vec3; 3] = [
    Vec3::new(-GRID_SIZE * 10., -GRID_SIZE * 8., 1.6),
    Vec3::new(0., -GRID_SIZE * 8., 1.3),
    Vec3::new(GRID_SIZE * 10., -GRID_SIZE * 8., 1.0),
];

pub const SHIP_SAILING_POSITION_Y: f32 = -12.0 * GRID_SIZE;
pub const SHIP_SPAWN_OFFSCREEN_POSITION: Vec3 =
    Vec3::new(-0.7 * WIDTH, SHIP_SAILING_POSITION_Y, 8.0);

pub const MAX_SPAWN_REQUESTS: usize = 5;

pub fn ship_spawning_system(
    mut commands: Commands,
    tutorial_level: Res<CurrentTutorialLevel>,
    time: Res<Time>,
    textures: Res<TextureAssets>,
    mut event_test: Local<RandomEvent>,
    mut next_test: Local<f64>,
    spawn_requests: Query<&RequestShip>,
    top_bar_query: Query<Entity, With<TopUiBar>>,
) {
    let elapsed = time.seconds_since_startup();
    if elapsed < *next_test || tutorial_level.0 < 3 {
        return;
    }

    if spawn_requests.iter().collect::<Vec<_>>().len() > MAX_SPAWN_REQUESTS {
        return;
    }

    *next_test = elapsed + 1.0;

    let mut rng = thread_rng();
    if event_test.test(&mut rng) {
        info!("Spawning a ship request");

        let top_bar = top_bar_query.single();

        let mut demands = vec![];
        for _ in 0..rng.gen_range(1..=4) {
            demands.push(*BOX_TYPES.choose(&mut rng).unwrap());
        }

        commands.entity(top_bar).with_children(|layout| {
            spawn_ship_request_button(
                &textures,
                layout,
                *DESTINATIONS.choose(&mut rng).unwrap(),
                demands,
            );
        });
    }
}

pub fn spawn_ship(
    commands: &mut Commands,
    tutorial_level: u8,
    textures: &TextureAssets,
    animations: &AnimationAssets,
    fonts: &FontAssets,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<CustomSpriteMaterial>,
    slot_id: usize,
    request: RequestShip,
) -> Entity {
    let mut rng = thread_rng();
    let mut entity: Option<Entity> = None;

    let slot_pos = SHIP_SLOTS_POSITIONS[slot_id];

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
        .insert(AnimateWithSpeed {
            speed: 20.,
            target: vec![
                Vec3::new(slot_pos.x - 3.0 * GRID_SIZE, SHIP_SAILING_POSITION_Y, 8.0),
                slot_pos,
            ],
        })
        .insert(Wave)
        .with_children(|child_commands| {
            let ship_hold = ShipHold {
                destination: request.destination,
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
                    .insert(ShipArriving(slot_id))
                    .insert(Ship::new(&mut rng))
                    .insert(animations.ship_unfurl.clone())
                    .insert(AnimationState::default())
                    .insert(ship_hold.clone())
                    .with_children(|ship_child_commands| {
                        for (idx, demand) in ship_hold.demands.iter().enumerate() {
                            ship_child_commands
                                .spawn_bundle(SpriteBundle {
                                    texture: demand.get_image(&textures).clone().into(),
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
