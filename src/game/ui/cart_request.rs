use bevy::prelude::*;

use crate::{
    game::{
        components::{CartQueueUi, CartQueueUiItem},
        spawners::CartSpawningState,
    },
    loader::TextureAssets,
};

pub fn update_cart_request_queue(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    cart_spawn: Res<CartSpawningState>,
    parent_query: Query<Entity, With<CartQueueUi>>,
    mut queue_items: Query<(Entity, &mut UiImage), With<CartQueueUiItem>>,
) {
    let mut ui_items = queue_items.iter_mut().collect::<Vec<_>>();
    let num_ui_items = ui_items.len();

    let num_queue_items = cart_spawn.items.len();

    let parent = parent_query.single();

    // loop through all the items and updated/add/delete accordingly
    for idx in 0..(num_ui_items.max(num_queue_items)) {
        // create
        if idx >= num_ui_items {
            commands.entity(parent).with_children(|p| {
                p.spawn_bundle(ImageBundle {
                    image: cart_spawn.items[idx].get_image(&textures).into(),
                    ..default()
                })
                .insert(CartQueueUiItem);
            });
            continue;
        }

        // delete
        if idx >= num_queue_items {
            commands.entity(ui_items[idx].0).despawn_recursive();
            continue;
        }

        // update
        *ui_items[idx].1 = cart_spawn.items[idx].get_image(&textures).into();
    }
}
