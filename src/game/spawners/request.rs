use bevy::{prelude::*, ui::FocusPolicy};

use crate::{
    game::components::{BoxType, CountDownTimer, ShipDestination, SpawnShipRequest},
    loader::TextureAssets,
};

pub fn spawn_ship_request_icon(
    layout: &mut ChildBuilder,
    textures: &TextureAssets,
    destination: ShipDestination,
    demands: Vec<BoxType>,
    expiry: f32,
) {
    layout
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Auto, Val::Px(40.0)),
                margin: UiRect::new(
                    Val::Undefined,
                    Val::Px(15.0),
                    Val::Undefined,
                    Val::Undefined,
                ),
                padding: UiRect::new(Val::Px(5.0), Val::Px(5.0), Val::Px(5.0), Val::Px(5.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(ImageBundle {
                    image: textures.countdown[9].clone().into(),
                    focus_policy: FocusPolicy::Pass,
                    ..default()
                })
                .insert(SpawnShipRequest {
                    destination: Some(destination),
                    demands: demands.clone(),
                    expiry,
                })
                .insert(CountDownTimer(Timer::from_seconds(15.0, false)));

            parent.spawn_bundle(ImageBundle {
                image: textures.ship_small.clone().into(),
                focus_policy: FocusPolicy::Pass,
                ..default()
            });

            for demand in demands.iter() {
                parent.spawn_bundle(ImageBundle {
                    image: demand.get_image(textures).into(),
                    focus_policy: FocusPolicy::Pass,
                    ..default()
                });
            }
        });
}
