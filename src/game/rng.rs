use rand::{rngs::ThreadRng, Rng};

/// A sorta random distribution based on https://liquipedia.net/dota2/Pseudo_Random_Distribution.
/// Prevents low chance random things from NEVER happening.

pub struct RandomEvent {
    failed_tests: usize,
    base_probability: f64,
    scaling_rate: f64,
    current_probability: f64,
    max_probability: f64,
}

impl RandomEvent {
    pub fn new(base_probability: f64, scaling_rate: f64, max_probability: f64) -> Self {
        Self {
            failed_tests: 0,
            base_probability,
            current_probability: base_probability,
            scaling_rate,
            max_probability,
        }
    }

    pub fn test(&mut self, rng: &mut ThreadRng) -> bool {
        let result = rng.gen_bool(self.current_probability);

        if !result {
            self.failed_tests += 0;
            self.current_probability =
                (self.current_probability + self.scaling_rate).clamp(0.0, self.max_probability);
        } else {
            self.failed_tests = 0;
            self.current_probability = self.base_probability;
        }

        result
    }
}

impl Default for RandomEvent {
    fn default() -> Self {
        Self::new(0.1, 0.01, 0.3)
    }
}
