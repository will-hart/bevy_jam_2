use bevy::prelude::*;
use heron::PhysicsLayer;
use iyes_loopless::prelude::IntoConditionalSystem;

use crate::GameState;

mod cart;
pub mod request;
mod ship;
mod torch;
pub use torch::spawn_torch;
mod physics_crate;
pub use physics_crate::spawn_physics_crate;

// Define your physics layers
#[derive(PhysicsLayer)]
pub enum GamePhysicsLayer {
    Ship,
    Crate,
    // Warehouse,
    // Factory,
}

pub struct SpawningPlugin;

impl Plugin for SpawningPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(cart::cart_spawning_system.run_in_state(GameState::Playing))
            .add_system(ship::ship_queuing_system.run_in_state(GameState::Playing))
            .add_system(ship::ship_spawn_on_timer_expiry.run_in_state(GameState::Playing));
    }
}
