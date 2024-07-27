//! Helper traits for creating common widgets.

use bevy::{
    color::palettes::{
        css::{BLACK, BLUE, GREEN, LIGHT_CYAN, LIGHT_GREEN, MAGENTA, ORANGE, PINK, PURPLE, RED, WHITE},
        tailwind::{
            GRAY_100, GRAY_200, GRAY_300, GRAY_400, GRAY_500, GRAY_600, GRAY_700, GRAY_800,
            GRAY_900,
        },
    },
    ecs::system::EntityCommands,
    math::VectorSpace,
    prelude::*,
    ui::Val::*,
};
use bevy_mod_picking::{events::{Click, Pointer}, picking_core::Pickable, prelude::On};
use num_bigint::BigUint;

use crate::game::{
    materials::materials::{SocketMaterial, SocketUiMaterial},
    spawn::level::SocketColor,
};

use super::{interaction::InteractionPalette, palette::*, shop::UpgradeButtonsContainer};

/// An extension trait for spawning UI widgets.
pub trait Widgets {
    /// Spawn a simple button with text.
    fn button(&mut self, text: impl Into<String>) -> EntityCommands;

    /// Spawn a simple header label. Bigger than [`Widgets::label`].
    fn header(&mut self, text: impl Into<String>) -> EntityCommands;

    /// Spawn a simple text label.
    fn label(&mut self, text: impl Into<String>) -> EntityCommands;

    fn horizontal_container(&mut self, justify_content: JustifyContent, align_items: AlignItems) -> EntityCommands;

    fn vertical_container(&mut self, justify_content: JustifyContent, gap: Val) -> EntityCommands;

    fn upgrade_shop(&mut self, font: Handle<Font>) -> EntityCommands;
    fn shop_button(
        &mut self,
        price: &BigUint,
        description: impl Into<String>,
        font: Handle<Font>,
    ) -> EntityCommands;

    fn scoreboard_cycles_text(
        &mut self,
        font: Handle<Font>,
    ) -> EntityCommands;

    fn scoreboard_currency_text(
        &mut self,
        font: Handle<Font>,
    ) -> EntityCommands;

    fn score_display(&mut self, font: Handle<Font>) -> EntityCommands;

    fn hotbar_description(
        &mut self,
        starting_text: impl Into<String>,
        socket_color: SocketColor,
        font: Handle<Font>,
        socket_material: Handle<SocketUiMaterial>,
    ) -> EntityCommands;

    fn hotbar(&mut self, starting_colors: Vec<SocketColor>) -> EntityCommands;

    fn hotbar_button(
        &mut self,
        socket_material: Handle<SocketUiMaterial>,
        hotkey_text: impl Into<String>,
        index: u32,
    ) -> EntityCommands;

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

    fn shop_button(
        &mut self,
        price: &BigUint,
        description: impl Into<String>,
        font: Handle<Font>,
    ) -> EntityCommands {
        let mut entity = self.spawn((
            Name::new("Button"),
            ButtonBundle {
                style: Style {
                    width: Px(200.0),
                    height: Px(65.0),
                    display: Display::Flex,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Px(1.)),
                    ..default()
                },
                background_color: GRAY_700.into(),
                border_color: GRAY_400.into(),
                ..default()
            },
            InteractionPalette {
                none: GRAY_700.into(),
                hovered: GRAY_600.into(),
                pressed: GRAY_500.into(),
            },
            ShopButton {
                price: price.clone(),
            },
        ));

