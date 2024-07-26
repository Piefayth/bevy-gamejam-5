//! The screen state for the main game loop.

use std::{f32::consts::PI, time::Duration};

use bevy::{
    color::palettes::css::{BLACK, BLUE, LIGHT_GREEN, ORANGE, PINK, WHITE},
    input::common_conditions::input_just_pressed,
    prelude::*,
    sprite::MaterialMesh2dBundle,
};
use bevy_tweening::{lens::TransformPositionLens, Animator, EaseFunction, Tween};
use num_bigint::BigUint;

use super::Screen;
use crate::{
    game::{
        assets::{FontKey, HandleMap, SoundtrackKey},
        audio::soundtrack::PlaySoundtrack,
        materials::materials::{RingMaterial, SocketMaterial, SocketUiMaterial},
        spawn::level::{
            map_socket_color, map_socket_color_hotkey, map_socket_color_trigger_duration,
            map_socket_highlight_color, socket_position, GameplayMeshes, Ring, Socket, SocketColor,
            SpawnLevel,
        },
    },
    ui::{
        hotbar::map_socket_color_description_text,
        shop::{NewShop, UpgradeHistory},
    },
};

use crate::ui::prelude::*;

#[derive(Resource, Default)]
pub struct Currency {
    pub amount: BigUint,
    pub pending_amount: BigUint,
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Playing), enter_playing);
    app.add_systems(OnExit(Screen::Playing), exit_playing);

    app.add_systems(
        Update,
        (
            return_to_title_screen
                .run_if(in_state(Screen::Playing).and_then(input_just_pressed(KeyCode::Escape))),
            (
                (progress_cycle, ring_cycle_display).chain(),
                despawn_after_system,
            )
                .run_if(in_state(Screen::Playing)),
        ),
    );

    app.init_resource::<Currency>();

    app.observe(on_socket_triggered);
    app.observe(on_cycle_complete);
}

fn enter_playing(
    mut commands: Commands,
    mut materials: ResMut<Assets<SocketUiMaterial>>,
    font_handles: ResMut<HandleMap<FontKey>>,
    upgrade_history: Res<UpgradeHistory>,
) {
    commands.trigger(SpawnLevel);
    commands.trigger(PlaySoundtrack::Key(SoundtrackKey::Gameplay));

    let mut gameplay_wrapper = Entity::PLACEHOLDER;

    commands
        .ui_root()
        .insert(StateScoped(Screen::Playing))
        .with_children(|root_children| {
            let mut gameplay_wrapper_commands = root_children.horizontal_container();
            gameplay_wrapper = gameplay_wrapper_commands.id();
            gameplay_wrapper_commands.with_children(|gameplay_wrapper_children| {
                // two children, the upgrade shop and the remaining layout
                gameplay_wrapper_children
                    .vertical_container(JustifyContent::SpaceBetween)
                    .with_children(|score_and_hotbar_wrapper| {
                        // two children, the score display and the hotbar
                        score_and_hotbar_wrapper
                            .score_display(font_handles[&FontKey::Default].clone());

                        let hotbar_first_position_socket_color = SocketColor::BLUE;
                        score_and_hotbar_wrapper
                            .vertical_container(JustifyContent::End)
                            .with_children(|hotbar_wrapper_children| {
                                // need to be different materials, even though right now they have the same values
                                let button_socket_material = materials.add(SocketUiMaterial {
                                    bevel_color: BLACK.into(),
                                    inserted_color: map_socket_color(
                                        hotbar_first_position_socket_color,
                                    ),
                                });

                                let description_socket_material = materials.add(SocketUiMaterial {
                                    bevel_color: BLACK.into(),
                                    inserted_color: map_socket_color(
                                        hotbar_first_position_socket_color,
                                    ),
                                });

                                hotbar_wrapper_children.hotbar_description(
                                    map_socket_color_description_text(
                                        hotbar_first_position_socket_color,
                                        &upgrade_history,
                                    ),
                                    hotbar_first_position_socket_color,
                                    font_handles[&FontKey::Default].clone(),
                                    description_socket_material.clone(),
                                );

                                let hotkey =
                                    map_socket_color_hotkey(hotbar_first_position_socket_color);

                                hotbar_wrapper_children
                                    .hotbar(vec![hotbar_first_position_socket_color])
                                    .with_children(|hotbar_children| {
                                        hotbar_children.hotbar_button(
                                            button_socket_material.clone(),
                                            format!("{}.", hotkey),
                                            hotkey - 1,
                                        ); // someday we will have real hotkeys
                                    });
                            });
                    });
            });
        });

    commands.trigger(NewShop {
        parent: gameplay_wrapper,
    });
}

