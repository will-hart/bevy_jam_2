use bevy::prelude::*;
use num_format::{Locale, ToFormattedString};

use crate::game::{components::ScoreUi, market::Market, ship_launch::OnShipScore};

pub struct OnCoinsReceived;

#[derive(Default, Debug)]
pub struct Score(pub f32);

pub fn score_display(score: Res<Score>, mut texts: Query<&mut Text, With<ScoreUi>>) {
    for mut text in texts.iter_mut() {
        text.sections[0].value = (score.0.ceil() as u32).to_formatted_string(&Locale::en);
    }
}

pub fn score_update(
    market: Res<Market>,
    mut score: ResMut<Score>,
    mut events: EventReader<OnShipScore>,
    mut score_effects: EventWriter<OnCoinsReceived>,
) {
    for evt in events.iter() {
        let hold = &evt.ship_hold;
        let mut total_score: f32 = 0.0;

        for box_type in hold.crates.iter() {
            let box_score = market.market.get(box_type).unwrap().current_price;
            info!(
                "Sold crate of {:?} to {} for {}",
                box_type, hold.destination, box_score
            );
            score.0 += box_score;
            total_score += box_score;
        }

        if total_score > 0.0 {
            score_effects.send(OnCoinsReceived);
        }
    }
}
