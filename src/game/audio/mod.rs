pub mod soundtrack;

use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_plugins((soundtrack::plugin));
}
