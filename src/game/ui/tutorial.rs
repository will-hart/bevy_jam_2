// Tutorials:
// level 0 - spawn help graphics

use bevy::prelude::*;
use iyes_loopless::prelude::{AppLooplessStateExt, ConditionHelpers, IntoConditionalSystem};
use leafwing_input_manager::prelude::ActionState;

use crate::{
    game::{
        actions::OnDropCrateOnShip,
        components::TutorialMarker,
        factory::events::{OnFactoryFinishProducing, OnFactoryStartProducing},
        spawners::OnCartSpawned,
    },
    input::PlayerActions,
    loader::TextureAssets,
    GameState,
};

/// A resource which holds the current tutorial level
pub struct CurrentTutorialLevel(pub u8);

pub struct TutorialPlugin;

impl Plugin for TutorialPlugin {
    fn build(&self, app: &mut App) {
        info!("Mounting tutorial plugin");
        // start tutorial at level 1 because level 0 spawns with the menu
        app.insert_resource(CurrentTutorialLevel(1))
            .add_enter_system(GameState::Playing, spawn_tutorial)
            .add_system(
                level1_tutorial
                    .run_if(is_tutorial_level::<1>)
                    .run_on_event::<OnCartSpawned>(),
            )
            .add_system(
                level2_tutorial
                    .run_if(is_tutorial_level::<2>)
                    .run_if(was_action_pressed),
            )
            .add_system(
                level3_tutorial
                    .run_if(is_tutorial_level::<3>)
                    .run_on_event::<OnFactoryStartProducing>(),
            )
            .add_system(
                level4_tutorial
                    .run_if(is_tutorial_level::<4>)
                    .run_on_event::<OnFactoryFinishProducing>(),
            )
            .add_system(
                level6_tutorial
                    .run_if(is_tutorial_level::<6>)
                    .run_if(was_action_pressed),
            )
            .add_system(
                level7_tutorial
                    .run_if(is_tutorial_level::<7>)
                    .run_on_event::<OnDropCrateOnShip>(),
            )
            .add_system(
                level8_tutorial
                    .run_if(is_tutorial_level::<8>)
                    .run_if(was_action_pressed),
            );
    }
}

fn spawn_tutorial(
    mut commands: Commands,
    tutorial: Res<CurrentTutorialLevel>,
    textures: Res<TextureAssets>,
) {
    info!("Spawning tutorial step 1");
    if tutorial.0 > 1 {
        info!("--> skipping as its already spawned");
        return;
    }

    spawn_tutorial_image(&mut commands, textures.tutorial_1.clone());
}

fn spawn_tutorial_image(commands: &mut Commands, texture: Handle<Image>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: texture.into(),
            transform: Transform::from_xyz(0.0, 0.0, 30.0),
            ..default()
        })
        .insert(TutorialMarker(0));
}

fn is_tutorial_level<const N: u8>(tutorial: Res<CurrentTutorialLevel>) -> bool {
    tutorial.0 == N
}

pub fn was_action_pressed(action_states: Query<&ActionState<PlayerActions>>) -> bool {
    let action_state = action_states.single();
    action_state.just_pressed(PlayerActions::Proceed)
}

fn level1_tutorial(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    mut tutorial: ResMut<CurrentTutorialLevel>,
    tutorial_items: Query<Entity, With<TutorialMarker>>,
) {
    info!("Completed tutorial step 1 - requesting resources");

    for entity in tutorial_items.iter() {
        commands.entity(entity).despawn_recursive();
    }

    // spawn the second level tutorial image
    spawn_tutorial_image(&mut commands, textures.tutorial_2.clone());

    // go to the next tutorial level
    tutorial.0 = 2;
}

fn level2_tutorial(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    mut tutorial: ResMut<CurrentTutorialLevel>,
    tutorial_items: Query<Entity, With<TutorialMarker>>,
) {
    info!("Completed tutorial step 2 - reviewing recipes");

    for entity in tutorial_items.iter() {
        commands.entity(entity).despawn_recursive();
    }

    // spawn the second level tutorial image
    spawn_tutorial_image(&mut commands, textures.tutorial_3.clone());

    // go to the next tutorial level
    tutorial.0 = 3;
}

fn level3_tutorial(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    mut tutorial: ResMut<CurrentTutorialLevel>,
    tutorial_items: Query<Entity, With<TutorialMarker>>,
) {
    info!("Completed tutorial step 3 - placing crates in a factory");

    for entity in tutorial_items.iter() {
        commands.entity(entity).despawn_recursive();
    }

    // spawn the second level tutorial image
    spawn_tutorial_image(&mut commands, textures.tutorial_4.clone());

    // go to the next tutorial level
    tutorial.0 = 4;
}

fn level4_tutorial(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    mut tutorial: ResMut<CurrentTutorialLevel>,
    tutorial_items: Query<Entity, With<TutorialMarker>>,
) {
    info!("Completed tutorial step 4 - waiting for production to finish");

    for entity in tutorial_items.iter() {
        commands.entity(entity).despawn_recursive();
    }

    // spawn the second level tutorial image
    spawn_tutorial_image(&mut commands, textures.tutorial_6.clone());

    // go to the next tutorial level, here a ship will be spawned and tutorial level 6 will be set
    // by the ship spawning system
    tutorial.0 = 5;
}

fn level6_tutorial(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    mut tutorial: ResMut<CurrentTutorialLevel>,
    tutorial_items: Query<Entity, With<TutorialMarker>>,
) {
    info!("Completed tutorial step 6 - review ship arrival");

    for entity in tutorial_items.iter() {
        commands.entity(entity).despawn_recursive();
    }

    // spawn the second level tutorial image
    spawn_tutorial_image(&mut commands, textures.tutorial_7.clone());

    // go to the next tutorial level
    tutorial.0 = 7;
}

fn level7_tutorial(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    mut tutorial: ResMut<CurrentTutorialLevel>,
    tutorial_items: Query<Entity, With<TutorialMarker>>,
) {
    info!("Completed tutorial step 7 - placing crates on ship");

    for entity in tutorial_items.iter() {
        commands.entity(entity).despawn_recursive();
    }

    // spawn the second level tutorial image
    spawn_tutorial_image(&mut commands, textures.tutorial_8.clone());

    // go to the next tutorial level
    tutorial.0 = 8;
}

fn level8_tutorial(
    mut commands: Commands,
    mut tutorial: ResMut<CurrentTutorialLevel>,
    tutorial_items: Query<Entity, With<TutorialMarker>>,
) {
    info!("Completed tutorial step 8, tutorial complete");

    for entity in tutorial_items.iter() {
        commands.entity(entity).despawn_recursive();
    }

    // go to the next tutorial level, here a ship will be spawned and tutorial level 6 will be set
    tutorial.0 = 9;
}
