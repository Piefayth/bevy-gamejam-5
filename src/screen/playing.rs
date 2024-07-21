//! The screen state for the main game loop.

use std::{f32::consts::PI, time::Duration};

use bevy::{
    color::palettes::css::{BLACK, BLUE, FOREST_GREEN, GREEN, LIGHT_GREEN, WHITE},
    input::common_conditions::input_just_pressed,
    prelude::*,
};
use bevy_tweening::{
    lens::{TransformPositionLens, TransformScaleLens},
    Animator, EaseFunction, Tween,
};
use num_bigint::BigUint;

use super::Screen;
use crate::{
    game::{
        assets::{FontKey, HandleMap, SoundtrackKey},
        audio::soundtrack::PlaySoundtrack,
        materials::materials::{HandMaterial, SocketMaterial, SocketUiMaterial},
        spawn::level::{Hand, Ring, Socket, SocketColor, SpawnLevel},
    },
    ui::shop::NewShop,
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
                (rotate_hands, trigger_sockets).chain(),
                despawn_after_system,
            )
                .run_if(in_state(Screen::Playing)),
        ),
    );

    app.init_resource::<Currency>();
}

fn enter_playing(
    mut commands: Commands,
    mut materials: ResMut<Assets<SocketUiMaterial>>,
    font_handles: ResMut<HandleMap<FontKey>>,
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
                        score_and_hotbar_wrapper
                            .hotbar(vec![SocketColor::BLUE])
                            .with_children(|hotbar_children| {
                                hotbar_children.hotbar_button(materials.add(SocketUiMaterial {
                                    bevel_color: BLACK.into(),
                                    inserted_color: BLUE.into(),
                                }));
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

fn rotate_hands(
    mut q_hand: Query<(&Handle<HandMaterial>, &mut Hand)>,
    mut materials: ResMut<Assets<HandMaterial>>,
    time: Res<Time>,
) {
    for (hand_mat_handle, mut hand) in q_hand.iter_mut() {
        let hand_material = materials.get_mut(hand_mat_handle).unwrap();

        //let rotations_per_second = 1. / 5.; // this will change per hand...
        let rotations_per_second = 1.;
        let rotations_per_second_rad = 2.0 * PI * rotations_per_second;

        hand_material.rotation_radians += rotations_per_second_rad * time.delta_seconds();
        hand_material.rotation_radians = hand_material.rotation_radians % (2.0 * PI);
        hand.rotation_radians = hand_material.rotation_radians;
    }
}

fn trigger_sockets(
    mut commands: Commands,
    mut q_socket: Query<(&mut Socket, &Handle<SocketMaterial>, &Transform)>,
    q_hand: Query<(&Hand, &Transform)>,
    mut q_ring: Query<(&mut Ring, &Transform)>,
    mut materials: ResMut<Assets<SocketMaterial>>,
    mut currency: ResMut<Currency>,
    font_handles: ResMut<HandleMap<FontKey>>,
    time: Res<Time>,
) {
    for (mut socket, socket_mat_handle, socket_transform) in q_socket.iter_mut() {
        let (hand, hand_transform) = q_hand.get(socket.hand).unwrap(); // we enforce socket always has a hand, because without the hand we can't trigger
        let (mut ring, ring_transform) = q_ring.get_mut(socket.ring).unwrap(); // also ring

        let unrotated_hand_endpoint = Vec2::new(
            hand_transform.translation.x,
            hand_transform.translation.y + (0.5 * 512.), // magic number, hand length percentage times the size of the quad its own
        );

        let trigger_target = rotate_around_point_2d(
            unrotated_hand_endpoint,
            hand_transform.translation.xy(),
            Quat::from_rotation_z(hand.rotation_radians),
        );

        if is_point_in_circle(
            trigger_target,
            socket_transform.translation.xy(),
            socket.radius,
        ) {
            let did_trigger_a_socket =
                !socket.triggered && (socket.color != SocketColor::NONE || socket.index == 0);
            if did_trigger_a_socket {
                socket.triggered = true;
                let socket_material = materials.get_mut(socket_mat_handle).unwrap();

                if socket.index == 0 {
                    ring.cycle_count += BigUint::from(1u32);

                    if ring.cycle.len() > 0 {
                        currency.amount += &ring.cycle_score;

                        let tween_seconds = 2;
                        let text_start_position =
                            (ring_transform.translation.xy() + Vec2::Y * 100.).extend(100.);
                        let tween = Tween::new(
                            EaseFunction::QuadraticIn,
                            Duration::from_secs(tween_seconds),
                            TransformPositionLens {
                                start: text_start_position,
                                end: text_start_position + Vec3::new(0., 500., 0.),
                            },
                        );

                        let text_style = TextStyle {
                            font: font_handles[&FontKey::Default].clone(),
                            font_size: 36.,
                            color: LIGHT_GREEN.into(),
                            ..default()
                        };

                        commands.spawn((
                            Text2dBundle {
                                text: Text::from_section(
                                    format!("${}", &ring.cycle_score),
                                    text_style.clone(),
                                )
                                .with_justify(JustifyText::Center),
                                transform: Transform::from_translation(text_start_position),
                                ..default()
                            },
                            Animator::new(tween),
                            DespawnAfter {
                                lifetime_seconds: 2.,
                                spawn_time: time.elapsed_seconds(),
                            },
                        ));
                    }
                    ring.cycle = Vec::new();
                    ring.cycle_score = BigUint::ZERO;
                }

                let old_score = ring.cycle_score.clone();

                if socket.color != SocketColor::NONE {
                    // add a socket's contents for the cycle and score the ring
                    socket_material.bevel_color = WHITE.into();
                    ring.cycle.push(socket.color);
                    ring.score();
                    currency.pending_amount = ring.cycle_score.clone();

                    let score_diff = &ring.cycle_score - old_score;

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
                            text: Text::from_section(
                                format!("+{}", score_diff),
                                text_style.clone(),
                            )
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
        } else if socket.triggered {
            socket.triggered = false;

            let socket_material = materials.get_mut(socket_mat_handle).unwrap();
            socket_material.bevel_color = BLACK.into();
        }
    }
}

fn is_point_in_circle(point: Vec2, center: Vec2, radius: f32) -> bool {
    let distance_squared = point.distance_squared(center);
    let radius_squared = radius * radius;

    distance_squared <= radius_squared
}

fn rotate_around_point_2d(point: Vec2, pivot: Vec2, rotation: Quat) -> Vec2 {
    let translated_point = Vec3::new(point.x - pivot.x, point.y - pivot.y, 0.0);
    let rotated_point = rotation * translated_point;
    -Vec2::new(rotated_point.y + pivot.y, rotated_point.x + pivot.x)
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
