use bevy::prelude::*;

use crate::game::{components::CartQueueUiButton, spawners::CartSpawningState};

pub const NORMAL_BUTTON: Color = Color::NONE;
pub const HOVERED_BUTTON: Color = Color::rgba(0.15, 0.15, 0.15, 0.25);
pub const PRESSED_BUTTON: Color = Color::rgba(0.15, 0.15, 0.15, 0.75);

#[allow(clippy::type_complexity)]
pub fn button_interaction(
    mut cart_queue: ResMut<CartSpawningState>,
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, Option<&CartQueueUiButton>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, cart_ui) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();

                if let Some(cui) = cart_ui {
                    info!("Requested spawn of {:?}", cui.0);
                    cart_queue.items.push(cui.0);
                }
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}
