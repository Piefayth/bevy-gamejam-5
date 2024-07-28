//! Spawn the main level by triggering other observers.

use std::f32::consts::PI;

use bevy::{
    color::palettes::{
        css::{BLACK, BLUE, RED, WHITE},
        tailwind::{
            BLUE_400, BLUE_600, GRAY_900, GRAY_950, GREEN_400, GREEN_600, ORANGE_400, ORANGE_600,
            PINK_400, PINK_600, RED_400, RED_600,
        },
    }, ecs::system::EntityCommands, math::VectorSpace, prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}, utils::hashbrown::HashMap
};
use bevy_mod_picking::{
    events::{Click, Pointer},
    picking_core::Pickable,
    pointer::PointerButton,
    prelude::On,
};
use num_bigint::BigUint;

use crate::{
    game::materials::materials::{RingMaterial, SocketMaterial},
    screen::playing::{Currency, CycleBonus},
    ui::widgets::Hotbar,
};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_level);
    app.observe(on_set_socket_color);
    app.init_resource::<GameplayMeshes>();
    app.init_resource::<RingIndex>();
    app.add_systems(Startup, prepare_meshes);
}

#[derive(Resource, Default)]
pub struct GameplayMeshes {
    pub quad512: Mesh2dHandle,
    pub quad64: Mesh2dHandle,
    pub quad32: Mesh2dHandle,
    pub quad16: Mesh2dHandle,
}

fn prepare_meshes(mut meshes: ResMut<Assets<Mesh>>, mut gameplay_meshes: ResMut<GameplayMeshes>) {
    gameplay_meshes.quad512 = Mesh2dHandle(meshes.add(Rectangle::from_size(Vec2::splat(512.))));
    gameplay_meshes.quad64 = Mesh2dHandle(meshes.add(Rectangle::from_size(Vec2::splat(64.))));
    gameplay_meshes.quad32 = Mesh2dHandle(meshes.add(Rectangle::from_size(Vec2::splat(32.))));
    gameplay_meshes.quad16 = Mesh2dHandle(meshes.add(Rectangle::from_size(Vec2::splat(16.))));
}

#[derive(Event, Debug)]
pub struct SpawnLevel;

#[derive(Component, Default)]
pub struct Ring {
    pub cycle: Vec<SocketColor>,
    pub previous_cycle: Vec<SocketColor>,
    pub previous_bonuses: Vec<CycleBonus>,
    pub pending_amount: BigUint,
    pub cycle_start_seconds: f32,
    pub cycle_duration: f32,
    pub cycle_score: BigUint,
    pub cycle_count: BigUint,
    pub cycle_multiplier: f32,
    pub sockets: Vec<Entity>,
    pub cycle_display_panels: Vec<Entity>,
    pub index: usize,
}

#[derive(Default, PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum SocketColor {
    #[default]
    NONE,
    BLUE,
    RED,
    GREEN,
    ORANGE,
    PINK,
}

impl SocketColor {
    pub fn as_str(&self) -> &'static str {
        match self {
            SocketColor::RED => "RED",
            SocketColor::NONE => "NONE",
            SocketColor::BLUE => "BLUE",
            SocketColor::GREEN => "GREEN",
            SocketColor::ORANGE => "ORANGE",
            SocketColor::PINK => "PINK",
        }
    }
}

pub const RING_QUAD_DIMENSIONS: Vec2 = Vec2::splat(512.);
pub const RING_RADIUS: f32 = 1. - 0.005;
pub const RING_THICKNESS: f32 = 0.05;
const DEFAULT_SOCKET_RADIUS: f32 = 32.;

#[derive(Component)]
pub struct Socket {
    pub color: SocketColor,
    pub radius: f32,
    pub ring: Entity,
    pub index: usize,
    pub last_triggered_time_seconds: f32,
    pub trigger_duration_seconds: f32,
}

#[derive(Resource, Default)]
pub struct RingIndex {
    pub rings: HashMap<IVec2, Entity>
}

