//! Spawn the main level by triggering other observers.

use std::f32::consts::PI;

use bevy::{
    color::palettes::{
        css::{BLACK, BLUE, DARK_SLATE_GRAY},
        tailwind::{GRAY_900, GRAY_950, SLATE_800, SLATE_900},
    },
    math::VectorSpace,
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_mod_picking::{
    events::{Click, Pointer},
    picking_core::Pickable,
    pointer::PointerButton,
    prelude::{ListenerInput, On},
};
use num_bigint::BigUint;

use crate::{
    game::materials::materials::{HandMaterial, RingMaterial, SocketMaterial},
    ui::widgets::{CycleDisplay, Hotbar},
};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_level);
    app.observe(on_set_socket_color);
    app.init_resource::<GameplayMeshes>();
    app.add_systems(Startup, prepare_meshes);
}

#[derive(Resource, Default)]
pub struct GameplayMeshes {
    pub quad512: Mesh2dHandle,
    pub quad64: Mesh2dHandle,
}

fn prepare_meshes(
    mut meshes: ResMut<Assets<Mesh>>,
    mut gamemplay_meshes: ResMut<GameplayMeshes>,
) {
    gamemplay_meshes.quad512 = Mesh2dHandle(meshes.add(Rectangle::from_size(Vec2::splat(512.))));
    gamemplay_meshes.quad64 = Mesh2dHandle(meshes.add(Rectangle::from_size(Vec2::splat(64.))));
}

#[derive(Event, Debug)]
pub struct SpawnLevel;

#[derive(Component, Default)]
pub struct Ring {
    pub cycle: Vec<SocketColor>,
    pub cycle_score: BigUint,
    pub cycle_count: BigUint,
    pub sockets: Vec<Entity>,
    pub hands: Vec<Entity>,
}

impl Ring {
    pub fn score(&mut self) {
        self.cycle_score = BigUint::ZERO;

        for color in &self.cycle {
            self.cycle_score += match color {
                SocketColor::NONE => BigUint::ZERO,
                SocketColor::BLUE => BigUint::from(1u32),
            };
        }
    }
}

#[derive(Component, Default)]
pub struct Hand {
    pub rotation_radians: f32,
    pub length: f32,
}

#[derive(Default, PartialEq, Clone, Copy, Debug)]
pub enum SocketColor {
    #[default]
    NONE,
    BLUE,
}

const RING_QUAD_DIMENSIONS: Vec2 = Vec2::splat(512.);
const RING_RADIUS: f32 = 1. - 0.005;
const RING_THICKNESS: f32 = 0.05;
const DEFAULT_SOCKET_RADIUS: f32 = 32.;

#[derive(Component)]
pub struct Socket {
    pub color: SocketColor,
    pub radius: f32,
    pub hand: Entity,
    pub ring: Entity,
    pub triggered: bool,
    pub index: usize,
}

