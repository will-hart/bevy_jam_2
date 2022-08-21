use std::fmt::Display;

use bevy::prelude::*;

#[derive(Component)]
pub struct Torch;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShipDestination {
    Americas,
    Carribean,
    China,
}

impl Display for ShipDestination {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            ShipDestination::Americas => "Americas",
            ShipDestination::Carribean => "Carribean",
            ShipDestination::China => "China",
        })
    }
}

/// used for randomg selection of destinations, see notes on [BOX_TYPES] below.
/// Keep up to date with ShipDestination above.
pub const DESTINATIONS: [ShipDestination; 3] = [
    ShipDestination::Americas,
    ShipDestination::Carribean,
    ShipDestination::China,
];

#[derive(Component, Debug)]
pub struct Ship {
    pub y_offset: f32,
    pub phase: f32,
}

#[derive(Component, Clone, Debug)]
pub struct ShipHold {
    pub destination: ShipDestination,
    pub crates: Vec<BoxType>,

    pub weight_capacity: u32,
    pub current_weight: u32,

    pub volume_capacity: u32,
    pub current_volume: u32,
}

impl ShipHold {
    pub fn accept_crate(&mut self, crate_type: BoxType) {
        self.current_volume += crate_type.get_volume();
        self.current_weight += crate_type.get_weight();

        self.crates.push(crate_type);
    }
}

#[derive(Component)]
pub struct AnimateWithSpeed {
    pub speed: f32,
    pub target: Vec2,
}

#[derive(Component)]
pub struct Sun;

#[derive(Component)]
pub struct Star;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BoxType {
    MedicalSupplies = 0,
    Fruit = 1,
    Iron = 2,
    Rum = 3,
}

// Because we can't loop or rand::choose over all enum values, we need
// to define a list of possible enum values here for randomly spawning
// crates. Make sure to keep this up to date with the enum above.
pub const BOX_TYPES: [BoxType; 4] = [
    BoxType::MedicalSupplies,
    BoxType::Fruit,
    BoxType::Iron,
    BoxType::Rum,
];

impl BoxType {
    pub fn get_weight(&self) -> u32 {
        match self {
            BoxType::MedicalSupplies => 1,
            BoxType::Fruit => 2,
            BoxType::Iron => 4,
            BoxType::Rum => 2,
        }
    }

    pub fn get_volume(&self) -> u32 {
        match self {
            BoxType::MedicalSupplies => 1,
            BoxType::Fruit => 1,
            BoxType::Iron => 1,
            BoxType::Rum => 1,
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

#[derive(Component)]
pub struct ScoreUi;

#[derive(Component)]
pub struct ShipLaunchButton(pub usize);

#[derive(Component)]
pub struct ShipText;
