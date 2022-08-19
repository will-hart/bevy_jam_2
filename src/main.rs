use audio::InternalAudioPlugin;
use bevy::prelude::*;
use camera::CameraPlugin;
use input::InputPlugin;
use iyes_loopless::prelude::AppLooplessStateExt;
use loader::LoadingPlugin;

mod audio;
mod camera;
mod input;
mod loader;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum GameState {
    Loading,
    // Menu, // TODO: when we have a menu later
    Playing,
}

fn main() {
    App::new()
        .add_loopless_state(GameState::Loading)
        .insert_resource(Msaa { samples: 1 })
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.3)))
        .add_plugins(DefaultPlugins)
        .add_plugin(CameraPlugin)
        .add_plugin(LoadingPlugin)
        .add_plugin(InputPlugin)
        .add_plugin(InternalAudioPlugin)
        .run();
}
