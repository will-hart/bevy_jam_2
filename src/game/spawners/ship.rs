use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::{seq::SliceRandom, thread_rng};

use crate::{
    game::{
        components::{
            AnimateWithSpeed, Ship, ShipArriving, ShipHold, ShipText, Wave, DESTINATIONS,
        },
        custom_sprite::CustomSpriteMaterial,
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

pub const SHIP_SAILING_POSITION_Y: f32 = -10.0 * GRID_SIZE;
pub const SHIP_SPAWN_OFFSCREEN_POSITION: Vec3 =
    Vec3::new(-0.6 * WIDTH, SHIP_SAILING_POSITION_Y, 8.0);

pub fn spawn_ship(
    commands: &mut Commands,
    textures: &TextureAssets,
    fonts: &FontAssets,
    animations: &AnimationAssets,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<CustomSpriteMaterial>,
    slot_id: usize,
    previous_ship: Option<Ship>,
) -> Entity {
    let mut rng = thread_rng();
    let mut entity: Option<Entity> = None;

    let text_style = TextStyle {
        font: fonts.default_font.clone(),
        font_size: 16.0,
        color: Color::WHITE,
    };

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
                destination: *DESTINATIONS.choose(&mut rng).unwrap(),
                crates: vec![],
                weight_capacity: 5,
                current_weight: 0,
                volume_capacity: 4,
                current_volume: 0,
            };

            entity = Some(
                child_commands
                    .spawn_bundle(SpriteSheetBundle {
                        texture_atlas: textures.ship.clone(),
                        transform: Transform::from_xyz(0.5 * GRID_SIZE, 0.0, -0.5),
                        ..Default::default()
                    })
                    .insert(ShipArriving(slot_id))
                    .insert(previous_ship.unwrap_or_else(|| Ship::new(&mut rng)))
                    .insert(animations.ship_unfurl.clone())
                    .insert(AnimationState::default())
                    .insert(ship_hold.clone())
                    .with_children(|ship_child_commands| {
                        ship_child_commands
                            .spawn_bundle(Text2dBundle {
                                text: Text::from_sections([
                                    TextSection::new("W: ", text_style.clone()),   // 0
                                    TextSection::new("0", text_style.clone()),     // 1
                                    TextSection::new(" / 0", text_style.clone()),  // 2
                                    TextSection::new("\nV: ", text_style.clone()), // 3
                                    TextSection::new("0", text_style.clone()),     // 4
                                    TextSection::new(" / 0", text_style.clone()),  // 5
                                    TextSection::new("\n", text_style.clone()),    // 6
                                    TextSection::new(
                                        format!("{}", ship_hold.destination),
                                        text_style.clone(),
                                    ), // 7
                                ])
                                .with_alignment(TextAlignment::TOP_LEFT),
                                transform: Transform::from_xyz(-110.0, -120.0, 1.2),
                                ..default()
                            })
                            .insert(ShipText);
                    })
                    .id(),
            )
        });

    entity.unwrap()
}
