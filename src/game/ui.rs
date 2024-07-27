use bevy::prelude::*;

use crate::{screen::Screen, ui::prelude::*};

use self::ui_palette::NODE_BACKGROUND;

use super::{
    assets::{HandleMap, ImageKey},
    build::BuildAction,
    gameplay::{GameplayManager, Resources},
    phase::PHASE_DURATION,
    spawn::building::BuildingType,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (update_resource_count, update_resource_gathered_count)
            .run_if(in_state(Screen::Playing).and_then(resource_changed::<Resources>)),
    );
    app.add_systems(Update, update_spinner.run_if(in_state(Screen::Playing)));
}

#[derive(Component)]
pub struct ResourceCountUi;

#[derive(Component)]
pub struct SpinnerCoreUi;

#[derive(Component)]
pub struct BuildDockUi;

#[derive(Component)]
pub struct GatherDockUi;

#[derive(Component)]
pub struct GatherResourceCountUi;

pub fn draw_ui(mut commands: Commands, image_handles: Res<HandleMap<ImageKey>>) {
    let style = TextStyle {
        font_size: 20.0,
        color: Color::WHITE,
        ..Default::default()
    };
    commands
        .ui_root()
        .insert(StateScoped(Screen::Playing))
        .with_children(|parent| {
            parent.spawn((
                Name::new("ResourceCount"),
                ResourceCountUi,
                TextBundle::from_sections([
                    TextSection::new("Resources: ", style.clone()),
                    TextSection::from_style(style.clone()),
                ])
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(10.0),
                    left: Val::Px(10.0),
                    ..Default::default()
                }),
            ));

            parent.spawn((
                Name::new("SpinnerFrame"),
                ImageBundle {
                    image: image_handles[&ImageKey::SpinnerFrame].clone_weak().into(),
                    style: Style {
                        position_type: PositionType::Absolute,
                        top: Val::Px(10.0),
                        right: Val::Px(10.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ));
            parent.spawn((
                Name::new("SpinnerCore"),
                SpinnerCoreUi,
                ImageBundle {
                    image: image_handles[&ImageKey::SpinnerCore].clone_weak().into(),
                    z_index: ZIndex::Local(-1),
                    style: Style {
                        position_type: PositionType::Absolute,
                        top: Val::Px(10.0),
                        right: Val::Px(10.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ));

            parent.dock().insert(BuildDockUi).with_children(|parent| {
                parent.building_button(
                    "Decoy",
                    "A decoy to distract enemies",
                    10,
                    BuildAction(BuildingType::Decoy),
                );
                parent.building_button(
                    "Turret",
                    "A turret to shoot enemies",
                    20,
                    BuildAction(BuildingType::Turret),
                );
            });

            parent
                .dock()
                .insert(GatherDockUi)
                .insert(Visibility::Hidden)
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Px(150.0),
                                height: Val::Px(60.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            background_color: BackgroundColor(NODE_BACKGROUND),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent.spawn((
                                Name::new("GatherResourceCount"),
                                GatherResourceCountUi,
                                TextBundle::from_sections([
                                    TextSection::new("Resources: ", style.clone()),
                                    TextSection::from_style(style),
                                ])
                                .with_style(Style {
                                    position_type: PositionType::Absolute,
                                    bottom: Val::Px(10.0),
                                    left: Val::Px(10.0),
                                    ..Default::default()
                                }),
                            ));
                        });
                });
        });
}

fn update_resource_count(
    mut query: Query<(&ResourceCountUi, &mut Text)>,
    resources: Res<Resources>,
) {
    for (_, mut text) in query.iter_mut() {
        text.sections[1].value = resources.delivered.to_string();
    }
}

fn update_resource_gathered_count(
    mut text_query: Query<&mut Text, With<GatherResourceCountUi>>,
    resources: Res<Resources>,
) {
    for mut text in text_query.iter_mut() {
        text.sections[1].value = resources.gathered.to_string();
    }
}

fn update_spinner(
    mut query: Query<&mut Transform, With<SpinnerCoreUi>>,
    manager: Res<GameplayManager>,
) {
    for mut transform in query.iter_mut() {
        let total_duration = PHASE_DURATION * 3.0;
        let angle = ((2.0 * std::f32::consts::PI) / total_duration)
            * (total_duration - manager.elapsed_time);
        transform.rotation = Quat::from_rotation_z(angle);
    }
}