fn exit_playing(mut commands: Commands) {
    // We could use [`StateScoped`] on the sound playing entites instead.
    commands.trigger(PlaySoundtrack::Disable);
}

fn return_to_title_screen(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}

#[derive(Component)]
pub struct CycleDisplayPanel;

fn ring_cycle_display(
    mut commands: Commands,
    mut q_ring: Query<(Entity, &mut Ring)>,
    mut q_cycle_display: Query<(Entity, &mut Transform), With<CycleDisplayPanel>>,
    gameplay_meshes: Res<GameplayMeshes>,
    mut socket_materials: ResMut<Assets<SocketMaterial>>,
) {
    let row_size = 8;
    let quad_size = 32.;
    for (ring_entity, mut ring) in q_ring.iter_mut() {
        if ring.cycle.len() > ring.cycle_display_panels.len() {
            // reposition existing panels
            for (i, panel_entity) in ring.cycle_display_panels.iter().enumerate() {
                let (_e, mut panel_transform) = q_cycle_display.get_mut(*panel_entity).unwrap();
                panel_transform.translation =
                    ring_cycle_display_panel_position(i, row_size, quad_size, ring.cycle.len())
                        .extend(1.);
            }

            // spawn new panels
            let start_index = ring.cycle_display_panels.len();
            let end_index = ring.cycle.len();

            for i in start_index..end_index {
                let socket_color = ring.cycle[i];

                commands.entity(ring_entity).with_children(|ring_children| {
                    let cycle_display_entity = ring_children
                        .spawn((
                            MaterialMesh2dBundle {
                                mesh: gameplay_meshes.quad32.clone(),
                                material: socket_materials.add(SocketMaterial {
                                    inserted_color: map_socket_color(socket_color),
                                    highlight_color: map_socket_highlight_color(socket_color),
                                    bevel_color: { BLACK.into() },
                                    data: Vec4::new(
                                        -1.,
                                        map_socket_color_trigger_duration(socket_color),
                                        0.,
                                        0.,
                                    ),
                                }),
                                transform: Transform::from_translation(
                                    ring_cycle_display_panel_position(
                                        i,
                                        row_size,
                                        quad_size,
                                        ring.cycle.len(),
                                    )
                                    .extend(1.),
                                ),
                                ..default()
                            },
                            CycleDisplayPanel,
                        ))
                        .id();

                    ring.cycle_display_panels.push(cycle_display_entity);
                });
            }
        }
    }
}

fn ring_cycle_display_panel_position(
    index: usize,
    row_size: u32,
    quad_size: f32,
    orb_count: usize,
) -> Vec2 {
    let orb_count = orb_count as u32;
    let index = index as u32;

    let current_row_size = if index < (orb_count - (orb_count % row_size)) {
        row_size
    } else {
        if orb_count % row_size == 0 {
            row_size
        } else {
            orb_count % row_size
        }
    };

    let total_rows = (orb_count + row_size - 1) / row_size;
    let center_x = (current_row_size as f32 * quad_size) / 2.0;

    let total_height = total_rows as f32 * quad_size;
    let center_y = total_height / 2.0;

    let row = index / row_size;
    let col = index % row_size;

    let x = col as f32 * quad_size - center_x + quad_size / 2.0;
    let y = (row as f32 * quad_size - center_y + quad_size / 2.0) * -1.0;

    Vec2::new(x, y)
}

