mod market_movement;

use bevy::{prelude::*, utils::HashMap};
use iyes_loopless::prelude::IntoConditionalSystem;
use rand::{rngs::ThreadRng, thread_rng, Rng};

use crate::GameState;

use super::components::BoxType;

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
            current_price: rng.gen_range((0.4 * starting_price)..(1.4 * starting_price)),
            price_acceleration: rng.gen_range(-1.0..1.0),
            volatility: rng.gen_range(0.01..0.10),
            starting_price,
        }
    }
}

impl MarketPrice {
    fn update(&mut self, rng: &mut ThreadRng) {
        self.current_price = (self.current_price + self.price_acceleration)
            .clamp(0.4 * self.starting_price, 1.4 * self.starting_price);
        self.price_acceleration = (self.price_acceleration
            + rng.gen_range(-self.volatility..self.volatility))
        .clamp(-1.0, 1.0);
    }
}

#[derive(Debug)]
pub struct Market {
    pub market: HashMap<BoxType, MarketPrice>,
}

impl Default for Market {
    fn default() -> Self {
        let mut market = Self {
            market: HashMap::from([
                (BoxType::MedicalSupplies, 85.0.into()),
                (BoxType::Fruit, 35.0.into()),
                (BoxType::Iron, 72.0.into()),
                (BoxType::Rum, 15.0.into()),
            ]),
        };

        // Ensure that our tutorial sale doesn't result in a loss
        // (unless a player is too slow I guess)
        let mut starting_item = market.market.get_mut(&BoxType::Fruit).unwrap();
        starting_item.current_price = starting_item.starting_price;
        starting_item.price_acceleration = 0.0;

        market
    }
}
