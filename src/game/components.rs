use std::fmt::Display;

use bevy::prelude::*;

#[derive(Component)]
pub struct Torch;

#[derive(Component, Debug)]
pub struct Ship {
    pub y_offset: f32,
    pub phase: f32,
}

#[derive(Component, Clone, Debug)]
pub struct ShipHold {
    pub crates: Vec<BoxType>,

    pub weight_capacity: u32,
    pub current_weight: u32,

    pub volume_capacity: u32,
    pub current_volume: u32,
}

impl Display for ShipHold {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "W: {}/{} V: {}/{}",
            self.current_weight, self.weight_capacity, self.current_volume, self.volume_capacity
        ))
    }
}

impl ShipHold {
    pub fn accept_crate(&mut self, crate_type: BoxType) {
        self.current_volume += crate_type.get_volume();
        self.current_weight += crate_type.get_weight();

        self.crates.push(crate_type);
    }
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

impl BoxType {
    pub fn get_weight(&self) -> u32 {
        match self {
            BoxType::Cotton => 1,
            BoxType::Apples => 2,
            BoxType::Bannanas => 2,
        }
    }

    pub fn get_volume(&self) -> u32 {
        match self {
            BoxType::Cotton => 2,
            BoxType::Apples => 1,
            BoxType::Bannanas => 1,
        }
    }
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

#[derive(Component)]
pub struct Wave;
