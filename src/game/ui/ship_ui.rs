use bevy::prelude::*;

use crate::game::{actions::OnCrateDroppedOnShip, components::ShipHold};

/// Updates the ship hold data when a new crate is dropped
pub fn update_ship_hold_data(
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
            if let Ok(mut text) = texts.get_mut(*child) {
                text.sections[1].value = format!("{}", ship_hold.current_weight);
                text.sections[1].style.color =
                    if ship_hold.current_weight > ship_hold.weight_capacity {
                        Color::RED
                    } else {
                        Color::WHITE
                    };

                text.sections[2].value = format!(" / {}", ship_hold.weight_capacity);
                text.sections[2].style.color =
                    if ship_hold.current_weight > ship_hold.weight_capacity {
                        Color::RED
                    } else {
                        Color::WHITE
                    };

                text.sections[4].value = format!("{}", ship_hold.current_volume);
                text.sections[4].style.color =
                    if ship_hold.current_volume > ship_hold.volume_capacity {
                        Color::RED
                    } else {
                        Color::WHITE
                    };

                text.sections[5].value = format!(" / {}", ship_hold.volume_capacity);
                text.sections[5].style.color =
                    if ship_hold.current_volume > ship_hold.volume_capacity {
                        Color::RED
                    } else {
                        Color::WHITE
                    };
            }
        }
    }
}
