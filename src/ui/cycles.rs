use bevy::prelude::*;

use crate::{game::spawn::level::Ring, screen::Screen};

use super::widgets::{CycleDisplay, CycleRow};


pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, (update_cycles_ui).run_if(in_state(Screen::Playing)));
}

fn update_cycles_ui(
    q_cycle_display: Query<Entity, With<CycleDisplay>>,
    q_cycle_rows: Query<(Entity, &CycleRow)>,
    q_ring: Query<&Ring>,
) {
    // HOLD UP LMAO
    // we need to display
        // 1. the current cycle in progress for EACH RING
        // 2. the cycle history
        // oh gosh


}