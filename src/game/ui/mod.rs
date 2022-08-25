mod countdown_timer;
mod game_ui_bar;
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
            .add_enter_system(GameState::Playing, game_ui_bar::spawn_ship_respawn_bar)
            .add_system(
                score::score_display
                    .run_in_state(GameState::Playing)
                    .label(SystemLabels::ScoreDisplay),
            );
    }
}
