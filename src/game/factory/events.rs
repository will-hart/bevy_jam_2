use crate::{
    game::{
        components::{BoxType, FactoryGraphic},
        spawners::spawn_physics_crate,
        Animation,
    },
    loader::{AnimationAssets, TextureAssets},
    GRID_SIZE,
};
use bevy::prelude::*;

pub struct OnDropInFactoryInput {
    pub box_type: BoxType,
}

pub struct OnFactoryStartProducing {
    pub box_type: BoxType,
}

pub struct OnFactoryFinishProducing {
    pub box_type: BoxType,
}

pub struct OnFactoryQueueItem {
    pub box_type: BoxType,
}

pub struct OnIncorrectFactoryRecipe(pub BoxType, pub BoxType, pub Vec3);

pub struct OnIncorrectFactoryRecipeEffects;

pub fn show_factory_on_animation(
    animations: Res<AnimationAssets>,
    mut items: Query<&mut Handle<Animation>, With<FactoryGraphic>>,
) {
    for mut item in items.iter_mut() {
        *item = animations.factory_on.clone();
    }
}

pub fn show_factory_off_animation(
    animations: Res<AnimationAssets>,
    mut items: Query<&mut Handle<Animation>, With<FactoryGraphic>>,
) {
    for mut item in items.iter_mut() {
        *item = animations.factory_off.clone();
    }
}

pub fn reject_crates_on_incorrect_input(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    mut incorrect_recipe_events: EventReader<OnIncorrectFactoryRecipe>,
    mut play_incorrect_recipe_effects: EventWriter<OnIncorrectFactoryRecipeEffects>,
) {
    for evt in incorrect_recipe_events.iter() {
        let e1 = commands
            .spawn_bundle(SpriteBundle {
                texture: evt.0.get_image(&textures),
                transform: Transform::from_xyz(evt.2.x - GRID_SIZE, evt.2.y, 5.0),
                ..default()
            })
            .id();

        let e2 = commands
            .spawn_bundle(SpriteBundle {
                texture: evt.1.get_image(&textures),
                transform: Transform::from_xyz(evt.2.x + GRID_SIZE, evt.2.y, 5.0),
                ..default()
            })
            .id();

        spawn_physics_crate(&mut commands, e1, evt.0, Vec2::new(-200.0, 50.0));
        spawn_physics_crate(&mut commands, e2, evt.1, Vec2::new(200.0, 50.0));
        play_incorrect_recipe_effects.send(OnIncorrectFactoryRecipeEffects);
        return;
    }
}
