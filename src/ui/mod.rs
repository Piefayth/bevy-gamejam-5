//! Reusable UI widgets & theming.

// Unused utilities and re-exports may trigger these lints undesirably.
#![allow(dead_code, unused_imports)]

pub mod cycles;
pub mod hotbar;
pub mod interaction;
pub mod palette;
pub mod scoreboard;
pub mod shop;
pub mod widgets;

pub mod prelude {
    pub use super::{
        interaction::{InteractionPalette, InteractionQuery},
        palette as ui_palette,
        widgets::{Containers as _, Widgets as _},
    };
}

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        interaction::plugin,
        cycles::plugin,
        scoreboard::plugin,
        hotbar::plugin,
        shop::plugin,
    ));
}
