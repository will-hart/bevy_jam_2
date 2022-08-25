use std::fmt::Display;

use bevy::prelude::*;
use rand::{rngs::ThreadRng, Rng};

use crate::{loader::TextureAssets, GRID_SIZE};

#[derive(Component)]
pub struct Torch;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShipDestination {
    NewWorld,
    Pirates,
    Eastern,
}

impl Display for ShipDestination {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            ShipDestination::NewWorld => "New World",
            ShipDestination::Pirates => "Pirate Kingdoms",
            ShipDestination::Eastern => "Eastern Realms",
        })
    }
}

/// used for randomg selection of destinations, see notes on [BOX_TYPES] below.
/// Keep up to date with ShipDestination above.
pub const DESTINATIONS: [ShipDestination; 3] = [
    ShipDestination::NewWorld,
    ShipDestination::Pirates,
    ShipDestination::Eastern,
];

#[derive(Component, Clone, Copy, Debug)]
pub struct Ship {
    pub y_offset: f32,
    pub phase: f32,
}

impl Ship {
    pub fn new(rng: &mut ThreadRng) -> Self {
        Self {
            y_offset: 4.0 * GRID_SIZE,
            phase: rng.gen_range(-3.1..3.1),
        }
    }
}

#[derive(Component)]
pub struct TopUiBar;

#[derive(Component, Clone, Debug)]
pub struct ShipHold {
    pub destination: ShipDestination,
    pub crates: Vec<BoxType>,
    pub demands: Vec<BoxType>,
}

impl ShipHold {
    pub fn accept_crate(&mut self, crate_type: BoxType) {
        // TODO: update demands
        self.crates.push(crate_type);
    }
}

#[derive(Component)]
pub struct ShipDemandItemMarker(pub BoxType);

#[derive(Component)]
pub struct MarketPriceIndicator(pub BoxType);

#[derive(Component)]
pub struct MarketPriceDirectionIndicator;

#[derive(Component)]
pub struct MarketPriceValueIndicator;

#[derive(Component)]
pub struct AnimateWithSpeed {
    pub speed: f32,
    pub target: Vec<Vec3>,
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

#[derive(Clone, Component, Debug)]
pub struct SpawnShipRequest {
    pub destination: Option<ShipDestination>,
    pub demands: Vec<BoxType>,
    pub expiry: f32,
}

#[derive(Component)]
pub struct TutorialMarker(pub u8);

#[derive(Component)]
pub struct CountDownTimer(pub Timer);