fn spawn_level(
    _trigger: Trigger<SpawnLevel>,
    mut commands: Commands,
    mut ring_index: ResMut<RingIndex>,
    mut ring_materials: ResMut<Assets<RingMaterial>>,
    mut socket_materials: ResMut<Assets<SocketMaterial>>,
    gameplay_meshes: Res<GameplayMeshes>,
    mut currency: ResMut<Currency>,
    time: Res<Time>,
) {
    if cfg!(feature = "dev") {
        let large_number_str = "500000";
        let large_number = BigUint::parse_bytes(large_number_str.as_bytes(), 10)
            .expect("Failed to parse big number");
        currency.amount = large_number;
    }

    spawn_ring(
        &mut commands,
        ring_index,
        gameplay_meshes.quad512.clone(),
        gameplay_meshes.quad64.clone(),
        ring_materials.add(RingMaterial {
            data: Vec4::new(RING_RADIUS, RING_THICKNESS, 0., 0.),
        }),
        socket_materials,
        2,
        time,
        0,
    );
}

pub fn spawn_ring(
    commands: &mut Commands,
    mut ring_index: ResMut<RingIndex>,
    ring_mesh: Mesh2dHandle,
    socket_mesh: Mesh2dHandle,
    ring_material: Handle<RingMaterial>,
    mut socket_materials: ResMut<Assets<SocketMaterial>>, // every socket needs a UNIQUE material instance
    num_sockets: usize,
    time: Res<Time>,
    index: usize,
) {
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
        ))
        .id();

    commands.entity(ring_entity).insert(
        MaterialMesh2dBundle {
            mesh: ring_mesh,
            material: ring_material,
            //transform: Transform::from_xyz(index as f32 * 512., 0., 0.),
            transform: Transform::from_translation(get_ring_position_and_update_index(index, 512., 100., ring_entity, &mut ring_index)),
            ..default()
        },
    );

    let mut starting_sockets: Vec<Entity> = vec![];

    commands
        .entity(ring_entity)
        .with_children(|ring_entity_children| {
            for i in 0..num_sockets {
                let socket_color = if i == 1 && num_sockets == 2 {
                    // the first ring spawns pre-socketed with 1 blue
                    SocketColor::BLUE
                } else {
                    SocketColor::NONE
                };

                let socket_entity = spawn_socket(
                    ring_entity_children,
                    socket_color.clone(),
                    ring_entity,
                    i,
                    socket_mesh.clone(),
                    socket_materials.add(SocketMaterial {
                        inserted_color: map_socket_color(socket_color),
                        highlight_color: map_socket_highlight_color(socket_color),
                        bevel_color: { BLACK.into() },
                        data: Vec4::new(
                            -1000.,
                            map_socket_color_trigger_duration(socket_color),
                            0.,
                            (socket_color as u8).saturating_sub(1) as f32,
                        ),
                        data2: Vec4::ZERO,
                    }),
                    socket_position(i, num_sockets).extend(1.),
                );

                starting_sockets.push(socket_entity);
            }
        });

    commands.entity(ring_entity).insert(Ring {
        sockets: starting_sockets,
        cycle_duration: 4.,
        cycle_start_seconds: time.elapsed_seconds(),
        cycle_multiplier: 1.,
        index,
        ..default()
    });
    
}

pub fn get_grid_coordinates(index: usize) -> IVec2 {
    let mut x = 0;
    let mut y = 0;
    let mut dx = 1;
    let mut dy = 0;
    let mut steps_in_current_layer = 1;
    let mut current_steps = 0;
    let mut turn_count = 0;
    let mut current_index = 0;

    // Find the grid coordinates of the given index in the spiral
    while current_index < index {
        x += dx;
        y += dy;
        current_steps += 1;
        current_index += 1;

        if current_steps == steps_in_current_layer {
            // Change direction clockwise: right -> up -> left -> down
            if dx == 1 && dy == 0 {
                dx = 0;
                dy = -1;
            } else if dx == 0 && dy == -1 {
                dx = -1;
                dy = 0;
            } else if dx == -1 && dy == 0 {
                dx = 0;
                dy = 1;
            } else if dx == 0 && dy == 1 {
                dx = 1;
                dy = 0;
            }

            turn_count += 1;
            current_steps = 0;

            // Increase step size every two turns (completing a layer)
            if turn_count == 2 {
                turn_count = 0;
                steps_in_current_layer += 1;
            }
        }
    }

    IVec2::new(x, y)
}

fn get_real_position(coords: IVec2, cell_size: f32, spacing: f32) -> Vec3 {
    let position_x = coords.x as f32 * (cell_size + spacing);
    let position_y = coords.y as f32 * (cell_size + spacing);
    Vec3::new(position_x, position_y, 0.0)
}

