mod market_movement;

use bevy::{prelude::*, utils::HashMap};
use iyes_loopless::prelude::IntoConditionalSystem;
use rand::{rngs::ThreadRng, thread_rng, Rng};

use crate::GameState;

use super::components::{BoxType, ShipDestination};

pub struct MarketPlugin;

impl Plugin for MarketPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Market>()
            .add_system(market_movement::market_movement.run_in_state(GameState::Playing));
    }
}

#[derive(Debug)]
pub struct MarketPrice {
    pub current_price: f32,
    pub price_acceleration: f32,
    volatility: f32,
    starting_price: f32,
}

impl From<f32> for MarketPrice {
    fn from(starting_price: f32) -> Self {
        let mut rng = thread_rng();

        MarketPrice {
            current_price: rng.gen_range(0.9 * starting_price..1.1 * starting_price),
            price_acceleration: rng.gen_range(-1.0..1.0),
            volatility: rng.gen_range(0.01..0.10),
            starting_price,
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
impl MarketPrice {
    fn update(&mut self, rng: &mut ThreadRng) {
        self.current_price = (self.current_price + self.price_acceleration)
            .clamp(0.3 * self.starting_price, 2.0 * self.starting_price);
        self.price_acceleration = (self.price_acceleration
            + rng.gen_range(-self.volatility..self.volatility))
        .clamp(-1.0, 1.0);
    }
}
