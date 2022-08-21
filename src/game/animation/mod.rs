use bevy::{log, prelude::*};

mod animate_entity;
pub use animate_entity::DespawnEntity;
mod animation;
mod ship_bob;
mod waves;
pub use animation::{Animation, AnimationState};
use iyes_loopless::prelude::IntoConditionalSystem;

use crate::GameState;

pub struct AnimationPlugin;

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        log::info!("Mounting AnimationPlugin");
        app.add_asset::<Animation>()
            .add_event::<DespawnEntity>()
            .init_asset_loader::<animation::BenimationLoader>()
            .add_system(animation::update_animation_frames)
            .add_system(animate_entity::animate_entity.run_not_in_state(GameState::Loading))
            .add_system(ship_bob::ship_bob.run_not_in_state(GameState::Loading))
            .add_system(waves::scroll_wave_texture.run_not_in_state(GameState::Loading));
    }
}
