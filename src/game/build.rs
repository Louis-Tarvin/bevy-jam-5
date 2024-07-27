use std::time::Duration;

use bevy::{prelude::*, window::PrimaryWindow};

use crate::{screen::Screen, ui::interaction::InteractionQuery, AppSet};

use super::{
    assets::SfxKey,
    audio::sfx::PlaySfx,
    phase::GamePhase,
    spawn::building::{BuildingType, SpawnBuilding},
};

pub(super) fn plugin(app: &mut App) {
    app.add_event::<EnterBuildMode>();
    app.register_type::<BuildLocationMarker>();
    app.add_systems(OnEnter(Screen::Playing), init_marker);
    app.add_systems(OnExit(GamePhase::Build), reset_marker);
    app.add_systems(Update, tick_debounce.in_set(AppSet::TickTimers));
    app.add_systems(
        Update,
        update_mouse_pos
            .run_if(in_state(GamePhase::Build))
            .in_set(AppSet::RecordInput),
    );
    app.add_systems(
        Update,
        (handle_build_action, listen_for_build_mode, update_marker)
            .chain()
            .run_if(in_state(GamePhase::Build))
            .in_set(AppSet::Update),
    );
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
struct BuildLocationMarker {
    mouse_world_pos: Vec2,
    mode: Option<BuildingType>,
    just_clicked: bool,
    click_debounce: Timer,
}

#[derive(Event, Debug)]
pub struct EnterBuildMode(pub BuildingType);

fn listen_for_build_mode(
    mut event_reader: EventReader<EnterBuildMode>,
    mut marker_query: Query<&mut BuildLocationMarker>,
) {
    for event in event_reader.read() {
        for mut marker in &mut marker_query {
            marker.mode = Some(event.0);
            marker
                .click_debounce
                .set_duration(Duration::from_secs_f32(0.5));
            marker.click_debounce.reset();
        }
    }
}

fn tick_debounce(mut marker_query: Query<&mut BuildLocationMarker>, time: Res<Time>) {
    for mut marker in &mut marker_query {
        marker.click_debounce.tick(time.delta());
    }
}

fn init_marker(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        BuildLocationMarker::default(),
        PbrBundle {
            mesh: meshes.add(Mesh::from(Cuboid::default())),
            material: materials.add(Color::WHITE),
            visibility: Visibility::Hidden,
            ..Default::default()
        },
        StateScoped(Screen::Playing),
    ));
}

fn reset_marker(mut marker_query: Query<(&mut BuildLocationMarker, &mut Visibility)>) {
    for (mut marker, mut visibility) in marker_query.iter_mut() {
        marker.mode = None;
        *visibility = Visibility::Hidden;
    }
}

fn update_mouse_pos(
    input: Res<ButtonInput<MouseButton>>,
    camera: Query<(&Camera, &GlobalTransform)>,
    window: Query<&Window, With<PrimaryWindow>>,
    mut marker_query: Query<&mut BuildLocationMarker>,
) {
    // Record mouse world position.
    let (camera, camera_transform) = camera.single();
    if let Ok(window) = window.get_single() {
        if let Some(ray) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        {
            if let Some(distance) = ray.intersect_plane(Vec3::ZERO, InfinitePlane3d::new(Vec3::Z)) {
                let world_position = ray.get_point(distance);
                for mut marker in &mut marker_query {
                    marker.mouse_world_pos = world_position.truncate();
                }
            }
        }
    }

    // Record attack input.
    for mut marker in &mut marker_query {
        marker.just_clicked =
            input.just_released(MouseButton::Left) && marker.click_debounce.finished();
    }
}

fn update_marker(
    mut marker_query: Query<(&mut BuildLocationMarker, &mut Transform, &mut Visibility)>,
    mut commands: Commands,
) {
    for (mut marker, mut transform, mut visibility) in marker_query.iter_mut() {
        transform.translation = marker.mouse_world_pos.extend(0.0);
        if marker.just_clicked {
            if let Some(building_type) = marker.mode.take() {
                commands.trigger(SpawnBuilding {
                    building_type,
                    position: marker.mouse_world_pos.extend(0.0),
                });
                commands.trigger(PlaySfx::Key(SfxKey::Build));
            }
        }
        *visibility = if marker.mode.is_some() {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

#[derive(Component, Debug)]
pub struct BuildAction(pub BuildingType);

fn handle_build_action(
    mut button_query: InteractionQuery<&BuildAction>,
    mut event_writer: EventWriter<EnterBuildMode>,
) {
    for (interaction, action) in &mut button_query {
        if matches!(interaction, Interaction::Pressed) {
            event_writer.send(EnterBuildMode(action.0));
        }
    }
}
