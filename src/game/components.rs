use bevy::prelude::*;
use rand::{rngs::ThreadRng, Rng};

use crate::{loader::TextureAssets, GRID_SIZE};

#[derive(Component)]
pub struct Torch;

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

#[derive(Component)]
pub struct ProductionQueueUi;

#[derive(Component)]
pub struct ProductionQueueUiItem(pub BoxType);

#[derive(Component, Clone, Debug)]
pub struct ShipHold {
    pub crates: Vec<BoxType>,
    pub demands: Vec<BoxType>,
}

impl ShipHold {
    pub fn get_unmet_demands(&self) -> Vec<BoxType> {
        let mut unmet_demands = self.demands.clone();
        for filled_crate in self.crates.iter() {
            // remove all crates that have already been filled
            if let Some(idx) = unmet_demands.iter().position(|item| *item == *filled_crate) {
                unmet_demands.remove(idx);
            }
        }

        unmet_demands
    }
}

#[derive(Component)]
pub struct ShipDemandItemMarker(pub BoxType);

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
    // inputs
    Glassware = 0,
    Grapes = 1,
    Apples = 2,
    Wheat = 3,
    Honey = 4,

    // outputs
    Beer = 5,
    Cider = 6,
    Mead = 7,
    Wine = 8,
}

// Because we can't loop or rand::choose over all enum values, we need
// to define a list of possible enum values here for randomly demanding
// crates. Make sure to keep this up to date with the enum above.
//
// To change the spawn likelihood, just add an item more often :laugh
pub const BOX_DEMANDS: [BoxType; 16] = [
    BoxType::Apples,
    BoxType::Grapes,
    BoxType::Honey,
    BoxType::Wheat,
    BoxType::Cider,
    BoxType::Cider,
    BoxType::Cider,
    BoxType::Wine,
    BoxType::Wine,
    BoxType::Wine,
    BoxType::Mead,
    BoxType::Mead,
    BoxType::Mead,
    BoxType::Beer,
    BoxType::Beer,
    BoxType::Beer,
];

impl BoxType {
    pub(crate) fn get_image(&self, textures: &TextureAssets) -> Handle<Image> {
        match self {
            BoxType::Glassware => textures.box_type_glassware.clone(),
            BoxType::Apples => textures.box_type_apples.clone(),
            BoxType::Grapes => textures.box_type_grapes.clone(),
            BoxType::Honey => textures.box_type_honey.clone(),
            BoxType::Wheat => textures.box_type_wheat.clone(),
            BoxType::Cider => textures.box_type_cider.clone(),
            BoxType::Wine => textures.box_type_wine2.clone(),
            BoxType::Mead => textures.box_type_mead.clone(),
            BoxType::Beer => textures.box_type_beer.clone(),
        }
    }
}

#[derive(Component)]
pub struct PhysicsCrate {
    pub box_type: BoxType,
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
    pub demands: Vec<BoxType>,
    pub expiry: f32,
}

#[derive(Component)]
pub struct TutorialMarker(pub u8);

#[derive(Component, Default, Debug)]
pub struct CountDownTimer(pub Timer);

#[derive(Component)]
pub struct FactoryInput;

#[derive(Component)]
pub struct FactoryGraphic;

#[derive(Component)]
pub struct FactoryProductionIndicator;

#[derive(Component)]
pub struct FactoryInputsDisplayItem(pub usize);

#[derive(Component)]
pub struct CountdownTimerSprite;

#[derive(Component)]
pub struct SplashCatcher;

/// A visual effect with a despawn time
#[derive(Component)]
pub struct VisualEffect(pub f32);

#[derive(Component)]
pub struct CartQueueUi;

#[derive(Component)]
pub struct CartQueueUiItem;

#[derive(Component)]
pub struct CartQueueUiButton(pub BoxType);

#[derive(Component)]
pub struct WorldEntity;

#[derive(Component)]
pub struct MenuItem;

#[derive(Component)]
pub struct CurrentDateMarker;
