use bevy::prelude::*;
use iyes_loopless::{
    prelude::{AppLooplessStateExt, ConditionHelpers, IntoConditionalSystem},
    state::NextState,
};

use crate::{
    game::{
        components::MenuItem,
        day_night_cycle::{get_start_date, TimeOfDay},
    },
    loader::{FontAssets, TextureAssets},
    GameState,
};

use super::tutorial::was_action_pressed;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::Menu, spawn_menu)
            .add_exit_system(GameState::Menu, despawn_menu)
            .add_enter_system(GameState::GameOver, spawn_game_over)
            .add_exit_system(GameState::GameOver, despawn_game_over)
            .add_system(
                start_playing
                    .run_if(was_action_pressed)
                    .run_not_in_state(GameState::Playing)
                    .run_not_in_state(GameState::Loading),
            );
    }
}

fn spawn_menu(mut commands: Commands, textures: Res<TextureAssets>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: textures.menu.clone(),
            ..default()
        })
        .insert(MenuItem);
}

fn despawn_menu(mut commands: Commands, menu_items: Query<Entity, With<MenuItem>>) {
    for item in menu_items.iter() {
        commands.entity(item).despawn_recursive();
    }
}

fn start_playing(mut commands: Commands) {
    commands.insert_resource(NextState(GameState::Playing));
}

fn spawn_game_over(
    mut commands: Commands,
    time_of_day: Res<TimeOfDay>,
    fonts: Res<FontAssets>,
    textures: Res<TextureAssets>,
) {
    let elapsed_days = time_of_day.today - get_start_date();

    commands
        .spawn_bundle(SpriteBundle {
            texture: textures.game_over.clone(),
            ..default()
        })
        .insert(MenuItem)
        .with_children(|parent| {
            parent.spawn_bundle(Text2dBundle {
                text: Text::from_section(
                    format!("Your company survived {} days", elapsed_days.num_days()),
                    TextStyle {
                        color: Color::ANTIQUE_WHITE,
                        font: fonts.default_font.clone(),
                        font_size: 24.0,
                    },
                )
                .with_alignment(TextAlignment::CENTER),
                transform: Transform::from_xyz(0.0, -32.0, 1.0),
                ..default()
            });
        });
}

fn despawn_game_over(mut commands: Commands, menu_items: Query<Entity, With<MenuItem>>) {
    for item in menu_items.iter() {
        commands.entity(item).despawn_recursive();
    }

    commands.insert_resource(NextState(GameState::Playing));
}
