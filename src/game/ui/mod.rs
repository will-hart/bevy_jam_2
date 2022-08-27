mod countdown_timer;
mod factory;
mod game_ui_bar;
mod production_queue;
mod score;
pub use score::OnCoinsReceived;
// TODO: mod tutorial;
// pub use tutorial::CurrentTutorialLevel;

use bevy::prelude::*;
use iyes_loopless::prelude::{AppLooplessStateExt, IntoConditionalSystem};

use crate::GameState;

use self::{
    countdown_timer::CountDownTimerPlugin,
    score::{OnShipScore, Score},
    // tutorial::TutorialPlugin,
};

use super::SystemLabels;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Score(0.0))
            .add_event::<OnCoinsReceived>()
            .add_event::<OnShipScore>()
            // .add_plugin(TutorialPlugin)
            .add_plugin(CountDownTimerPlugin)
            .add_enter_system(GameState::Playing, game_ui_bar::spawn_game_ui)
            .add_enter_system(GameState::Playing, factory::spawn_factory_ui)
            .add_system(
                score::score_display
                    .run_in_state(GameState::Playing)
                    .label(SystemLabels::ScoreDisplay),
            )
            .add_system(production_queue::update_production_queue.run_in_state(GameState::Playing))
            .add_system(factory::update_factory_input_ui.run_in_state(GameState::Playing));
    }
}
