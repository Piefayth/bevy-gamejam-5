mod credits;
mod playing;
mod title;

use bevy::prelude::*;

use crate::core::booting::BootingState;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Screen>()
        .enable_state_scoped_entities::<Screen>();

    app.add_plugins((title::plugin, credits::plugin, playing::plugin));
}

#[derive(SubStates, Debug, Hash, PartialEq, Eq, Clone, Default)]
#[source(BootingState = BootingState::Ready)]
pub enum Screen {
    #[default]
    Title,
    Credits,
    Playing,
}
