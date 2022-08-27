use bevy::prelude::*;
use iyes_loopless::prelude::{AppLooplessStateExt, IntoConditionalSystem};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

use crate::{
    loader::{AnimationAssets, TextureAssets},
    GameState, HEIGHT, WIDTH,
};

use super::{
    components::{Star, Sun, Torch, WorldEntity},
    Animation,
};

const RANDOM_SEED: u64 = 349678046248609346;

const TORCH_THRESHOLD: f32 = 1.0;

// The amount of world time that elapses per game second
const TIME_OF_DAY_HOURS_PER_GAME_SECONDS: f32 = 0.35;

const NUM_COLOURS: usize = 8;
const HOURS_PER_COLOUR: f32 = 24.0 / (NUM_COLOURS as f32);

const SUN_UP: f32 = 6.0;
const SUN_DOWN: f32 = 19.0;

const STAR_SPEED: f32 = -2.3;

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

pub struct DayNightCyclePlugin;

impl Plugin for DayNightCyclePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SkyColourCycles::default())
            .insert_resource(TimeOfDay { time_of_day: 5.8 })
            .add_event::<OnSunEvent>()
            .add_system(day_night_cycle.run_in_state(GameState::Playing))
            .add_system(torch_visibility.run_in_state(GameState::Playing))
            .add_system(star_and_sun_spawner.run_in_state(GameState::Playing))
            .add_system(sun_movement.run_in_state(GameState::Playing))
            .add_system(star_movement.run_in_state(GameState::Playing))
            .add_enter_system(GameState::Playing, reset_day_night_cycle);
    }
}

struct OnSunEvent(pub bool);

struct SkyColourCycles {
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

struct TimeOfDay {
    pub time_of_day: f32,
}

fn reset_day_night_cycle(mut cycle: ResMut<TimeOfDay>) {
    cycle.time_of_day = 5.8;
}

fn day_night_cycle(
    time: Res<Time>,
    mut cycle: ResMut<SkyColourCycles>,
    mut clear_colour: ResMut<ClearColor>,
    mut time_of_day: ResMut<TimeOfDay>,
    mut sun_events: EventWriter<OnSunEvent>,
) {
    let dt = time.delta_seconds();
    let elapsed = dt * TIME_OF_DAY_HOURS_PER_GAME_SECONDS;

    let prev_time_of_day = time_of_day.time_of_day;
    time_of_day.time_of_day = (time_of_day.time_of_day + elapsed) % 24.0;

    // check if we've wrapped over midnight
    if prev_time_of_day > 23.0 && time_of_day.time_of_day < 1.0 {
        // true if we've just wrapped day, we need to toggle the colour pattern
        cycle.is_sunny = !cycle.is_sunny;
    }

    if prev_time_of_day < 18.0 && time_of_day.time_of_day >= 18.0 {
        sun_events.send(OnSunEvent(false));
    }

    if prev_time_of_day < 6.0 && time_of_day.time_of_day >= 6.0 {
        sun_events.send(OnSunEvent(true));
    }

    let from_idx = (time_of_day.time_of_day / HOURS_PER_COLOUR).floor() as usize;
    let to_idx = (from_idx + 1) % NUM_COLOURS;

    let cycle_data = if cycle.is_sunny {
        cycle.sunny
    } else {
        cycle.stormy
    };

    let colour = cycle_data[from_idx].lerp(
        cycle_data[to_idx],
        (time_of_day.time_of_day % HOURS_PER_COLOUR) / HOURS_PER_COLOUR,
    );

    clear_colour.0 = Color::from(colour.extend(1.0));
}

fn torch_visibility(
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

fn star_and_sun_spawner(
    mut commands: Commands,
    mut sun_events: EventReader<OnSunEvent>,
    textures: Res<TextureAssets>,
    cycle: Res<SkyColourCycles>,
    suns: Query<Entity, With<Sun>>,
    stars: Query<Entity, With<Star>>,
) {
    for event in sun_events.iter() {
        match event.0 {
            true => {
                // despawn stars out of hours
                for star in stars.iter() {
                    commands.entity(star).despawn();
                }

                if cycle.is_sunny {
                    commands
                        .spawn_bundle(SpriteSheetBundle {
                            texture_atlas: textures.sun.clone(),
                            transform: Transform::from_xyz(get_sun_x(0.0), get_sun_y(0.0), -0.01),
                            ..Default::default()
                        })
                        .insert(Sun)
                        .insert(WorldEntity);
                }
            }
            false => {
                // despawn suns out of hours
                for sun in suns.iter() {
                    commands.entity(sun).despawn();
                }

                let mut rng = ChaCha8Rng::seed_from_u64(RANDOM_SEED);
                let num_stars: u32 = rng.gen_range(30..=70);

                for _ in 0..num_stars {
                    commands
                        .spawn_bundle(SpriteBundle {
                            texture: textures.star.clone(),
                            transform: Transform::from_xyz(
                                rng.gen_range((-WIDTH / 2.0)..(WIDTH / 2.0)),
                                rng.gen_range(0.0..HEIGHT / 2.0),
                                -0.01,
                            ),
                            sprite: Sprite {
                                color: Color::from(Vec4::new(1.0, 1.0, 1.0, 0.0)),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(Star)
                        .insert(WorldEntity);
                }
            }
        }
    }
}

fn get_sun_x(t: f32) -> f32 {
    WIDTH * (0.5 - t)
}

fn get_sun_y(t: f32) -> f32 {
    -8. * (9. * t - 3.0).powi(2) + 300.
}

fn sun_movement(tod: Res<TimeOfDay>, mut suns: Query<&mut Transform, With<Sun>>) {
    let proportion_through_sun_up_time = (tod.time_of_day - SUN_UP) / (SUN_DOWN - SUN_UP);
    let x = get_sun_x(proportion_through_sun_up_time);
    let y = get_sun_y(proportion_through_sun_up_time);

    for mut sun in suns.iter_mut() {
        sun.translation.x = x;
        sun.translation.y = y;
    }
}

fn star_movement(
    tod: Res<TimeOfDay>,
    time: Res<Time>,
    mut stars: Query<(&mut Sprite, &mut Transform), With<Star>>,
) {
    let alpha = if tod.time_of_day > SUN_DOWN {
        // fading in
        ((tod.time_of_day - SUN_DOWN) / (23.0 - SUN_DOWN)).clamp(0.0, 1.0)
    } else {
        1.0 - (tod.time_of_day / (SUN_UP - 2.0)).clamp(0.0, 1.0)
    };

    let dt = time.delta_seconds();

    for (mut star, mut tx) in stars.iter_mut() {
        star.color =
            Color::from(Vec4::new(star.color.r(), star.color.g(), star.color.b(), alpha).abs());
        tx.translation.x += dt * STAR_SPEED;
    }
}
