use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::{thread_rng, Rng};

use crate::{
    game::{
        components::{Ship, ShipHold, Wave},
        custom_sprite::CustomSpriteMaterial,
    },
    loader::{FontAssets, TextureAssets},
    GRID_SIZE,
};

pub fn spawn_ship(
    commands: &mut Commands,
    textures: &TextureAssets,
    fonts: &FontAssets,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<CustomSpriteMaterial>,
    location: Vec3,
) -> Entity {
    let mut rng = thread_rng();
    let mut entity: Option<Entity> = None;

    let text_style = TextStyle {
        font: fonts.default_font.clone(),
        font_size: 16.0,
        color: Color::WHITE,
    };

    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad::new(Vec2::new(288.0, 64.0))))
                .into(),
            transform: Transform::from_translation(location - Vec3::Y * GRID_SIZE * 4.0),
            material: materials.add(textures.waves.clone().into()),
            ..Default::default()
        })
        .insert(Wave)
        .with_children(|child_commands| {
            let ship_hold = ShipHold {
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
                    .insert(Ship {
                        y_offset: 4.0 * GRID_SIZE,
                        phase: rng.gen_range(-3.1..3.1),
                    })
                    .insert(ship_hold.clone())
                    .with_children(|ship_child_commands| {
                        ship_child_commands.spawn_bundle(Text2dBundle {
                            text: Text::from_sections([
                                TextSection::new("W: ", text_style.clone()),      // 0
                                TextSection::new("0", text_style.clone()),        // 1
                                TextSection::new(" / 0", text_style.clone()),     // 2
                                TextSection::new("\nV: ", text_style.clone()),    // 3
                                TextSection::new("0", text_style.clone()),        // 4
                                TextSection::new(" / 0", text_style.clone()),     // 5
                                TextSection::new("\n", text_style.clone()),       // 6
                                TextSection::new("Americas", text_style.clone()), // 7
                            ])
                            .with_alignment(TextAlignment::TOP_LEFT),
                            transform: Transform::from_xyz(-110.0, -120.0, 1.2),
                            ..default()
                        });
                    })
                    .id(),
            )
        });

    entity.unwrap()
}
