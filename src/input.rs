use bevy::prelude::*;
use iyes_loopless::prelude::IntoConditionalSystem;
use leafwing_input_manager::{
    prelude::{ActionState, InputManagerPlugin, InputMap},
    Actionlike, InputManagerBundle,
};

use crate::{camera::MainCamera, GameState};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MousePosition::default())
            .add_plugin(InputManagerPlugin::<PlayerActions>::default())
            .add_startup_system(init_input_manager)
            .add_system(update_mouse_position.run_in_state(GameState::Playing));
    }
}

#[derive(Actionlike, Component, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum PlayerActions {
    Click,
}

/// Initialises the input manager, linking commands
fn init_input_manager(mut commands: Commands) {
    let input_map = InputMap::default();

    // input_map.insert(MouseButton::Left, PlayerActions::Confirm);
    commands
        .spawn()
        .insert_bundle(InputManagerBundle::<PlayerActions> {
            input_map,
            action_state: ActionState::default(),
        });
}

/// A resource to store the current position of the mouse in both screen and world coordinates
/// Only updated in the Playing game state
#[derive(Default, Debug)]
pub struct MousePosition {
    /// The current location of the mouse in screen coordinates
    pub screen: Vec2,

    /// The current location of the mouse in world coordinates
    pub world: Vec2,

    /// A flag which is true if the mouse is currently within the window, false otherwise
    pub in_screen: bool,
}

/// Keeps the mouse position up to date, and flags whether the mouse is in the window or not
fn update_mouse_position(
    windows: Res<Windows>,
    mut mouse_position: ResMut<MousePosition>,
    cameras: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let window = match windows.get_primary() {
        Some(w) => w,
        None => {
            return;
        }
    };

    if let Some(position) = window.cursor_position() {
        mouse_position.screen = position.clone();
        mouse_position.in_screen = true;

        // now convert screen to world coords for the camera
        let (camera, camera_transform) = cameras.single();
        let win_size = Vec2::new(window.width() as f32, window.height() as f32);
        let ndc = (position / win_size) * 2.0 - Vec2::ONE;
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        mouse_position.world = world_pos.truncate();
    } else {
        mouse_position.in_screen = false;
    }
}
