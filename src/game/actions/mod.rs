use bevy::prelude::*;
use iyes_loopless::prelude::IntoConditionalSystem;

use crate::{GameState, GRID_SIZE};

use self::{
    dragging::{DraggingBox, OnStartDragging},
    dropping::OnDropCrate,
};

mod detect_crate_drop_on_target;
pub use detect_crate_drop_on_target::OnCrateSplashedInWater;
mod dragging;
mod dropping;
pub use dropping::OnDropCrateOnShip;
mod pickup;

// Numbers that are the range cart boxes can fall into
pub const CART_MIN_Y: f32 = -2.5 * GRID_SIZE + 24.0; // magic numbers
pub const CART_MAX_Y: f32 = CART_MIN_Y + 28.;
const CART_SPRITE_HALF_WIDTH: f32 = 80.0;

pub struct ActionPlugin;

impl Plugin for ActionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DraggingBox {
            box_entity: None,
            box_type: None,
            is_front_slot: true,
        })
        .add_event::<OnStartDragging>()
        .add_event::<OnDropCrate>()
        .add_event::<OnDropCrateOnShip>()
        .add_event::<OnCrateSplashedInWater>()
        .add_system(pickup::click_to_pickup.run_in_state(GameState::Playing))
        .add_system(dragging::start_dragging.run_in_state(GameState::Playing))
        .add_system(dragging::mouse_follower.run_in_state(GameState::Playing))
        .add_system(dropping::handle_drop.run_in_state(GameState::Playing))
        .add_system(
            detect_crate_drop_on_target::detect_physics_crate_out_of_bounds
                .run_in_state(GameState::Playing),
        )
        .add_system(
            detect_crate_drop_on_target::detect_crate_drop_on_ship.run_in_state(GameState::Playing),
        );
    }
}