        entity.with_children(|children| {
            children
                .horizontal_container(JustifyContent::Start, AlignItems::Center)
                .with_children(|text_container| {
                    text_container
                        .spawn((
                            Name::new("Button Price Text"),
                            TextBundle::from_section(
                                format!("${}", price),
                                TextStyle {
                                    font_size: 14.0,
                                    color: ORANGE.into(),
                                    font: font.clone(),
                                    ..default()
                                },
                            ),
                        ))
                        .insert(Style {
                            margin: UiRect::all(Px(8.)),
                            ..default()
                        });

                    text_container
                        .spawn((
                            Name::new("Button Description Text"),
                            TextBundle::from_section(
                                description,
                                TextStyle {
                                    font_size: 14.0,
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
        font: Handle<Font>,
    ) -> EntityCommands {
        let mut entity = self.spawn((
            Name::new("Scoreboard Text"),
            NodeBundle {
                style: Style {
                    padding: UiRect::all(Px(8.)),
                    display: Display::Flex,
                    row_gap: Px(8.),
                    column_gap: Px(8.),
                    ..default()
                },
                ..default()
            },
        ));

        entity.with_children(|children| {
            children.spawn((
                Name::new("Scoreboard Description Text"),
                TextBundle::from_section(
                    String::from("Cycles"),
                    TextStyle {
                        font: font.clone(),
                        font_size: 16.,
                        ..default()
                    },
                ),
            ));

            children.spawn((
                Name::new("Scoreboard Count Text"),
                TextBundle::from_section(
                    String::from("0"),
                    TextStyle {
                        font: font,
                        font_size: 16.,
                        color: LIGHT_GREEN.into(),
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
        font: Handle<Font>,
    ) -> EntityCommands {
        let mut entity = self.spawn((
            Name::new("Scoreboard Text"),
            NodeBundle {
                style: Style {
                    padding: UiRect::all(Px(8.)),
                    display: Display::Flex,
                    ..default()
                },
                ..default()
            },
        ));

        entity.with_children(|children| {
            children.spawn((
                Name::new("Scoreboard Text Text"),
                TextBundle::from_section(
                    String::from("$0"),
                    TextStyle {
                        font: font.clone(),
                        font_size: 16.,
                        color: ORANGE.into(),
                        ..default()
                    },
                ),
                CurrencyText,
            ));

            children.spawn((
                Name::new("Scoreboard Text Text"),
                TextBundle::from_section(
                    String::from("(Pending"),
                    TextStyle {
                        font: font.clone(),
                        font_size: 16.,
                        color: WHITE.into(),
                        ..default()
                    },
                ),
            ));

            children.spawn((
                Name::new("Scoreboard Text Text"),
                TextBundle::from_section(
                    String::from(" $0"),
                    TextStyle {
                        font: font.clone(),
                        font_size: 16.,
                        color: ORANGE.into(),
                        ..default()
                    },
                ),
                PendingCurrencyText
            ));

            children.spawn((
                Name::new("Scoreboard Text Text"),
                TextBundle::from_section(
                    String::from(")"),
                    TextStyle {
                        font: font,
                        font_size: 16.,
                        color: WHITE.into(),
                        ..default()
                    },
                ),
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

    fn horizontal_container(&mut self, justify_content: JustifyContent, align_items: AlignItems) -> EntityCommands {
        let entity = self.spawn((
            Name::new("FlexWrapper"),
            NodeBundle {
                style: Style {
                    width: Percent(100.),
                    height: Percent(100.),
                    display: Display::Flex,
                    justify_content,
                    align_items,
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

    fn vertical_container(&mut self, justify_content: JustifyContent, gap: Val) -> EntityCommands {
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
                    row_gap: gap,
                    column_gap: gap,
                    ..default()
                },
                //background_color: RED.into(),
                ..default()
            },
            Pickable::IGNORE,
        ));

        entity
    }

    fn upgrade_shop(&mut self, font: Handle<Font>) -> EntityCommands {
        let mut shop_entity = self.spawn((
            Name::new("UpgradeShop"),
            NodeBundle {
                style: Style {
                    width: Px(250.),
                    margin: UiRect::top(Px(8.)).with_right(Px(8.)),
                    padding: UiRect::all(Px(8.)),
                    border: UiRect::all(Px(2.)),
                    ..default()
                },
                background_color: BackgroundColor(GRAY_800.into()),
                border_color: GRAY_900.into(),
                ..default()
            },
            UpgradeShop,
            Pickable::IGNORE,
        ));

        shop_entity.with_children(|shop_children| {
            shop_children
                .vertical_container(JustifyContent::Start, Px(8.)) // container for header AND buttons
                .with_children(|shop_vertical_children| {
                    shop_vertical_children
                        .spawn(TextBundle::from_section(
                            String::from("Upgrade Shop"),
                            TextStyle {
                                font,
                                font_size: 16.0,
                                color: WHITE.into(),
                                ..default()
                            },
                        ))
                        .insert(Style {
                            padding: UiRect::bottom(Px(16.)),
                            ..default()
                        });

                    shop_vertical_children
                        .vertical_container(JustifyContent::Start, Px(8.))
                        .insert(UpgradeButtonsContainer);
                });
        });

        shop_entity
    }

    fn score_display(&mut self, font: Handle<Font>) -> EntityCommands {
        let mut entity = self.spawn((
            Name::new("ScoreDisplay"),
            NodeBundle {
                style: Style {
                    min_width: Px(250.),
                    margin: UiRect::top(Px(8.)).with_left(Px(8.)),
                    border: UiRect::all(Px(2.)),
                    ..default()
                },
                background_color: BackgroundColor(GRAY_800.into()),
                border_color: BorderColor(GRAY_900.into()),
                ..default()
            },
            Pickable::IGNORE,
        ));

        entity.with_children(|score_display| {
            score_display
                .vertical_container(JustifyContent::Start, Px(0.))
                .with_children(|score_display_container| {
                    score_display_container
                        .scoreboard_cycles_text(font.clone());
                    score_display_container
                        .scoreboard_currency_text(font.clone());
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

    fn hotbar_description(
        &mut self,
        starting_text: impl Into<String>,
        socket_color: SocketColor,
        font: Handle<Font>,
        socket_material: Handle<SocketUiMaterial>,
    ) -> EntityCommands {
        let mut entity = self.spawn((
            Name::new("HotbarDescription"),
            NodeBundle {
                style: Style {
                    display: Display::Flex,
                    align_items: AlignItems::Center,
                    margin: UiRect::left(Px(8.)),
                    border: UiRect::all(Px(2.)),
                    padding: UiRect::all(Px(8.)),
                    max_width: Px(300.),
                    ..default()
                },
                background_color: GRAY_800.into(),
                border_color: GRAY_900.into(),
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
                        min_width: Px(16.),
                        min_height: Px(16.),
                        max_width: Px(16.),
                        max_height: Px(16.),
                        margin: UiRect::right(Px(8.)),
                        ..default()
                    },
                    material: socket_material,
                    ..default()
                },
                HotbarDescriptionIcon {
                    current_socket_color: socket_color,
                },
            ));

            children.spawn((
                Name::new("HotbarDescriptionText"),
                TextBundle::from_section(
                    starting_text,
                    TextStyle {
                        font,
                        font_size: 13.0,
                        color: WHITE.into(),
                        ..default()
                    },
                ),
                HotbarDescriptionText,
            ));
        });

        entity
    }

    fn hotbar_button(
        &mut self,
        socket_material: Handle<SocketUiMaterial>,
        hotkey_text: impl Into<String>,
        index: u32,
    ) -> EntityCommands {
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
                background_color: GRAY_800.into(),
                border_color: GRAY_900.into(),
                ..default()
            },
            HotbarButton { index },
            On::<Pointer<Click>>::commands_mut(move |_, c| {
                c.trigger(HotbarChanged {
                    index
                })
            })
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
        let entity = self.spawn(MaterialNodeBundle {
            style: Style {
                width: Px(64.),
                height: Px(64.),
                ..default()
            },
            material: socket_material,
            ..default()
        });

        entity
    }
}

#[derive(Component)]
pub struct CyclesCountText;

#[derive(Component)]
pub struct CurrencyText;

#[derive(Component)]
pub struct PendingCurrencyText;

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

#[derive(Component)]
pub struct ShopButton {
    pub price: BigUint,
}

#[derive(Event)]
pub struct HotbarChanged {
    pub index: u32,
}

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
