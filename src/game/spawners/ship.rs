use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::thread_rng;

use crate::{
    game::{
        components::{
            AnimateWithSpeed, RequestShip, Ship, ShipArriving, ShipDemandItemMarker, ShipHold, Wave,
        },
        custom_sprite::CustomSpriteMaterial,
        AnimationState,
    },
    loader::{AnimationAssets, TextureAssets},
    GRID_SIZE, WIDTH,
};

pub const SHIP_SLOTS_POSITIONS: [Vec3; 3] = [
    Vec3::new(-GRID_SIZE * 10., -GRID_SIZE * 8., 1.6),
    Vec3::new(0., -GRID_SIZE * 8., 1.3),
    Vec3::new(GRID_SIZE * 10., -GRID_SIZE * 8., 1.0),
];

pub const SHIP_SAILING_POSITION_Y: f32 = -10.0 * GRID_SIZE;
pub const SHIP_SPAWN_OFFSCREEN_POSITION: Vec3 =
    Vec3::new(-0.7 * WIDTH, SHIP_SAILING_POSITION_Y, 8.0);

pub fn spawn_ship(
    commands: &mut Commands,
    textures: &TextureAssets,
    animations: &AnimationAssets,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<CustomSpriteMaterial>,
    slot_id: usize,
    request: RequestShip,
) -> Entity {
    let mut rng = thread_rng();
    let mut entity: Option<Entity> = None;

    let slot_pos = SHIP_SLOTS_POSITIONS[slot_id];

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
                    })
                    .id(),
            )
        });

    entity.unwrap()
}
