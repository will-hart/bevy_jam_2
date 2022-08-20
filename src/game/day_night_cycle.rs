use bevy::prelude::*;

use crate::loader::AnimationAssets;

use super::{components::Torch, Animation};

const TORCH_THRESHOLD: f32 = 1.0;

// The amount of world time that elapses per game second
const TIME_OF_DAY_HOURS_PER_GAME_SECONDS: f32 = 0.35;

const NUM_COLOURS: usize = 8;
const HOURS_PER_COLOUR: f32 = 24.0 / (NUM_COLOURS as f32);

// Note for smooth lerping, these palettes should start and end on the same colour as each other
const SUNNY_COLOR_CYCLE: [Vec3; NUM_COLOURS] = [
    /*  0am */ Vec3::new(0.1, 0.1, 0.2),
    /*  3am */ Vec3::new(0.15, 0.15, 0.25),
    /*  6am */ Vec3::new(0.3, 0.2, 0.4),
    /*  9am */ Vec3::new(0.4, 0.5, 0.7),
    /* 12pm */ Vec3::new(0.4, 0.8, 0.9),
    /*  3pm */ Vec3::new(0.3, 0.7, 0.8),
    /*  6pm */ Vec3::new(0.4, 0.3, 0.4),
    /*  9pm */ Vec3::new(0.18, 0.18, 0.25),
];

const STORMY_COLOR_CYCLE: [Vec3; NUM_COLOURS] = [
    /*  0am */ Vec3::new(0.1, 0.1, 0.2),
    /*  3am */ Vec3::new(0.15, 0.15, 0.25),
    /*  6am */ Vec3::new(0.3, 0.25, 0.35),
    /*  9am */ Vec3::new(0.5, 0.5, 0.55),
    /* 12pm */ Vec3::new(0.5, 0.5, 0.55),
    /*  3pm */ Vec3::new(0.45, 0.45, 0.5),
    /*  6pm */ Vec3::new(0.3, 0.3, 0.35),
    /*  9pm */ Vec3::new(0.18, 0.18, 0.25),
];

pub struct SkyColourCycles {
    pub sunny: [Vec3; NUM_COLOURS],
    pub stormy: [Vec3; NUM_COLOURS],
    pub is_sunny: bool,
}

impl Default for SkyColourCycles {
    fn default() -> Self {
        Self {
            sunny: SUNNY_COLOR_CYCLE,
            stormy: STORMY_COLOR_CYCLE,
            is_sunny: true,
        }
    }
}

pub fn day_night_cycle(
    time: Res<Time>,
    mut cycle: ResMut<SkyColourCycles>,
    mut clear_colour: ResMut<ClearColor>,
    mut time_of_day: Local<f32>,
) {
    let dt = time.delta_seconds();
    let elapsed = dt * TIME_OF_DAY_HOURS_PER_GAME_SECONDS;

    let is_end_of_day = *time_of_day > 23.0;
    *time_of_day = (*time_of_day + elapsed) % 24.0;
    if is_end_of_day && *time_of_day < 1.0 {
        // true if we've just wrapped day, we need to toggle the colour pattern
        cycle.is_sunny = !cycle.is_sunny;
    }

    let from_idx = (*time_of_day / HOURS_PER_COLOUR).floor() as usize;
    let to_idx = (from_idx + 1) % NUM_COLOURS;

    let cycle_data = if cycle.is_sunny {
        cycle.sunny
    } else {
        cycle.stormy
    };

    let colour = cycle_data[from_idx].lerp(
        cycle_data[to_idx],
        (*time_of_day % HOURS_PER_COLOUR) / HOURS_PER_COLOUR,
    );

    clear_colour.0 = Color::from(colour.extend(1.0));
}

pub fn torch_visibility(
    clear_colour: Res<ClearColor>,
    animations: Res<AnimationAssets>,
    mut torches: Query<&mut Handle<Animation>, With<Torch>>,
) {
    let colour_darkness = clear_colour.0.r() + clear_colour.0.g() + clear_colour.0.b();
    let anim = if colour_darkness < TORCH_THRESHOLD {
        animations.torch.clone()
    } else {
        animations.torch_off.clone()
    };

    for mut torch in torches.iter_mut() {
        *torch = anim.clone();
    }
}
