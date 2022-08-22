use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;

use crate::{
    game::{actions::CART_SPRITE_HALF_WIDTH, components::Cart},
    input::{MousePosition, PlayerActions},
    GRID_SIZE,
};

use super::{
    dragging::{DraggingBox, OnStartDragging},
    dropping::{OnDropCrate, ShipSlots},
    CART_MAX_Y, CART_MIN_Y, SHIP_MAX_Y, SHIP_MIN_Y, SHIP_ZONES,
};

pub fn click_to_pickup(
    mouse_pos: Res<MousePosition>,
    dragging: Res<DraggingBox>,
    ship_slots: Res<ShipSlots>,
    mut start_events: EventWriter<OnStartDragging>,
    mut stop_events: EventWriter<OnDropCrate>,
    action_state_query: Query<&ActionState<PlayerActions>>,
    mut carts: Query<(Entity, &Transform), With<Cart>>,
) {
    let action_state = action_state_query.single();
    let x = mouse_pos.world.x;
    let y = mouse_pos.world.y;

    if action_state.just_pressed(PlayerActions::Click) {
        if y < CART_MIN_Y || y > CART_MAX_Y {
            return;
        }

        for (cart_ent, cart_tx) in carts.iter_mut() {
            let delta = x - cart_tx.translation.x;
            // carts are 160px wide, the last two grid squares (32px) are for boxes.
            // be a bit flexible with the clicking (i.e. doen't require directly on the sprite)
            if delta < (CART_SPRITE_HALF_WIDTH - 2.0 * GRID_SIZE) || delta > CART_SPRITE_HALF_WIDTH
            {
                continue;
            }

            let zone =
                (((x - cart_tx.translation.x) - (GRID_SIZE / 2.0)) / GRID_SIZE).floor() as u32;

            info!(
                "Clicked box {} on cart {:?} at {}",
                zone, cart_ent, cart_tx.translation
            );

            // trigger the event to handle drag start
            start_events.send(OnStartDragging {
                cart_entity: cart_ent,
                is_front_slot: zone == 0,
            });
        }
    } else if dragging.cart_entity.is_some() && action_state.just_released(PlayerActions::Click) {
        if y > SHIP_MIN_Y && y < SHIP_MAX_Y {
            for (idx, r) in SHIP_ZONES.iter().enumerate() {
                if r.contains(&x) {
                    info!("Dropped crate on ship slot {}", idx);

                    match ship_slots.slots[idx] {
                        Some(entity) => {
                            stop_events.send(OnDropCrate { ship: Some(entity) });
                            return;
                        }
                        None => {
                            info!("--> ship slot {} is empty", idx);
                            break;
                        }
                    }
                }
            }
        }

        stop_events.send(OnDropCrate { ship: None });
    }
}
