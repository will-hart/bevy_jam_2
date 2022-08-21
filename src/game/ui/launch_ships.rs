use bevy::prelude::*;

use crate::{game::components::ShipLaunchButton, loader::FontAssets, GRID_SIZE};

const NORMAL_BUTTON: Color = Color::rgba(0.15, 0.15, 0.15, 0.5);
const HOVERED_BUTTON: Color = Color::rgba(0.15, 0.15, 0.15, 0.75);
const PRESSED_BUTTON: Color = Color::rgba(0.15, 0.15, 0.15, 0.25);

pub fn spawn_ship_buttons(mut commands: Commands, fonts: Res<FontAssets>) {
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
                                        size: Size::new(Val::Px(60.0), Val::Px(32.0)),
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

pub fn button_interaction(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &ShipLaunchButton),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, ship) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                warn!("Launch ship {}", ship.0);
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
