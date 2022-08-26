mod events;
mod production;
pub mod recipes;
pub use events::OnDropInFactoryInput;

use bevy::prelude::*;
use iyes_loopless::prelude::IntoConditionalSystem;

use crate::GameState;

use self::recipes::Recipes;

pub struct FactoryPlugin;

impl Plugin for FactoryPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Recipes::default())
            .insert_resource(production::Factory::default())
            .add_event::<events::OnDropInFactoryInput>()
            .add_system(events::handle_drop_factory_input.run_in_state(GameState::Playing))
            .add_system(production::add_item_to_factory.run_in_state(GameState::Playing));
    }
}
