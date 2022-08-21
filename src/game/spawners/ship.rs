use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::{thread_rng, Rng};

use crate::{
    game::{
        components::{Ship, Wave},
        custom_sprite::CustomSpriteMaterial,
    },
    loader::TextureAssets,
    GRID_SIZE,
};

pub fn spawn_ship(
    commands: &mut Commands,
    textures: &TextureAssets,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<CustomSpriteMaterial>,
    location: Vec3,
) -> Entity {
    let mut rng = thread_rng();
    let mut entity: Option<Entity> = None;

    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad::new(Vec2::new(228.0, 64.0))))
                .into(),
            transform: Transform::from_translation(location - Vec3::Y * GRID_SIZE * 5.0),
            material: materials.add(textures.waves.clone().into()),
            ..Default::default()
        })
        .insert(Wave)
        .with_children(|child_commands| {
            entity = Some(
                child_commands
                    .spawn_bundle(SpriteSheetBundle {
                        texture_atlas: textures.ship.clone(),
                        transform: Transform::from_xyz(0.5 * GRID_SIZE, 0.0, -0.5),
                        ..Default::default()
                    })
                    .insert(Ship {
                        y: 4.0 * GRID_SIZE,
                        phase: rng.gen_range(-3.1..3.1),
                        crates: vec![],
                    })
                    .id(),
            )
        });

    entity.unwrap()
}
