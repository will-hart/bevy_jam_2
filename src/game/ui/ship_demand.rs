use bevy::prelude::*;

use crate::game::{
    actions::OnDropCrateOnShip,
    components::{ShipDemandItemMarker, ShipHold},
};

pub fn remove_ship_demands_when_met(
    mut commands: Commands,
    mut drop_events: EventReader<OnDropCrateOnShip>,
    ships: Query<&Children, With<ShipHold>>,
    demand_markers: Query<(Entity, &ShipDemandItemMarker)>,
) {
    for evt in drop_events.iter() {
        if let Ok(ship_children) = ships.get(evt.ship_entity) {
            for child_entity in ship_children.iter().rev() {
                if let Ok((demand_entity, demand_item)) = demand_markers.get(*child_entity) {
                    if demand_item.0 == evt.box_type {
                        commands.entity(demand_entity).despawn_recursive();
                        break;
                    }
                }
            }
        }
    }
}
