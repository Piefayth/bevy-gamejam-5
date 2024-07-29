use bevy::{
    color::palettes::{css::{BLACK, WHITE, WHITE_SMOKE}, tailwind::{GRAY_300, GRAY_400, GRAY_900}},
    input::keyboard::Key,
    prelude::*,
};

use crate::{game::{materials::materials::SocketUiMaterial, spawn::level::{map_socket_color, SocketColor}}, screen::Screen};

use super::{shop::{EnhanceColorUpgrade, UpgradeHistory, UpgradeKind}, widgets::{Hotbar, HotbarButton, HotbarChanged, HotbarDescriptionIcon, HotbarDescriptionText}};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (update_hotbar_text, update_hotbar_selection, update_hotbar_style).run_if(in_state(Screen::Playing)),
    );

    app.observe(on_hotbar_changed);
}

fn update_hotbar_selection(keys: Res<ButtonInput<KeyCode>>, mut query: Query<&mut Hotbar>) {
    for mut hotbar in query.iter_mut() {
        if keys.just_pressed(KeyCode::Digit1) && hotbar.color_mappings.len() > 0 {
            hotbar.selected_index = 0;
        } else if keys.just_pressed(KeyCode::Digit2) && hotbar.color_mappings.len() > 1 {
            hotbar.selected_index = 1;
        } else if keys.just_pressed(KeyCode::Digit3) && hotbar.color_mappings.len() > 2 {
            hotbar.selected_index = 2;
        } else if keys.just_pressed(KeyCode::Digit4) && hotbar.color_mappings.len() > 3 {
            hotbar.selected_index = 3;
        } else if keys.just_pressed(KeyCode::Digit5) && hotbar.color_mappings.len() > 4 {
            hotbar.selected_index = 4;
        } else if keys.just_pressed(KeyCode::Digit6) && hotbar.color_mappings.len() > 5 {
            hotbar.selected_index = 5;
        } else if keys.just_pressed(KeyCode::Digit7) && hotbar.color_mappings.len() > 6 {
            hotbar.selected_index = 6;
        } else if keys.just_pressed(KeyCode::Digit8) && hotbar.color_mappings.len() > 7 {
            hotbar.selected_index = 7;
        } else if keys.just_pressed(KeyCode::Digit9) && hotbar.color_mappings.len() > 8 {
            hotbar.selected_index = 8;
        } else if keys.just_pressed(KeyCode::Digit0) && hotbar.color_mappings.len() > 9 {
            hotbar.selected_index = 9;
        }
    }
}

fn on_hotbar_changed(
    trigger: Trigger<HotbarChanged>,
    mut q_hotbar: Query<&mut Hotbar>
) {
    let mut hotbar = q_hotbar.single_mut();
    hotbar.selected_index = trigger.event().index;
}

fn update_hotbar_text(
    q_hotbar: Query<&Hotbar>,
    q_changed_hotbar: Query<Entity, Changed<Hotbar>>,
    mut q_hotbar_text: Query<&mut Text, With<HotbarDescriptionText>>,
    mut socket_ui_materials: ResMut<Assets<SocketUiMaterial>>,
    mut q_hotbar_description_icon: Query<(&Handle<SocketUiMaterial>, &mut HotbarDescriptionIcon)>,
    upgrade_history: Res<UpgradeHistory>,
) {
    if !q_changed_hotbar.is_empty() && !upgrade_history.is_changed() {
        return
    }

    let hotbar = q_hotbar.single();
    let mut hotbar_text = q_hotbar_text.single_mut();
    let (icon_mat_handle, mut hotbar_icon) = q_hotbar_description_icon.single_mut();
    
    hotbar_icon.current_socket_color = hotbar.color_mappings[hotbar.selected_index as usize];
    hotbar_text.sections[0].value = map_socket_color_description_text(hotbar_icon.current_socket_color, &upgrade_history);


    let socket_ui_material = socket_ui_materials.get_mut(icon_mat_handle).expect("HotbarDescriptionIcon should've had a SocketUiMaterial");

    socket_ui_material.inserted_color = map_socket_color(hotbar_icon.current_socket_color);
}

pub fn map_socket_color_description_text(socket_color: SocketColor, upgrade_history: &UpgradeHistory) -> String {
    match socket_color {
        SocketColor::NONE => String::from("???"),
        SocketColor::BLUE => {
            if upgrade_history.history.contains(&UpgradeKind::EnhanceColor(EnhanceColorUpgrade {
                color: SocketColor::BLUE,
                tier: 3,
            })) {
                format!("Slots into a socket. Grants $6 for ALL other socketed blue orbs. Triggers ALL other blue orbs. ")
            } else if upgrade_history.history.contains(&UpgradeKind::EnhanceColor(EnhanceColorUpgrade {
                color: SocketColor::BLUE,
                tier: 2,
            })) {
                format!("Slots into a socket. Grants $4 for ALL other socketed blue orbs.")
            } else if upgrade_history.history.contains(&UpgradeKind::EnhanceColor(EnhanceColorUpgrade {
                color: SocketColor::BLUE,
                tier: 1,
            })) {
                format!("Slots into a socket. Grants $2 for ALL other socketed blue orbs.")
            }else {
                format!("Slots into a socket. Grants ${} every time it is triggered.", 1.)
            }
        },
        SocketColor::RED => {
            if upgrade_history.history.contains(&UpgradeKind::EnhanceColor(EnhanceColorUpgrade {
                color: SocketColor::RED,
                tier: 1,
            })){
                format!("Grants no $. Triggers adjacent sockets when triggered. Sockets triggered this way are twice as effective.")
            } else {
                format!("Grants no $. Triggers adjacent sockets when triggered.")
            }
        },
        SocketColor::GREEN => {
            if upgrade_history.history.contains(&UpgradeKind::EnhanceColor(EnhanceColorUpgrade {
                color: SocketColor::GREEN,
                tier: 1,
            })){
                format!("Grants ${} for each trigger in the ALL rings' previous cycles. Retriggers pay 5x.", 1.)
            } else {
                format!("Grants ${} for each trigger in the ring's previous cycle.", 1.)
            }
        },
        SocketColor::ORANGE => {
            if upgrade_history.history.contains(&UpgradeKind::EnhanceColor(EnhanceColorUpgrade {
                color: SocketColor::ORANGE,
                tier: 1,
            })){
                format!("Reduces the cooldown of all sockets in the ring by {} second(s).", 1.0)
            } else {
                format!("Reduces the cooldown of all sockets in the ring by {} second(s).", 0.5)
            }
        },
        SocketColor::PINK => format!("When triggered, increases the cycle's multiplier by {}.", 1.),
    }
}

fn update_hotbar_style(
    q_hotbar: Query<&Hotbar>,
    mut q_hotbar_button: Query<(&HotbarButton, &mut BorderColor)>,
) {
    let hotbar = q_hotbar.single();

    for (hotbar_button, mut border_color) in q_hotbar_button.iter_mut() {
        if hotbar_button.index == hotbar.selected_index {
            *border_color = GRAY_400.into();
        } else {
            *border_color = GRAY_900.into();
        }
    }
}
