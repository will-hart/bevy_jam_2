use bevy::prelude::*;

use crate::{game::spawners::spawn_physics_crate, input::MousePosition};

use super::dragging::DraggingBox;

pub const CRATE_DROP_VELOCITY_FACTOR: f32 = 1.5;

pub struct OnDropCrate;

pub fn handle_drop(
    mut commands: Commands,
    mouse_position: Res<MousePosition>,
    mut on_drop_events: EventReader<OnDropCrate>,
    mut dragging: ResMut<DraggingBox>,
) {
    let mut done = false;
    for _ in on_drop_events.iter() {
        if done {
            warn!("Repeat drop event being ignored");
            continue;
        }
        done = true;

        // spawn a physics box and hope it lands on something
        spawn_physics_crate(
            &mut commands,
            dragging.box_entity.unwrap(),
            dragging.box_type.unwrap(),
            mouse_position.velocity * CRATE_DROP_VELOCITY_FACTOR,
        );

        // reset the dragging state
        dragging.box_type = None;
        dragging.box_entity = None;
    }
}
