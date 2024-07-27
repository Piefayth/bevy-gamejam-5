//! Helper traits for creating common widgets.

use bevy::{
    color::palettes::{
        css::{BLUE, GREEN, LIGHT_CYAN, MAGENTA, ORANGE, PURPLE, RED, WHITE},
        tailwind::{GRAY_100, GRAY_200},
    },
    ecs::system::EntityCommands,
    math::VectorSpace,
    prelude::*,
    ui::Val::*,
};
use bevy_mod_picking::picking_core::Pickable;

use crate::game::{
    materials::materials::{SocketMaterial, SocketUiMaterial},
    spawn::level::SocketColor,
};

use super::{interaction::InteractionPalette, palette::*};

/// An extension trait for spawning UI widgets.
pub trait Widgets {
    /// Spawn a simple button with text.
    fn button(&mut self, text: impl Into<String>) -> EntityCommands;

    /// Spawn a simple header label. Bigger than [`Widgets::label`].
    fn header(&mut self, text: impl Into<String>) -> EntityCommands;

    /// Spawn a simple text label.
    fn label(&mut self, text: impl Into<String>) -> EntityCommands;

    fn horizontal_container(&mut self) -> EntityCommands;

    fn vertical_container(&mut self, justify_content: JustifyContent) -> EntityCommands;

    fn upgrade_shop(&mut self) -> EntityCommands;
    fn shop_button(&mut self, text: impl Into<String>, font: Handle<Font>) -> EntityCommands;

    fn scoreboard_cycles_text(
        &mut self,
        text: impl Into<String>,
        font: Handle<Font>,
    ) -> EntityCommands;

    fn scoreboard_currency_text(
        &mut self,
        text: impl Into<String>,
        font: Handle<Font>,
    ) -> EntityCommands;

    fn score_display(&mut self, font: Handle<Font>) -> EntityCommands;
    
    fn hotbar_description(&mut self, starting_text: impl Into<String>, socket_color: SocketColor, font: Handle<Font>, socket_material: Handle<SocketUiMaterial>) -> EntityCommands;

    fn hotbar(&mut self, starting_colors: Vec<SocketColor>) -> EntityCommands;

    fn hotbar_button(&mut self, socket_material: Handle<SocketUiMaterial>, hotkey_text: impl Into<String>, index: u32) -> EntityCommands;

    fn socket(&mut self, socket_material: Handle<SocketUiMaterial>) -> EntityCommands;
}

