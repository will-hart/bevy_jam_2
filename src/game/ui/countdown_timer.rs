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
    mut timers: Query<(&mut UiImage, &CountDownTimer)>,
) {
    for (mut image, timer) in timers.iter_mut() {
        let remaining = (10.0 - timer.0.elapsed().as_secs_f32())
            .clamp(0.0, 9.9)
            .floor() as usize;
        *image = textures.countdown[remaining].clone().into();
    }
}
