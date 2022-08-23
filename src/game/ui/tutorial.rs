// Tutorials:
// level 0 - display helpful spawn text, hidden in request_ship::ship_spawn_handler

use bevy::prelude::*;
use iyes_loopless::prelude::IntoConditionalSystem;

use crate::{
    game::{
        actions::OnCrateDroppedOnShip,
        components::{RequestShip, TopUiBar, TutorialMarker},
        ship_launch::OnLaunchShip,
    },
    loader::FontAssets,
};

/// A resource which holds the current tutorial level
pub struct CurrentTutorialLevel(pub u8);

pub struct TutorialPlugin;

impl Plugin for TutorialPlugin {
    fn build(&self, app: &mut App) {
        // start tutorial at level 1 because level 0 spawns with the menu
        app.insert_resource(CurrentTutorialLevel(1))
            .add_system(level1_tutorial.run_if(is_tutorial_level::<1>))
            .add_system(level2_tutorial.run_if(is_tutorial_level::<2>))
            .add_system(level3_tutorial.run_if(is_tutorial_level::<3>));
    }
}

fn is_tutorial_level<const N: u8>(tutorial: Res<CurrentTutorialLevel>) -> bool {
    tutorial.0 == N
}

fn level1_tutorial(
    mut tutorial: ResMut<CurrentTutorialLevel>,
    mut on_crate_dropped: EventReader<OnCrateDroppedOnShip>,
    mut tutorials: Query<(&mut TutorialMarker, &mut Text)>,
) {
    for _ in on_crate_dropped.iter() {
        for (mut marker, mut text) in tutorials.iter_mut() {
            if marker.0 == 1 {
                text.sections[0].value = "Good! Now \"Set Sail\" when the ship is loaded!".into();
                marker.0 = 2;
            }
        }

        tutorial.0 = 2;
    }
}

fn level2_tutorial(
    mut commands: Commands,
    fonts: Res<FontAssets>,
    mut tutorial: ResMut<CurrentTutorialLevel>,
    mut events: EventReader<OnLaunchShip>,
    tutorials: Query<(Entity, &TutorialMarker)>,
    top_ui_bar: Query<Entity, With<TopUiBar>>,
) {
    for _ in events.iter() {
        for (ent, marker) in tutorials.iter() {
            if marker.0 == 2 {
                commands.entity(ent).despawn_recursive();
            }
        }

        tutorial.0 = 3;

        let bar = top_ui_bar.single();
        commands.entity(bar).with_children(|builder| {
            builder
                .spawn_bundle(TextBundle {
                    text: Text::from_section(
                        "More ships will appear here!",
                        TextStyle {
                            font: fonts.default_font.clone(),
                            font_size: 12.0,
                            color: Color::WHITE,
                        },
                    ),
                    ..default()
                })
                .insert(TutorialMarker(3));
        });
    }
}

fn level3_tutorial(
    mut commands: Commands,
    mut tutorial: ResMut<CurrentTutorialLevel>,
    items: Query<Entity, Added<RequestShip>>,
    tutorials: Query<(Entity, &TutorialMarker)>,
) {
    if items.is_empty() {
        return;
    }

    for (entity, marker) in tutorials.iter() {
        if marker.0 >= tutorial.0 {
            continue;
        }

        commands.entity(entity).despawn_recursive();
    }

    tutorial.0 = 4;
}
