mod events;
pub use events::OnDropInFactoryInput;

use bevy::prelude::*;
use iyes_loopless::prelude::IntoConditionalSystem;

use crate::GameState;

pub struct FactoryPlugin;

impl Plugin for FactoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<events::OnDropInFactoryInput>()
            .add_system(events::handle_drop_factory_input.run_in_state(GameState::Playing));
    }
}
