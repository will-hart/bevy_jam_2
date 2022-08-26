use std::ops::Range;

use rand::{rngs::ThreadRng, Rng};

pub struct RandomSpawnTimer {
    pub spawn_range: Range<f64>,
    next_spawn: f64,
}

impl RandomSpawnTimer {
    pub fn tick(&mut self, rng: &mut ThreadRng, current_time: f64) -> bool {
        if self.next_spawn < current_time {
            self.next_spawn = current_time + rng.gen_range(self.spawn_range.clone());
            true
        } else {
            false
        }
    }
}

impl Default for RandomSpawnTimer {
    fn default() -> Self {
        Self {
            spawn_range: 15.0..20.0,
            next_spawn: 5.0,
        }
    }
}
