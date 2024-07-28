use bevy::{
    color::palettes::{css::WHITE, tailwind::{GRAY_600, GRAY_700}},
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_mod_picking::{
    events::{Drag, Pointer},
    picking_core::Pickable,
    prelude::On,
};

use super::Screen;
use crate::{
    game::{
        assets::{FontKey, HandleMap}, materials::materials::{BackgroundMaterial, RingMaterial}, spawn::level::{spawn_ring, Ring, Socket, RING_RADIUS, RING_THICKNESS}
    },
    ui::prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Title), enter_title);
    app.add_systems(OnExit(Screen::Title), exit_title);

    app.register_type::<TitleAction>();
    app.add_systems(Update, (handle_title_action, cycle_title_ring).run_if(in_state(Screen::Title)));
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Component)]
enum TitleAction {
    Play,
    Credits,
    /// Exit doesn't work well with embedded applications.
    #[cfg(not(target_family = "wasm"))]
    Exit,
}

#[derive(Component)]
pub struct Background;

fn enter_title(
    mut commands: Commands,
    mut ring_materials: ResMut<Assets<RingMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<BackgroundMaterial>>,
    font_handles: ResMut<HandleMap<FontKey>>,
    time: Res<Time>,
) {
    commands
        .ui_root()
        .insert(StateScoped(Screen::Title))
        .with_children(|children| {
            children.spawn((
                Name::new("Title VerticalBox"),
                NodeBundle {
                    style: Style {
                        padding: UiRect::all(Val::Px(8.)),
                        column_gap: Val::Px(8.),
                        display: Display::Flex,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                },
            )).with_children(|vertical| {
                vertical.spawn((
                    Name::new("Title Text"),
                    TextBundle::from_section(
                        String::from("every\nfew\nseconds"),
                        TextStyle {
                            font: font_handles[&FontKey::Default].clone(),
                            font_size: 30.,
                            color: WHITE.into(),
                            ..default()
                        },
                    ),
                ));

                vertical.spawn((
                    Name::new("Title HorizontalBox"),
                    NodeBundle {
                        style: Style {
                            padding: UiRect::all(Val::Px(8.)),
                            row_gap: Val::Px(16.),
                            display: Display::Flex,
                            flex_direction: FlexDirection::Column,
                            ..default()
                        },
                        ..default()
                    },
                )).with_children(|horizontal| {
                    horizontal.button("Play", font_handles[&FontKey::Default].clone()).insert(TitleAction::Play);
    
                    #[cfg(not(target_family = "wasm"))]
                    horizontal.button("Exit", font_handles[&FontKey::Default].clone()).insert(TitleAction::Exit);
                });
            });


        });

    commands.spawn((
        Background,
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(100000.0, 100000.0))),
            material: materials.add(BackgroundMaterial {
                base_color: GRAY_600.into(),
                blend_color: GRAY_700.into(),
            }),
            transform: Transform::from_xyz(0., 0., -999.),
            ..default()
        },
    ));

    let ring_entity = commands
        .spawn((
            Ring {
                // dummy ring, we replace it at the end
                ..default()
            },
            Pickable {
                should_block_lower: false,
                is_hoverable: false,
            },
            DespawnAfterTitle
        ))
        .id();

    commands.entity(ring_entity).insert(MaterialMesh2dBundle {
        mesh: Mesh2dHandle(meshes.add(Rectangle::new(512.0, 512.0))),
        material: ring_materials.add(RingMaterial {
            data: Vec4::new(RING_RADIUS, RING_THICKNESS, 0., 0.),
        }),
        transform: Transform::from_translation(Vec3::splat(0.)),
        ..default()
    });

    commands.entity(ring_entity).insert(Ring {
        sockets: vec![],
        cycle_duration: 4.,
        cycle_start_seconds: time.elapsed_seconds(),
        cycle_multiplier: 1.,
        index: 0,
        ..default()
    });
}

fn cycle_title_ring(
    mut commands: Commands,
    q_socket: Query<(Entity, &Socket, &Transform)>,
    mut q_ring: Query<(Entity, &mut Ring, &Handle<RingMaterial>)>,
    time: Res<Time>,
    mut old_progress_pcts: Local<Vec<f32>>,
    mut ring_materials: ResMut<Assets<RingMaterial>>,
) {
    for (ring_entity, mut ring, ring_mat_handle) in q_ring.iter_mut() {
        let seconds_since_cycle_start = time.elapsed_seconds() - ring.cycle_start_seconds;
        let cycle_time_remaining = ring.cycle_duration - seconds_since_cycle_start;

        if cycle_time_remaining < 0. {
            ring.cycle_start_seconds = time.elapsed_seconds() + cycle_time_remaining;
        }

        let progress_pct = 1. - cycle_time_remaining / ring.cycle_duration;

        let ring_mat = ring_materials
            .get_mut(ring_mat_handle)
            .expect("Ring should've had a RingMaterial.");

        ring_mat.data[2] = progress_pct;
        
    }
}

#[derive(Component)]
struct DespawnAfterTitle;

fn exit_title(
    mut commands: Commands,
    q_depsawn_me: Query<Entity, With<DespawnAfterTitle>>
) {
    for entity in &q_depsawn_me {
        commands.entity(entity).despawn_recursive();
    }
}

fn handle_title_action(
    mut next_screen: ResMut<NextState<Screen>>,
    mut button_query: InteractionQuery<&TitleAction>,
    #[cfg(not(target_family = "wasm"))] mut app_exit: EventWriter<AppExit>,
) {
    for (interaction, action) in &mut button_query {
        if matches!(interaction, Interaction::Pressed) {
            match action {
                TitleAction::Play => next_screen.set(Screen::Playing),
                TitleAction::Credits => next_screen.set(Screen::Credits),

                #[cfg(not(target_family = "wasm"))]
                TitleAction::Exit => {
                    app_exit.send(AppExit::Success);
                }
            }
        }
    }
}
