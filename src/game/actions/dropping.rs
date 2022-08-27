use bevy::prelude::*;

use crate::{
    game::{components::BoxType, spawners::spawn_physics_crate},
    input::MousePosition,
};

use super::dragging::DraggingBox;

pub const CRATE_DROP_VELOCITY_FACTOR: f32 = 1.5;

/// Event triggered when a crate is released
pub struct OnDropCrate;

/// Event triggered when a crate is dropped on a ship
pub struct OnDropCrateOnShip {
    pub ship_entity: Entity,
    pub box_type: BoxType,
    pub location: Vec3,

    /// Is the crate in the ship's demands
    pub was_demanded: bool,
}

pub fn handle_drop(
    mut commands: Commands,
    mouse_position: Res<MousePosition>,
    mut on_drop_events: EventReader<OnDropCrate>,
    mut dragging: ResMut<DraggingBox>,
) {
    for _ in on_drop_events.iter() {
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
