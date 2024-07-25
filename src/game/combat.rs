use bevy::{prelude::*, window::PrimaryWindow};

use crate::AppSet;

use super::spawn::player::PlayerTurret;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<CombatController>();
    app.add_systems(Update, tick_attack_timer.in_set(AppSet::TickTimers));
    app.add_systems(Update, record_combat_controller.in_set(AppSet::RecordInput));
    app.add_systems(Update, rotate_towards_mouse.in_set(AppSet::Update));
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct CombatController {
    attack_timer: Timer,
    rotation_speed: f32,
    mouse_world_pos: Vec2,
    shoot: bool,
}
impl CombatController {
    pub fn new(attack_time: f32, rotation_speed: f32) -> Self {
        Self {
            attack_timer: Timer::from_seconds(attack_time, TimerMode::Repeating),
            rotation_speed,
            mouse_world_pos: Vec2::ZERO,
            shoot: false,
        }
    }
}

fn tick_attack_timer(mut controller_query: Query<&mut CombatController>, time: Res<Time>) {
    for mut controller in &mut controller_query {
        controller.attack_timer.tick(time.delta());
    }
}

fn record_combat_controller(
    input: Res<ButtonInput<MouseButton>>,
    camera: Query<(&Camera, &GlobalTransform)>,
    window: Query<&Window, With<PrimaryWindow>>,
    mut controller_query: Query<&mut CombatController>,
) {
    // Record mouse world position.
    let (camera, camera_transform) = camera.single();
    let window = window.single();

    if let Some(ray) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
    {
        if let Some(distance) = ray.intersect_plane(Vec3::ZERO, InfinitePlane3d::new(Vec3::Z)) {
            let world_position = ray.get_point(distance);
            for mut controller in &mut controller_query {
                controller.mouse_world_pos = world_position.truncate();
            }
        }
    }

    // Record attack input.
    for mut controller in &mut controller_query {
        controller.shoot = input.pressed(MouseButton::Left);
    }
}

fn rotate_towards_mouse(
    ship_query: Query<(&GlobalTransform, &CombatController)>,
    mut turret_query: Query<&mut Transform, With<PlayerTurret>>,
    time: Res<Time>,
) {
    for (global_transform, controller) in ship_query.iter() {
        let mut transform = turret_query
            .get_single_mut()
            .expect("Expected single turret");
        let target_direction = (controller.mouse_world_pos
            - global_transform.translation().truncate())
        .normalize_or_zero()
        .extend(0.0);
        let current_direction = transform.rotation * Vec3::Y;
        let rotation = current_direction.angle_between(target_direction);
        if rotation > 0.0 {
            let rotation_direction = current_direction.cross(target_direction).z;
            let rotation_direction = if rotation_direction > 0.0 { 1.0 } else { -1.0 };
            transform.rotate(Quat::from_rotation_z(
                rotation_direction * controller.rotation_speed * rotation * time.delta_seconds(),
            ));
        }
    }
}
