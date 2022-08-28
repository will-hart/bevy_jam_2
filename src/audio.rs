use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use iyes_loopless::{
    condition::IntoConditionalSystem,
    prelude::{AppLooplessStateExt, ConditionHelpers},
};

use crate::{
    game::{
        actions::{OnCrateSplashedInWater, OnDropCrateOnShip},
        OnCoinsReceived, OnShipSpawned,
    },
    loader::AudioAssets,
    GameState,
};

#[derive(Component, Default, Clone)]
struct MusicChannel;
#[derive(Component, Default, Clone)]
struct EffectsChannel;
#[derive(Component, Default, Clone)]
struct RainChannel;

pub struct InternalAudioPlugin;

// This plugin is responsible to control the game audio
impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AudioPlugin)
            .add_audio_channel::<MusicChannel>()
            .add_audio_channel::<RainChannel>()
            .add_audio_channel::<EffectsChannel>()
            .add_system(
                on_coin_drop
                    .run_in_state(GameState::Playing)
                    .run_on_event::<OnCoinsReceived>(),
            )
            .add_system(
                on_box_drop
                    .run_in_state(GameState::Playing)
                    .run_on_event::<OnDropCrateOnShip>(),
            )
            .add_system(
                on_splash
                    .run_in_state(GameState::Playing)
                    .run_on_event::<OnCrateSplashedInWater>(),
            )
            .add_system(
                on_ship_spawn
                    .run_in_state(GameState::Playing)
                    .run_on_event::<OnShipSpawned>(),
            )
            .add_enter_system(GameState::Menu, play_wind)
            .add_enter_system(GameState::Playing, play_music)
            .add_exit_system(GameState::Playing, stop_all_music);
    }
}

fn play_music(music_channel: Res<AudioChannel<MusicChannel>>, audio_assets: Res<AudioAssets>) {
    music_channel
        .play(audio_assets.music.clone())
        .with_volume(0.15)
        .looped();
}

fn play_wind(music_channel: Res<AudioChannel<MusicChannel>>, audio_assets: Res<AudioAssets>) {
    music_channel
        .play(audio_assets.wind.clone())
        .with_volume(0.15)
        .looped();
}

fn stop_all_music(music_channel: Res<AudioChannel<MusicChannel>>) {
    music_channel.stop();
}

fn on_coin_drop(
    effects_channel: Res<AudioChannel<EffectsChannel>>,
    audio_assets: Res<AudioAssets>,
) {
    info!("Playing coin sound");
    effects_channel.play(audio_assets.coin_drop.clone());
}

fn on_box_drop(effects_channel: Res<AudioChannel<EffectsChannel>>, audio_assets: Res<AudioAssets>) {
    info!("Playing box sound");
    effects_channel.play(audio_assets.box_drop.clone());
}

fn on_splash(effects_channel: Res<AudioChannel<EffectsChannel>>, audio_assets: Res<AudioAssets>) {
    info!("Playing splash sound");
    effects_channel.play(audio_assets.splash.clone());
}

fn on_ship_spawn(
    effects_channel: Res<AudioChannel<EffectsChannel>>,
    audio_assets: Res<AudioAssets>,
) {
    info!("Playing ship spawn sound");
    effects_channel.play(audio_assets.ships_bell.clone());
}