#[derive(Event)]
pub struct SocketTriggered {
    socket: Entity,
    ring: Entity,
}

#[derive(Event)]
pub struct CycleComplete {
    ring: Entity,
    new_cycle_start_seconds: f32,
}

pub enum SocketEffect {
    ReduceCooldown(f32),
}

fn on_socket_triggered(
    trigger: Trigger<SocketTriggered>,
    mut commands: Commands,
    mut materials: ResMut<Assets<SocketMaterial>>,
    mut q_socket: Query<(Entity, &mut Socket, &Handle<SocketMaterial>, &Transform)>,
    mut q_ring: Query<(Entity, &mut Ring)>,
    mut currency: ResMut<Currency>,
    font_handles: ResMut<HandleMap<FontKey>>,
    time: Res<Time>,
) {
    let (ring_entity, mut ring) = q_ring
        .get_mut(trigger.event().ring)
        .expect("SocketTriggered.ring should've referenced an Entity with a Ring component.");

    let mut pending_socket_effects: Vec<SocketEffect> = vec![];

    // First block, mutate the triggered socket
    {
        let (socket_entity, mut socket, socket_mat_handle, socket_transform) =
            q_socket.get_mut(trigger.event().socket).expect(
                "SocketTriggered.socket should've referenced an Entity with a Socket component.",
            );

        let triggered_successfully = socket.color != SocketColor::NONE
            && (socket.last_triggered_time_seconds + socket.trigger_duration_seconds
                < time.elapsed_seconds());

        if triggered_successfully {
            let old_score = ring.cycle_score.clone();

            ring.cycle.push(socket.color);

            match socket.color {
                SocketColor::BLUE => {
                    let score_gained = BigUint::from(1u32);
                    ring.cycle_score += score_gained;
                }
                SocketColor::RED => {
                    let num_sockets = ring.sockets.len();
                    let prev_index = (socket.index + num_sockets - 1) % num_sockets;
                    let next_index = (socket.index + 1) % num_sockets;
                    commands.trigger(SocketTriggered {
                        socket: ring.sockets[prev_index],
                        ring: ring_entity,
                    });
                    commands.trigger(SocketTriggered {
                        socket: ring.sockets[next_index],
                        ring: ring_entity,
                    });
                }
                SocketColor::GREEN => {
                    let score_gained = ring.previous_cycle.len();
                    ring.cycle_score += score_gained;
                }
                SocketColor::ORANGE => {
                    pending_socket_effects.push(SocketEffect::ReduceCooldown(1.));
                },
                SocketColor::NONE => panic!("Shouldn't get points for an empty socket."),
            }

            currency.pending_amount = ring.cycle_score.clone();

            let socket_material = materials.get_mut(socket_mat_handle).unwrap();
            socket_material.data[0] = time.elapsed_seconds();
            socket.last_triggered_time_seconds = time.elapsed_seconds();

            let score_diff = &ring.cycle_score - old_score;

            if score_diff != BigUint::ZERO {
                let tween_seconds = 1;
                let text_start_position = (socket_transform.translation.xy()).extend(100.);
                let tween = Tween::new(
                    EaseFunction::QuadraticIn,
                    Duration::from_secs(tween_seconds),
                    TransformPositionLens {
                        start: text_start_position,
                        end: text_start_position + Vec3::new(0., 100., 0.),
                    },
                );

                let text_style = TextStyle {
                    font: font_handles[&FontKey::Default].clone(),
                    font_size: 20.,
                    ..default()
                };

                commands.spawn((
                    Text2dBundle {
                        text: Text::from_section(format!("+${}", score_diff), text_style.clone())
                            .with_justify(JustifyText::Center),
                        transform: Transform::from_translation(text_start_position),
                        ..default()
                    },
                    Animator::new(tween),
                    DespawnAfter {
                        lifetime_seconds: 1.,
                        spawn_time: time.elapsed_seconds(),
                    },
                ));
            }
        }
    }

    // Second block, mutate the other sockets
    {
        for socket_entity in &ring.sockets {

            // the triggered socket can't apply an effect to itself
            if *socket_entity == trigger.event().socket {
                continue;
            }

            let (_, mut socket, socket_mat_handle, _) = q_socket.get_mut(*socket_entity).expect("Socket in sockets array pls");
            for effect in &pending_socket_effects { // note, right now, effects apply to every socket, later we might need to filter them
                match effect {
                    SocketEffect::ReduceCooldown(amount) => {
                        
                        if socket.last_triggered_time_seconds > 0. {
                            let socket_material = materials.get_mut(socket_mat_handle).unwrap();
    
                            socket.last_triggered_time_seconds -= amount;
                            socket_material.data[0] -= amount;
    
                        }

                        
                    }
                }
            }
        }
    }
}

