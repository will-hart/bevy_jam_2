use bevy::{
    asset::{AssetLoader, BoxedFuture, Error, LoadContext, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
};

// An animation asset based on benimator::Animation
#[derive(TypeUuid, Deref)]
#[uuid = "ae6a74db-f6fa-43c4-ac16-01d13b50e4c6"]
pub struct Animation(benimator::Animation);

// An animation state component based on benimator::State
#[derive(Default, Component, Deref, DerefMut)]
pub struct AnimationState(benimator::State);

// Create (and implement) the asset loader
#[derive(Default)]
pub struct BenimationLoader;

impl AssetLoader for BenimationLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, anyhow::Result<(), Error>> {
        Box::pin(async move {
            // `Animation` implements `serde::Deserialize`,
            // so you may use any serde-compatible deserialization library
            let animation: Animation = Animation(serde_yaml::from_slice(bytes)?);
            load_context.set_default_asset(LoadedAsset::new(animation));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["animation.yml"]
    }
}

pub fn update_animation_frames(
    time: Res<Time>,
    animations: Res<Assets<Animation>>,
    mut query: Query<(
        &mut AnimationState,
        &mut TextureAtlasSprite,
        &Handle<Animation>,
    )>,
) {
    for (mut anim_state, mut texture, handle) in query.iter_mut() {
        // Get the animation from handle (or skip this entity if not yet loaded)
        let animation = match animations.get(handle) {
            Some(anim) => anim,
            None => continue,
        };

        // Update the state
        anim_state.update(animation, time.delta());

        // Update the texture atlas
        texture.index = anim_state.frame_index();
    }
}
