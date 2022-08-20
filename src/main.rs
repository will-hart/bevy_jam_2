use audio::InternalAudioPlugin;
use bevy::prelude::*;
use camera::CameraPlugin;
use game::GamePlugin;
use input::InputPlugin;
use iyes_loopless::prelude::AppLooplessStateExt;
use loader::LoadingPlugin;

mod audio;
mod camera;
mod game;
mod input;
mod loader;

pub const WIDTH: f32 = 1024.;
pub const HEIGHT: f32 = 768.;
pub const GRID_SIZE: f32 = 32.;

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
        .insert_resource(WindowDescriptor {
            width: 1024.,
            height: 768.,
            title: "Bevy Jam 2".to_string(),
            canvas: Some("#bevy".to_owned()),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(CameraPlugin)
        .add_plugin(LoadingPlugin)
        .add_plugin(InputPlugin)
        .add_plugin(InternalAudioPlugin)
        .add_plugin(GamePlugin)
        .run();
}
