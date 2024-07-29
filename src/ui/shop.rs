use std::time::Duration;

use bevy::{
    audio::PlaybackMode, color::palettes::{
        css::BLACK,
        tailwind::{GRAY_400, GRAY_500, GRAY_600, GRAY_700, GRAY_800, GRAY_900},
    }, math::VectorSpace, prelude::*, utils::HashSet
};
use bevy_mod_picking::{
    events::{Click, Pointer},
    prelude::On,
};
use bevy_tweening::{lens::{TransformPositionLens, TransformScaleLens}, Animator, EaseFunction, Tween};
use num_bigint::BigUint;

use crate::{
    game::{
        assets::{FontKey, HandleMap, SfxKey}, audio::soundtrack::PlaySfx, camera::CAMERA_DISABLE_TWEEN_NUMBER, materials::materials::{RingMaterial, SocketMaterial, SocketUiMaterial}, spawn::level::{
            map_socket_color, map_socket_color_hotkey, map_socket_highlight_color, socket_position,
            spawn_ring, spawn_socket, GameplayMeshes, Ring, RingIndex, Socket, SocketColor,
            RING_RADIUS, RING_THICKNESS,
        }
    },
    screen::{playing::Currency, Screen},
    ui::widgets::Widgets,
};

use super::{
    interaction::InteractionPalette,
    widgets::{Hotbar, ShopButton, UpgradeShop},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (fade_stuff_you_cant_afford).run_if(in_state(Screen::Playing)),
    );

    app.observe(on_new_shop);
    app.observe(on_purchase);

    app.init_resource::<Unlocks>();
    app.init_resource::<UpgradeHistory>();
}

#[derive(Resource, Default, PartialEq, Clone)]
pub struct UpgradeHistory {
    pub history: HashSet<UpgradeKind>,
}

#[derive(Default, PartialEq, Eq, Hash, Clone, Copy)]
pub enum UpgradeKind {
    #[default]
    None,
    AddSocket(AddSocketUpgrade),
    AddColor(AddColorUpgrade),
    AddRing(AddRingUpgrade),
    EnhanceColor(EnhanceColorUpgrade),
}

