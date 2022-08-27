use bevy::prelude::*;
use iyes_loopless::prelude::IntoConditionalSystem;

use crate::{game::components::CountDownTimer, loader::TextureAssets, GameState};

pub struct CountDownTimerPlugin;

impl Plugin for CountDownTimerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_countdown_timers.run_in_state(GameState::Playing))
            .add_system(update_countdown_animations.run_in_state(GameState::Playing));
    }
}

fn update_countdown_timers(time: Res<Time>, mut timers: Query<&mut CountDownTimer>) {
    let delta = time.delta();
    for mut timer in timers.iter_mut() {
        timer.0.tick(delta);
    }
}

fn update_countdown_animations(
    textures: Res<TextureAssets>,
    mut timers: Query<(
        Option<&mut UiImage>,
        Option<&mut Handle<Image>>,
        &CountDownTimer,
    )>,
) {
    let num_textures = textures.countdown.len() as f32;
    for (ui_image, image, timer) in timers.iter_mut() {
        let remaining_proportion = (1.0
            - timer.0.elapsed().as_secs_f32() / timer.0.duration().as_secs_f32())
        .clamp(0.0, 0.999);
        let idx = (remaining_proportion * num_textures).floor() as usize;

        match ui_image {
            Some(mut img) => *img = textures.countdown[idx].clone().into(),
            None => {}
        }

        match image {
            Some(mut img) => *img = textures.countdown[idx].clone().into(),
            None => {}
        }
    }
}
