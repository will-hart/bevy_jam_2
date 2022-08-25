use crate::{
    game::components::{BoxType, FollowMouse, PhysicsCrate},
    GRID_SIZE,
};
use bevy::prelude::*;
use heron::{CollisionLayers, CollisionShape, Collisions, PhysicMaterial, RigidBody, Velocity};

use super::GamePhysicsLayer;

pub fn spawn_physics_crate(
    commands: &mut Commands,
    sprite_entity: Entity,
    box_type: BoxType,
    initial_velocity: Vec2,
) {
    commands
        .entity(sprite_entity)
        .remove::<FollowMouse>()
        .insert(RigidBody::Dynamic)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(GRID_SIZE / 2.0, GRID_SIZE / 2.0, GRID_SIZE / 2.0),
            border_radius: Some(2.0),
        })
        .insert(PhysicsCrate { box_type })
        .insert(
            CollisionLayers::none()
                .with_group(GamePhysicsLayer::Crate)
                .with_mask(GamePhysicsLayer::Ship),
        )
        .insert(PhysicMaterial {
            friction: 1.0,
            density: 1.0,
            ..Default::default()
        })
        .insert(Velocity::from_linear(initial_velocity.extend(0.0)))
        .insert(Collisions::default());
}
