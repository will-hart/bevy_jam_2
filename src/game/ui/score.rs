use bevy::prelude::*;
use num_format::{Locale, ToFormattedString};

use crate::game::{
    animation::ShipArrivedAtDestination,
    components::{ScoreUi, Ship, ShipHold, Wave},
    market::Market,
};

#[derive(Default, Debug)]
pub struct Score(pub f32);

pub fn score_display(score: Res<Score>, mut texts: Query<&mut Text, With<ScoreUi>>) {
    for mut text in texts.iter_mut() {
        text.sections[0].value = (score.0.ceil() as u32).to_formatted_string(&Locale::en);
    }
}

pub fn score_update(
    market: Res<Market>,
    time: Res<Time>,
    mut score: ResMut<Score>,
    mut events: EventReader<ShipArrivedAtDestination>,
    waves: Query<&Children, With<Wave>>,
    ships: Query<(&Ship, &ShipHold)>,
) {
    for evt in events.iter() {
        match waves.get(evt.0) {
            Ok(wave_children) => {
                for child in wave_children.iter() {
                    let (_, hold) = ships.get(*child).expect("Should have a ship hold");

                    for box_type in hold.crates.iter() {
                        let box_score = market
                            .market
                            .get(&hold.destination)
                            .unwrap()
                            .get(box_type)
                            .unwrap()
                            .current_price;
                        info!(
                            "Sold crate of {:?} to {} for {}",
                            box_type, hold.destination, box_score
                        );
                        score.0 += box_score;
                    }
                }
            }
            _ => {
                // not a wave
                warn!("Somehow ship despawn event was called on not a ship :confused:")
            }
        }
    }

    if score.0 < 0.01 {
        warn!("Game Over");
        return;
    }

    let _dt = time.delta_seconds();
}
