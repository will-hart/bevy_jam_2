use bevy::prelude::*;

use crate::{
    game::{
        components::{RequestShip, ShipLaunchButton},
        ship_launch::OnLaunchShip,
    },
    loader::FontAssets,
    GRID_SIZE,
};

use super::request_ship::OnRequestShipSpawn;

pub const NORMAL_BUTTON: Color = Color::rgba(0.15, 0.15, 0.15, 0.05);
pub const HOVERED_BUTTON: Color = Color::rgba(0.15, 0.15, 0.15, 0.25);
pub const PRESSED_BUTTON: Color = Color::rgba(0.15, 0.15, 0.15, 0.75);

/// NOT A SYSTEM, called by ship_respawn_bar to spawn ship buttons
pub fn spawn_ship_buttons(commands: &mut ChildBuilder, fonts: &FontAssets) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceAround,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .with_children(|layout_node| {
            [-10.0 * GRID_SIZE, 0.0, 10.0 * GRID_SIZE]
                .iter()
                .enumerate()
                .for_each(|(i, _)| {
                    layout_node
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(33.0), Val::Px(100.0)),
                                justify_content: JustifyContent::FlexStart,
                                ..default()
                            },
                            color: Color::NONE.into(),
                            ..default()
                        })
                        .with_children(|button_node| {
                            button_node
                                .spawn_bundle(ButtonBundle {
                                    style: Style {
                                        size: Size::new(Val::Px(70.0), Val::Px(40.0)),
                                        margin: UiRect::all(Val::Auto),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        border: UiRect::all(Val::Px(2.0)),
                                        ..default()
                                    },
                                    color: NORMAL_BUTTON.into(),
                                    ..default()
                                })
                                .with_children(|parent| {
                                    parent.spawn_bundle(TextBundle::from_section(
                                        "Launch",
                                        TextStyle {
                                            font: fonts.default_font.clone(),
                                            font_size: 18.,
                                            color: Color::WHITE,
                                        },
                                    ));
                                })
                                .insert(ShipLaunchButton(i));
                        });
                });
        });
}

pub fn button_visibility(
    mut hide_events: EventReader<OnLaunchShip>,
    mut buttons: Query<(&mut Visibility, &ShipLaunchButton)>,
) {
    for evt in hide_events.iter() {
        let slot_idx = evt.slot_id;

        for (mut button_vis, button) in buttons.iter_mut() {
            if button.0 == slot_idx {
                button_vis.is_visible = false;
            }
        }
    }
}

pub fn button_interaction(
    mut launch_events: EventWriter<OnLaunchShip>,
    mut spawn_events: EventWriter<OnRequestShipSpawn>,
    mut interaction_query: Query<
        (
            Entity,
            &Interaction,
            &mut UiColor,
            Option<&ShipLaunchButton>,
            Option<&RequestShip>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (btn_entity, interaction, mut color, button_data, ship_request) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();

                if let Some(b) = button_data {
                    info!("Launching ship from slot {}", b.0);
                    launch_events.send(OnLaunchShip { slot_id: b.0 })
                }

                if let Some(sr) = ship_request {
                    info!("Attemping to spawn ship {:?}", sr);
                    spawn_events.send(OnRequestShipSpawn(btn_entity, sr.clone()));
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
