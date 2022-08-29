use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::{game::Animation, GameState};

/// A plugin that automatically loads in fonts, audio assets, textures, animations, maps etc
/// from file. See the various asset collection classes for the assets being loaded
pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .with_collection::<FontAssets>()
                .with_collection::<AudioAssets>()
                .with_collection::<TextureAssets>()
                .with_collection::<AnimationAssets>()
                .continue_to_state(GameState::Menu),
        );
    }
}

/// Contains font assets loaded from file
#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/JosefinSans-SemiBold.ttf")]
    pub default_font: Handle<Font>,
}

/// Contains audio assets loaded from file
#[derive(AssetCollection)]
pub struct AudioAssets {
    #[asset(path = "audio/box_drop.ogg")]
    pub box_drop: Handle<bevy_kira_audio::AudioSource>,

    #[asset(path = "audio/coin_drop.ogg")]
    pub coin_drop: Handle<bevy_kira_audio::AudioSource>,

    #[asset(path = "audio/music.ogg")]
    pub music: Handle<bevy_kira_audio::AudioSource>,

    #[asset(path = "audio/wind.ogg")]
    pub wind: Handle<bevy_kira_audio::AudioSource>,

    #[asset(path = "audio/rain.ogg")]
    pub rain: Handle<bevy_kira_audio::AudioSource>,

    #[asset(path = "audio/ships_bell.ogg")]
    pub ships_bell: Handle<bevy_kira_audio::AudioSource>,

    #[asset(path = "audio/splash.ogg")]
    pub splash: Handle<bevy_kira_audio::AudioSource>,

    #[asset(path = "audio/i_cant_make_that.ogg")]
    pub i_cant_make_that: Handle<bevy_kira_audio::AudioSource>,
}

/// Contains texture assets loaded from file
#[derive(AssetCollection)]
pub struct TextureAssets {
    #[asset(texture_atlas(tile_size_x = 96., tile_size_y = 96., columns = 5, rows = 1))]
    #[asset(path = "textures/torch.png")]
    pub torch: Handle<TextureAtlas>,

    #[asset(path = "textures/icon.png")]
    pub bevy_icon: Handle<Image>,

    #[asset(path = "textures/background.png")]
    pub background: Handle<Image>,

    #[asset(texture_atlas(tile_size_x = 160., tile_size_y = 64., columns = 3, rows = 1))]
    #[asset(path = "textures/horse_and_cart.png")]
    pub horse_and_cart: Handle<TextureAtlas>,

    #[asset(path = "textures/cart_shadow.png")]
    pub cart_shadow: Handle<Image>,

    #[asset(texture_atlas(tile_size_x = 64., tile_size_y = 64., columns = 1, rows = 1))]
    #[asset(path = "textures/sun.png")]
    pub sun: Handle<TextureAtlas>,

    #[asset(path = "textures/star.png")]
    pub star: Handle<Image>,

    #[asset(path = "textures/waves.png")]
    pub waves: Handle<Image>,

    #[asset(path = "textures/coin.png")]
    pub coin: Handle<Image>,

    #[asset(path = "textures/ship_small.png")]
    pub ship_small: Handle<Image>,

    #[asset(path = "textures/up.png")]
    pub up: Handle<Image>,

    #[asset(path = "textures/down.png")]
    pub down: Handle<Image>,

