//! The title screen that appears when the game starts.

use bevy::{color::palettes::{css::{BLUE, DARK_GRAY, GRAY, LIGHT_CORAL, LIGHT_CYAN, LIGHT_GRAY, RED}, tailwind::{GRAY_400, GRAY_600, GRAY_700}}, prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}};

use super::Screen;
use crate::{game::materials::materials::BackgroundMaterial, ui::prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Title), enter_title);

    app.register_type::<TitleAction>();
    app.add_systems(Update, handle_title_action.run_if(in_state(Screen::Title)));
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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<BackgroundMaterial>>,
) {
    commands
        .ui_root()
        .insert(StateScoped(Screen::Title))
        .with_children(|children| {
            children.button("Play").insert(TitleAction::Play);
            children.button("Credits").insert(TitleAction::Credits);

            #[cfg(not(target_family = "wasm"))]
            children.button("Exit").insert(TitleAction::Exit);
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
