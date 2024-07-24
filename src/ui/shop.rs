use bevy::{color::palettes::css::BLACK, prelude::*, utils::HashSet};
use bevy_mod_picking::{
    events::{Click, Pointer},
    prelude::On,
};
use num_bigint::BigUint;

use crate::{
    game::{
        assets::{FontKey, HandleMap},
        materials::materials::{SocketMaterial, SocketUiMaterial},
        spawn::level::{
            map_socket_color, map_socket_highlight_color, socket_position, spawn_socket, GameplayMeshes, Ring, Socket, SocketColor
        },
    },
    screen::{playing::Currency, Screen},
    ui::widgets::Widgets,
};

use super::widgets::{Hotbar, UpgradeShop};

pub(super) fn plugin(app: &mut App) {
    // app.add_systems(
    //     Update,
    //     ().run_if(in_state(Screen::Playing)),
    // );

    app.observe(on_new_shop);
    app.observe(on_purchase);

    app.init_resource::<Unlocks>();
    app.init_resource::<UpgradeHistory>();
}

#[derive(Resource, Default, PartialEq)]
struct UpgradeHistory {
    pub history: HashSet<UpgradeKind>,
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

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct AddSocketUpgrade {
    level: u32,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct AddColorUpgrade {
    color: SocketColor,
}

#[derive(Default, PartialEq, Eq, Hash, Clone, Copy)]
pub enum UpgradeKind {
    #[default]
    None,
    AddSocket(AddSocketUpgrade),
    AddColor(AddColorUpgrade),
}

#[derive(Default, PartialEq, Eq, Hash, Clone)]
pub struct Upgrade {
    upgrade_kind: UpgradeKind,
    cost: BigUint,
}

fn upgrade_cost(upgrade_kind: UpgradeKind) -> BigUint {
    match upgrade_kind {
        UpgradeKind::None => BigUint::ZERO,
        UpgradeKind::AddSocket(upgrade) => {
            let base_add_socket_cost = BigUint::from(5u32);
            let scale_factor_per_level = 0.15;
            multiply_biguint_with_float(&base_add_socket_cost, (upgrade.level as f32).powf(1. + scale_factor_per_level * upgrade.level as f32))
        
        }
        UpgradeKind::AddColor(upgrade) => match upgrade.color {
            SocketColor::NONE => panic!("No such upgrade for baseline Socket Color NONE"),
            SocketColor::BLUE => panic!("No such upgrade for baseline Socket Color BLUE"),
            SocketColor::RED => BigUint::from(15u32),
        },
    }
}

fn multiply_biguint_with_float(bigint: &BigUint, float: f32) -> BigUint {
    let scale = 1_000_000u32;
    let scaled_float = (float * scale as f32).round() as u64;
    let scaled_product = bigint * scaled_float;
    scaled_product / BigUint::from(scale)
}

#[derive(Component)]
struct UpgradeButtonsContainer;

fn on_new_shop(trigger: Trigger<NewShop>, mut commands: Commands, mut unlocks: ResMut<Unlocks>) {
    // define every unlock

    unlocks.0 = add_socket_unlocks();
    unlocks.0.extend(add_color_unlocks());

    let parent = trigger.event().parent;
    commands.entity(parent).with_children(|gameplay_parent| {
        gameplay_parent
            .upgrade_shop()
            .with_children(|upgrade_shop_children| {
                upgrade_shop_children
                    .vertical_container(JustifyContent::Start)
                    .insert(UpgradeButtonsContainer);
            });
    });

    commands.trigger(Purchase {
        upgrade: Upgrade {
            upgrade_kind: UpgradeKind::None,
            cost: upgrade_cost(UpgradeKind::None),
        },
        upgrade_button_entity: Entity::PLACEHOLDER,
    })
}

fn add_socket_unlocks() -> Vec<Unlock> {
    (0..20)
        .collect::<Vec<usize>>()
        .iter()
        .map(|elem| {
            if *elem == 0 {
                Unlock {
                    when: vec![],
                    then: UpgradeKind::AddSocket(AddSocketUpgrade { level: 1 }),
                }
            } else {
                Unlock {
                    when: vec![UpgradeKind::AddSocket(AddSocketUpgrade {
                        level: *elem as u32,
                    })],
                    then: UpgradeKind::AddSocket(AddSocketUpgrade {
                        level: *elem as u32 + 1,
                    }),
                }
            }
        })
        .collect()
}

fn add_color_unlocks() -> Vec<Unlock> {
    vec![Unlock {
        when: vec![UpgradeKind::AddSocket(AddSocketUpgrade { level: 2 })],
        then: UpgradeKind::AddColor(AddColorUpgrade {
            color: SocketColor::RED,
        }),
    }]
}

#[derive(Default, Resource)]
struct Unlocks(Vec<Unlock>);

struct Unlock {
    when: Vec<UpgradeKind>,
    then: UpgradeKind,
}

fn on_purchase(
    trigger: Trigger<Purchase>,
    mut upgrade_history: ResMut<UpgradeHistory>,
    mut q_rings: Query<(Entity, &mut Ring)>,
    mut q_sockets: Query<(Entity, &Socket, &mut Transform)>,
    mut q_hotbar: Query<(Entity, &mut Hotbar)>,
    mut commands: Commands,
    mut currency: ResMut<Currency>,
    mut socket_materials: ResMut<Assets<SocketMaterial>>,
    mut socket_ui_materials: ResMut<Assets<SocketUiMaterial>>,
    mut unlocks: ResMut<Unlocks>,
    q_upgrade_button_container: Query<Entity, With<UpgradeButtonsContainer>>,
    gameplay_meshes: Res<GameplayMeshes>,
    font_handles: ResMut<HandleMap<FontKey>>,
    time: Res<Time>,
) {
    let purchase = trigger.event();

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

    match &purchase.upgrade.upgrade_kind {
        UpgradeKind::None => {}
        UpgradeKind::AddSocket(_) => {
            for (ring_entity, mut ring) in q_rings.iter_mut() {
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
                    data: Vec4::new(-1., socket_trigger_duration, 0., 0.)
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
                inserted_color: map_socket_color(SocketColor::RED),
            });

            commands
                .entity(hotbar_entity)
                .with_children(|hotbar_children| {
                    hotbar_children.hotbar_button(socket_ui_material, "2.", 1);
                });
            // need to spawn the new hotbar button
            // so we need to get access to the hotbar i guess?
        }
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
                        upgrade_text(&new_upgrade),
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

fn upgrade_text(upgrade: &Upgrade) -> impl Into<String> {
    let description = match upgrade.upgrade_kind {
        UpgradeKind::None => "Errmm.. This shouldn't be for sale",
        UpgradeKind::AddSocket(_) => "Add a socket",
        UpgradeKind::AddColor(upgrade) => &format!("Add {} orbs", upgrade.color.as_str()),
    };
    format!("${} | {}", upgrade.cost, description)
}
