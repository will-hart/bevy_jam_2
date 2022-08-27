use bevy::{prelude::*, ui::FocusPolicy};

use crate::{
    game::{
        components::{BoxType, CartQueueUi, CartQueueUiItem, ProductionQueueUi, ScoreUi, TopUiBar},
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

                    // score
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

            // Cart Spawn Bar
            layout
                .spawn_bundle(NodeBundle {
                    color: Color::rgba(0.15, 0.15, 0.15, 0.35).into(),
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Px(42.0)),
                        padding: UiRect::all(Val::Px(5.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|cart_spawn_bar| {
                    cart_spawn_bar
                        .spawn_bundle(NodeBundle {
                            color: Color::NONE.into(),
                            style: Style {
                                size: Size::new(
                                    Val::Px(1024.0 - 5.0 * 24.0 - 100.0),
                                    Val::Px(32.0),
                                ),
                                ..default()
                            },
                            ..default()
                        })
                        .insert(CartQueueUi);

                    cart_spawn_bar
                        .spawn_bundle(NodeBundle {
                            color: Color::NONE.into(),
                            style: Style {
                                size: Size::new(Val::Px(100.0 + 5.0 * 24.0), Val::Px(32.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|spawn_button_layout| {
                            spawn_button_layout.spawn_bundle(TextBundle {
                                text: Text::from_section("REQUEST:  ", small_text_style.clone()),
                                ..default()
                            });

                            for item in [
                                BoxType::Glassware,
                                BoxType::Apples,
                                BoxType::Grapes,
                                BoxType::Honey,
                                BoxType::Wheat,
                            ]
                            .iter()
                            {
                                spawn_button_layout
                                    .spawn_bundle(ButtonBundle {
                                        color: Color::NONE.into(),
                                        ..Default::default()
                                    })
                                    .insert(CartQueueUiItem(*item))
                                    .with_children(|button| {
                                        button.spawn_bundle(ImageBundle {
                                            image: item.get_image(&textures).into(),
                                            focus_policy: FocusPolicy::Pass,
                                            style: Style {
                                                size: Size::new(Val::Px(24.0), Val::Px(24.0)),
                                                ..default()
                                            },
                                            ..default()
                                        });
                                    });
                            }
                        });
                });

            // recipes
            let recipe_scale = Vec3::splat(0.75);
            layout
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(
                            Val::Px(5.0 * recipe_scale.x * GRID_SIZE),
                            Val::Px((1 + recipes.0.len()) as f32 * recipe_scale.y * GRID_SIZE),
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
                    for (inputs, outputs) in recipes.0.iter() {
                        recipe_table
                            .spawn_bundle(NodeBundle {
                                style: Style {
                                    size: Size::new(
                                        Val::Percent(100.),
                                        Val::Px(recipe_scale.y * GRID_SIZE),
                                    ),
                                    ..default()
                                },
                                color: Color::NONE.into(),
                                ..default()
                            })
                            .with_children(|recipe_row| {
                                recipe_row.spawn_bundle(ImageBundle {
                                    image: inputs.0.get_image(&textures).into(),
                                    style: Style {
                                        size: Size::new(
                                            Val::Px(recipe_scale.x * GRID_SIZE),
                                            Val::Px(recipe_scale.y * GRID_SIZE),
                                        ),
                                        ..default()
                                    },
                                    ..default()
                                });

                                recipe_row.spawn_bundle(ImageBundle {
                                    image: textures.plus.clone().into(),
                                    style: Style {
                                        size: Size::new(
                                            Val::Px(recipe_scale.x * GRID_SIZE),
                                            Val::Px(recipe_scale.y * GRID_SIZE),
                                        ),
                                        ..default()
                                    },
                                    ..default()
                                });

                                recipe_row.spawn_bundle(ImageBundle {
                                    image: inputs.1.get_image(&textures).into(),
                                    style: Style {
                                        size: Size::new(
                                            Val::Px(recipe_scale.x * GRID_SIZE),
                                            Val::Px(recipe_scale.y * GRID_SIZE),
                                        ),
                                        ..default()
                                    },
                                    ..default()
                                });

                                recipe_row.spawn_bundle(ImageBundle {
                                    image: textures.arrow.clone().into(),
                                    style: Style {
                                        size: Size::new(
                                            Val::Px(recipe_scale.x * GRID_SIZE),
                                            Val::Px(recipe_scale.y * GRID_SIZE),
                                        ),
                                        ..default()
                                    },
                                    ..default()
                                });

                                recipe_row.spawn_bundle(ImageBundle {
                                    image: outputs.get_image(&textures).into(),
                                    style: Style {
                                        size: Size::new(
                                            Val::Px(recipe_scale.x * GRID_SIZE),
                                            Val::Px(recipe_scale.y * GRID_SIZE),
                                        ),
                                        ..default()
                                    },
                                    ..default()
                                });
                            });
                    }
                });
        });
}
