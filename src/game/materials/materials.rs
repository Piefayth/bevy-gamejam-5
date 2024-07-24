use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct RingMaterial {
    #[uniform(0)]
    pub data: Vec4, // `radius`, `thickness` and padding
}

impl Material2d for RingMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/ring.wgsl".into()
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct SocketMaterial {
    #[uniform(0)]
    pub inserted_color: LinearRgba,

    #[uniform(1)]
    pub bevel_color: LinearRgba,

    #[uniform(2)]
    pub highlight_color: LinearRgba,

    #[uniform(3)]
    pub data: Vec4 // [start time seconds, trigger_duration, padding, padding]
}

impl Material2d for SocketMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/socket.wgsl".into()
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct SocketUiMaterial {
    #[uniform(0)]
    pub inserted_color: LinearRgba,

    #[uniform(0)]
    pub bevel_color: LinearRgba,
}

impl UiMaterial for SocketUiMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/ui_socket.wgsl".into()
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct BackgroundMaterial {
    #[uniform(0)]
    pub base_color: LinearRgba,

    #[uniform(1)]
    pub blend_color: LinearRgba,
}

impl Material2d for BackgroundMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/background.wgsl".into()
    }
}
