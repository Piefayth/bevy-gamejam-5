//! Development tools for the game. This plugin is only enabled in dev builds.

use crate::screen::Screen;
use bevy::{dev_tools::states::log_transitions, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_picking::debug::DebugPickingMode;

pub(super) fn plugin(app: &mut App) {
    // Print state transitions in dev builds
    app.add_systems(Update, log_transitions::<Screen>);
    app.add_plugins(WorldInspectorPlugin::new());
    app.insert_resource(DebugPickingMode::Normal);
}
