pub mod materials;

use bevy::{prelude::*, sprite::Material2dPlugin};
use materials::{BackgroundMaterial, HandMaterial, RingMaterial, SocketMaterial, SocketUiMaterial};

pub fn plugin(app: &mut App) {
    app.add_plugins((
        Material2dPlugin::<RingMaterial>::default(),
        Material2dPlugin::<HandMaterial>::default(),
        Material2dPlugin::<SocketMaterial>::default(),
        UiMaterialPlugin::<SocketUiMaterial>::default(),
        Material2dPlugin::<BackgroundMaterial>::default(),
    ));
}
