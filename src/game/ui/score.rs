use bevy::prelude::*;
use num_format::{Locale, ToFormattedString};

use crate::{game::components::ScoreUi, loader::FontAssets, HEIGHT, WIDTH};

#[derive(Default, Debug)]
pub struct Score(pub u32);

pub fn score_display(score: Res<Score>, mut texts: Query<&mut Text, With<ScoreUi>>) {
    for mut text in texts.iter_mut() {
        text.sections[0].value = score.0.to_formatted_string(&Locale::en);
    }
}

pub fn score_update(mut score: ResMut<Score>) {
    if score.0 == 0 {
        warn!("Game Over");
        return;
    }

    score.0 -= 1;
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
