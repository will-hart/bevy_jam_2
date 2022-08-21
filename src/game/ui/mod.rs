use bevy::prelude::*;
use iyes_loopless::prelude::IntoConditionalSystem;

use crate::GameState;

use super::{actions::OnCrateDroppedOnShip, components::ShipHold};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_ship_hold_data.run_not_in_state(GameState::Loading));
    }
}

/// Updates the ship hold data when a new crate is dropped
fn update_ship_hold_data(
    mut events: EventReader<OnCrateDroppedOnShip>,
    ships: Query<(&Children, &ShipHold)>,
    mut texts: Query<&mut Text>,
) {
    let mut done = false;

    for evt in events.iter() {
        if done {
            continue;
        }
        done = true;

        let (children, ship_hold) = ships.get(evt.0).expect("ship should exist");

        for child in children.iter() {
            match texts.get_mut(*child) {
                Ok(mut text) => text.sections[0].value = format!("{}", ship_hold),
                _ => {}
            }
        }
    }
}
