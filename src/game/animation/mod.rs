use bevy::{log, prelude::*};

mod animation;
pub use animation::{Animation, AnimationState};

pub struct AnimationPlugin;

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        log::info!("Mounting AnimationPlugin");
        app.add_asset::<Animation>()
            .init_asset_loader::<animation::BenimationLoader>()
            .add_system(animation::update_animation_frames);
    }
}
