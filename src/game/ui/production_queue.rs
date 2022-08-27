use bevy::prelude::*;

use crate::{
    game::{
        components::{ProductionQueueUi, ProductionQueueUiItem},
        factory::events::{OnFactoryFinishProducing, OnFactoryQueueItem},
    },
    loader::TextureAssets,
    GRID_SIZE,
};

pub fn update_production_queue(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    mut started_production: EventReader<OnFactoryQueueItem>,
    mut finished_production: EventReader<OnFactoryFinishProducing>,
    menu_bar: Query<Entity, With<ProductionQueueUi>>,
    queue_items: Query<(Entity, &ProductionQueueUiItem)>,
) {
    let parent = menu_bar.single();
    for new_item in started_production.iter() {
        info!(
            "New production item {:?} added to production queue UI",
            new_item.box_type
        );
        commands.entity(parent).with_children(|p| {
            p.spawn_bundle(ImageBundle {
                image: new_item.box_type.get_image(&textures).into(),
                style: Style {
                    size: Size::new(Val::Px(GRID_SIZE), Val::Px(GRID_SIZE)),
                    ..default()
                },
                ..default()
            })
            .insert(ProductionQueueUiItem(new_item.box_type));
        });
    }

    for removed_item in finished_production.iter() {
        for (queue_ent, item) in queue_items.iter() {
            if item.0 == removed_item.box_type {
                info!(
                    "Removed production item {:?} from production queue UI",
                    removed_item.box_type
                );
                commands.entity(queue_ent).despawn_recursive();
                break;
            }
        }
    }
}
