use bevy::prelude::*;
use iyes_loopless::state::NextState;
use num_format::{Locale, ToFormattedString};

use crate::{
    game::{
        actions::OnDropCrateOnShip,
        animation::OnShipArrivedAtDestination,
        components::{ScoreUi, ShipHold, Wave},
    },
    GameState,
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
    mut commands: Commands,
    mut score: ResMut<Score>,
    mut drop_on_ship_event: EventReader<OnDropCrateOnShip>,
) {
    for evt in drop_on_ship_event.iter() {
        score.0 += if evt.was_demanded { 10.0 } else { -5.0 };
    }

    if score.0 <= 0.0 {
        warn!("Game over, transitioning to game over state");
        commands.insert_resource(NextState(GameState::GameOver));
    }
}

pub fn despawn_ships_and_penalise(
    mut commands: Commands,
    mut score: ResMut<Score>,
    mut despawn_events: EventReader<OnShipArrivedAtDestination>,
    waves: Query<&Children, With<Wave>>,
    holds: Query<&ShipHold>,
) {
    for event in despawn_events.iter() {
        let wave_children = waves.get(event.0).expect("Should have a wave");

        for child in wave_children.iter() {
            if let Ok(hold) = holds.get(*child) {
                let unmet_demands = hold.get_unmet_demands().len() as f32;
                score.0 -= unmet_demands * 50.0;
                break;
            }
        }

        commands.entity(event.0).despawn_recursive();
    }
}
