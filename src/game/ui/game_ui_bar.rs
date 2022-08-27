use bevy::prelude::*;

use crate::{
    game::{
        components::{ProductionQueueUi, ScoreUi, TopUiBar},
        factory::recipes::Recipes,
    },
    loader::{FontAssets, TextureAssets},
    GRID_SIZE,
};

pub fn spawn_game_ui(
    mut commands: Commands,
    fonts: Res<FontAssets>,
    textures: Res<TextureAssets>,
    recipes: Res<Recipes>,
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
                                size: Size::new(Val::Percent(50.0), Val::Px(48.0)),
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
                                size: Size::new(Val::Percent(30.0), Val::Px(48.0)),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            color: Color::NONE.into(),
                            ..default()
                        })
                        .with_children(|production_queue_layout| {
                            production_queue_layout
                                .spawn_bundle(NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(30.0), Val::Px(48.0)),
                                        flex_direction: FlexDirection::Row,
                                        justify_content: JustifyContent::FlexStart,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    color: Color::NONE.into(),
                                    ..default()
                                })
                                .with_children(|p| {
                                    p.spawn_bundle(TextBundle {
                                        text: Text::from_section(
                                            "Production:  ",
                                            small_text_style.clone(),
                                        ),
                                        ..default()
                                    });
                                });

                            production_queue_layout
                                .spawn_bundle(NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(70.0), Val::Px(48.0)),
                                        justify_content: JustifyContent::FlexStart,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    color: Color::NONE.into(),
                                    ..default()
                                })
                                .insert(ProductionQueueUi);
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

            // recipes
            layout
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(
                            Val::Px(5.0 * GRID_SIZE),
                            Val::Px(GRID_SIZE + recipes.0.len() as f32 * GRID_SIZE),
                        ),
                        margin: UiRect::all(Val::Px(10.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::ColumnReverse,
                        ..default()
                    },
                    color: Color::rgba(0.15, 0.15, 0.15, 0.35).into(),
                    ..default()
                })
                .with_children(|recipe_table| {
                    // header row
                    recipe_table
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.), Val::Px(GRID_SIZE)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            color: Color::rgba(0.15, 0.15, 0.15, 0.35).into(),
                            ..default()
                        })
                        .with_children(|header_row| {
                            header_row.spawn_bundle(TextBundle {
                                text: Text::from_section("Recipes", small_text_style.clone()),
                                ..default()
                            });
                        });

                    // draw recipes
                    let recipe_scale = Vec3::splat(0.95);

                    for (inputs, outputs) in recipes.0.iter() {
                        recipe_table
                            .spawn_bundle(NodeBundle {
                                style: Style {
                                    size: Size::new(Val::Percent(100.), Val::Px(GRID_SIZE)),
                                    ..default()
                                },
                                color: Color::NONE.into(),
                                ..default()
                            })
                            .with_children(|recipe_row| {
                                recipe_row.spawn_bundle(ImageBundle {
                                    image: inputs.0.get_image(&textures).into(),
                                    transform: Transform::from_scale(recipe_scale),
                                    ..default()
                                });

                                recipe_row.spawn_bundle(ImageBundle {
                                    image: textures.plus.clone().into(),
                                    transform: Transform::from_scale(recipe_scale),
                                    ..default()
                                });

                                recipe_row.spawn_bundle(ImageBundle {
                                    image: inputs.1.get_image(&textures).into(),
                                    transform: Transform::from_scale(recipe_scale),
                                    ..default()
                                });

                                recipe_row.spawn_bundle(ImageBundle {
                                    image: textures.arrow.clone().into(),
                                    transform: Transform::from_scale(recipe_scale),
                                    ..default()
                                });

                                recipe_row.spawn_bundle(ImageBundle {
                                    image: outputs.get_image(&textures).into(),
                                    transform: Transform::from_scale(recipe_scale),
                                    ..default()
                                });
                            });
                    }
                });
        });
}