#[derive(Default, PartialEq, Eq, Hash, Clone)]
pub struct Upgrade {
    upgrade_kind: UpgradeKind,
    cost: BigUint,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct AddSocketUpgrade {
    level: u32,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct AddRingUpgrade {
    pub level: u32,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct AddColorUpgrade {
    color: SocketColor,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct EnhanceColorUpgrade {
    pub color: SocketColor,
    pub tier: u32,
}

#[derive(Event)]
pub struct NewShop {
    pub parent: Entity,
}

#[derive(Event)]
pub struct Purchase {
    pub upgrade: Upgrade,
    pub upgrade_button_entity: Entity,
}

fn upgrade_cost(upgrade_kind: UpgradeKind) -> BigUint {
    match upgrade_kind {
        UpgradeKind::None => BigUint::ZERO,
        UpgradeKind::AddSocket(upgrade) => {
            let base_add_socket_cost = BigUint::from(4u32);

            if upgrade.level <= 10 {
                let scale_factor_per_level = 0.115;
                multiply_biguint_with_float(
                    &base_add_socket_cost,
                    (upgrade.level as f32).powf(1. + scale_factor_per_level * upgrade.level as f32),
                )
            } else {
                let scale_factor_per_level = 0.125;
                let cost_at_scaling_threshold = multiply_biguint_with_float(
                    &base_add_socket_cost,
                    (upgrade.level as f32).powf(1. + scale_factor_per_level * 10. as f32),
                );

                let scaling_factor_after_threshold = 0.07;
                let cost_after_threshold = multiply_biguint_with_float(
                    &base_add_socket_cost,
                    (upgrade.level as f32).powf(1. + scaling_factor_after_threshold * upgrade.level as f32),
                );

                cost_at_scaling_threshold + cost_after_threshold
            }

        }
        UpgradeKind::AddColor(upgrade) => match upgrade.color {
            SocketColor::NONE => panic!("No such upgrade for baseline Socket Color NONE"),
            SocketColor::BLUE => panic!("No such upgrade for baseline Socket Color BLUE"),
            SocketColor::RED => BigUint::from(15u32),
            SocketColor::GREEN => BigUint::from(40u32),
            SocketColor::ORANGE => BigUint::from(100u32),
            SocketColor::PINK => BigUint::from(250u32),
        },
        UpgradeKind::AddRing(upgrade) => {
            match upgrade.level {
                1 => BigUint::from(1250u32),
                2 => BigUint::from(200000u32),
                3 => BigUint::from(2000000u32),
                4 => BigUint::from(20000000u32),
                5 => BigUint::from(200000000u32),
                6 => BigUint::from(2000000000u32),
                7 => BigUint::from(20000000000u64),
                8 => BigUint::from(200000000000u64),
                9 => BigUint::from(2000000000000u64),
                _ => {
                    let base: BigUint = BigUint::from(10u32);
                    let exponent = upgrade.level + 1;
                    let pow: BigUint = base.pow(exponent);
                    BigUint::from(20u32) * pow
                }
            }
        }
        UpgradeKind::EnhanceColor(upgrade) => match upgrade.color {
            SocketColor::NONE => todo!(),
            SocketColor::BLUE => match upgrade.tier {
                1 => BigUint::from(1000u32),
                2 => BigUint::from(25000u32),
                3 => BigUint::from(50000u32),
                _ => panic!("oops"),
            },
            SocketColor::RED => BigUint::from(3000u32),
            SocketColor::GREEN => BigUint::from(100000u32),
            SocketColor::ORANGE => BigUint::from(20000u32),
            SocketColor::PINK => BigUint::from(1000000u32),
        },
    }
}

pub fn multiply_biguint_with_float(bigint: &BigUint, float: f32) -> BigUint {
    let scale = 1_000_000u32;
    let scaled_float = (float * scale as f32).round() as u64;
    let scaled_product = bigint * scaled_float;
    scaled_product / BigUint::from(scale)
}

#[derive(Component)]
pub struct UpgradeButtonsContainer;

fn on_new_shop(
    trigger: Trigger<NewShop>,
    mut commands: Commands,
    mut unlocks: ResMut<Unlocks>,
    font_handles: ResMut<HandleMap<FontKey>>,
) {
    unlocks.0 = add_ring_and_socket_unlocks();
    unlocks.0.extend(build_color_unlocks());
    unlocks.0.extend(build_color_enhance_unlocks());

    let parent = trigger.event().parent;
    commands.entity(parent).with_children(|gameplay_parent| {
        gameplay_parent.upgrade_shop(font_handles[&FontKey::Default].clone());
    });

    commands.trigger(Purchase {
        upgrade: Upgrade {
            upgrade_kind: UpgradeKind::None,
            cost: upgrade_cost(UpgradeKind::None),
        },
        upgrade_button_entity: Entity::PLACEHOLDER,
    })
}

fn add_ring_and_socket_unlocks() -> Vec<Unlock> {
    let mut unlocks = Vec::new();
    let sockets_per_ring = 10;

    for ring_level in 0..64 {
        // Generate 15 socket unlocks for each ring level
        for socket_level in 1..=sockets_per_ring {
            let socket_index = ring_level * sockets_per_ring + socket_level; // Calculate global socket level

            let unlock = if socket_index == 1 {
                if ring_level > 0 {
                    Unlock {
                        when: vec![UpgradeKind::AddRing(AddRingUpgrade{ level: ring_level })],
                        then: UpgradeKind::AddSocket(AddSocketUpgrade { level: 1 }),
                    }
                } else {
                    // First socket unlock has no prerequisites
                    Unlock {
                        when: vec![],
                        then: UpgradeKind::AddSocket(AddSocketUpgrade { level: 1 }),
                    }
                }

            } else {
                if ring_level > 0 {
                    Unlock {
                        when: vec![UpgradeKind::AddSocket(AddSocketUpgrade {
                            level: socket_index as u32 - 1,
                        }), UpgradeKind::AddRing(AddRingUpgrade{ level: ring_level })],
                        then: UpgradeKind::AddSocket(AddSocketUpgrade {
                            level: socket_index as u32,
                        }),
                    }
                } else {
                    Unlock {
                        when: vec![UpgradeKind::AddSocket(AddSocketUpgrade {
                            level: socket_index as u32 - 1,
                        })],
                        then: UpgradeKind::AddSocket(AddSocketUpgrade {
                            level: socket_index as u32,
                        }),
                    }
                }

            };

            unlocks.push(unlock);
        }

        // Generate the ring unlock, which requires the last socket of the current set
        let when = if ring_level == 0 {
            // Special case for the first ring, requires pink color and the last socket
            vec![
                UpgradeKind::AddColor(AddColorUpgrade {
                    color: SocketColor::PINK,
                }),
                UpgradeKind::AddSocket(AddSocketUpgrade {
                    level: (ring_level * sockets_per_ring + sockets_per_ring) as u32,
                }),
            ]
        } else {
            // Regular case, just requires the last socket
            vec![UpgradeKind::AddSocket(AddSocketUpgrade {
                level: (ring_level * sockets_per_ring + sockets_per_ring) as u32,
            })]
        };

        unlocks.push(Unlock {
            when,
            then: UpgradeKind::AddRing(AddRingUpgrade {
                level: (ring_level + 1) as u32,
            }),
        });
    }

    unlocks
}

fn build_color_unlocks() -> Vec<Unlock> {
    vec![
        Unlock {
            when: vec![UpgradeKind::AddSocket(AddSocketUpgrade { level: 2 })],
            then: UpgradeKind::AddColor(AddColorUpgrade {
                color: SocketColor::RED,
            }),
        },
        Unlock {
            when: vec![UpgradeKind::AddColor(AddColorUpgrade {
                color: SocketColor::RED,
            })],
            then: UpgradeKind::AddColor(AddColorUpgrade {
                color: SocketColor::GREEN,
            }),
        },
        Unlock {
            when: vec![UpgradeKind::AddColor(AddColorUpgrade {
                color: SocketColor::GREEN,
            })],
            then: UpgradeKind::AddColor(AddColorUpgrade {
                color: SocketColor::ORANGE,
            }),
        },
        Unlock {
            when: vec![UpgradeKind::AddColor(AddColorUpgrade {
                color: SocketColor::ORANGE,
            })],
            then: UpgradeKind::AddColor(AddColorUpgrade {
                color: SocketColor::PINK,
            }),
        },
    ]
}

fn build_color_enhance_unlocks() -> Vec<Unlock> {
    vec![
        Unlock {
            when: vec![UpgradeKind::AddRing(AddRingUpgrade { level: 1 })],
            then: UpgradeKind::EnhanceColor(EnhanceColorUpgrade {
                color: SocketColor::BLUE,
                tier: 1,
            }),
        },
        Unlock {
            when: vec![UpgradeKind::EnhanceColor(EnhanceColorUpgrade {
                color: SocketColor::BLUE,
                tier: 1,
            })],
            then: UpgradeKind::EnhanceColor(EnhanceColorUpgrade {
                color: SocketColor::RED,
                tier: 1,
            }),
        },
        Unlock {
            when: vec![UpgradeKind::EnhanceColor(EnhanceColorUpgrade {
                color: SocketColor::RED,
                tier: 1,
            })],
            then: UpgradeKind::EnhanceColor(EnhanceColorUpgrade {
                color: SocketColor::ORANGE,
                tier: 1,
            }),
        },
        Unlock {
            when: vec![UpgradeKind::EnhanceColor(EnhanceColorUpgrade {
                color: SocketColor::ORANGE,
                tier: 1,
            })],
            then: UpgradeKind::EnhanceColor(EnhanceColorUpgrade {
                color: SocketColor::GREEN,
                tier: 1,
            }),
        },
        Unlock {
            when: vec![UpgradeKind::EnhanceColor(EnhanceColorUpgrade {
                color: SocketColor::ORANGE,
                tier: 1,
            })],
            then: UpgradeKind::EnhanceColor(EnhanceColorUpgrade {
                color: SocketColor::BLUE,
                tier: 2,
            }),
        },
        // todo: orange unlocks pink?
        Unlock {
            when: vec![UpgradeKind::EnhanceColor(EnhanceColorUpgrade {
                color: SocketColor::BLUE,
                tier: 2,
            })],
            then: UpgradeKind::EnhanceColor(EnhanceColorUpgrade {
                color: SocketColor::BLUE,
                tier: 3,
            }),
        }
    ]
}

#[derive(Default, Resource)]
struct Unlocks(Vec<Unlock>);

struct Unlock {
    when: Vec<UpgradeKind>,
    then: UpgradeKind,
}

fn on_purchase(
    trigger: Trigger<Purchase>,
    mut ring_index: ResMut<RingIndex>,
    mut upgrade_history: ResMut<UpgradeHistory>,
    mut q_rings: Query<(Entity, &mut Ring)>,
    mut q_sockets: Query<(Entity, &Socket, &mut Transform)>,
    mut q_hotbar: Query<(Entity, &mut Hotbar)>,
    mut commands: Commands,
    mut currency: ResMut<Currency>,
    mut materials: (
        ResMut<Assets<SocketMaterial>>,
        ResMut<Assets<RingMaterial>>
    ),
    mut socket_ui_materials: ResMut<Assets<SocketUiMaterial>>,
    mut unlocks: ResMut<Unlocks>,
    q_camera: Query<(Entity, &Transform), (With<Camera>, Without<Socket>)>,
    q_upgrade_button_container: Query<Entity, With<UpgradeButtonsContainer>>,
    gameplay_meshes: Res<GameplayMeshes>,
    font_handles: ResMut<HandleMap<FontKey>>,
    time: Res<Time>,
    sfx_handles: Res<HandleMap<SfxKey>>,
) {
    let purchase = trigger.event();
    let (mut socket_materials, mut ring_materials) = materials;

    // 1. grant what was purchased

    let cost = &purchase.upgrade.cost;
    if cost > &currency.amount {
        // error event
        return;
    }

    currency.amount -= cost;

    if purchase.upgrade.upgrade_kind != UpgradeKind::None {
        upgrade_history
            .history
            .insert(purchase.upgrade.upgrade_kind.clone());

        commands
            .entity(purchase.upgrade_button_entity)
            .despawn_recursive();
    }

    commands.trigger(PlaySfx {
        key: SfxKey::Unlock,
        volume: 2.
    });

    let ring_count = q_rings.iter().count();

    match &purchase.upgrade.upgrade_kind {
        UpgradeKind::None => {}
        UpgradeKind::AddSocket(_) => {
            for (ring_entity, mut ring) in q_rings.iter_mut() {
                if ring.index != ring_count - 1 {
                    // sockets only get added to the newest ring
                    continue;
                }

                let mesh = gameplay_meshes.quad64.clone();

                for socket_entity in &ring.sockets {
                    let (_e, socket, mut socket_transform) =
                        q_sockets.get_mut(*socket_entity).unwrap();
                    socket_transform.translation =
                        socket_position(socket.index, ring.sockets.len() + 1).extend(1.)
                }

                let count = ring.sockets.len();
                let socket_trigger_duration = 0.;
                let socket_material = socket_materials.add(SocketMaterial {
                    inserted_color: map_socket_color(SocketColor::NONE),
                    highlight_color: map_socket_highlight_color(SocketColor::NONE),
                    bevel_color: { BLACK.into() },
                    data: Vec4::new(-1000., socket_trigger_duration, 0., 0.),
                    data2: Vec4::ZERO,
                });

                commands
                    .entity(ring_entity)
                    .with_children(move |ring_children| {
                        let new_socket_entity = spawn_socket(
                            ring_children,
                            SocketColor::NONE,
                            ring_entity,
                            count,
                            mesh,
                            socket_material,
                            socket_position(count, count + 1).extend(1.),
                        );

                        ring.sockets.push(new_socket_entity);
                    });
            }
        }
        UpgradeKind::AddColor(color_upgrade) => {
            let (hotbar_entity, mut hotbar) = q_hotbar.single_mut();
            hotbar.color_mappings.push(color_upgrade.color);

            let socket_ui_material = socket_ui_materials.add(SocketUiMaterial {
                bevel_color: BLACK.into(),
                inserted_color: map_socket_color(color_upgrade.color),
                data: Vec4::new((color_upgrade.color as u8).saturating_sub(1) as f32, 0., 0., 0.)
            });

            let hotkey = map_socket_color_hotkey(color_upgrade.color);

            commands
                .entity(hotbar_entity)
                .with_children(|hotbar_children| {
                    hotbar_children.hotbar_button(
                        socket_ui_material,
                        format!("{}.", hotkey),
                        hotkey - 1,
                    );
                });
        }
        UpgradeKind::AddRing(_) => {
            let existing_ring_count = q_rings.iter().count();

            let (camera_entity, camera_transform)= q_camera.single();

            if existing_ring_count == 1 {
                let tween = Tween::new(
                    EaseFunction::QuadraticIn,
                    Duration::from_secs(2),
                    TransformScaleLens {
                        start: camera_transform.scale,
                        end: camera_transform.scale * 2.,
                    },
                ).with_completed_event(CAMERA_DISABLE_TWEEN_NUMBER);
                
                commands.entity(camera_entity).insert(Animator::new(tween));
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
                existing_ring_count,
            )
        }
        UpgradeKind::EnhanceColor(_) => {}
    }

    // 2. Unlock what is available now
    let container = q_upgrade_button_container.single();

    let mut indices_to_remove = Vec::new();

    for (index, unlock) in unlocks.0.iter().enumerate() {
        if unlock
            .when
            .iter()
            .all(|item| upgrade_history.history.contains(item))
        {
            commands
                .entity(container)
                .with_children(|button_container| {
                    let new_upgrade = Upgrade {
                        upgrade_kind: unlock.then,
                        cost: upgrade_cost(unlock.then),
                    };

                    let mut button_entity_commands = button_container.shop_button(
                        &new_upgrade.cost,
                        upgrade_description(&new_upgrade),
                        font_handles[&FontKey::Default].clone(),
                    );
                    let button_entity = button_entity_commands.id();

                    button_entity_commands.insert(On::<Pointer<Click>>::commands_mut(
                        move |_ev, commands| {
                            commands.trigger(Purchase {
                                upgrade: new_upgrade.clone(),
                                upgrade_button_entity: button_entity,
                            });
                        },
                    ));
                });

            indices_to_remove.push(index);
        }
    }

    for index in indices_to_remove.iter().rev() {
        unlocks.0.remove(*index);
    }
}

fn fade_stuff_you_cant_afford(
    currency: Res<Currency>,
    mut q_shop_button: Query<(&mut BackgroundColor, &mut InteractionPalette, &mut BorderColor, &ShopButton, &Interaction)>,
) {
    let default_palette = InteractionPalette {
        none: GRAY_700.into(),
        hovered: GRAY_600.into(),
        pressed: GRAY_500.into(),
    };

    let disabled_palette = InteractionPalette {
        none: GRAY_900.into(),
        hovered: GRAY_900.into(),
        pressed: GRAY_900.into(),
    };

    for (mut bg_color, mut palette, mut border_color, button, interaction) in q_shop_button.iter_mut() {
        if button.price > currency.amount {
            *palette = disabled_palette.clone();
            *border_color = BorderColor(GRAY_700.into());
            *bg_color = BackgroundColor(disabled_palette.none);
        } else {
            *palette = default_palette.clone();

            if *interaction == Interaction::None {
                *border_color = BorderColor(GRAY_400.into());
                *bg_color = BackgroundColor(default_palette.none);
            }
        }
    }
}

fn upgrade_description(upgrade: &Upgrade) -> impl Into<String> {
    let description = match upgrade.upgrade_kind {
        UpgradeKind::None => "Errmm.. This shouldn't be for sale",
        UpgradeKind::AddSocket(_) => "Add a socket",
        UpgradeKind::AddColor(upgrade) => &format!("Add {} orbs", upgrade.color.as_str()),
        UpgradeKind::AddRing(_) => "Add a ring",
        UpgradeKind::EnhanceColor(upgrade) => match upgrade {
            EnhanceColorUpgrade {
                color: SocketColor::BLUE,
                tier: 1,
            } => "BLUE orbs new behavior",
            EnhanceColorUpgrade {
                color: SocketColor::BLUE,
                tier: 2,
            } => "BLUE orbs more effective",
            EnhanceColorUpgrade {
                color: SocketColor::BLUE,
                tier: 3,
            } => "BLUE orbs new behavior",
            EnhanceColorUpgrade {
                color: SocketColor::RED,
                tier: 1,
            } => "RED orbs new behavior",
            EnhanceColorUpgrade {
                color: SocketColor::GREEN,
                tier: 1,
            } => "GREEN orbs new behavior",
            EnhanceColorUpgrade {
                color: SocketColor::ORANGE,
                tier: 1,
            } => "ORANGE orbs more effective",
            _ => "You are seeing this message because I made a mistake.",
        },
    };
    format!("{}", description)
}
