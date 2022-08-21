mod launch_ships;
mod score;
mod ship_ui;

use bevy::prelude::*;
use iyes_loopless::prelude::{AppLooplessStateExt, IntoConditionalSystem};

use crate::GameState;

use self::score::Score;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Score(10000))
            .add_system(ship_ui::update_ship_hold_data.run_not_in_state(GameState::Loading))
            .add_enter_system(GameState::Playing, score::spawn_score_ui)
            .add_system(
                score::score_display
                    .run_in_state(GameState::Playing)
                    .label("score_display"),
            )
            .add_system(
                score::score_update
                    .run_in_state(GameState::Playing)
                    .before("score_display"),
            )
            .add_enter_system(GameState::Playing, launch_ships::spawn_ship_buttons)
            .add_system(launch_ships::button_interaction.run_in_state(GameState::Playing));
    }
}
