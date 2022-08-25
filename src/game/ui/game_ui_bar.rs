use bevy::prelude::*;

use crate::{
    game::components::{ScoreUi, TopUiBar},
    loader::{FontAssets, TextureAssets},
};

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
                        .with_children(|_respawn_bar_layout| {
                            // TODO: first tutorial message
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
        });
}
