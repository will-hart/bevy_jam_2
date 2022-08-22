use bevy::{prelude::*, utils::HashMap};
use rand::{thread_rng, Rng};

use super::components::{BoxType, ShipDestination};

#[derive(Debug)]
pub struct MarketPrice {
    pub current_price: f32,
    pub price_acceleration: f32,
    pub volatility: f32,
}

impl From<f32> for MarketPrice {
    fn from(current_price: f32) -> Self {
        let mut rng = thread_rng();

        MarketPrice {
            current_price,
            price_acceleration: rng.gen_range(-1.0..1.0),
            volatility: rng.gen_range(0.01..0.10),
        }
    }
}

#[derive(Debug)]
pub struct Market {
    pub market: HashMap<ShipDestination, HashMap<BoxType, MarketPrice>>,
}

impl Default for Market {
    fn default() -> Self {
        Self {
            market: HashMap::from([
                (
                    ShipDestination::Americas,
                    HashMap::from([
                        (BoxType::MedicalSupplies, 85.0.into()),
                        (BoxType::Fruit, 35.0.into()),
                        (BoxType::Iron, 72.0.into()),
                        (BoxType::Rum, 15.0.into()),
                    ]),
                ),
                (
                    ShipDestination::Carribean,
                    HashMap::from([
                        (BoxType::MedicalSupplies, 85.0.into()),
                        (BoxType::Fruit, 35.0.into()),
                        (BoxType::Iron, 72.0.into()),
                        (BoxType::Rum, 15.0.into()),
                    ]),
                ),
                (
                    ShipDestination::China,
                    HashMap::from([
                        (BoxType::MedicalSupplies, 85.0.into()),
                        (BoxType::Fruit, 35.0.into()),
                        (BoxType::Iron, 72.0.into()),
                        (BoxType::Rum, 15.0.into()),
                    ]),
                ),
            ]),
        }
    }
}

pub struct MarketPlugin;

impl Plugin for MarketPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Market>();
    }
}