#[derive(Clone)]
pub enum CycleBonus {
    Overflow(BigUint),
}

fn calculate_cycle_bonuses(ring: &Ring) -> Vec<CycleBonus> {
    let mut result = vec![];

    // calculate oversized
    if ring.cycle.len() > ring.sockets.len() {
        result.push(CycleBonus::Overflow(BigUint::from(
            ring.cycle.len() - ring.sockets.len(),
        )))
    }

    return result;
}

fn score_bonus(bonus: &CycleBonus) -> BigUint {
    match bonus {
        CycleBonus::Overflow(size) => size.clone(),
    }
}

fn on_cycle_complete(
    trigger: Trigger<CycleComplete>,
    mut commands: Commands,
    mut q_ring: Query<(&mut Ring, &Transform)>,
    mut currency: ResMut<Currency>,
    font_handles: ResMut<HandleMap<FontKey>>,
    time: Res<Time>,
) {
    let (mut ring, ring_transform) = q_ring
        .get_mut(trigger.event().ring)
        .expect("CycleComplete event referenced ring that doesn't exist.");

    let bonuses = calculate_cycle_bonuses(&ring);

    let bonus_score = bonuses
        .iter()
        .fold(BigUint::ZERO, |acc, bonus| acc + score_bonus(bonus));

    let cycle_score = ring.cycle_score.clone() + bonus_score;

    ring.cycle_count += BigUint::from(1u32);
    ring.previous_bonuses = bonuses.clone();
    ring.previous_cycle = ring.cycle.clone();

    currency.amount += &cycle_score;

    ring.cycle = Vec::new();
    ring.cycle_score = BigUint::ZERO;
    ring.cycle_start_seconds = trigger.event().new_cycle_start_seconds;

    // reset the cycle display
    for cycle_display_entity in &ring.cycle_display_panels {
        commands.entity(*cycle_display_entity).despawn_recursive();
    }

    ring.cycle_display_panels = vec![];

    // display the change in $ if it was positive
    if cycle_score > BigUint::ZERO {
        spawn_scrolling_text(
            &mut commands,
            format!("+${}", cycle_score),
            (ring_transform.translation.xy()).extend(100.) + Vec3::Y * 50.,
            2.,
            200.,
            TextScrollDirection::UP,
            LIGHT_GREEN.into(),
            time.elapsed_seconds(),
            font_handles[&FontKey::Default].clone(),
            36.,
        );
    }

    // display the bonuses, if any
    for (index, bonus) in bonuses.iter().enumerate() {
        spawn_scrolling_text(
            &mut commands,
            bonus_text(bonus),
            (ring_transform.translation.xy()).extend(100.)
                + Vec3::Y * (50 - 30 * (index + 1)) as f32,
            2.,
            200.,
            TextScrollDirection::UP,
            bonus_color(bonus),
            time.elapsed_seconds(),
            font_handles[&FontKey::Default].clone(),
            16.,
        );
    }
}

