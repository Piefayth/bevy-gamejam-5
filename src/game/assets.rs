use bevy::{
    prelude::*,
    utils::HashMap,
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<HandleMap<ImageKey>>();
    app.init_resource::<HandleMap<ImageKey>>();

    app.register_type::<HandleMap<SfxKey>>();
    app.init_resource::<HandleMap<SfxKey>>();

    app.register_type::<HandleMap<SoundtrackKey>>();
    app.init_resource::<HandleMap<SoundtrackKey>>();

    app.register_type::<HandleMap<ShaderKey>>();
    app.init_resource::<HandleMap<ShaderKey>>();

    app.register_type::<HandleMap<FontKey>>();
    app.init_resource::<HandleMap<FontKey>>();
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum ImageKey {
    Ducky,
}

impl AssetKey for ImageKey {
    type Asset = Image;
}

impl FromWorld for HandleMap<ImageKey> {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        [].into()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum SfxKey {
    Unlock,
    Affirm,
    Neg,
    Click,
    Click2,
    Click3,
    CycleC,
    CycleD,
    CycleLowF,
    CycleLowG,
    CycleHighF,
    CycleHighG,
}

impl AssetKey for SfxKey {
    type Asset = AudioSource;
}

impl FromWorld for HandleMap<SfxKey> {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        [
            (SfxKey::Unlock, asset_server.load("audio/sfx/unlock.ogg")),
            (SfxKey::Affirm, asset_server.load("audio/sfx/muted-guitar-affirm.ogg")),
            (SfxKey::Neg, asset_server.load("audio/sfx/muted-guitar-neg.ogg")),
            (SfxKey::Click, asset_server.load("audio/sfx/click.ogg")),
            (SfxKey::Click2, asset_server.load("audio/sfx/click2.ogg")),
            (SfxKey::Click3, asset_server.load("audio/sfx/click3.ogg")),
            (SfxKey::CycleC, asset_server.load("audio/sfx/cycle-c.ogg")),
            (SfxKey::CycleD, asset_server.load("audio/sfx/cycle-d.ogg")),
            (SfxKey::CycleLowF, asset_server.load("audio/sfx/cycle-low-f.ogg")),
            (SfxKey::CycleLowG, asset_server.load("audio/sfx/cycle-low-g.ogg")),
            (SfxKey::CycleHighF, asset_server.load("audio/sfx/cycle-high-f.ogg")),
            (SfxKey::CycleHighG, asset_server.load("audio/sfx/cycle-high-g.ogg")),
        ]
        .into()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum SoundtrackKey {
    Credits,
    Gameplay,
}

impl AssetKey for SoundtrackKey {
    type Asset = AudioSource;
}

impl FromWorld for HandleMap<SoundtrackKey> {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        [
            (
                SoundtrackKey::Gameplay,
                asset_server.load("audio/soundtracks/soundtrack.ogg"),
            ),
        ]
        .into()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum ShaderKey {
    Ring,
    Hand,
    Socket,
    UiSocket,
    Background,
}

impl AssetKey for ShaderKey {
    type Asset = Shader;
}

impl FromWorld for HandleMap<ShaderKey> {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        [
            (ShaderKey::Ring, asset_server.load("shaders/ring.wgsl")),
            (ShaderKey::Hand, asset_server.load("shaders/hand.wgsl")),
            (ShaderKey::Socket, asset_server.load("shaders/socket.wgsl")),
            (
                ShaderKey::UiSocket,
                asset_server.load("shaders/ui_socket.wgsl"),
            ),
            (
                ShaderKey::UiSocket,
                asset_server.load("shaders/background.wgsl"),
            ),
        ]
        .into()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Reflect, Default)]
pub enum FontKey {
    #[default]
    Default,
}

impl AssetKey for FontKey {
    type Asset = Font;
}

impl FromWorld for HandleMap<FontKey> {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        [(
            FontKey::Default,
            asset_server.load("fonts/Goli-Regular.ttf"),
        )]
        .into()
    }
}

pub trait AssetKey: Sized {
    type Asset: Asset;
}

#[derive(Resource, Reflect, Deref, DerefMut)]
#[reflect(Resource)]
pub struct HandleMap<K: AssetKey>(HashMap<K, Handle<K::Asset>>);

impl<K: AssetKey, T> From<T> for HandleMap<K>
where
    T: Into<HashMap<K, Handle<K::Asset>>>,
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

impl<K: AssetKey> HandleMap<K> {
    pub fn all_loaded(&self, asset_server: &AssetServer) -> bool {
        self.values()
            .all(|x| asset_server.is_loaded_with_dependencies(x))
    }
}
