use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;

use crate::{
    game::{
        actions::{dragging::StartDraggingItem, CART_SPRITE_HALF_WIDTH},
        components::{Cart, PhysicsCrate},
    },
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
    carts: Query<(Entity, &Transform), With<Cart>>,
    boxes: Query<(Entity, &Transform), With<PhysicsCrate>>,
) {
    let action_state = action_state_query.single();
    let x = mouse_pos.world.x;
    let y = mouse_pos.world.y;

    if action_state.just_pressed(PlayerActions::Click) {
        for (cart_ent, cart_tx) in carts.iter() {
            let delta = x - cart_tx.translation.x;
            // carts are 160px wide, the last two grid squares (32px) are for boxes.
            // be a bit flexible with the clicking (i.e. doen't require directly on the sprite)
            if !((CART_SPRITE_HALF_WIDTH - 2.0 * GRID_SIZE)..=CART_SPRITE_HALF_WIDTH)
                .contains(&delta)
                || !(CART_MIN_Y..=CART_MAX_Y).contains(&y)
            {
                continue;
            }

            let zone =
                (((x - cart_tx.translation.x) - (GRID_SIZE / 2.0)) / GRID_SIZE).floor() as u32;

            info!(
                "Picking up box {} on cart {:?} at {} in the {} zone",
                zone,
                cart_ent,
                cart_tx.translation,
                if zone == 0 { "front" } else { "back" }
            );

            // trigger the event to handle drag start
            start_events.send(OnStartDragging {
                dragged_entity: StartDraggingItem::Cart(cart_ent),
                is_front_slot: zone == 0,
            });

            return;
        }

        // go through the physics boxes and see if we should pick one up
        // this test is a bit simpler because we can just check x/y bounds
        for (box_ent, box_tx) in boxes.iter() {
            let in_x = ((box_tx.translation.x - GRID_SIZE / 2.0)
                ..(box_tx.translation.x + GRID_SIZE / 2.0))
                .contains(&x);
            let in_y = ((box_tx.translation.y - GRID_SIZE / 2.0)
                ..(box_tx.translation.y + GRID_SIZE / 2.0))
                .contains(&y);

            if in_x && in_y {
                info!(
                    "Picking up physics crate {:?} from the warehouse at {}",
                    box_ent, box_tx.translation
                );

                // trigger the event to handle drag start
                start_events.send(OnStartDragging {
                    dragged_entity: StartDraggingItem::PhysicsCrate(box_ent),
                    is_front_slot: true,
                });
                return;
            }
        }
    } else if dragging.box_entity.is_some() && action_state.just_released(PlayerActions::Click) {
        drop_events.send(OnDropCrate);
    }
}
