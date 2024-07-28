use bevy::{audio::{PlaybackMode, Volume}, prelude::*};

use crate::{game::assets::{HandleMap, SfxKey, SoundtrackKey}, ui::scoreboard::AudioSettings};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<IsSoundtrack>();
    app.observe(play_soundtrack);
    app.observe(on_sfx);
}

fn play_soundtrack(
    trigger: Trigger<PlaySoundtrack>,
    mut commands: Commands,
    soundtrack_handles: Res<HandleMap<SoundtrackKey>>,
    soundtrack_query: Query<Entity, With<IsSoundtrack>>,
) {
    for entity in &soundtrack_query {
        commands.entity(entity).despawn_recursive();
    }

    let soundtrack_key = match trigger.event() {
        PlaySoundtrack::Key(key) => *key,
        PlaySoundtrack::Disable => return,
    };
    commands.spawn((
        AudioSourceBundle {
            source: soundtrack_handles[&soundtrack_key].clone_weak(),
            settings: PlaybackSettings {
                mode: PlaybackMode::Loop,
                ..default()
            },
        },
        IsSoundtrack,
    ));
}

/// Trigger this event to play or disable the soundtrack.
/// Playing a new soundtrack will overwrite the previous one.
/// Soundtracks will loop.
#[derive(Event)]
pub enum PlaySoundtrack {
    Key(SoundtrackKey),
    Disable,
}

/// Marker component for the soundtrack entity so we can find it later.
#[derive(Component, Reflect)]
#[reflect(Component)]
struct IsSoundtrack;


#[derive(Event)]
pub struct PlaySfx {
    pub key: SfxKey,
    pub volume: f32,
}

fn on_sfx(
    trigger: Trigger<PlaySfx>,
    mut commands: Commands,
    sfx_handles: Res<HandleMap<SfxKey>>,
    audio_settings: Res<AudioSettings>,
) {
    if audio_settings.enabled {
        commands.spawn(AudioSourceBundle {
            source: sfx_handles[&trigger.event().key].clone_weak(),
            settings: PlaybackSettings {
                mode: PlaybackMode::Despawn,
                volume: Volume::new(trigger.event().volume),
                ..default()
            },
        });
    }

}