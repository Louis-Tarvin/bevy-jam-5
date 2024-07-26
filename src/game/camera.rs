use bevy::prelude::*;

use crate::AppSet;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<CameraTarget>();
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
