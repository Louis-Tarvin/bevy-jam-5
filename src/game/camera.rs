use bevy::{input::mouse::MouseMotion, prelude::*};

use crate::AppSet;

use super::{build::BuildLocationMarker, phase::GamePhase};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<CameraTarget>();
    app.add_systems(
        Update,
        (pan_camera_target, reset_camera_target)
            .run_if(in_state(GamePhase::Build))
            .in_set(AppSet::Update),
    );
    app.add_systems(Update, move_camera_to_target.in_set(AppSet::PostUpdate));
}

#[derive(Resource, Default)]
pub struct CameraTarget(pub Option<Entity>);

fn move_camera_to_target(
    camera_target: Res<CameraTarget>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
    target_query: Query<&GlobalTransform, Without<Camera>>,
) {
    if let Some(target) = camera_target.0 {
        if let Ok(target_transform) = target_query.get(target) {
            for mut camera_transform in camera_query.iter_mut() {
                camera_transform.translation = camera_transform
                    .translation
                    .lerp(target_transform.translation(), 0.1);
            }
        }
    }
}

const PAN_SENSITIVITY: f32 = 0.1;

fn pan_camera_target(
    camera_target: Res<CameraTarget>,
    mut target_query: Query<&mut Transform>,
    mut mouse_motion: EventReader<MouseMotion>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    build_marker: Query<&BuildLocationMarker>,
) {
    let mut total_motion: Vec2 = mouse_motion.read().map(|ev| ev.delta).sum();
    total_motion.y = -total_motion.y;

    // Hacky way to prevent panning when placing a building
    if let Some(marker) = build_marker.iter().next() {
        if marker.mode.is_some() {
            return;
        }
    }

    if mouse_input.pressed(MouseButton::Left) {
        if let Some(target) = camera_target.0 {
            if let Ok(mut target_transform) = target_query.get_mut(target) {
                target_transform.translation.x -= total_motion.x * PAN_SENSITIVITY;
                target_transform.translation.y -= total_motion.y * PAN_SENSITIVITY;
            }
        }
    }
}

fn reset_camera_target(
    camera_target: Res<CameraTarget>,
    mut target_query: Query<&mut Transform>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Space) || input.just_pressed(KeyCode::KeyE) {
        if let Some(target) = camera_target.0 {
            if let Ok(mut target_transform) = target_query.get_mut(target) {
                target_transform.translation.x = 0.0;
                target_transform.translation.y = 0.0;
            }
        }
    }
}