fn get_ring_position_and_update_index(index: usize, cell_size: f32, spacing: f32, ring: Entity, ring_index: &mut RingIndex) -> Vec3 {
    let coords = get_grid_coordinates(index);
    ring_index.rings.insert(coords, ring);
    get_real_position(coords, cell_size, spacing)
}

pub fn socket_position(index: usize, num_points: usize) -> Vec2 {
    let angle = 2.0 * PI * index as f32 / num_points as f32 + PI / 2.0;
    let ring_center_radius = RING_RADIUS - RING_THICKNESS / 2.0;
    let x = 0.5 * ring_center_radius * RING_QUAD_DIMENSIONS.x * angle.cos();
    let y = 0.5 * ring_center_radius * RING_QUAD_DIMENSIONS.y * angle.sin();

    Vec2 { x, y }
}

pub fn spawn_socket(
    commands: &mut ChildBuilder,
    color: SocketColor,
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
                ring,
                index,
                radius: DEFAULT_SOCKET_RADIUS,
                last_triggered_time_seconds: -100.,
                trigger_duration_seconds: map_socket_color_trigger_duration(color),
            },
            MaterialMesh2dBundle {
                mesh,
                material,
                transform: Transform::from_translation(translation),
                ..default()
            },
        ))
        .insert(On::<Pointer<Click>>::commands_mut(|ev, commands| {
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
        SocketColor::BLUE => BLUE_600,
        SocketColor::NONE => GRAY_950,
        SocketColor::RED => RED_600,
        SocketColor::GREEN => GREEN_600,
        SocketColor::ORANGE => ORANGE_600,
        SocketColor::PINK => PINK_600,
    }
    .into()
}

pub fn map_socket_color_trigger_duration(socket_color: SocketColor) -> f32 {
    match socket_color {
        SocketColor::BLUE => 0.4,
        SocketColor::NONE => 0.,
        SocketColor::RED => 3.,
        SocketColor::GREEN => 7.,
        SocketColor::ORANGE => 0.3,
        SocketColor::PINK => 14.,
    }
    .into()
}

pub fn map_socket_highlight_color(socket_color: SocketColor) -> LinearRgba {
    match socket_color {
        SocketColor::BLUE => BLUE_400,
        SocketColor::NONE => GRAY_900,
        SocketColor::RED => RED_400,
        SocketColor::GREEN => GREEN_400,
        SocketColor::ORANGE => ORANGE_400,
        SocketColor::PINK => PINK_400,
    }
    .into()
}

// surely we shouldn't do this this way LMAO
pub fn map_socket_color_hotkey(socket_color: SocketColor) -> u32 {
    match socket_color {
        SocketColor::NONE => panic!("uhhh"),
        SocketColor::BLUE => 1,
        SocketColor::RED => 2,
        SocketColor::GREEN => 3,
        SocketColor::ORANGE => 4,
        SocketColor::PINK => 5,
    }
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
    time: Res<Time>,
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

    let new_trigger_duration = map_socket_color_trigger_duration(selected_color);

    material.inserted_color = map_socket_color(new_color);
    material.highlight_color = map_socket_highlight_color(new_color);
    material.data[3] = (new_color as u8).saturating_sub(1) as f32;

    let current_time = time.elapsed_seconds();
    let cooldown_remaining =
        (socket.last_triggered_time_seconds + socket.trigger_duration_seconds) - current_time;

    if cooldown_remaining > 0. {
        let new_cooldown_end = current_time + new_trigger_duration;
        let old_cooldown_end = socket.last_triggered_time_seconds + socket.trigger_duration_seconds;

        if new_cooldown_end < old_cooldown_end {
            socket.last_triggered_time_seconds = old_cooldown_end - new_trigger_duration;
            material.data[0] = old_cooldown_end - new_trigger_duration;
        } else {
            socket.last_triggered_time_seconds =
                current_time - (new_trigger_duration - cooldown_remaining);
            material.data[0] = current_time - (new_trigger_duration - cooldown_remaining);
        }
    } else {
        socket.last_triggered_time_seconds = 0.;
        material.data[0] = 0.;
    }

    material.data[1] = new_trigger_duration;
    socket.trigger_duration_seconds = new_trigger_duration;

    socket.color = new_color;
}
