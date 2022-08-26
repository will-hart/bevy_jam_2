use bevy::prelude::*;

use crate::{
    game::{components::FactoryInputsDisplayItem, factory::Factory},
    loader::TextureAssets,
    GRID_SIZE,
};

pub fn spawn_factory_ui(mut commands: Commands, textures: Res<TextureAssets>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: textures.box_type_fruit.clone(),
            transform: Transform::from_xyz(-1.5 * GRID_SIZE, GRID_SIZE, 3.0)
                .with_scale(Vec3::splat(0.75)),
            ..default()
        })
        .insert(FactoryInputsDisplayItem(0));
}

pub fn update_factory_input_ui(
    textures: Res<TextureAssets>,
    factory: Res<Factory>,
    mut items: Query<(
        &mut Handle<Image>,
        &mut Visibility,
        &FactoryInputsDisplayItem,
    )>,
) {
    for (mut image, mut visibility, factory_info) in items.iter_mut() {
        visibility.is_visible = factory.inputs[factory_info.0].is_some();
        if !visibility.is_visible {
            continue;
        }

        *image = factory.inputs[factory_info.0]
            .unwrap()
            .get_image(&textures)
            .into();
    }
}
