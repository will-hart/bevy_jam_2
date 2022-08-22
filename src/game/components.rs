use std::fmt::Display;

use bevy::prelude::*;
use rand::{rngs::ThreadRng, Rng};

use crate::{loader::TextureAssets, GRID_SIZE};

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

impl ShipDestination {
    /// The amount of time a ship is unavailable for when sailing
    /// counted from the time the ship despawns
    pub fn get_travel_duration(&self) -> f32 {
        match self {
            ShipDestination::Americas => 12.0,
            ShipDestination::Carribean => 11.0,
            ShipDestination::China => 9.0,
        }
    }
}

/// used for randomg selection of destinations, see notes on [BOX_TYPES] below.
/// Keep up to date with ShipDestination above.
pub const DESTINATIONS: [ShipDestination; 3] = [
    ShipDestination::Americas,
    ShipDestination::Carribean,
    ShipDestination::China,
];

#[derive(Component, Clone, Copy, Debug)]
pub struct Ship {
    pub y_offset: f32,
    pub phase: f32,
    pub next_available: f32,
    pub maintenance_cost_per_second: f32,
    pub purchase_cost: u32,
}

impl Ship {
    pub fn new(rng: &mut ThreadRng) -> Self {
        Self {
            y_offset: 4.0 * GRID_SIZE,
            phase: rng.gen_range(-3.1..3.1),
            next_available: 0.0,
            maintenance_cost_per_second: 1.0,
            purchase_cost: 350,
        }
    }
}

#[derive(Component)]
pub struct ShipRespawnTimer {
    pub ship_to_respawn: Ship,
    pub respawn_at: f32,
}

#[derive(Component)]
pub struct ShipRespawnBar;

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
pub struct MarketPriceIndicator(pub ShipDestination, pub BoxType);

#[derive(Component)]
pub struct MarketPriceDirectionIndicator;

#[derive(Component)]
pub struct MarketPriceValueIndicator;

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

    pub(crate) fn get_image(&self, textures: &TextureAssets) -> Handle<Image> {
        match self {
            BoxType::MedicalSupplies => textures.box_type_medical.clone(),
            BoxType::Fruit => textures.box_type_fruit.clone(),
            BoxType::Iron => textures.box_type_iron.clone(),
            BoxType::Rum => textures.box_type_rum.clone(),
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
