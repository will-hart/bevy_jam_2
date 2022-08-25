use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;

use crate::{
    game::{actions::CART_SPRITE_HALF_WIDTH, components::Cart},
    input::{MousePosition, PlayerActions},
    GRID_SIZE,
};

use super::{
    dragging::{DraggingBox, OnStartDragging},
    dropping::OnDropCrate,
    CART_MAX_Y, CART_MIN_Y,
};

pub fn click_to_pickup(
    mouse_pos: Res<MousePosition>,
    dragging: Res<DraggingBox>,
    mut start_events: EventWriter<OnStartDragging>,
    mut drop_events: EventWriter<OnDropCrate>,
    action_state_query: Query<&ActionState<PlayerActions>>,
    mut carts: Query<(Entity, &Transform), With<Cart>>,
) {
    let action_state = action_state_query.single();
    let x = mouse_pos.world.x;
    let y = mouse_pos.world.y;

    if action_state.just_pressed(PlayerActions::Click) {
        if !(CART_MIN_Y..=CART_MAX_Y).contains(&y) {
            return;
        }

        for (cart_ent, cart_tx) in carts.iter_mut() {
            let delta = x - cart_tx.translation.x;
            // carts are 160px wide, the last two grid squares (32px) are for boxes.
            // be a bit flexible with the clicking (i.e. doen't require directly on the sprite)
            if !((CART_SPRITE_HALF_WIDTH - 2.0 * GRID_SIZE)..=CART_SPRITE_HALF_WIDTH)
                .contains(&delta)
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
    } else if dragging.box_entity.is_some() && action_state.just_released(PlayerActions::Click) {
        drop_events.send(OnDropCrate);
    }
}
