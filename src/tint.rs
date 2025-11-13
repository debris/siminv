use bevy::{prelude::*, render::render_resource::*};

#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
struct TintMaterial {
    #[uniform(0)]
    color: Vec4,
    #[texture(1)]
    #[sampler(3)]
    color_texture: Handle<Image>,
}

impl UiMaterial for TintMaterial {
    fn fragment_shader() -> bevy::shader::ShaderRef {
        "shaders/tint_ui.wgsl".into() 
    }
}

