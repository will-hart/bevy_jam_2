use bevy::prelude::*;

#[derive(Component)]
pub struct MainCamera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    info!("Spawning camera");
    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert(MainCamera);
}
