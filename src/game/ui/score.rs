use bevy::prelude::*;
use num_format::{Locale, ToFormattedString};

use crate::game::{
    actions::OnDropCrateOnShip,
    components::{ScoreUi, ShipHold},
};

/// Event triggered when a player receives coins, allowing effects to be played
pub struct OnCoinsReceived;

/// Event triggered when a player should receive a score update, based on the ship hold
pub struct OnShipScore {
    pub ship_hold: ShipHold,
}

#[derive(Default, Debug)]
pub struct Score(pub f32);

pub fn score_display(score: Res<Score>, mut texts: Query<&mut Text, With<ScoreUi>>) {
    for mut text in texts.iter_mut() {
        text.sections[0].value = (score.0.ceil() as u32).to_formatted_string(&Locale::en);
    }
}

pub fn score_update(
    mut score: ResMut<Score>,
    mut drop_on_ship_event: EventReader<OnDropCrateOnShip>,
) {
    for evt in drop_on_ship_event.iter() {
        score.0 += if evt.was_demanded { 10.0 } else { -5.0 };
    }
}