fn bonus_text(bonus: &CycleBonus) -> String {
    match bonus {
        CycleBonus::Overflow(oversized_amount) => {
            format!("Overflow ({}) +${}", oversized_amount, score_bonus(bonus))
        }
    }
}

fn bonus_color(bonus: &CycleBonus) -> LinearRgba {
    match bonus {
        CycleBonus::Overflow(_) => PINK.into(),
    }
}

pub enum TextScrollDirection {
    UP,
    DOWN,
}

fn spawn_scrolling_text(
    commands: &mut Commands,
    text: impl Into<String>,
    start_position: Vec3,
    duration_seconds: f32,
    distance: f32,
    direction: TextScrollDirection,
    color: LinearRgba,
    current_time: f32,
    font: Handle<Font>, // is it idiomatic to make the caller clone it? lol
    font_size: f32,
) {
    let signed_distance_y = distance
        * match direction {
            TextScrollDirection::DOWN => -1.,
            _ => 1.,
        };

    let tween = Tween::new(
        EaseFunction::QuadraticIn,
        Duration::from_secs(duration_seconds.round() as u64),
        TransformPositionLens {
            start: start_position,
            end: start_position + Vec3::new(0., signed_distance_y, 100.),
        },
    );

    let text_style = TextStyle {
        font,
        font_size,
        color: color.into(),
        ..default()
    };

    commands.spawn((
        Text2dBundle {
            text: Text::from_section(text, text_style.clone()).with_justify(JustifyText::Center),
            transform: Transform::from_translation(start_position),
            ..default()
        },
        Animator::new(tween),
        DespawnAfter {
            lifetime_seconds: duration_seconds,
            spawn_time: current_time,
        },
    ));
}
fn progress_cycle(
    mut commands: Commands,
    q_socket: Query<(Entity, &Socket, &Transform)>,
    q_ring: Query<(Entity, &mut Ring, &Handle<RingMaterial>)>,
    time: Res<Time>,
    mut old_progress_pct: Local<f32>,
    mut ring_materials: ResMut<Assets<RingMaterial>>,
) {
    for (ring_entity, ring, ring_mat_handle) in &q_ring {
        let seconds_since_cycle_start = time.elapsed_seconds() - ring.cycle_start_seconds;
        let cycle_time_remaining = ring.cycle_duration - seconds_since_cycle_start;

        if cycle_time_remaining < 0. {
            let new_cycle_start_seconds = time.elapsed_seconds() + cycle_time_remaining;
            commands.trigger(CycleComplete {
                ring: ring_entity,
                new_cycle_start_seconds,
            });
        }

        let progress_pct = 1. - cycle_time_remaining / ring.cycle_duration;

        let ring_mat = ring_materials
            .get_mut(ring_mat_handle)
            .expect("Ring should've had a RingMaterial.");
        ring_mat.data[2] = progress_pct;

        for socket_entity in &ring.sockets {
            let (socket_entity, socket, _t) = q_socket
                .get(*socket_entity)
                .expect("Ring's socket Vec contained Entity that was not Socket!");
            let socket_position_pct =
                (ring.sockets.len() as f32 - socket.index as f32) / ring.sockets.len() as f32;

            if (*old_progress_pct <= socket_position_pct && socket_position_pct <= progress_pct)
                || (socket_position_pct == 1. && *old_progress_pct > progress_pct)
            {
                commands.trigger(SocketTriggered {
                    socket: socket_entity,
                    ring: ring_entity,
                });
            }
        }

        *old_progress_pct = progress_pct;
    }
}

#[derive(Component, Default)]
struct DespawnAfter {
    lifetime_seconds: f32,
    spawn_time: f32,
}

fn despawn_after_system(
    mut commands: Commands,
    time: Res<Time>,
    query: Query<(Entity, &DespawnAfter)>,
) {
    for (entity, despawn_after) in query.iter() {
        let elapsed_time = time.elapsed_seconds() - despawn_after.spawn_time;
        if elapsed_time >= despawn_after.lifetime_seconds {
            commands.entity(entity).despawn_recursive();
        }
    }
}
