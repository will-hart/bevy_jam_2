use bevy::prelude::*;

#[derive(Component)]
pub struct Torch;

#[derive(Component, Debug)]
pub struct Ship {
    pub y: f32,
    pub phase: f32,
    pub crates: Vec<BoxType>,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoxType {
    Cotton = 0,
    Apples = 1,
    Bannanas = 2,
}

#[derive(Component)]
pub struct Cart {
    pub front: Option<BoxType>,
    pub back: Option<BoxType>,
}

#[derive(Component)]
pub struct FollowMouse;

#[derive(Component)]
pub struct CartCrate {
    pub is_front_slot: bool,
}
