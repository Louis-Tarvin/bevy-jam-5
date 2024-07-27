//! Helper traits for creating common widgets.

use bevy::{ecs::system::EntityCommands, prelude::*, ui::Val::*};

use super::{interaction::InteractionPalette, palette::*};

/// An extension trait for spawning UI widgets.
pub trait Widgets {
    /// Spawn a simple button with text.
    fn button(&mut self, text: impl Into<String>) -> EntityCommands;

    /// Spawn a simple header label. Bigger than [`Widgets::label`].
    fn header(&mut self, text: impl Into<String>) -> EntityCommands;

    /// Spawn a simple text label.
    fn label(&mut self, text: impl Into<String>) -> EntityCommands;

    fn tooltip_label(&mut self, text: impl Into<String>) -> EntityCommands;

    fn dock(&mut self) -> EntityCommands;

    fn building_button(
        &mut self,
        name: impl Into<String>,
        description: impl Into<String>,
        cost: u32,
        component: impl Component,
    ) -> EntityCommands;
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

    fn tooltip_label(&mut self, text: impl Into<String>) -> EntityCommands {
        let mut entity = self.spawn((
            Name::new("Label"),
            NodeBundle {
                style: Style {
                    width: Px(600.0),
                    justify_content: JustifyContent::Start,
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
                        font_size: 20.0,
                        color: LABEL_TEXT,
                        ..default()
                    },
                ),
            ));
        });
        entity
    }

    fn dock(&mut self) -> EntityCommands {
        self.spawn((
            Name::new("Dock"),
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    bottom: Px(10.0),
                    width: Auto,
                    height: Auto,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Row,
                    row_gap: Px(10.0),
                    padding: UiRect::all(Px(10.0)),
                    ..default()
                },
                background_color: BackgroundColor(NODE_BORDER),
                ..default()
            },
        ))
    }

    fn building_button(
        &mut self,
        name: impl Into<String>,
        description: impl Into<String>,
        cost: u32,
        component: impl Component,
    ) -> EntityCommands {
        let mut entity = self.spawn((
            Name::new("Building Button Container"),
            NodeBundle {
                style: Style {
                    width: Px(200.0),
                    height: Px(160.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    row_gap: Px(10.0),
                    padding: UiRect::all(Px(10.0)),
                    margin: UiRect {
                        left: Px(5.0),
                        right: Px(5.0),
                        ..default()
                    },
                    ..default()
                },
                background_color: BackgroundColor(NODE_BACKGROUND),
                ..default()
            },
        ));
        entity.with_children(|children| {
            children.spawn((
                Name::new("Building Name"),
                TextBundle::from_section(
                    name,
                    TextStyle {
                        font_size: 20.0,
                        color: BUTTON_TEXT,
                        ..default()
                    },
                ),
            ));
            children.spawn((
                Name::new("Building Description"),
                TextBundle::from_section(
                    description,
                    TextStyle {
                        font_size: 13.0,
                        color: LABEL_TEXT,
                        ..default()
                    },
                ),
            ));
            children.spawn((
                Name::new("Building Cost"),
                TextBundle::from_section(
                    format!("Cost: {}", cost),
                    TextStyle {
                        font_size: 13.0,
                        color: LABEL_TEXT,
                        ..default()
                    },
                ),
            ));
            children
                .spawn((
                    Name::new("Building Button"),
                    ButtonBundle {
                        style: Style {
                            width: Px(150.0),
                            height: Px(45.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: BackgroundColor(BUILDING_BUTTON_BACKGROUND),
                        ..default()
                    },
                    InteractionPalette {
                        none: BUILDING_BUTTON_BACKGROUND,
                        hovered: BUTTON_HOVERED_BACKGROUND,
                        pressed: BUTTON_PRESSED_BACKGROUND,
                    },
                    component,
                ))
                .with_children(|button_children| {
                    button_children.spawn((
                        Name::new("Button Text"),
                        TextBundle::from_section(
                            "Build",
                            TextStyle {
                                font_size: 20.0,
                                color: BUTTON_TEXT,
                                ..default()
                            },
                        ),
                    ));
                });
        });

        entity
    }
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
