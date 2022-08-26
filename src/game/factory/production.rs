use bevy::prelude::*;

use crate::{
    game::{
        components::{BoxType, CountDownTimer, FactoryProductionIndicator},
        factory::utils::new_timer,
        spawners::spawn_physics_crate,
    },
    loader::TextureAssets,
    GRID_SIZE,
};

use super::{
    events::{OnFactoryFinishProducing, OnFactoryStartProducing},
    recipes::Recipes,
    OnDropInFactoryInput,
};

pub const FACTORY_OUTPUT_LOCATION: Vec3 = Vec3::new(0.0, 5.0 * GRID_SIZE, 0.0);
pub const FACTORY_OUTPUT_INITIAL_VELOCITY: Vec2 = Vec2::new(-200.0, 100.0);
pub const FACTORY_OUTPUT_INDICATOR_LOCATION: Vec3 =
    Vec3::new(-0.5 * GRID_SIZE, 3.0 * GRID_SIZE, 6.0);

#[derive(Default, Debug)]
pub struct Factory {
    pub inputs: [Option<BoxType>; 2],
    pub output_queue: Vec<BoxType>,
    pub is_producing: bool,
}

impl Factory {
    pub fn drop(&mut self, box_type: BoxType) {
        if self.inputs[0].is_none() {
            self.inputs[0] = Some(box_type);
        } else {
            self.inputs[1] = Some(box_type);
        }
    }

    /// Reset the factory, setting all its inputs to 0
    pub fn reset(&mut self) {
        self.inputs[0] = None;
        self.inputs[1] = None;
    }

    // Does the factory have a completed recipe
    pub fn has_completed_recipe(&self) -> bool {
        self.inputs[0].is_some() && self.inputs[1].is_some()
    }
}

pub fn add_item_to_factory(
    recipes: Res<Recipes>,
    mut factory: ResMut<Factory>,
    mut drop_events: EventReader<OnDropInFactoryInput>,
) {
    for drop_event in drop_events.iter() {
        factory.drop(drop_event.box_type);

        if !factory.has_completed_recipe() {
            return;
        }

        if let Some(bt) = recipes.get_output(&factory.inputs) {
            info!(
                "Factory receive two inputs {:?} and {:?} which has a recipe of {:?}",
                factory.inputs[0], factory.inputs[1], bt
            );

            factory.output_queue.push(bt);
        } else {
            info!(
                "No recipe available, factory contains {:?}. Resetting",
                factory
            );
        };

        factory.reset();
    }
}

pub fn start_factory_production(
    mut factory: ResMut<Factory>,
    mut start_production_events: EventWriter<OnFactoryStartProducing>,
) {
    if factory.output_queue.is_empty() || factory.is_producing {
        return;
    }

    info!("Started production of {:?}", factory.output_queue[0]);
    start_production_events.send(OnFactoryStartProducing {
        box_type: factory.output_queue[0],
    });
    factory.is_producing = true;
}

pub fn finish_factory_production(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    mut factory: ResMut<Factory>,
    mut produced_events: EventWriter<OnFactoryFinishProducing>,
    production_items: Query<(Entity, &CountDownTimer), With<FactoryProductionIndicator>>,
) {
    for (entity, timer) in production_items.iter() {
        if timer.0.just_finished() {
            let built = factory.output_queue.remove(0);
            produced_events.send(OnFactoryFinishProducing { box_type: built });
            factory.is_producing = false;
            info!("Finished production of {:?}", built);

            // spawn a sprite
            commands.entity(entity).despawn_recursive();
            let sprite = commands
                .spawn_bundle(SpriteBundle {
                    texture: built.get_image(&textures).into(),
                    transform: Transform::from_translation(FACTORY_OUTPUT_LOCATION),
                    ..default()
                })
                .id();

            // turn it into a physics box
            spawn_physics_crate(
                &mut commands,
                sprite,
                built,
                FACTORY_OUTPUT_INITIAL_VELOCITY,
            );
        }
    }
}

pub fn handle_production_started(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    mut started_events: EventReader<OnFactoryStartProducing>,
) {
    for evt in started_events.iter() {
        commands
            .spawn_bundle(SpriteBundle {
                texture: textures.countdown[9].clone().into(),
                transform: Transform::from_translation(FACTORY_OUTPUT_INDICATOR_LOCATION),
                ..default()
            })
            .insert(new_timer())
            .insert(FactoryProductionIndicator)
            .with_children(|children| {
                children.spawn_bundle(SpriteBundle {
                    texture: evt.box_type.get_image(&textures),
                    transform: Transform::from_translation(Vec3::new(GRID_SIZE, 0.0, 0.0)),
                    ..default()
                });
            });
    }
}
