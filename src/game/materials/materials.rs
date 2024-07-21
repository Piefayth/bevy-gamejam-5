use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct RingMaterial {
    #[uniform(0)]
    pub radius: f32,

    #[uniform(1)]
    pub thickness: f32,
}

impl Material2d for RingMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/ring.wgsl".into()
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct HandMaterial {
    #[uniform(0)]
    pub width: f32,

    #[uniform(1)]
    pub height: f32,

    #[uniform(2)]
    pub rotation_radians: f32,
}

impl Material2d for HandMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/hand.wgsl".into()
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct SocketMaterial {
    #[uniform(0)]
    pub inserted_color: LinearRgba,

    #[uniform(1)]
    pub bevel_color: LinearRgba,
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
