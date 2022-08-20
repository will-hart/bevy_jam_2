use bevy::prelude::*;

#[derive(Component)]
pub struct Torch;

#[derive(Component)]
pub struct Ship {
    pub y: f32,
    pub phase: f32,
}

#[derive(Component)]
pub struct AnimateX {
    pub speed: f32,
    pub looped: bool,
}

#[derive(Component)]
pub struct Sun;

#[derive(Component)]
pub struct Star;