impl<T: Spawn> Widgets for T {
    fn button(&mut self, text: impl Into<String>) -> EntityCommands {
        let mut entity = self.spawn((
            Name::new("Button"),
            ButtonBundle {
                style: Style {
                    width: Px(200.0),
                    height: Px(65.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BackgroundColor(NODE_BACKGROUND),
                ..default()
            },
            InteractionPalette {
                none: NODE_BACKGROUND,
                hovered: BUTTON_HOVERED_BACKGROUND,
                pressed: BUTTON_PRESSED_BACKGROUND,
            },
        ));
        entity.with_children(|children| {
            children.spawn((
                Name::new("Button Text"),
                TextBundle::from_section(
                    text,
                    TextStyle {
                        font_size: 40.0,
                        color: BUTTON_TEXT,
                        ..default()
                    },
                ),
            ));
        });
        entity
    }

    fn shop_button(&mut self, text: impl Into<String>, font: Handle<Font>) -> EntityCommands {
        let mut entity = self.spawn((
            Name::new("Button"),
            ButtonBundle {
                style: Style {
                    width: Px(200.0),
                    height: Px(65.0),
                    display: Display::Flex,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BackgroundColor(NODE_BACKGROUND),
                ..default()
            },
            InteractionPalette {
                none: NODE_BACKGROUND,
                hovered: BUTTON_HOVERED_BACKGROUND,
                pressed: BUTTON_PRESSED_BACKGROUND,
            },
        ));
        entity.with_children(|children| {
            children
                .spawn((
                    Name::new("Button Text"),
                    TextBundle::from_section(
                        text,
                        TextStyle {
                            font_size: 16.0,
                            color: BUTTON_TEXT,
                            font: font,
                            ..default()
                        },
                    ),
                ))
                .insert(Style {
                    margin: UiRect::all(Px(8.)),
                    ..default()
                });
        });
        entity
    }

    fn header(&mut self, text: impl Into<String>) -> EntityCommands {
        let mut entity = self.spawn((
            Name::new("Header"),
            NodeBundle {
                style: Style {
                    width: Px(500.0),
                    height: Px(65.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BackgroundColor(NODE_BACKGROUND),
                ..default()
            },
        ));
        entity.with_children(|children| {
            children.spawn((
                Name::new("Header Text"),
                TextBundle::from_section(
                    text,
                    TextStyle {
                        font_size: 40.0,
                        color: HEADER_TEXT,
                        ..default()
                    },
                ),
            ));
        });
        entity
    }

    fn scoreboard_cycles_text(
        &mut self,
        text: impl Into<String>,
        font: Handle<Font>,
    ) -> EntityCommands {
        let mut entity = self.spawn((
            Name::new("Scoreboard Text"),
            NodeBundle {
                style: Style {
                    padding: UiRect::all(Px(8.)),
                    ..default()
                },
                ..default()
            },
        ));

        entity.with_children(|children| {
            children.spawn((
                Name::new("Scoreboard Text Text"),
                TextBundle::from_section(
                    text,
                    TextStyle {
                        font: font,
                        font_size: 16.,
                        ..default()
                    },
                ),
                CyclesCountText,
            ));
        });

        entity
    }

    fn scoreboard_currency_text(
        &mut self,
        text: impl Into<String>,
        font: Handle<Font>,
    ) -> EntityCommands {
        let mut entity = self.spawn((
            Name::new("Scoreboard Text"),
            NodeBundle {
                style: Style {
                    padding: UiRect::all(Px(8.)),
                    ..default()
                },
                ..default()
            },
        ));

        entity.with_children(|children| {
            children.spawn((
                Name::new("Scoreboard Text Text"),
                TextBundle::from_section(
                    text,
                    TextStyle {
                        font: font,
                        font_size: 16.,
                        ..default()
                    },
                ),
                CurrencyText,
            ));
        });

        entity
    }

    fn label(&mut self, text: impl Into<String>) -> EntityCommands {
        let mut entity = self.spawn((
            Name::new("Label"),
            NodeBundle {
                style: Style {
                    width: Px(500.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
        ));
        entity.with_children(|children| {
            children.spawn((
                Name::new("Label Text"),
                TextBundle::from_section(
                    text,
                    TextStyle {
                        font_size: 24.0,
                        color: LABEL_TEXT,
                        ..default()
                    },
                ),
            ));
        });
        entity
    }

    fn horizontal_container(&mut self) -> EntityCommands {
        let entity = self.spawn((
            Name::new("FlexWrapper"),
            NodeBundle {
                style: Style {
                    width: Percent(100.),
                    height: Percent(100.),
                    display: Display::Flex,
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::FlexStart,
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                //background_color: RED.into(),
                ..default()
            },
            Pickable {
                should_block_lower: false,
                is_hoverable: false,
            },
        ));

        entity
    }

    fn vertical_container(&mut self, justify_content: JustifyContent) -> EntityCommands {
        let entity = self.spawn((
            Name::new("FlexWrapper"),
            NodeBundle {
                style: Style {
                    width: Percent(100.),
                    height: Percent(100.),
                    display: Display::Flex,
                    justify_content: justify_content,
                    align_items: AlignItems::FlexStart,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                //background_color: RED.into(),
                ..default()
            },
            Pickable::IGNORE,
        ));

        entity
    }

    fn upgrade_shop(&mut self) -> EntityCommands {
        let entity = self.spawn((
            Name::new("UpgradeShop"),
            NodeBundle {
                style: Style {
                    width: Px(250.),
                    height: Percent(100.),
                    margin: UiRect::top(Px(8.)),
                    ..default()
                },
                ..default()
            },
            UpgradeShop,
            Pickable::IGNORE,
        ));

        entity
    }

    fn score_display(&mut self, font: Handle<Font>) -> EntityCommands {
        let mut entity = self.spawn((
            Name::new("ScoreDisplay"),
            NodeBundle {
                style: Style {
                    min_width: Px(250.),
                    min_height: Px(400.),
                    margin: UiRect::top(Px(8.)),
                    ..default()
                },
                ..default()
            },
            Pickable::IGNORE,
        ));

        entity.with_children(|score_display| {
            score_display
                .vertical_container(JustifyContent::Start)
                .with_children(|score_display_container| {
                    score_display_container
                        .scoreboard_cycles_text(format!("Cycle {}", 1.), font.clone());
                    score_display_container
                        .scoreboard_currency_text(format!("$ {}", 0.), font.clone());
                });
        });

        entity
    }

    fn hotbar(&mut self, starting_colors: Vec<SocketColor>) -> EntityCommands {
        let entity = self.spawn((
            Name::new("Hotbar"),
            NodeBundle {
                style: Style {
                    min_width: Px(500.),
                    min_height: Px(100.),
                    display: Display::Flex,
                    align_items: AlignItems::Center,
                    ..default()
                },
                //background_color: ORANGE.into(),
                ..default()
            },
            Hotbar {
                color_mappings: starting_colors,
                ..default()
            },
            Pickable::IGNORE,
        ));

        entity
    }

    fn hotbar_description(&mut self, starting_text: impl Into<String>, socket_color: SocketColor, font: Handle<Font>, socket_material: Handle<SocketUiMaterial>) -> EntityCommands {
        let mut entity = self.spawn((
            Name::new("HotbarDescription"),
            NodeBundle {
                style: Style {
                    display: Display::Flex,
                    align_items: AlignItems::Center,
                    margin: UiRect::left(Px(8.)),
                    ..default()
                },
                ..default()
            },
            HotbarDescription,
            Pickable::IGNORE,
        ));

        entity.with_children(|children| {
            children.spawn((
                Name::new("HotbarDescriptionIcon"),
                MaterialNodeBundle {
                    style: Style {
                        width: Px(16.),
                        height: Px(16.),
                        bottom: Px(1.),
                        left: Px(8.),
                        margin: UiRect::right(Px(16.)),
                        ..default()
                    },
                    material: socket_material,
                    ..default()
                },
                HotbarDescriptionIcon {
                    current_socket_color: socket_color,
                }
            ));

            children.spawn((
                Name::new("HotbarDescriptionText"),
                TextBundle::from_section(
                    starting_text,
                    TextStyle {
                        font,
                        font_size: 16.0,
                        color: WHITE.into(),
                        ..default()
                    },
                ),
                HotbarDescriptionText
            ));
        });

        entity
    }

    fn hotbar_button(&mut self, socket_material: Handle<SocketUiMaterial>, hotkey_text: impl Into<String>, index: u32) -> EntityCommands {
        let mut entity = self.spawn((
            Name::new("HotbarButton"),
            NodeBundle {
                style: Style {
                    width: Px(64.),
                    height: Px(64.),
                    margin: UiRect::left(Px(8.)),
                    display: Display::Flex,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                background_color: LinearRgba::BLACK.into(),
                border_color: Color::BLACK.into(),
                ..default()
            },
            HotbarButton { index },
        ));

        entity.with_children(|hotbar_button| {
            hotbar_button.spawn((
                MaterialNodeBundle {
                    style: Style {
                        width: Px(32.),
                        height: Px(32.),
                        ..default()
                    },
                    material: socket_material,
                    ..default()
                },
                Pickable::IGNORE,
            ));

            hotbar_button
                .spawn((
                    Name::new("Hotbar Text"),
                    TextBundle::from_section(
                        hotkey_text,
                        TextStyle {
                            font_size: 12.0,
                            color: GRAY_200.into(),
                            ..default()
                        },
                    ),
                    Pickable::IGNORE,
                ))
                .insert(Style {
                    position_type: PositionType::Absolute,
                    top: Px(2.),
                    left: Px(2.),
                    ..default()
                });
        });

        entity
    }

    fn socket(&mut self, socket_material: Handle<SocketUiMaterial>) -> EntityCommands {
        let entity = self.spawn(
            MaterialNodeBundle {
                style: Style {
                    width: Px(64.),
                    height: Px(64.),
                    ..default()
                },
                material: socket_material,
                ..default()
            },
        );

        entity
    }
}

#[derive(Component)]
pub struct CyclesCountText;

#[derive(Component)]
pub struct CurrencyText;

#[derive(Component)]
pub struct CycleRow {
    row_number: u32,
}

#[derive(Component, Default)]
pub struct Hotbar {
    pub selected_index: u32,
    pub color_mappings: Vec<SocketColor>,
}

#[derive(Component)]
pub struct HotbarDescription;

#[derive(Component)]
pub struct HotbarDescriptionText;

#[derive(Component)]
pub struct HotbarDescriptionIcon {
    pub current_socket_color: SocketColor,
}

#[derive(Component)]
pub struct HotbarButton {
    pub index: u32,
}

#[derive(Component)]
pub struct UpgradeShop;

/// An extension trait for spawning UI containers.
pub trait Containers {
    /// Spawns a root node that covers the full screen
    /// and centers its content horizontally and vertically.
    fn ui_root(&mut self) -> EntityCommands;
}

impl Containers for Commands<'_, '_> {
    fn ui_root(&mut self) -> EntityCommands {
        self.spawn((
            Name::new("UI Root"),
            NodeBundle {
                style: Style {
                    width: Percent(100.0),
                    height: Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    row_gap: Px(10.0),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                ..default()
            },
            Pickable {
                should_block_lower: false,
                is_hoverable: false,
            },
        ))
    }
}

/// An internal trait for types that can spawn entities.
/// This is here so that [`Widgets`] can be implemented on all types that
/// are able to spawn entities.
/// Ideally, this trait should be [part of Bevy itself](https://github.com/bevyengine/bevy/issues/14231).
trait Spawn {
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands;
}

impl Spawn for Commands<'_, '_> {
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands {
        self.spawn(bundle)
    }
}

impl Spawn for ChildBuilder<'_> {
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands {
        self.spawn(bundle)
    }
}
