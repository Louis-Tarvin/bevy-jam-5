use bevy::prelude::*;

use crate::AppSet;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, destroy_after_secs.in_set(AppSet::TickTimers));
    app.add_systems(Update, spin.in_set(AppSet::Update));
}

#[derive(Component)]
pub struct DestroyAfterSecs {
    timer: Timer,
}
impl DestroyAfterSecs {
    pub fn new(duration: f32) -> Self {
        Self {
            timer: Timer::from_seconds(duration, TimerMode::Once),
        }
    }
}

fn destroy_after_secs(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut DestroyAfterSecs)>,
) {
    for (entity, mut destroy_after_secs) in query.iter_mut() {
        destroy_after_secs.timer.tick(time.delta());
        if destroy_after_secs.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}

#[derive(Component, Debug)]
pub struct Spin {
    pub rotation_axis: Vec3,
    pub rotation_speed: f32,
}

fn spin(mut query: Query<(&Spin, &mut Transform)>, time: Res<Time>) {
    for (spinner, mut transform) in query.iter_mut() {
        transform.rotate(Quat::from_axis_angle(
            spinner.rotation_axis,
            time.delta_seconds() * spinner.rotation_speed,
        ));
    }
}