fn spawn_level(
    _trigger: Trigger<SpawnLevel>,
    mut commands: Commands,
    mut ring_materials: ResMut<Assets<RingMaterial>>,
    mut hand_materials: ResMut<Assets<HandMaterial>>,
    mut socket_materials: ResMut<Assets<SocketMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    gameplay_meshes: Res<GameplayMeshes>,
    q_ui_cycle_display: Query<Entity, With<CycleDisplay>>,
) {
    // this is being triggered from the playing screen

    // spawn a quad and lets put a shader on it


    let ring_entity = commands
        .spawn((
            Ring { ..default() },
            MaterialMesh2dBundle {
                mesh: gameplay_meshes.quad512.clone(),
                material: ring_materials.add(RingMaterial {
                    radius: RING_RADIUS,
                    thickness: RING_THICKNESS,
                }),
                ..default()
            },
            Pickable {
                should_block_lower: false,
                is_hoverable: false,
            },
        ))
        .id();

    let mut starting_sockets: Vec<Entity> = vec![];
    let mut hand_entity: Entity = Entity::PLACEHOLDER;

    commands
        .entity(ring_entity)
        .with_children(|ring_entity_children| {
            let hand_thickness = 10. / 512.;
            let hand_length = 0.5;

            hand_entity = ring_entity_children
                .spawn((
                    Name::new("Hand"),
                    Hand {
                        length: hand_length,
                        ..default()
                    },
                    MaterialMesh2dBundle {
                        mesh: gameplay_meshes.quad512.clone(),
                        material: hand_materials.add(HandMaterial {
                            width: hand_length,
                            height: hand_thickness,
                            rotation_radians: PI / 2.,
                        }),
                        transform: Transform::from_xyz(0., 0., 5.),
                        ..default()
                    },
                    Pickable {
                        should_block_lower: false,
                        is_hoverable: false,
                    },
                ))
                .id();

            let num_points = 2; // how many sockets to start with

            for i in 0..num_points {
                let socket_color = if i == 0 {
                    SocketColor::BLUE
                } else {
                    SocketColor::NONE
                };

                let socket_entity = spawn_socket(
                    ring_entity_children,
                    socket_color.clone(),
                    hand_entity,
                    ring_entity,
                    i,
                    gameplay_meshes.quad64.clone(),
                    socket_materials.add(SocketMaterial {
                        inserted_color: map_socket_color(socket_color),
                        bevel_color: { BLACK.into() },
                    }),
                    socket_position(i, num_points).extend(1.),
                );

                starting_sockets.push(socket_entity);
            }
        });

        commands.entity(ring_entity).insert(Ring {
            sockets: starting_sockets,
            hands: vec![hand_entity],
            ..default()
        });
}

pub fn socket_position(
    index: usize,
    num_points: usize,
) -> Vec2 {
    let angle = 2.0 * PI * index as f32 / num_points as f32 + PI / 2.0;
    let ring_center_radius = RING_RADIUS - RING_THICKNESS / 2.0;
    let x = 0.5 * ring_center_radius * RING_QUAD_DIMENSIONS.x * angle.cos();
    let y = 0.5 * ring_center_radius * RING_QUAD_DIMENSIONS.y * angle.sin();

    Vec2 { x, y }
}

pub fn spawn_socket(
    mut commands: &mut ChildBuilder,
    color: SocketColor,
    hand: Entity,
    ring: Entity,
    index: usize,
    mesh: Mesh2dHandle,
    material: Handle<SocketMaterial>,
    translation: Vec3,
) -> Entity {
    commands
        .spawn((
            Name::new("Socket"),
            Socket {
                color,
                hand,
                ring,
                triggered: false,
                index,
                radius: DEFAULT_SOCKET_RADIUS,
            },
            MaterialMesh2dBundle {
                mesh,
                material,
                transform: Transform::from_translation(translation),
                ..default()
            },
        ))
        .insert(On::<Pointer<Click>>::commands_mut(|ev, mut commands| {
            if ev.event.button == PointerButton::Primary {
                commands.trigger(UpdateSocketColor {
                    socket: ev.target,
                    remove: false,
                });
            } else if ev.event.button == PointerButton::Secondary {
                commands.trigger(UpdateSocketColor {
                    socket: ev.target,
                    remove: true,
                });
            }
        }))
        .id()
}

pub fn map_socket_color(socket_color: SocketColor) -> LinearRgba {
    match socket_color {
        SocketColor::BLUE => BLUE,
        SocketColor::NONE => GRAY_950,
    }
    .into()
}

#[derive(Event)]
struct UpdateSocketColor {
    socket: Entity,
    remove: bool,
}

fn on_set_socket_color(
    trigger: Trigger<UpdateSocketColor>,
    mut q_sockets: Query<(&mut Socket, &Handle<SocketMaterial>)>,
    mut materials: ResMut<Assets<SocketMaterial>>,
    q_hotbar: Query<&Hotbar>,
) {
    let hotbar = q_hotbar.single();
    let selected_color = hotbar.color_mappings[hotbar.selected_index as usize];

    let (mut socket, socket_material_handle) = q_sockets.get_mut(trigger.event().socket).unwrap();
    let material = materials.get_mut(socket_material_handle).unwrap();

    let new_color = if trigger.event().remove {
        SocketColor::NONE
    } else {
        selected_color
    };

    material.inserted_color = map_socket_color(new_color);

    socket.color = new_color;
}
