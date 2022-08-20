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
                .continue_to_state(GameState::Playing),
        );
    }
}

/// Contains font assets loaded from file
#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans: Handle<Font>,
}

/// Contains audio assets loaded from file
#[derive(AssetCollection)]
pub struct AudioAssets {
    // #[asset(path = "audio/flying.ogg")]
    // pub flying: Handle<AudioSource>,
}

/// Contains texture assets loaded from file
#[derive(AssetCollection)]
pub struct TextureAssets {
    #[asset(texture_atlas(tile_size_x = 32., tile_size_y = 64., columns = 5, rows = 1))]
    #[asset(path = "textures/torch.png")]
    pub torch: Handle<TextureAtlas>,

    #[asset(path = "textures/icon.png")]
    pub bevy_icon: Handle<Image>,

    #[asset(path = "textures/background.png")]
    pub background: Handle<Image>,

    #[asset(texture_atlas(tile_size_x = 160., tile_size_y = 64., columns = 3, rows = 1))]
    #[asset(path = "textures/horse_and_cart.png")]
    pub horse_and_cart: Handle<TextureAtlas>,

    #[asset(path = "textures/crate1_on_cart.png")]
    pub crate1_on_cart: Handle<Image>,

    #[asset(path = "textures/crate2_on_cart.png")]
    pub crate2_on_cart: Handle<Image>,

    #[asset(path = "textures/crate.png")]
    pub cart: Handle<Image>,

    #[asset(path = "textures/waves.png")]
    pub waves: Handle<Image>,

    #[asset(texture_atlas(tile_size_x = 288., tile_size_y = 224., columns = 4, rows = 1))]
    #[asset(path = "textures/ship.png")]
    pub ship: Handle<TextureAtlas>,
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
}
