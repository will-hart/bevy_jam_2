use bevy::prelude::*;
use iyes_loopless::prelude::IntoConditionalSystem;

use crate::{GameState, GRID_SIZE};

use self::{
    dragging::{DraggingBox, OnStartDragging},
    dropping::OnDropCrate,
};

mod dragging;
mod dropping;
mod pickup;

pub use dropping::OnCrateDroppedOnShip;

// Numbers that are the range cart boxes can fall into
pub const CART_MIN_Y: f32 = -2.5 * GRID_SIZE + 24.0; // magic numbers
pub const CART_MAX_Y: f32 = CART_MIN_Y + 28.;
const CART_SPRITE_HALF_WIDTH: f32 = 80.0;

pub const SHIP_MIN_Y: f32 = -7.5 * GRID_SIZE;
pub const SHIP_MAX_Y: f32 = SHIP_MIN_Y + 2.0 * GRID_SIZE;

pub struct ActionPlugin;

impl Plugin for ActionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DraggingBox {
            cart_entity: None,
            box_type: None,
            is_front_slot: true,
        })
        .add_event::<OnStartDragging>()
        .add_event::<OnDropCrate>()
        .add_event::<OnDropCrate>()
        .add_event::<OnCrateDroppedOnShip>()
        .add_system(pickup::click_to_pickup.run_in_state(GameState::Playing))
        .add_system(dragging::start_dragging.run_in_state(GameState::Playing))
        .add_system(dragging::mouse_follower.run_in_state(GameState::Playing))
        .add_system(dropping::handle_drop.run_in_state(GameState::Playing))
        .add_system(dropping::handle_drop_side_effects.run_in_state(GameState::Playing));
    }
}
