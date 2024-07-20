//! Handle player input and translate it into movement.

use bevy::{prelude::*, window::PrimaryWindow};

use crate::AppSet;

use super::spawn::player::PlayerTurret;

pub(super) fn plugin(app: &mut App) {
    // Record directional input as movement controls.
    app.register_type::<MovementController>();
    app.add_systems(
        Update,
        record_movement_controller.in_set(AppSet::RecordInput),
    );

    // Apply movement based on controls.
    app.register_type::<Velocity>();
    app.add_systems(
        Update,
        (
            (update_velocity, apply_velocity).chain(),
            rotate_towards_mouse,
        )
            .in_set(AppSet::Update),
    );
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct MovementController {
    thrust_multiplier: f32,
    friction: f32,
    rotation_speed: f32,
    velocity_limit: f32,
    mouse_world_pos: Vec2,
    thrust: Vec2,
    shoot: bool,
    interact: bool,
}
impl MovementController {
    pub fn new(
        thrust_multiplier: f32,
        friction: f32,
        rotation_speed: f32,
        velocity_limit: f32,
    ) -> Self {
        Self {
            thrust_multiplier,
            friction,
            rotation_speed,
            velocity_limit,
            ..Default::default()
        }
    }
}

fn record_movement_controller(
    input: Res<ButtonInput<KeyCode>>,
    camera: Query<(&Camera, &GlobalTransform)>,
    window: Query<&Window, With<PrimaryWindow>>,
    mut controller_query: Query<&mut MovementController>,
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

    // Collect directional input.
    let mut intent = Vec2::ZERO;
    if input.pressed(KeyCode::KeyW) || input.pressed(KeyCode::ArrowUp) {
        intent.y += 1.0;
    }
    if input.pressed(KeyCode::KeyS) || input.pressed(KeyCode::ArrowDown) {
        intent.y -= 1.0;
    }
    if input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft) {
        intent.x -= 1.0;
    }
    if input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight) {
        intent.x += 1.0;
    }

    // Normalize so that diagonal movement has the same speed as
    // horizontal and vertical movement.
    let intent = intent.normalize_or_zero();

    // Apply movement intent to controllers.
    for mut controller in &mut controller_query {
        controller.thrust = intent;
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Velocity(pub Vec2);

fn update_velocity(mut query: Query<(&MovementController, &mut Velocity)>, time: Res<Time>) {
    for (controller, mut velocity) in query.iter_mut() {
        // Apply friction.
        velocity.0 *= 1.0 - controller.friction * time.delta_seconds();

        // Apply thrust.
        velocity.0 += controller.thrust * controller.thrust_multiplier * time.delta_seconds();

        // Limit max velocity.
        if velocity.0.length() > controller.velocity_limit {
            velocity.0 = velocity.0.normalize() * controller.velocity_limit;
        }
    }
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation += Vec3::new(velocity.0.x, velocity.0.y, 0.0) * time.delta_seconds();
    }
}

fn rotate_towards_mouse(
    ship_query: Query<(&GlobalTransform, &MovementController)>,
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
