use bevy::{prelude::*, ui::FocusPolicy};

use crate::{
    game::components::{
        BoxType, MarketPriceDirectionIndicator, MarketPriceIndicator, MarketPriceValueIndicator,
        RequestShip, ScoreUi, ShipDestination, TopUiBar, TutorialMarker, BOX_TYPES, DESTINATIONS,
    },
    loader::{FontAssets, TextureAssets},
    GRID_SIZE,
};

use super::launch_ships::{spawn_ship_buttons, NORMAL_BUTTON};

pub fn spawn_ship_respawn_bar(
    mut commands: Commands,
    fonts: Res<FontAssets>,
    textures: Res<TextureAssets>,
) {
    let text_style = TextStyle {
        font: fonts.default_font.clone(),
        font_size: 24.0,
        color: Color::WHITE,
    };
    let small_text_style = TextStyle {
        font: fonts.default_font.clone(),
        font_size: 16.0,
        color: Color::WHITE,
    };

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::ColumnReverse,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .with_children(|layout| {
            // top menu
            layout
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Px(48.0)),
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    color: Color::rgba(0.15, 0.15, 0.15, 0.35).into(),
                    ..Default::default()
                })
                .with_children(|bar_layout| {
                    bar_layout
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(80.0), Val::Px(48.0)),
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            color: Color::NONE.into(),
                            ..default()
                        })
                        .insert(TopUiBar)
                        .with_children(|respawn_bar_layout| {
                            // first spawn ship button
                            spawn_ship_request_button(
                                &textures,
                                respawn_bar_layout,
                                ShipDestination::NewWorld,
                                vec![BoxType::Fruit, BoxType::Fruit],
                            );

                            // tutorial text
                            respawn_bar_layout
                                .spawn_bundle(TextBundle {
                                    text: Text::from_section(
                                        "< click here to request a ship",
                                        small_text_style.clone(),
                                    ),
                                    ..default()
                                })
                                .insert(TutorialMarker(0));
                        });

                    bar_layout
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(20.0), Val::Auto),
                                justify_content: JustifyContent::FlexEnd,
                                align_items: AlignItems::Center,
                                margin: UiRect::new(
                                    Val::Undefined,
                                    Val::Px(10.0),
                                    Val::Undefined,
                                    Val::Undefined,
                                ),
                                ..default()
                            },
                            color: Color::NONE.into(),
                            ..default()
                        })
                        .with_children(|score_layout| {
                            score_layout.spawn_bundle(ImageBundle {
                                style: Style {
                                    size: Size::new(Val::Px(32.0), Val::Auto),
                                    ..default()
                                },
                                image: textures.coin.clone().into(),
                                ..default()
                            });

                            score_layout
                                .spawn_bundle(TextBundle {
                                    text: Text::from_section("0", text_style.clone()),
                                    ..default()
                                })
                                .insert(ScoreUi);
                        });
                });

            // market prices
            layout
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(13. * GRID_SIZE), Val::Px(4.0 * GRID_SIZE)),
                        justify_content: JustifyContent::FlexStart,
                        align_items: AlignItems::FlexEnd,
                        flex_direction: FlexDirection::ColumnReverse,
                        margin: UiRect::new(
                            Val::Px(15.0),
                            Val::Undefined,
                            Val::Px(15.0),
                            Val::Undefined,
                        ),
                        padding: UiRect::all(Val::Px(5.0)),
                        ..default()
                    },
                    color: Color::rgba(0.15, 0.15, 0.15, 0.35).into(),
                    ..Default::default()
                })
                .with_children(|market_table| {
                    // table row
                    market_table
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.), Val::Px(GRID_SIZE)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::FlexEnd,
                                ..default()
                            },
                            color: Color::NONE.into(),
                            ..default()
                        })
                        .with_children(|parent_row| {
                            parent_row
                                .spawn_bundle(NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Px(4.0 * GRID_SIZE), Val::Auto),
                                        align_items: AlignItems::Center,
                                        justify_content: JustifyContent::Center,
                                        align_content: AlignContent::Center,
                                        ..default()
                                    },
                                    color: Color::NONE.into(),
                                    ..default()
                                })
                                .with_children(|builder| {
                                    builder.spawn_bundle(TextBundle {
                                        text: Text::from_section(
                                            "PROFIT",
                                            small_text_style.clone(),
                                        ),
                                        ..default()
                                    });
                                });

                            BOX_TYPES.iter().for_each(|bt| {
                                parent_row
                                    .spawn_bundle(NodeBundle {
                                        style: Style {
                                            size: Size::new(
                                                Val::Px(2.0 * GRID_SIZE),
                                                Val::Px(GRID_SIZE),
                                            ),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        color: Color::NONE.into(),
                                        ..default()
                                    })
                                    .with_children(|nb| {
                                        nb.spawn_bundle(ImageBundle {
                                            image: bt.get_image(&textures).into(),
                                            transform: Transform::from_scale(Vec3::splat(0.9)),
                                            ..default()
                                        });
                                    });
                            });
                        });

                    // draw the market rows
                    DESTINATIONS.iter().for_each(|destination| {
                        market_table
                            .spawn_bundle(NodeBundle {
                                style: Style {
                                    size: Size::new(Val::Percent(100.), Val::Px(GRID_SIZE)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                color: Color::NONE.into(),
                                ..default()
                            })
                            .with_children(|market_row| {
                                market_row.spawn_bundle(ImageBundle {
                                    image: destination.get_image(&textures).into(),
                                    style: Style {
                                        size: Size::new(Val::Px(GRID_SIZE), Val::Px(GRID_SIZE)),
                                        ..default()
                                    },
                                    ..default()
                                });

                                market_row.spawn_bundle(TextBundle {
                                    text: Text::from_section(
                                        format!("{}", destination),
                                        small_text_style.clone(),
                                    ),
                                    style: Style {
                                        size: Size::new(
                                            Val::Px(4.0 * GRID_SIZE),
                                            Val::Px(GRID_SIZE),
                                        ),
                                        ..default()
                                    },
                                    ..default()
                                });

                                BOX_TYPES.iter().for_each(|bt| {
                                    market_row
                                        .spawn_bundle(NodeBundle {
                                            style: Style {
                                                size: Size::new(
                                                    Val::Px(2.0 * GRID_SIZE),
                                                    Val::Auto,
                                                ),
                                                justify_content: JustifyContent::Center,
                                                align_items: AlignItems::Center,
                                                ..default()
                                            },
                                            color: Color::NONE.into(),
                                            ..default()
                                        })
                                        .insert(MarketPriceIndicator(*destination, *bt))
                                        .with_children(|market_item| {
                                            market_item
                                                .spawn_bundle(TextBundle {
                                                    text: Text::from_section(
                                                        "0",
                                                        small_text_style.clone(),
                                                    ),
                                                    ..default()
                                                })
                                                .insert(MarketPriceValueIndicator);
                                            market_item
                                                .spawn_bundle(ImageBundle {
                                                    image: textures.up.clone().into(),
                                                    transform: Transform::from_scale(Vec3::splat(
                                                        0.7,
                                                    )),
                                                    ..default()
                                                })
                                                .insert(MarketPriceDirectionIndicator);
                                        });
                                });
                            });
                    });
                });

            // launch buttons
            layout
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Px(120.0)),
                        position_type: PositionType::Absolute,
                        position: UiRect::new(
                            Val::Px(0.0),
                            Val::Px(0.0),
                            Val::Undefined,
                            Val::Px(0.0),
                        ),
                        ..default()
                    },
                    color: Color::NONE.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    spawn_ship_buttons(parent, &fonts);
                });
        });
}

pub fn spawn_ship_request_button(
    textures: &TextureAssets,
    layout: &mut ChildBuilder,
    destination: ShipDestination,
    demands: Vec<BoxType>,
) {
    layout
        .spawn_bundle(ButtonBundle {
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
            color: NORMAL_BUTTON.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(ImageBundle {
                image: textures.ship_small.clone().into(),
                focus_policy: FocusPolicy::Pass,
                ..default()
            });

            for demand in demands.iter() {
                parent.spawn_bundle(ImageBundle {
                    image: demand.get_image(&textures).into(),
                    focus_policy: FocusPolicy::Pass,
                    ..default()
                });
            }
        })
        .insert(RequestShip {
            destination,
            demands,
        });
}
