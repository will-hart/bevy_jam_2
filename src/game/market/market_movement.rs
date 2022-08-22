use bevy::prelude::*;
use rand::thread_rng;

use super::Market;

const MARKET_UPDATE_FREQUENCY: f64 = 1.5;

pub fn market_movement(time: Res<Time>, mut market: ResMut<Market>, mut next_update: Local<f64>) {
    if time.seconds_since_startup() < *next_update {
        return;
    }

    *next_update += MARKET_UPDATE_FREQUENCY;

    let mut rng = thread_rng();

    for (_, region) in market.market.iter_mut() {
        for (_, price) in region.iter_mut() {
            price.update(&mut rng)
        }
    }
}
