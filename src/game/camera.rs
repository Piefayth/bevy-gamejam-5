use bevy::prelude::*;
use bevy::input::{keyboard::KeyCode, mouse::{MouseButton, MouseWheel, MouseMotion}, ButtonInput};
use bevy::window::{CursorGrabMode, PrimaryWindow};
use bevy_mod_picking::*;
use bevy_tweening::TweenCompleted;
use events::{Drag, DragEnd, DragStart, Pointer};
use pointer::PointerButton;
use prelude::ListenerInput;

use crate::screen::title::Background;
use crate::screen::Screen;
use crate::ui::shop::{AddRingUpgrade, UpgradeHistory, UpgradeKind};

pub struct CameraControlPlugin;

impl Plugin for CameraControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                zoom_keyboard_input,
                zoom_mouse_scroll,
                move_camera_keyboard_input,
                (
                    camera_drag_start,
                    camera_drag_end,
                    move_camera_mouse
                ).chain()
            ).run_if(in_state(Screen::Playing))
        );

        app.observe(on_disable_disable_zoom);
    }
}

fn move_camera_mouse(
    mut ev_drag: EventReader<Pointer<Drag>>,
    q_bg: Query<Entity, With<Background>>,
    mut q_camera: Query<&mut Transform, With<Camera>>,
    upgrade_history: Res<UpgradeHistory>,
) {
    if !upgrade_history.history.contains(&UpgradeKind::AddRing(AddRingUpgrade {level: 1u32})) { // how do i refactor this into a run condition?
        return;
    }

    let bg_entity = q_bg.single();
    for trigger in ev_drag.read() {
        if trigger.event.button == PointerButton::Secondary  && trigger.target == bg_entity {
            let mut transform = q_camera.single_mut();

            const MOVE_SENSITIVITY: f32 = 0.75;
            transform.translation.x += trigger.delta.x * -MOVE_SENSITIVITY;
            transform.translation.y += trigger.delta.y * MOVE_SENSITIVITY;
        }
    }
}

fn camera_drag_start(
    q_bg: Query<Entity, With<Background>>,
    mut ev_drag: EventReader<Pointer<DragStart>>,
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
    upgrade_history: Res<UpgradeHistory>,
) {
    if !upgrade_history.history.contains(&UpgradeKind::AddRing(AddRingUpgrade {level: 1u32})) {
        return;
    }

    let bg_entity = q_bg.single();

    for trigger in ev_drag.read() {
        if trigger.event.button == PointerButton::Secondary  && trigger.target == bg_entity {     
            let mut primary_window = q_windows.single_mut();
            primary_window.cursor.visible = false;
        }
    }
}

fn camera_drag_end(
    q_bg: Query<Entity, With<Background>>,
    mut ev_drag: EventReader<Pointer<DragEnd>>,
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
    upgrade_history: Res<UpgradeHistory>,
) {
    if !upgrade_history.history.contains(&UpgradeKind::AddRing(AddRingUpgrade {level: 1u32})) {
        return;
    }

    let bg_entity = q_bg.single();

    for trigger in ev_drag.read() {
        if trigger.event.button == PointerButton::Secondary  && trigger.target == bg_entity {     
            let mut primary_window = q_windows.single_mut();

            primary_window.cursor.grab_mode = CursorGrabMode::None;
            primary_window.cursor.visible = true;
        }
    }
}

#[derive(Component)]
pub struct DisableZoom;

pub const CAMERA_DISABLE_TWEEN_NUMBER: u64 = 999u64;

fn on_disable_disable_zoom(
    trigger: Trigger<TweenCompleted>,
    mut commands: Commands,
    q_disabled: Query<Entity, With<DisableZoom>>
) {
    if trigger.event().user_data != CAMERA_DISABLE_TWEEN_NUMBER {
        return
    }

    for cam in &q_disabled {
        commands.entity(cam).remove::<DisableZoom>();
    }
}

// System to handle keyboard input for zooming
fn zoom_keyboard_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, (With<Camera>, Without<DisableZoom>)>,
    upgrade_history: Res<UpgradeHistory>,
) {
    if !upgrade_history.history.contains(&UpgradeKind::AddRing(AddRingUpgrade {level: 1u32})) {
        return;
    }

    let mut zoom = 1.0;
    const ZOOM_SPEED: f32 = 0.02;

    if keys.pressed(KeyCode::KeyE) {
        zoom -= ZOOM_SPEED;
    }
    if keys.pressed(KeyCode::KeyQ) {
        zoom += ZOOM_SPEED;
    }

    for mut transform in query.iter_mut() {
        transform.scale = (transform.scale * Vec3::splat(zoom)).max(Vec3::ONE);
    }
}

fn zoom_mouse_scroll(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query: Query<&mut Transform, (With<Camera>, Without<DisableZoom>)>,
    upgrade_history: Res<UpgradeHistory>,
) {
    if !upgrade_history.history.contains(&UpgradeKind::AddRing(AddRingUpgrade {level: 1u32})) {
        return;
    }

    let mut zoom = 1.0;

    #[cfg(not(target_family = "wasm"))]
    const SCROLL_ZOOM_SPEED: f32 = 0.1;

    #[cfg(target_family = "wasm")]
    const SCROLL_ZOOM_SPEED: f32 = 0.0005;

    for event in mouse_wheel_events.read() {
        zoom -= event.y * SCROLL_ZOOM_SPEED;
    }

    for mut transform in query.iter_mut() {
        transform.scale = (transform.scale * Vec3::splat(zoom)).max(Vec3::ONE);
    }
}

fn move_camera_keyboard_input(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Camera>>,
    upgrade_history: Res<UpgradeHistory>,
) {
    if !upgrade_history.history.contains(&UpgradeKind::AddRing(AddRingUpgrade {level: 1u32})) {
        return;
    }

    let mut direction = Vec3::ZERO;
    const MOVE_SPEED: f32 = 2500.0;

    if keys.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }
    if keys.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if keys.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }
    if keys.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }

    if direction != Vec3::ZERO {
        direction = direction.normalize();
    }

    for mut transform in query.iter_mut() {
        transform.translation += direction * MOVE_SPEED * time.delta_seconds();
    }
}