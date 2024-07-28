use std::time::Duration;

use bevy::{prelude::*, window::PrimaryWindow};

use crate::{screen::Screen, ui::interaction::InteractionQuery, AppSet};

use super::{
    assets::SfxKey,
    audio::sfx::PlaySfx,
    camera::CameraTarget,
    gameplay::Resources,
    notifications::Notification,
    phase::GamePhase,
    spawn::{
        asteroid::{Asteroid, ASTEROID_WAYPOINT_COLOR},
        building::{BuildingType, Destructable, SpawnBuilding},
    },
    waypoint::Waypointed,
};

pub(super) fn plugin(app: &mut App) {
    app.add_event::<EnterBuildMode>();
    app.register_type::<BuildLocationMarker>();
    app.add_systems(OnEnter(Screen::Playing), init_marker);
    app.add_systems(OnExit(GamePhase::Build), reset_marker);
    app.add_systems(Update, tick_debounce.in_set(AppSet::TickTimers));
    app.add_systems(
        Update,
        (
            update_mouse_pos,
            exit_build_mode_on_esc,
            toggle_camera_distance,
        )
            .run_if(in_state(GamePhase::Build))
            .in_set(AppSet::RecordInput),
    );
    app.add_systems(
        Update,
        (
            (handle_build_action, listen_for_build_mode, update_marker).chain(),
            scan,
        )
            .run_if(in_state(GamePhase::Build))
            .in_set(AppSet::Update),
    );
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct BuildLocationMarker {
    mouse_world_pos: Vec2,
    pub mode: Option<BuildingType>,
    just_clicked: bool,
    click_debounce: Timer,
    white_material: Handle<StandardMaterial>,
    red_material: Handle<StandardMaterial>,
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
    let white_material = materials.add(Color::WHITE);
    let red_material = materials.add(Color::srgb(1.0, 0.0, 0.0));
    commands.spawn((
        BuildLocationMarker {
            white_material: white_material.clone(),
            red_material,
            ..Default::default()
        },
        PbrBundle {
            mesh: meshes.add(Mesh::from(Cuboid::new(2.0, 2.0, 2.0))),
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
    mouse_input: Res<ButtonInput<MouseButton>>,
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

    // Record mouse click.
    for mut marker in &mut marker_query {
        marker.just_clicked =
            mouse_input.just_released(MouseButton::Left) && marker.click_debounce.finished();
    }
}

fn exit_build_mode_on_esc(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut marker_query: Query<&mut BuildLocationMarker>,
    mut resources: ResMut<Resources>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        for mut marker in &mut marker_query {
            if let Some(building_type) = marker.mode.take() {
                // refund resources
                resources.delivered += building_type.cost();
            }
        }
    }
}

fn update_marker(
    mut marker_query: Query<
        (
            &mut BuildLocationMarker,
            &mut Transform,
            &mut Visibility,
            &mut Handle<StandardMaterial>,
        ),
        Changed<BuildLocationMarker>,
    >,
    mut commands: Commands,
    buildings_query: Query<&Transform, (With<Destructable>, Without<BuildLocationMarker>)>,
    mut notification_writer: EventWriter<Notification>,
) {
    for (mut marker, mut transform, mut visibility, mut material) in marker_query.iter_mut() {
        transform.translation = marker.mouse_world_pos.extend(0.0);
        let mut can_build = true;
        for building_transform in buildings_query.iter() {
            if building_transform
                .translation
                .xy()
                .distance_squared(marker.mouse_world_pos)
                < 100.0
            {
                can_build = false;
                break;
            }
        }
        if can_build {
            *material = marker.white_material.clone();
        } else {
            *material = marker.red_material.clone();
        }
        if marker.just_clicked && marker.mode.is_some() {
            if can_build {
                if let Some(building_type) = marker.mode.take() {
                    commands.trigger(SpawnBuilding {
                        building_type,
                        position: marker.mouse_world_pos.extend(0.0),
                    });
                    commands.trigger(PlaySfx::Key(SfxKey::Build));
                }
            } else {
                notification_writer
                    .send(Notification("Too close to existing structure".to_string()));
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
pub struct BuildAction {
    pub building_type: BuildingType,
}

fn handle_build_action(
    mut button_query: InteractionQuery<&BuildAction>,
    mut event_writer: EventWriter<EnterBuildMode>,
    mut notification_writer: EventWriter<Notification>,
    mut resources: ResMut<Resources>,
) {
    for (interaction, action) in &mut button_query {
        if matches!(interaction, Interaction::Pressed) {
            if resources.delivered < action.building_type.cost() {
                notification_writer.send(Notification("Not enough resources".to_string()));
                continue;
            }
            resources.delivered -= action.building_type.cost();
            event_writer.send(EnterBuildMode(action.building_type));
        }
    }
}

fn scan(
    mouse_input: Res<ButtonInput<MouseButton>>,
    asteroids_query: Query<(Entity, &Transform), With<Asteroid>>,
    location_marker_query: Query<&BuildLocationMarker>,
    mut notification_writer: EventWriter<Notification>,
    mut commands: Commands,
) {
    if mouse_input.just_pressed(MouseButton::Right) {
        let marker = location_marker_query.iter().next().unwrap();
        let scan_pos = marker.mouse_world_pos;
        let mut nearest_distance = f32::MAX;
        for (entity, transform) in asteroids_query.iter() {
            let sq_distance = transform.translation.xy().distance_squared(scan_pos);
            if sq_distance < nearest_distance {
                nearest_distance = sq_distance;
            }
            if sq_distance < 400.0 {
                commands
                    .entity(entity)
                    .insert(Visibility::Visible)
                    .insert(Waypointed::new(ASTEROID_WAYPOINT_COLOR));
            }
        }
        if nearest_distance < 400.0 {
            notification_writer.send(Notification("Asteroid detected".to_string()));
        } else {
            notification_writer.send(Notification(format!(
                "Nearest asteroid: {:.2} units away",
                nearest_distance,
            )));
        }
    }
}

fn toggle_camera_distance(
    camera_target: Res<CameraTarget>,
    mut target_query: Query<&mut Transform>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::KeyZ) {
        if let Some(target) = camera_target.0 {
            if let Ok(mut target_transform) = target_query.get_mut(target) {
                if target_transform.translation.z < 250.0 {
                    target_transform.translation.z = 600.0;
                } else {
                    target_transform.translation.z = 200.0;
                }
            }
        }
    }
}
