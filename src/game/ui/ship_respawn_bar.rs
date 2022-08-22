use bevy::prelude::*;

use crate::{
    game::components::{
        MarketPriceDirectionIndicator, MarketPriceIndicator, MarketPriceValueIndicator, ScoreUi,
        ShipRespawnBar, BOX_TYPES, DESTINATIONS,
    },
    loader::{FontAssets, TextureAssets},
};

use super::launch_ships::spawn_ship_buttons;

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
                                ..default()
                            },
                            color: Color::NONE.into(),
                            ..default()
                        })
                        .insert(ShipRespawnBar);

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
                        size: Size::new(Val::Percent(100.0), Val::Px(48.0)),
                        padding: UiRect::all(Val::Px(10.0)),
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    color: Color::rgba(0.15, 0.15, 0.15, 0.35).into(),
                    ..Default::default()
                })
                .with_children(|market_layout| {
                    DESTINATIONS.iter().for_each(|destination| {
                        market_layout.spawn_bundle(TextBundle {
                            text: Text::from_section(
                                format!("{}", destination),
                                small_text_style.clone(),
                            ),
                            ..default()
                        });

                        BOX_TYPES.iter().for_each(|bt| {
                            market_layout
                                .spawn_bundle(NodeBundle {
                                    style: Style {
                                        margin: UiRect::new(
                                            Val::Undefined,
                                            Val::Px(5.0),
                                            Val::Undefined,
                                            Val::Undefined,
                                        ),
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    color: Color::NONE.into(),
                                    ..default()
                                })
                                .insert(MarketPriceIndicator(destination.clone(), bt.clone()))
                                .with_children(|market_item| {
                                    market_item.spawn_bundle(ImageBundle {
                                        image: bt.get_image(&textures).into(),
                                        transform: Transform::from_scale(Vec3::splat(0.7)),
                                        ..default()
                                    });
                                    market_item
                                        .spawn_bundle(TextBundle {
                                            text: Text::from_section("0", small_text_style.clone()),
                                            ..default()
                                        })
                                        .insert(MarketPriceValueIndicator);
                                    market_item
                                        .spawn_bundle(ImageBundle {
                                            image: textures.up.clone().into(),
                                            transform: Transform::from_scale(Vec3::splat(0.7)),
                                            ..default()
                                        })
                                        .insert(MarketPriceDirectionIndicator);
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
