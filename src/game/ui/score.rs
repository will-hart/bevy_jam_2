use bevy::prelude::*;
use num_format::{Locale, ToFormattedString};

use crate::{
    game::{
        animation::DespawnEntity,
        components::{ScoreUi, ShipHold, Wave},
        market::Market,
    },
    loader::FontAssets,
    HEIGHT, WIDTH,
};

#[derive(Default, Debug)]
pub struct Score(pub u32);

pub fn score_display(score: Res<Score>, mut texts: Query<&mut Text, With<ScoreUi>>) {
    for mut text in texts.iter_mut() {
        text.sections[0].value = score.0.to_formatted_string(&Locale::en);
    }
}

pub fn score_update(
    mut commands: Commands,
    market: Res<Market>,
    mut score: ResMut<Score>,
    mut events: EventReader<DespawnEntity>,
    waves: Query<&Children, With<Wave>>,
    ships: Query<&ShipHold>,
) {
    for evt in events.iter() {
        match waves.get(evt.0) {
            Ok(wave_children) => {
                for child in wave_children.iter() {
                    let hold = ships.get(*child).expect("Should have a ship hold");

                    for box_type in hold.crates.iter() {
                        let box_score = market
                            .market
                            .get(&hold.destination)
                            .unwrap()
                            .get(&box_type)
                            .unwrap()
                            .current_price
                            .ceil() as u32;
                        info!(
                            "Sold crate of {:?} to {} for {}",
                            box_type, hold.destination, box_score
                        );
                        score.0 += box_score;
                    }
                }

                commands.entity(evt.0).despawn_recursive();
            }
            _ => {
                // not a wave
            }
        }
    }

    if score.0 == 0 {
        warn!("Game Over");
        return;
    }

    // score.0 -= 1;
}

pub fn spawn_score_ui(mut commands: Commands, fonts: Res<FontAssets>) {
    let text_style = TextStyle {
        font: fonts.default_font.clone(),
        font_size: 24.0,
        color: Color::WHITE,
    };

    commands
        .spawn_bundle(Text2dBundle {
            text: Text::from_section("0", text_style).with_alignment(TextAlignment::TOP_RIGHT),
            transform: Transform::from_xyz(WIDTH / 2.0 - 15.0, HEIGHT / 2.0 - 15.0, 0.3),
            ..default()
        })
        .insert(ScoreUi);
}
