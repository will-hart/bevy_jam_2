use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use iyes_loopless::condition::IntoConditionalSystem;

use crate::{game::actions::OnCrateDroppedOnShip, loader::AudioAssets, GameState};

pub struct InternalAudioPlugin;

// This plugin is responsible to control the game audio
impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AudioPlugin)
            .add_system(on_box_drop.run_not_in_state(GameState::Loading));
    }
}

fn on_box_drop(
    mut events: EventReader<OnCrateDroppedOnShip>,
    audio: Res<bevy_kira_audio::prelude::Audio>,
    audio_assets: Res<AudioAssets>,
) {
    let mut done = false;
    for _ in events.iter() {
        if done {
            continue;
        }

        done = true;
        info!("Playing dropped box sound");
        audio.play(audio_assets.box_drop.clone());
    }
}
