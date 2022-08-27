use std::time::Duration;

use bevy::time::Timer;

use crate::game::components::CountDownTimer;

/// Generates a new factory production timer
pub fn new_timer() -> CountDownTimer {
    CountDownTimer(Timer::new(Duration::from_secs_f32(5.0), false))
}