    #[asset(texture_atlas(tile_size_x = 32., tile_size_y = 32., columns = 9, rows = 1))]
    #[asset(path = "textures/crates.png")]
    pub crates: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 160., tile_size_y = 64., columns = 2, rows = 5))]
    #[asset(path = "textures/cart_boxes.png")]
    pub cart_boxes: Handle<TextureAtlas>,

    #[asset(path = "textures/box_type_glassware.png")]
    pub box_type_glassware: Handle<Image>,

    #[asset(path = "textures/box_type_apples.png")]
    pub box_type_apples: Handle<Image>,

    #[asset(path = "textures/box_type_grapes.png")]
    pub box_type_grapes: Handle<Image>,

    #[asset(path = "textures/box_type_honey.png")]
    pub box_type_honey: Handle<Image>,

    #[asset(path = "textures/box_type_wheat.png")]
    pub box_type_wheat: Handle<Image>,

    #[asset(path = "textures/box_type_cider.png")]
    pub box_type_cider: Handle<Image>,

    #[asset(path = "textures/box_type_mead.png")]
    pub box_type_mead: Handle<Image>,

    #[asset(path = "textures/box_type_beer.png")]
    pub box_type_beer: Handle<Image>,

    #[asset(path = "textures/box_type_wine.png")]
    pub box_type_wine: Handle<Image>,

    #[asset(path = "textures/box_type_wine2.png")]
    pub box_type_wine2: Handle<Image>,

    #[asset(texture_atlas(tile_size_x = 288., tile_size_y = 224., columns = 1, rows = 1))]
    #[asset(path = "textures/ship.png")]
    pub ship: Handle<TextureAtlas>,

    #[asset(path = "textures/arrow.png")]
    pub arrow: Handle<Image>,

    #[asset(path = "textures/plus.png")]
    pub plus: Handle<Image>,

    #[asset(path = "textures/tutorial_1.png")]
    pub tutorial_1: Handle<Image>,

    #[asset(path = "textures/tutorial_2.png")]
    pub tutorial_2: Handle<Image>,

    #[asset(path = "textures/tutorial_3.png")]
    pub tutorial_3: Handle<Image>,

    #[asset(path = "textures/tutorial_4.png")]
    pub tutorial_4: Handle<Image>,

    #[asset(path = "textures/tutorial_6.png")]
    pub tutorial_6: Handle<Image>,

    #[asset(path = "textures/tutorial_7.png")]
    pub tutorial_7: Handle<Image>,

    #[asset(path = "textures/tutorial_8.png")]
    pub tutorial_8: Handle<Image>,

    #[asset(path = "textures/menu.png")]
    pub menu: Handle<Image>,

    #[asset(path = "textures/game_over.png")]
    pub game_over: Handle<Image>,

    #[asset(path = "textures/rain_drop.png")]
    pub rain_drop: Handle<Image>,

    // can't do spritesheets in UI so need to load individual countdown images :(
    // also, folders aren't supported in web builds :facepalm:
    #[asset(
        paths(
            "textures/countdown/countdown_1.png",
            "textures/countdown/countdown_2.png",
            "textures/countdown/countdown_3.png",
            "textures/countdown/countdown_4.png",
            "textures/countdown/countdown_5.png",
            "textures/countdown/countdown_6.png",
            "textures/countdown/countdown_7.png",
            "textures/countdown/countdown_8.png",
            "textures/countdown/countdown_9.png",
            "textures/countdown/countdown_10.png"
        ),
        collection(typed)
    )]
    pub countdown: Vec<Handle<Image>>,

    #[asset(texture_atlas(tile_size_x = 32., tile_size_y = 32., columns = 6, rows = 1))]
    #[asset(path = "textures/splashes.png")]
    pub splashes: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 160., tile_size_y = 192., columns = 8, rows = 1))]
    #[asset(path = "textures/factory.png")]
    pub factory: Handle<TextureAtlas>,
}

/// Contains animation assets loaded from file using a custom animation loader
#[derive(AssetCollection)]
pub struct AnimationAssets {
    #[asset(path = "animations/torch.animation.yml")]
    pub torch: Handle<Animation>,

    #[asset(path = "animations/torch-off.animation.yml")]
    pub torch_off: Handle<Animation>,

    #[asset(path = "animations/cart.animation.yml")]
    pub cart: Handle<Animation>,

    #[asset(path = "animations/ship_idle.animation.yml")]
    pub ship_idle: Handle<Animation>,

    #[asset(path = "animations/splashes.animation.yml")]
    pub splashes: Handle<Animation>,

    #[asset(path = "animations/factory_on.animation.yml")]
    pub factory_on: Handle<Animation>,

    #[asset(path = "animations/factory_off.animation.yml")]
    pub factory_off: Handle<Animation>,

    #[asset(path = "animations/factory_tutorial.animation.yml")]
    pub factory_tutorial: Handle<Animation>,
}
