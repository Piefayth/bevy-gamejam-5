use bevy::{
    color::palettes::css::{BLACK, WHITE},
    input::keyboard::Key,
    prelude::*,
};

use crate::screen::Screen;

use super::widgets::{Hotbar, HotbarButton};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (update_hotbar_selection, update_hotbar_style).run_if(in_state(Screen::Playing)),
    );
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

fn update_hotbar_style(
    q_hotbar: Query<&Hotbar>,
    mut q_hotbar_button: Query<(&HotbarButton, &mut BorderColor)>,
) {
    let hotbar = q_hotbar.single();

    for (hotbar_button, mut border_color) in q_hotbar_button.iter_mut() {
        if hotbar_button.index == hotbar.selected_index {
            *border_color = WHITE.into();
        } else {
            *border_color = BLACK.into();
        }
    }
}
