mod launch_ships;

mod market;
mod request_ship;
mod score;
mod ship_respawn_bar;
pub use ship_respawn_bar::spawn_ship_request_button;
mod tutorial;
pub use tutorial::CurrentTutorialLevel;

use bevy::prelude::*;
use iyes_loopless::prelude::{AppLooplessStateExt, IntoConditionalSystem};

use crate::GameState;

use self::{request_ship::OnRequestShipSpawn, score::Score, tutorial::TutorialPlugin};

use super::SystemLabels;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Score(0.0))
            .add_event::<OnRequestShipSpawn>()
            .add_plugin(TutorialPlugin)
            .add_enter_system(GameState::Playing, ship_respawn_bar::spawn_ship_respawn_bar)
            .add_system(
                score::score_display
                    .run_in_state(GameState::Playing)
                    .label(SystemLabels::ScoreDisplay),
            )
            .add_system(
                score::score_update
                    .run_in_state(GameState::Playing)
                    .before(SystemLabels::ScoreDisplay)
                    .after(SystemLabels::ShipAnimationAndDespawn),
            )
            .add_system(launch_ships::button_visibility.run_in_state(GameState::Playing))
            .add_system(launch_ships::button_interaction.run_in_state(GameState::Playing))
            .add_system(market::update_market_price_ui.run_in_state(GameState::Playing))
            .add_system(request_ship::ship_spawn_handler.run_in_state(GameState::Playing));
    }
}
