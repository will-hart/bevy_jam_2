use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::{
        render_asset::RenderAssets,
        render_resource::{AsBindGroup, AsBindGroupShaderType, ShaderRef, ShaderType},
    },
    sprite::{Material2d, Material2dPlugin},
};
use rand::{thread_rng, Rng};

pub struct CustomSpritePlugin;

impl Plugin for CustomSpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(Material2dPlugin::<CustomSpriteMaterial>::default());
    }
}

// This is the struct that will be passed to your shader
#[derive(AsBindGroup, Debug, Clone, TypeUuid)]
#[uuid = "aabbccdd-e3ca-4e1e-bb9d-4d8bc1ad8c19"]
#[uniform(0, CustomSpriteMaterialUniform)]
pub struct CustomSpriteMaterial {
    pub x_offset: f32,

    #[texture(2)]
    #[sampler(3)]
    pub texture: Option<Handle<Image>>,
}

impl Material2d for CustomSpriteMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/custom_sprite.wgsl".into()
    }
}

impl From<Handle<Image>> for CustomSpriteMaterial {
    fn from(texture: Handle<Image>) -> Self {
        let mut rng = thread_rng();

        CustomSpriteMaterial {
            texture: Some(texture),
            x_offset: rng.gen_range(0.0..=1.0),
        }
    }
}

/// The GPU representation of the uniform data of a [`CustomSpriteMaterial`].
#[derive(Clone, Default, ShaderType)]
pub struct CustomSpriteMaterialUniform {
    pub x_offset: f32,
}

impl AsBindGroupShaderType<CustomSpriteMaterialUniform> for CustomSpriteMaterial {
    fn as_bind_group_shader_type(
        &self,
        _images: &RenderAssets<Image>,
    ) -> CustomSpriteMaterialUniform {
        CustomSpriteMaterialUniform {
            x_offset: self.x_offset,
        }
    }
}
