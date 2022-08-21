#import bevy_sprite::mesh2d_types
#import bevy_sprite::mesh2d_view_bindings

struct CustomSpriteMaterial {
    x_offset: f32,
};
@group(1) @binding(0)
var<uniform> material: CustomSpriteMaterial;
@group(1) @binding(2)
var texture: texture_2d<f32>;
@group(1) @binding(3)
var texture_sampler: sampler;

@group(2) @binding(0)
var<uniform> mesh: Mesh2d;

struct FragmentInput {
    @builtin(front_facing) is_front: bool,
    #import bevy_sprite::mesh2d_vertex_output
};

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
    var output_color: vec4<f32> = textureSample(texture, texture_sampler, vec2((in.uv.x + material.x_offset) % 1.0, in.uv.y));
    return output_color;
}