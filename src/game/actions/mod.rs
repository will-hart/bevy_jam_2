use bevy::prelude::*;
use iyes_loopless::prelude::IntoConditionalSystem;

use crate::{GameState, GRID_SIZE};

use self::dragging::{DraggingBox, StartDragging, StopDragging};

mod dragging;
mod pickup;

// Numbers that are the range cart boxes can fall into
const CART_MIN_Y: f32 = -2.5 * GRID_SIZE + 24.0; // magic numbers
const CART_MAX_Y: f32 = CART_MIN_Y + 28.;
const CART_SPRITE_HALF_WIDTH: f32 = 80.0;

pub struct ActionPlugin;

impl Plugin for ActionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DraggingBox {
            cart_entity: None,
            box_type: None,
            is_front_slot: true,
        })
        .add_event::<StartDragging>()
        .add_event::<StopDragging>()
        .add_system(pickup::click_to_pickup.run_in_state(GameState::Playing))
        .add_system(dragging::start_dragging.run_in_state(GameState::Playing))
        .add_system(dragging::mouse_follower.run_in_state(GameState::Playing))
        .add_system(dragging::stop_dragging.run_in_state(GameState::Playing));
    }
}
