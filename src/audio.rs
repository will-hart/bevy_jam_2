use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use iyes_loopless::{condition::IntoConditionalSystem, prelude::AppLooplessStateExt};

use crate::{game::actions::OnCrateDroppedOnShip, loader::AudioAssets, GameState};

#[derive(Component, Default, Clone)]
struct MusicChannel;
#[derive(Component, Default, Clone)]
struct EffectsChannel;

pub struct InternalAudioPlugin;

// This plugin is responsible to control the game audio
impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AudioPlugin)
            .add_audio_channel::<MusicChannel>()
            .add_audio_channel::<EffectsChannel>()
            .add_system(on_box_drop.run_not_in_state(GameState::Loading))
            .add_exit_system(GameState::Loading, play_music);
    }
}

fn play_music(music_channel: Res<AudioChannel<MusicChannel>>, audio_assets: Res<AudioAssets>) {
    music_channel
        .play(audio_assets.music.clone())
        .with_volume(0.6)
        .looped();
}

fn on_box_drop(
    mut events: EventReader<OnCrateDroppedOnShip>,
    effects_channel: Res<AudioChannel<EffectsChannel>>,
    audio_assets: Res<AudioAssets>,
) {
    let mut done = false;
    for _ in events.iter() {
        if done {
            continue;
        }

        done = true;
        info!("Playing dropped box sound");
        effects_channel.play(audio_assets.box_drop.clone());
    }
}
