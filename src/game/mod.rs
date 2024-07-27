//! Game mechanics and content.

use bevy::prelude::*;

pub mod assets;
pub mod audio;
pub mod materials;
pub mod spawn;
pub mod camera;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        audio::plugin,
        assets::plugin,
        materials::plugin,
        spawn::plugin,
        camera::CameraControlPlugin,
    ));
}
