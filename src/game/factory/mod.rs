mod events;
mod production;
pub mod recipes;
mod utils;
pub use events::OnDropInFactoryInput;

use bevy::prelude::*;
use iyes_loopless::prelude::IntoConditionalSystem;

use crate::GameState;

use self::recipes::Recipes;

use super::SystemLabels;

pub struct FactoryPlugin;

impl Plugin for FactoryPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Recipes::default())
            .insert_resource(production::Factory::default())
            .add_event::<events::OnDropInFactoryInput>()
            .add_event::<events::OnFactoryStartProducing>()
            .add_event::<events::OnFactoryFinishProducing>()
            .add_system(events::handle_drop_factory_input.run_in_state(GameState::Playing))
            .add_system(
                production::add_item_to_factory
                    .run_in_state(GameState::Playing)
                    .before(SystemLabels::FactoryProduction),
            )
            .add_system(
                production::finish_factory_production
                    .run_in_state(GameState::Playing)
                    .label(SystemLabels::FactoryProduction),
            )
            .add_system(
                production::start_factory_production
                    .run_in_state(GameState::Playing)
                    .after(SystemLabels::FactoryProduction),
            )
            .add_system(
                production::handle_production_started
                    .run_in_state(GameState::Playing)
                    .after(SystemLabels::FactoryProduction),
            );
    }
}
