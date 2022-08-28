mod button_interaction;
mod cart_request;
mod countdown_timer;
mod factory;
mod game_ui_bar;
mod menu;
mod production_queue;
mod score;
mod ship_demand;
pub mod tutorial;
pub use score::{OnCoinsReceived, Score};

use bevy::prelude::*;
use iyes_loopless::prelude::{AppLooplessStateExt, IntoConditionalSystem};

use crate::{game::ui::menu::MenuPlugin, GameState};

use self::{countdown_timer::CountDownTimerPlugin, score::OnShipScore, tutorial::TutorialPlugin};

use super::SystemLabels;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        info!("Mounting UI Plugin");

        app.insert_resource(Score(0.0))
            .add_event::<OnCoinsReceived>()
            .add_event::<OnShipScore>()
            .add_plugin(MenuPlugin)
            .add_plugin(TutorialPlugin)
            .add_plugin(CountDownTimerPlugin)
            .add_enter_system(GameState::Playing, game_ui_bar::spawn_game_ui)
            .add_enter_system(GameState::Playing, factory::spawn_factory_ui)
            .add_system(
                score::score_display
                    .run_in_state(GameState::Playing)
                    .label(SystemLabels::ScoreDisplay),
            )
            .add_system(button_interaction::button_interaction.run_not_in_state(GameState::Loading))
            .add_system(
                score::score_update
                    .run_in_state(GameState::Playing)
                    .before(SystemLabels::ScoreDisplay),
            )
            .add_system(
                score::despawn_ships_and_penalise
                    .run_in_state(GameState::Playing)
                    .before(SystemLabels::ScoreDisplay),
            )
            .add_system(score::update_current_date.run_in_state(GameState::Playing))
            .add_system(cart_request::update_cart_request_queue.run_in_state(GameState::Playing))
            .add_system(ship_demand::remove_ship_demands_when_met.run_in_state(GameState::Playing))
            .add_system(production_queue::update_production_queue.run_in_state(GameState::Playing))
            .add_system(factory::update_factory_input_ui.run_in_state(GameState::Playing));
    }
}
